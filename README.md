# rust_mnist_decoder

A rust crate used to decode the mnist dataset.


The API is a single function which returns a vector of images, it's implementation expects the files to be matching [Yann LeCun's MNIST database](http://yann.lecun.com/exdb/mnist/).


```rust
pub fn read_mnist(data_path: String) -> Vec<Image>;
```

```rust
pub struct Image {
  pixels: Vec<u8>,
  label: u8
}
```
