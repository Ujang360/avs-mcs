#!/usr/bin/env bash

OPENCV_VERSION="3.4.9"
OPENCV_CONTRIB="NO"

pkg-config --exists opencv

if [ $? -eq 0 ]; then
    installed_version=
    echo "OpenCV already installed [v$(pkg-config --modversion opencv)]"
    exit 0
else
    echo "Installing OpenCV version $OPENCV_VERSION..."
fi

sudo apt-get -y update

# Build tools
sudo apt-get install -y build-essential cmake

# GUI (if you want GTK, change 'qt5-default' to 'libgtkglext1-dev' and remove '-DWITH_QT=ON')
sudo apt-get install -y qt5-default libvtk6-dev

# Media I/O
sudo apt-get install -y zlib1g-dev libjpeg-dev libwebp-dev libpng-dev libtiff5-dev libjasper-dev \
                        libopenexr-dev libgdal-dev

# Video I/O
sudo apt-get install -y libdc1394-22-dev libavcodec-dev libavformat-dev libswscale-dev \
                        libtheora-dev libvorbis-dev libxvidcore-dev libx264-dev yasm \
                        libopencore-amrnb-dev libopencore-amrwb-dev libv4l-dev libxine2-dev

# Parallelism and linear algebra libraries
sudo apt-get install -y libtbb-dev libeigen3-dev

# Utils
sudo apt-get install -y doxygen unzip wget

wget https://github.com/opencv/opencv/archive/${OPENCV_VERSION}.zip
unzip ${OPENCV_VERSION}.zip && rm ${OPENCV_VERSION}.zip
mv opencv-${OPENCV_VERSION} OpenCV

if [ $OPENCV_CONTRIB = "YES" ]; then
    wget https://github.com/opencv/opencv_contrib/archive/${OPENCV_VERSION}.zip
    unzip ${OPENCV_VERSION}.zip && rm ${OPENCV_VERSION}.zip
    mv opencv_contrib-${OPENCV_VERSION} opencv_contrib
    mv opencv_contrib OpenCV
fi

cd OpenCV && mkdir build && cd build

if [ $OPENCV_CONTRIB = "NO" ]; then
    cmake -DWITH_QT=ON -DWITH_OPENGL=ON -DFORCE_VTK=ON -DWITH_TBB=ON -DWITH_GDAL=ON \
          -DWITH_XINE=ON -DENABLE_PRECOMPILED_HEADERS=OFF ..
fi

if [ $OPENCV_CONTRIB = "YES" ]; then
    cmake -DWITH_QT=ON -DWITH_OPENGL=ON -DFORCE_VTK=ON -DWITH_TBB=ON -DWITH_GDAL=ON \
          -DWITH_XINE=ON -DENABLE_PRECOMPILED_HEADERS=OFF \
          -DOPENCV_EXTRA_MODULES_PATH=../opencv_contrib/modules ..
fi

make -j8
sudo make install
sudo ldconfig
