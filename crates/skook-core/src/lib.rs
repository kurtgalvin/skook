//! The skook stream processing engine.
//!
//! This crate is pure Rust and knows nothing about Python; the Python
//! bindings live in `skook-python`.

/// Returns a greeting. A placeholder to prove the Rust -> Python path works.
pub fn hello(name: &str) -> String {
    format!("Hello, {name}! skook is running.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_greets_by_name() {
        assert_eq!(hello("world"), "Hello, world! skook is running.");
    }
}
