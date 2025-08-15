use ark_ff::PrimeField;
use ark_poly::DenseUVPolynomial;
use ark_poly::univariate::DensePolynomial;
use num_bigint::{BigInt, BigUint};
use num_traits::Zero;
use pyo3::prelude::*;
use pyo3::types::PyList;

// Declare the ntt module
pub mod ntt;
use ntt::domain::Domain;
use ntt::fields::Field192;

#[pyfunction(name = "ntt")]
fn ntt_py(degree: usize, log_rho_inv: usize, poly_coeffs: &PyList) -> PyResult<PyObject> {
    let py = poly_coeffs.py();
    let d = Domain::<Field192>::new(degree, log_rho_inv).unwrap();
    let poly_coeffs_vec: Vec<Field192> = poly_coeffs
        .iter()
        .map(|x| {
            let bigint: BigInt = x.extract()?;
            let biguint = bigint.magnitude().clone(); // Convert BigInt to BigUint
            Ok(Field192::from(biguint))
        })
        .collect::<PyResult<Vec<_>>>()?;

    let polynomial = DensePolynomial::from_coefficients_vec(poly_coeffs_vec);

    let evals = polynomial
        .evaluate_over_domain_by_ref(d.backing_domain)
        .evals;

    let evals_as_bigints: Vec<BigInt> = evals
        .into_iter()
        .map(|field_elem| {
            let limbs = field_elem.into_bigint().0;
            let mut bytes_le = Vec::with_capacity(limbs.len() * 8);
            for limb in &limbs {
                bytes_le.extend_from_slice(&limb.to_le_bytes());
            }
            BigInt::from(BigUint::from_bytes_le(&bytes_le))
        })
        .collect();

    Ok(evals_as_bigints.into_py(py))
}

#[pymodule]
fn fast_ntt_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ntt_py, m)?)?;

    Ok(())
}
