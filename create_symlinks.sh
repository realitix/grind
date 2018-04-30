#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

cd $DIR/target/debug
ln -s libgrindegl.so libEGL.so.1
ln -s libgrindopengles2.so libGLESv2.so.2
ln -s libgrindwaylandegl.so libwayland-egl.so.1
