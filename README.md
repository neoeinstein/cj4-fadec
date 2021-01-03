# Working Title CJ4 Web Assembly module

Includes experimental work on an <strong>Av</strong>iation
<strong>Math</strong>ematics crate, which is likely to be spun off as a separate
crate at a later point.

## Development

### Compilation

Using `cargo-wasi` is recommended for compiling this project. You can get it by
running the following command:

```sh
cargo install cargo-wasi
```

As a result, running `wasm-opt` has been disabled, which means we miss out on a
few optimizations that would be available from the tool.

Compilation can be done by running:

```sh
cargo wasi build -p wt_cj4_wasm
```

Adding `--release` will build the module with full optimizations.
Post-processing optimizations provided by `wasm-opt` are disabled on all builds.
Unfortunately, the `wasm-opt` tool used by `cargo-wasi` produces optimized WASM
files that don't behave well within the MSFS runtime.

The compiled WASM module can be found in the
`target/wasm32-wasi/{debug,release}` folder as appropriate.

### Testing

Tests can be run with:

```sh
cargo test --all
```

### Lints and Formatting

Lints are checked using:

```sh
cargo clippy --all
```

Formatting is done with:

```sh
cargo fmt --all
```

CI builds require all code to be properly formatted and build with out any
Clippy warnings.
