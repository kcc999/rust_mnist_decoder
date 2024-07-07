#!/bin/sh

# Define URLs
train_images="https://ossci-datasets.s3.amazonaws.com/mnist/train-images-idx3-ubyte.gz"
train_labels="https://ossci-datasets.s3.amazonaws.com/mnist/train-labels-idx1-ubyte.gz"

# Create MNIST/raw directory if it doesn't exist
mkdir -p MNIST/raw

# Change to the MNIST/raw directory
cd MNIST/raw

# Download files
curl "$train_images" -o train-images-idx3-ubyte.gz
curl "$train_labels" -o train-labels-idx1-ubyte.gz

# Extract files
gunzip -f train-images-idx3-ubyte.gz
gunzip -f train-labels-idx1-ubyte.gz

echo "Files downloaded and extracted in MNIST/raw directory"
cd ../..