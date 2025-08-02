# wu-clib-demo

Demo for the `wu-clib-rs` module. Check the live version [here][1].

This is a simple example of a Rust `wasm32-unknown-unknown` application with C and C++ dependencies. Particularly it uses Dear ImGui, a C++ library, and IJG's `libjpeg`, a C library.

It also depends on `easy-imgui` and `glow` that available for `wasm32-unknown-unknown` but not for any other wasm32 target.

# How to build

First clone the project to your local system. Don't forget the submodules!

```sh
$ git clone --recurse-submodules https://github.com/rodrigorc/wu-clib-demo
```

Then go into that directory and build the project, it uses `xtask`[2]:

```sh
$ cd wu-clib-demo
$ cargo xtask pack
```

If it fails saying that it can't find `wc-clib-rs` maybe you forgot to clone the submodules. If so you can fix it with:

```sh
$ git submodule update --init --recursive .
```

If you get weird long compiler or linker errors, maybe you don't have the `wasm32-unknown-unknown` target installed. If so try this:

```sh
$ rustup target install wasm32-unknown-unknown
```

Or maybe you don't have `wasm-pack`. Try this:

```sh
$ cargo install wasm-pack
```

Or maybe you don't have the Clang wasi target installed. This doesn't use `wasi` but currently `clang` doesn't ship a proper `wasm32-unknown-unknown` builtin library, so I'm highjacking `wasis`'s. Check that this command returns the name of an existing file:

```sh
$ clang -print-libgcc-file-name --target=wasm32-unknown-wasi
```
For example, in ArchLinux it is in the package `wasi-compiler-rt`.

Finally you can serve the page with:

```sh
$ python -m http.server
```

And open the local URL: http://localhost:8000/

[1]: https://rodrigorc.github.io/wu-clib-demo/
[2]: https://github.com/matklad/cargo-xtask
