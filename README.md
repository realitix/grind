# grind-kernel

Grind kernel is the heart of grind.


## How to use

First compile:

```
cargo build
```

Then create the symlinks:

```
./create_symlinks.sh
```

All libraries are in `target/debug/`, you can use them with `LD_LIBRARY_PATH`
