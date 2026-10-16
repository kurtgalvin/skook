# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

skook is an embeddable stream processing engine written in Rust. It runs inside the host application and should scale out when needed. **The primary use case is Python code calling the engine**, so the Python API is the main product surface, not an afterthought.

The project is at the scaffolding stage: the only function is a placeholder `hello`, which proves the Rust → Python path works.

## Commands

```sh
uv sync                                   # create .venv, build the Rust extension, install dev deps
uv run pytest                             # Python tests; rebuilds the extension first if Rust changed
uv run pytest tests/test_hello.py::test_hello_name   # single Python test
cargo test --workspace                    # Rust tests
cargo test -p skook-core hello_greets_by_name        # single Rust test
cargo clippy --workspace --all-targets    # lints
cargo fmt --all                           # format
uv run maturin develop --uv --release     # optimized build, for benchmarking only
```

- Any `cargo` command needs `.venv` to exist (run `uv sync` once). `.cargo/config.toml` sets `PYO3_PYTHON` to `.venv/bin/python`. Without it, PyO3 picks up the macOS system Python 3.9 and the build fails, because of the `abi3-py310` minimum.
- Rust rebuilds triggered by `uv run` are debug builds. Don't judge performance from them.

## Architecture

```
crates/skook-core     pure Rust engine, no Python dependency: all engine logic goes here
crates/skook-python   thin PyO3 layer (cdylib) compiled to the native module skook._skook
python/skook/         the Python package users import; re-exports from skook._skook
```

- Keep `skook-core` free of `pyo3`. `skook-python` should only convert types and delegate to `skook-core`. Rule of thumb: logic goes in core; anything touching `pyo3` goes in python.
- The native module name `_skook` is coupled in three places that must agree: `[lib] name` in `crates/skook-python/Cargo.toml`, the `#[pyo3::pymodule] mod _skook` in its `lib.rs`, and `module-name = "skook._skook"` in `pyproject.toml`.
- Exposing a new function or class to Python touches four places: implement it in `skook-core`, wrap it in `crates/skook-python/src/lib.rs`, re-export it from `python/skook/__init__.py` (and `__all__`), and add its signature to the hand-written stub `python/skook/_skook.pyi`.
- `pyproject.toml`'s `[tool.uv] cache-keys` makes uv rebuild the extension whenever `.rs` or `Cargo.toml` files change. If you add Rust sources outside `crates/`, update that list.
- Version, edition and license are set once in the root `Cargo.toml` under `[workspace.package]` and inherited by the crates. The Python package version is read from Cargo (`dynamic = ["version"]`).

## Design direction

These are the decisions already discussed for the engine:
- Moving data across the Python/Rust boundary is the main cost. Prefer columnar batches via Apache Arrow (`arrow-rs` / `pyarrow`) over per-row Python object conversion.
- Python callbacks (for example `map(lambda ...)`) need the GIL and are the slow path. Favor built-in operators implemented in Rust, in the style of Polars' expression API.
- If the engine runs on an async runtime, it must run on its own threads and not block the Python caller.

## Conventions

- PyO3 is 0.30.x, which is newer than most training data. It uses the `#[pymodule] mod name { ... }` syntax and no longer needs the `extension-module` feature. Check the PyO3 docs or the crate source rather than relying on memory of older versions.
- The `abi3-py310` feature builds one wheel per platform for CPython 3.10 and newer. It restricts PyO3 to the stable Python ABI, so some PyO3 APIs aren't available.
- License: dual MIT OR Apache-2.0, with no CLA. New crates should use `license.workspace = true`.
