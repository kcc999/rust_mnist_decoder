use std::{fs::File, io::{BufReader, Cursor, Read, Error, ErrorKind}, iter};

struct LabelsContainer {
  magic_number: u32,
  count: u32,
  data: Vec<u8>
}

struct ImagesContainer {
  magic_number: u32,
  count: u32,
  rows: u32,
  cols: u32,
  data: Vec<u8>
}

pub struct Image {
  pixels: Vec<u8>,
  label: u8
}

fn decode_labels(path: String) -> Result<LabelsContainer, Error> {
  let content = match std::fs::read(path) {
    Ok(c) => c,
    Err(e) => return Err(e)
  };
  
  if content.len() < 8 {
    return Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid file size"));
  }

  let m: [u8; 4] = content[0..4].try_into().unwrap();
  let s: [u8; 4] = content[4..8].try_into().unwrap();

  let magic_number = u32::from_be_bytes(m);
  let count = u32::from_be_bytes(s);
  let data = &content[8..];
  
  return Ok(LabelsContainer{
    magic_number,
    count,
    data: data.to_vec()
  });
}

fn decode_images(path: String) -> Result<ImagesContainer, Error> {
  let content = match std::fs::read(path) {
    Ok(c) => c,
    Err(e) => return Err(e)
  };

  if content.len() < 17 {
     return Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid file size")); 
  }

  let m: [u8; 4] = content[0..4].try_into().unwrap();
  let s: [u8; 4] = content[4..8].try_into().unwrap();
  let r: [u8; 4] = content[8..12].try_into().unwrap();
  let c: [u8; 4] = content[12..16].try_into().unwrap();

  let magic_number = u32::from_be_bytes(m);
  let count = u32::from_be_bytes(s);
  let rows = u32::from_be_bytes(r);
  let cols = u32::from_be_bytes(c);
  let data = &content[16..];

  return Ok(ImagesContainer{
    magic_number,
    count,
    rows,
    cols,
    data: data.to_vec()
  });
}

fn extract_images_from_container(image_container: &ImagesContainer,
                                     labels_container: &LabelsContainer) -> Vec<Image> {
  let mut images : Vec<Vec<u8>> = Vec::with_capacity(image_container.count as usize);
  for i in 0..image_container.count {
    let start = (i * image_container.rows * image_container.cols) as usize;
    let end = ((i + 1) * image_container.rows * image_container.cols) as usize;
    images.push(image_container.data[start..end].to_vec());
  }

  let mut data : Vec<Image> = Vec::new();

  for i in 0..labels_container.count {
    let label = labels_container.data[i as usize];
    let pixels = images[i as usize].clone();
    data.push(Image{pixels, label});
  }
  return data;
}

pub fn read_mnist(data_path: String) -> Result<Vec<Image>, Error> {
  let train_images_path = format!("{}/train-images-idx3-ubyte", data_path);
  let label_images_path = format!("{}/train-labels-idx1-ubyte", data_path);

  let images = match decode_images(train_images_path) {
    Ok(decoded) => decoded,
    Err(e) => return Err(e),
  };

  let labels = match decode_labels(label_images_path) {
    Ok(decoded) => decoded,
    Err(e) => return Err(e),
  };

  return Ok(extract_images_from_container(&images, &labels));
}

#[test]
fn test_label_decode() {
  const LABELS_MAGIC_NUMBER: u32 = 2049;
  let labels = decode_labels("MNIST/raw/train-labels-idx1-ubyte".to_string());
  assert!(labels.is_ok());
  assert_eq!(labels.unwrap().magic_number, LABELS_MAGIC_NUMBER);
}

#[test]
fn test_image_decode() {
  const IMAGES_MAGIC_NUMBER: u32 = 2051;
  let images = decode_images("MNIST/raw/train-images-idx3-ubyte".to_string()).unwrap();

  let labels = decode_labels("MNIST/raw/train-labels-idx1-ubyte".to_string()).unwrap();
  assert_eq!(images.magic_number, IMAGES_MAGIC_NUMBER);

  let extracted = extract_images_from_container(&images, &labels);
  assert_eq!(extracted.len(), 60000);
}

#[test]
fn test_non_existing_file() {
  let labels = decode_labels("MNIST/raw/non-existing-file".to_string());
  assert!(labels.is_err());
}