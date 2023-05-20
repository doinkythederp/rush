# Authoring plugins

Make sure you have Cargo and Rustup installed before continuing with this guide.

## Environment setup

Plugin development requires you to have the `wasm32-wasi` Rust target installed. The easiest way to do that is with Rustup:

```sh
rustup target add wasm32-wasi
```

## Creating your plugin

Rush plugins follow a normal Rust project structure; use Cargo to create a new library crate.

```
cargo new --lib my-plugin
```

In your new project's `Cargo.toml`, update it to build as a `cdylib`, and install `rush-pdk` as a dependency.

```diff
  [package]
  name = "my-plugin"
  version = "0.1.0"
  edition = "2021"
  
+ [lib]
+ crate_type = ["cdylib"]
+
+ [dependencies]
+ rush-pdk = { git = "https://github.com/doinkythederp/rush", branch = "plugins" }
```

Rush plugins can create "Hooks" (i.e. event handlers) which are functions called by the shell when certain things happen.
