//! Python bindings for skook. Keep this layer thin: convert types and
//! delegate to `skook-core`.

/// The native extension module, imported in Python as `skook._skook`.
#[pyo3::pymodule]
mod _skook {
    use pyo3::prelude::*;

    /// Returns a greeting from the Rust engine.
    #[pyfunction]
    #[pyo3(signature = (name = "world"))]
    fn hello(name: &str) -> String {
        skook_core::hello(name)
    }
}
