#!/usr/bin/env bash

pkg-config --exists libudev

if [ $? -eq 0 ]; then
    echo "libudev already installed [v$(pkg-config --modversion libudev)]"
    exit 0
else
    echo "Installing latest libudev-dev..."
fi

sudo apt-get -y update

# Build tools
sudo apt-get install -y build-essential cmake

# libudev-dev
sudo apt-get install libudev-dev
