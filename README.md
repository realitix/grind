# grind-kernel

Grind kernel is the heart of grind.


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
