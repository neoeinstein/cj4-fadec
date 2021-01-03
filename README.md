# Working Title CJ4 Web Assembly module

## Orientation

### Integration with MSFS

The `gauge-sys` and `simconnect-sys` crates do the heavy lifting of integrating
with the low-level Gauge and SimConnect APIs exposed by MSFS. Unsafe FFI code
is primarily held in these libraries and the actual WASM module itself.

### Aviation Mathematics

This repository currently includes experimental work on an
<strong>Av</strong>iation <strong>Math</strong>ematics crate. This crate is
likely to be spun off as a separate crate at a later point. Right now it is used
primarily to provide some strong typing adding context to different types of
altitudes.

### Working Title

The `wt_*` modules are the new modules that provide functionality for the CJ4
and core functions. `wt_cj4` is for values that are specific to the CJ4, while
`wt_systems` provides functionality that can be reused across multiple aircraft.

### WASM module

The WASM module itself is exposed at the root of the namespace. Code for the
module is found in the `src` directory. Because the module currently utilizes
the Gauge API, the entry point is the `FdGauge_gauge_callback` function in
`lib.rs`. This receives events from the legacy Gauge API and then polls the
SimConnect API for updates prior to processing updates to the FADEC function.

## Development

### Compilation

Using `cargo-wasi` is recommended for compiling this project. You can get it by
running the following command:

```sh
cargo install cargo-wasi
```

Once the `cargo-wasi` tool is installed, the module can be compiled by running:

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

Embedded tests are run with:

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
