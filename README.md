# skook
An embeddable stream processing engine written in Rust. Runs inside your app and scales out when you need it.

skook, short for skookum: strong, solid, built right.

## Development

skook is a Rust engine (`crates/skook-core`) exposed to Python through
[PyO3](https://pyo3.rs) bindings (`crates/skook-python`), built with
[maturin](https://www.maturin.rs). You need [Rust](https://rustup.rs) and
[uv](https://docs.astral.sh/uv/).

```sh
uv sync                  # create .venv, build the Rust extension, install dev tools
uv run pytest            # Python tests (rebuilds automatically if Rust changed)
cargo test --workspace   # Rust tests (run `uv sync` first)
cargo clippy --workspace # Rust lints
```

For benchmarking, build an optimized extension with
`uv run maturin develop --uv --release`.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
