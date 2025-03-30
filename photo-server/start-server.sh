#!/usr/bin/bash

# Define variables
CONTAINER_NAME="rust-photo-server"
HOST_BINARY_DIR="/volume1/home/admin/rust-binaries/photo-server"
CONTAINER_BINARY_DIR="/photo-server"
BINARY_NAME="photo-server"
DOCKER_IMAGE="rust:latest"

# Run the Docker container
docker run --rm -it \
  --name $CONTAINER_NAME \
  -v $HOST_BINARY_DIR:$CONTAINER_BINARY_DIR \
  $DOCKER_IMAGE \
  $CONTAINER_BINARY_DIR/$BINARY_NAME


# docker run --rm -it --privileged \
#   --name rust-photo-server \
#   -v /volume1/home/admin/rust-binaries:/rust-binaries \
#   rust:latest \
#   /rust-binaries/photo-server
