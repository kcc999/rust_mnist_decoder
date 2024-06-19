use std::{fmt::Error, fs::File, io::{BufReader, Cursor, Read}, iter};

struct LabelsContainer {
  magic_nunber: u32,
  items_count: u32,
  data: Vec<u8>
}

struct ImagesContainer {
  magic_nunber: u32,
  items_count: u32,
  rows: u32,
  cols: u32,
  data: Vec<u8>
}

pub struct Image {
  pixels: Vec<u8>,
  label: u8
}

fn decode_labels(path: String) -> Result<LabelsContainer, Error> {
  let mut content = std::fs::read(path).unwrap();

  let m: [u8; 4] = content[0..4].try_into().unwrap();
  let s: [u8; 4] = content[4..8].try_into().unwrap();

  let magic_number = u32::from_be_bytes(m);
  let count = u32::from_be_bytes(s);
  let data = &content[8..];
  
  return Ok(LabelsContainer{
    magic_nunber: magic_number,
    items_count: count,
    data: data.to_vec()
  });
}

fn decode_images(path: String) -> Result<ImagesContainer, Error> {
  let mut content = std::fs::read(path).unwrap();

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
    magic_nunber: magic_number,
    items_count: count,
    rows: rows,
    cols: cols,
    data: data.to_vec()
  });
}

fn extract_images_from_container(image_container: &ImagesContainer,
                                     labels_contaienr: &LabelsContainer) -> Vec<Image> {
  let mut images : Vec<Vec<u8>> = Vec::with_capacity(image_container.items_count as usize);
  for i in 0..image_container.items_count {
    let start = (i * image_container.rows * image_container.cols) as usize;
    let end = ((i + 1) * image_container.rows * image_container.cols) as usize;
    images.push(image_container.data[start..end].to_vec());
  }

  let mut data : Vec<Image> = Vec::new();

  for i in 0..labels_contaienr.items_count {
    let label = labels_contaienr.data[i as usize];
    let pixels = images[i as usize].clone();
    data.push(Image{pixels: pixels, label: label});
  }
  return data;
}

pub fn read_mnist(data_path: String) -> Vec<Image> {
  let train_images_path = format!("{}/train-images-idx3-ubyte", data_path);
  let label_images_path = format!("{}/train-labels-idx1-ubyte", data_path);
  let images = decode_images(train_images_path).unwrap();
  let labels = decode_labels(label_images_path.clone()).unwrap();
  return extract_images_from_container(&images, &labels);
}

#[test]
fn test_label_decode() {
  const LABELS_MAGIC_NUMBER: u32 = 2049;
  let labels = decode_labels("MNIST/raw/train-labels-idx1-ubyte".to_string());
  assert!(labels.is_ok());
  assert_eq!(labels.unwrap().magic_nunber, LABELS_MAGIC_NUMBER);
}

#[test]
fn test_image_decode() {
  const IMAGES_MAGIC_NUMBER: u32 = 2051;
  let images = decode_images("MNIST/raw/train-images-idx3-ubyte".to_string()).unwrap();

  let labels = decode_labels("MNIST/raw/train-labels-idx1-ubyte".to_string()).unwrap();
  assert_eq!(images.magic_nunber, IMAGES_MAGIC_NUMBER);

  let extracted = extract_images_from_container(&images, &labels);
  assert_eq!(extracted.len(), 60000);
}