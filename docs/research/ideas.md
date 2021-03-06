ARCHI LOGICIEL

OpenGLES3 INTERFACE -> OPENGL DRIVER ES3 <-> KERNEL <-> VULKAN DRIVER -> VULKAN

Au niveau des dépendances des drivers OpenGL, on a:

OpenGLBase

- OpenGL2.1
- OpenGL3
  - OpenGL3.0
  - OpenGL3.1
  - OpenGL3.2
  - OpenGL3.3
- OpenGL 4
  - OpenGL 4.0
  - OpenGL 4.1
  - OpenGL 4.2
  - OpenGL 4.3
  - OpenGL 4.4
  - OpenGL 4.5
- OpenGLES2
- OpenGLES 3
    - OpenGLES 3.0
    - OpenGLES 3.1
    
Dans chaque classe, on implémente toutes les fonctions communes aux classes sous jacentes
Il peut y avoir d'autre classes intermédiaire (par exemple regroupé les version 4.3,4.4,4.5 si besoin)

OPENGL CONTEXT
--------------

J'avais peur que ce soit complètement lié au système.
Alors en effet, selon l'OS, ce ne sont pas les même fonctions qui sont appelées en fonction du serveur d'affichage.
Contrairement à Vulkan qui a uniformisé tout ca.
MAIS la bonne nouvelle, c'est que cela reste une dll ou .so habituelle, c'est une librarie partagé classique et donc
cela peut être remplacé au niveau système!

Au niveau install:

-> Soit le projet embarque la librairie statique et link avec à la compilation
-> Soit le projet ne connait pas cette librairie, et on l'installe au niveau système
   |-> Au niveau système, le driver OpenGL ne sera plus accessible et il faudra donc implémenter toutes les version d'OPENGL
   |-> Ca peut se vendre aux fabricants de pilotes
   
   

   
Idée d'implémentation:
------------------------
On créé un module pour chaque interface EGL/WGL/XGL
Ces modules sont importés à la compilation avec le mécanisme des features
Prenons pour exmple EGL.

Le module EGL va créer une instance du GrindKernel et sera responsable de son cycle de vie.
Le module EGL va ensuite créer une instance du driver OpenGL (dans la bonne version) et sera aussi responsable de son cycle de vie.
Le module EGL va passer le GrindKernel à l'OpenGLDriver

EGL et OpenGLDriver appelleront des fonctions du GrindKernel.
Les fonctions exposées par le GrindKernel seront similaires aux fonction Vulkan

Le GrindKernel prend un argument lors de son initialisation, le DriverOut Trait
EGL initialise le Driver Vulkan (implémentant DriverOut) et le passe au GrindKernel
Le GrindKernel prend la responsabilité du DriverOut

Pour charger les fonctions, chaque Driver OpenGL contient une hashmap [nom_de_la_fonction => address]
Lors de la fonction getProcAddr, on récupère l'adresse de la fonction via la hashmap du driver
