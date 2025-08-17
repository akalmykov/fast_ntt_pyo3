# fast_ntt_pyo3

A Python extension module written in Rust using [PyO3](https://pyo3.rs/) and [num-bigint](https://docs.rs/num-bigint/) to perform NTT efficiently over large fields. Developed as a replacement for Python's Galois library.

## Features

Rust-native implementation using [arkworks](https://arkworks.rs/) for Number Theoretic Transform computations. Supports 192-bit finite fields (and can be extended to the fields of any size)

## Build & Install

1. Install [uv](https://github.com/astral-sh/uv) (if not already):
   ```sh
   pip install uv
   ```
2. Install Python build dependencies:
   ```sh
   uv pip install -r requirements.txt
   ```
3. Build and install the extension in your Python environment:
   ```sh
   maturin develop
   ```
4. Run test:

   ```sh
   python test.py
   ```

   Before running tests, un-archive test data in `test` directory.
   ``sh
   cat evals_2pow20.json.tar.gz.\* | tar -xzf -
   tar -xzf \*.tar.gz

``

## Usage

```python
from fast_ntt_pyo3 import ntt

# Example: Compute NTT over a 192-bit finite field
degree = 1024  # Polynomial degree (2^10)
log_rho_inv = 2  # Logarithm of the rate parameter inverse
poly_coeffs = [1, 2, 3, 4, ...]  # Polynomial coefficients as Python integers

# Compute NTT evaluations
evaluations = ntt(degree, log_rho_inv, poly_coeffs)

# The function returns a list of integers representing
# the polynomial evaluations at the NTT domain points
print(f"Computed {len(evaluations)} evaluations")
```

### Parameters

- `degree` (int): The degree of the polynomial (e.g., 1024 for 2^10)
- `log_rho_inv` (int): Logarithm base 2 of the rate parameter inverse (typically 2)
- `poly_coeffs` (list): List of polynomial coefficients as Python integers

### Returns

- List of integers representing the polynomial evaluations over the NTT domain

## Documentation

- **PyO3**: [Official documentation](https://pyo3.rs/) for Rust-Python bindings
- **arkworks**: [Ecosystem documentation](https://arkworks.rs/) for algebraic cryptography in Rust

## Performance

### Benchmark Results

Performance comparison between this Rust implementation (via PyO3/arkworks) and Python's `galois` library over a 192-bit finite field:

| **Degree** | **Galois GF init (ms)** | **PyO3 NTT (ms)** | **Galois NTT (ms)** | **Speedup** |
| ---------- | ----------------------- | ----------------- | ------------------- | ----------- |
| 2^10       | 14,471 ± 777            | 2.15 ± 0.23       | 338.57 ± 26.8       | **157.4×**  |
| 2^20       | 14,652 ± 1,403          | 3,785 ± 356       | 625,201 ± 75,550    | **165.2×**  |

_Table: NTT performance comparison between Rust `pyo3` (arkworks) and Python `galois` over a 192-bit field. Times are mean ± std. deviation over 10 runs. Speedup = (Galois NTT time) / (PyO3 NTT time)._

### Analysis

**Performance Advantage**: Rust-native `arkworks`, executed through PyO3, computes NTTs ~160 times faster than Python's `galois` library. The gap widens in absolute terms with larger degrees: for 2^20, Rust completes in ≈3.8s vs Python's ≈625s, making Python's version virtually unusable for large polynomials.

**Why `galois` is slow**: The main performance bottleneck in `galois` lies in its inability to use JIT-compiled ufuncs for large finite fields:

- **Ufuncs limitation**: NumPy's universal functions (ufuncs) are vectorized, element-wise operations that avoid explicit Python loops. `galois` uses Numba to specialize functions for fixed-width dtypes (e.g., `int64`, `uint32`) and compile them to machine code via LLVM.

- **Large field fallback**: Since ufuncs are limited to NumPy integer data types (largest being `numpy.int64`), operations on 192-bit fields fall back to `numpy.object` with `numpy.frompyfunc()`, which wraps Python integers but cannot be JIT-compiled.

- **Initialization overhead**: `galois` takes a significant amount of time (~15 seconds) to initialize a 192-bit field, despite having no JIT compilation for large fields.

**Test Environment**: AMD Ryzen 7 3750H, single threaded
