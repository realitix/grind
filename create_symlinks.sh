#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

cd $DIR/target/debug
ln -s libgrindegl.so libEGL.so.1
ln -s libgrindopengles2.so libGLESv2.so.2
ln -s libgrindwaylandegl.so libwayland-egl.so.1

# Strange linking error when launching tests
cd $DIR/target/debug/deps
ln -s libgrindegl.so libEGL.so
ln -s libgrindopengles2.so libGLESv2.so
ln -s libgrindwaylandegl.so libwayland-egl.so

# Wayland
ln -s /lib/x86_64-linux-gnu/libwayland-client.so.0 libwayland-client.so
ln -s /lib/x86_64-linux-gnu/libwayland-cursor.so.0 libwayland-cursor.so

