from os import path
from cffi import FFI


HERE = path.dirname(path.realpath(__file__))

# Load library
ffi = FFI()
ffi.cdef(open(path.join(HERE, 'egl.h')).read())
dl = ffi.dlopen('libEGL.so')


# Define constants
EGL_DEFAULT_DISPLAY = ffi.cast('EGLNativeDisplayType', 0)


# Define functions
def eglGetDisplay(native_display):
    return dl.eglGetDisplay(native_display)
