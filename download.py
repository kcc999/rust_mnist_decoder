import torch
import torchvision

train_data = torchvision.datasets.MNIST("./", train=True, download=True)
