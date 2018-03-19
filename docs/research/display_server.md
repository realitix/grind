# Display server

> Linux, why are you always so complicated !


## Wayland on Linux

We need to get: `wl_display` and `wl_surface`

- `wl_display`: `eglGetDisplay`
- `wl_surface`:
   - get `wl_egl_window` from `eglCreateWindowSurface`
   - retrieve the surface from it (need to expose the `wl_egl_window_create` function

On Wayland, we need to provide 3 libs:

1. libwayland-egl1-mesa:amd64: /usr/lib/x86_64-linux-gnu/libwayland-egl.so.1
2. libegl1-mesa:amd64: /usr/lib/x86_64-linux-gnu/mesa-egl/libEGL.so.1.0.0
3. libgles2-mesa:amd64: /usr/lib/x86_64-linux-gnu/mesa-egl/libGLESv2.so.2.0.0

1 -> Provide wl_egl_window_create function which gives us the wl_surface
2 -> Provide the egl functions
3 -> Provide the gles2 function


## XCB on Linux

See: https://github.com/chadversary/gl-examples/blob/master/src/x11-egl/create-x-egl-gl-surface.cpp

On XCB, we need `xcb_connection_t` and `xcb_window_t` :

- `xcb_window_t` from `eglCreateWindowSurface`.`
- `xcb_connection_t`
  -> Get the `Display` from `eglGetDisplay`
  and then: `xcb_connection_t *connection = XGetXCBConnection(display);`


## Windows

On Windows, we need `HINSTANCE` and `HWDN`, how to get them ?

We retrieve the `HWDN` with `eglCreateWindowSurface(eglDisplay, windowConfig, hwnd, surfaceAttributes)`.
To get the `HINSTANCE`, we use the following code:

```python
_ffi.cdef('long __stdcall GetWindowLongA(void* hWnd, int nIndex);')
_lib = _ffi.dlopen('User32.dll')
return _lib.GetWindowLongA(_ffi.cast('void*', hWnd), -6)
```

This code is in Python but we can adapt it.

So for Windows, it's simple.


## Android

On Android, we need only `ANativeWindow`:

`ANativeWindow` is passed to the `eglCreateWindowSurface` function so easy !


## Off screen rendering

With EGL, you can enable Off-screen rendering like that:

- Call `eglGetDisplay` with `EGL_NO_DISPLAY`
- Instead of calling `eglCreateWindowSurface`, call `eglCreatePbufferSurface`
