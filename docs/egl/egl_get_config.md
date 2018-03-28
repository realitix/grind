# eglGetConfig

Cette page va présenter un mapping des paramètres de config EGL vers Vulkan

Dans Vulkan, les structs suivantes contiennent beaucoup d'infos:
VkFormat, VkPhysicalDeviceFeatures, VkPhysicalDeviceLimits, VkFormatProperties

EGL_ALPHA_SIZE -> VkFormat
EGL_ALPHA_MASK_SIZE -> Je ne sais pas
EGL_BIND_TO_TEXTURE_RGB -> EGL_TRUE
EGL_BIND_TO_TEXTURE_RGBA -> EGL_TRUE
EGL_BLUE_SIZE -> VkFormat
EGL_BUFFER_SIZE -> RGBA Size
EGL_COLOR_BUFFER_TYPE -> A determiner
EGL_CONFIG_CAVEAT -> toujours EGL_NONE
EGL_CONFIG_ID -> Générer un id pour la config
EGL_CONFORMANT -> toujours EGL_OPENGL_ES2_BIT
EGL_DEPTH_SIZE -> C'est moi qui le défini pendant la création du buffer depth dans la swapchain
EGL_GREEN_SIZE -> VkFormat
EGL_LEVEL -> return 0
EGL_LUMINANCE_SIZE -> 
EGL_MAX_PBUFFER_WIDTH 
EGL_MAX_PBUFFER_HEIGHT 
EGL_MAX_PBUFFER_PIXELS 
EGL_MAX_SWAP_INTERVAL 
EGL_MIN_SWAP_INTERVAL 
EGL_NATIVE_RENDERABLE 
EGL_NATIVE_VISUAL_ID 
EGL_NATIVE_VISUAL_TYPE 
EGL_RED_SIZE 
EGL_RENDERABLE_TYPE 
EGL_SAMPLE_BUFFERS 
EGL_SAMPLES 
EGL_STENCIL_SIZE 
EGL_SURFACE_TYPE 
EGL_TRANSPARENT_TYPE 
EGL_TRANSPARENT_RED_VALUE 
EGL_TRANSPARENT_GREEN_VALUE 
EGL_TRANSPARENT_BLUE_VALUE 


Quand on créé le context, on créé 1 FrameBuffer contenant jusqu'a 4 images:
Back et Front por le double buffered, LEFt et RIGHT pour le stereo buffered
Opengl ES2 ne supporte pas le quad buffering, seulement le double buffering.
On sélectionne simple ou double buffering avec la fonction eglCreateWindowSurface


Les swapchain (ou framebuffer en offscreen) sont créés par la fonction eglCreateWindowSurface ou eglCreatePBufferSurface
Ces surfaces sont ensuites associés à un context avec eglMakeCurrent

Dans l'ordre:
1 - eglGetDisplay
2 - eglInitialize
3 - eglCreateWindowSurface or eglCreatePBufferSurface -> Création des swapchains ou framebuffer
4 - eglCreateContext -> Création du context
5 - eglMakeCurrent -> Liaison entre surface et context + context dans le thread
