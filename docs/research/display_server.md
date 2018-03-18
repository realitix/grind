# Display server

## Wayland

On Wayland, we need to provide 3 libs:

1. libwayland-egl1-mesa:amd64: /usr/lib/x86_64-linux-gnu/libwayland-egl.so.1
2. libegl1-mesa:amd64: /usr/lib/x86_64-linux-gnu/mesa-egl/libEGL.so.1.0.0
3. libgles2-mesa:amd64: /usr/lib/x86_64-linux-gnu/mesa-egl/libGLESv2.so.2.0.0

1 -> Provide wl_egl_window_create function which gives us the wl_surface
2 -> Provide the egl functions
3 -> Provide the gles2 function

