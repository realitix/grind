from wrapper import egl


def test_yolo():
    display = egl.eglGetDisplay(egl.EGL_DEFAULT_DISPLAY)
