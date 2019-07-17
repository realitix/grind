# Grind

Grind provides OpenGL libraries. Behind the scene, Vulkan is used.
This module is not ready, I'm working on it.

## How to Launch tests

First compile:

```
cargo build
```

Then create the symlinks:

```
./create_symlinks.sh
```

All libraries are in `target/debug/`, you can use them with `LD_LIBRARY_PATH`

Then you can launch tests:

```
cargo test
```

If you want to launch the tests directly with your OpenGL driver (to write a new test for example), remove the simlinks.
