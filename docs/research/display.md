# Let's try to understand what is an EGLDisplay

To do that, we'll look at the Angle source code.
Let's follow the call stack when we call eglGetDisplay on Linux with Vulkan backend:

## eglGetDisplay with EGL_DEFAULT_DISPLAY

1 src/libEGL/libEGL.cpp:75:EGLDisplay EGLAPIENTRY eglGetDisplay(EGLNativeDisplayType display_id)
2 src/libGLESv2/entry_points_egl.cpp:67:EGLDisplay EGLAPIENTRY GetDisplay(EGLNativeDisplayType display_id)
3 src/libANGLE/Display.cpp:285:Display *Display::GetDisplayFromNativeDisplay(EGLNativeDisplayType nativeDisplay,
4 src/libANGLE/Display.cpp:134:rx::DisplayImpl *CreateDisplayFromAttribs(const AttributeMap &attribMap, const DisplayState &state)

I stop here to work on the POC.
