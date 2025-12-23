# Roadmap

This package provides Python bindings for linear algebra over GF(2), built on top of the Rust crate `lin_algebra`.

The main goals are usability, clarity, and accessibility, while still benefiting from a high-performance Rust backend.

---

## 1. User-friendly and Pythonic API

- Improve method naming and consistency
- Make common operations easy to discover
- Provide clear error messages
- Keep the API intuitive for Python users

---

## 2. Performance with debuggability

- Expose optimized Rust implementations when available
- Preserve a clear, non-bit-packed representation for debugging
- Allow users to reason about matrix operations easily
- Avoid hiding complexity behind opaque abstractions

---

## 3. Documentation and examples

- Expand usage examples
- Add educational explanations
- Document algorithmic behavior and limitations
---

## Relationship with `lin_algebra`

Most algorithmic complexity and performance optimizations are implemented in the Rust crate `lin_algebra`.

This package serves as:
- a Python-friendly interface
- a validation layer for the Rust design
- an accessible entry point for users who do not use Rust
