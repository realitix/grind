# Comment fonctionne le context OPENGL

On commence par Wayland, et on va décortiquer la SDL pour comprendre ce qui se passe.
Voici comment fonctionne la SDL:


- On commence par intialiser avec SDL_video:SDL_VideoInit
   -> Cette fonction va parcourir le tableau de bootstrap (serveur d'affichage) disponible SDL_video:60, du type VideoBootstrap
          -> VideoBootstrap défini la fonction permettant de tester, et la fonction permettant de créer la fenetre
             -> Sous wayland, Wayland_bootstrap est sélectionné, wayland/SDL_waylandvideo:205.
                -> Dans SDL_VideoInit, _this devient le retour de la fonction wayland/SDL_waylandvideo:Wayland_CreateDevice et implémente pas mal de fonction
                   -> Lance la fonction Wayland_VideoInit. Cela sert a préparer la fenetre, aucun rapport avec OpenGL pour l'instant
                      -> La fonction CreateWindowFramebuffer est déterminé dans ShouldUseTextureFramebuffer mais je ne comprends pas encore l'intéret
                         
                         - On créé la fenetre avec SDL_CreateWindow avec le flag SDL_WINDOW_OPENGL
                            -> A savoir, pour Wayland, c'est le nouveau système EGL qui permet de créer un context OpenGL
                               -> la fonction Wayland_GLES_LoadLibrary est appelée avec path à NULL
                                  -> La fonction Wayland_GLES_CreateContext est appelée 
                                     
                                      Je commence a comprendre
                                       
                                        Pour faire l'interfacage entre LE context Opengl et la fenetre,
                                        On utilise historiquement WGL et GLX mais une nouvelle API est plus générique:
                                        EGL
                                        EGL est d'ailleurs la seule disponible pour wayland

                                        Il existe 4 moyens de créer un context OpenGL:

                                        AGL sur OSX / GLX sur X11 / WGL sur Windob
                                        EGL Sur les 3

