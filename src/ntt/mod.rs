//! NTT (Number Theoretic Transform) module
//!
//! This module provides efficient implementations of Number Theoretic Transforms
//! for polynomial operations in finite fields.

pub mod domain;
pub mod fields;

pub use domain::Domain;
pub use fields::Field64;

// Re-export commonly used types and traits
pub use ark_ff::FftField;
pub use ark_poly::EvaluationDomain;
