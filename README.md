# fast_ntt_pyo3

A Python extension module written in Rust using [PyO3](https://pyo3.rs/) and [num-bigint](https://docs.rs/num-bigint/) to sum large Python integers efficiently.

## Features

- Exposes a `sum_bigints` function to Python that takes a list of Python integers (arbitrary size) and returns their sum using Rust's `num-bigint`.

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

## Usage

```python
from fast_ntt_pyo3 import sum_bigints
print(sum_bigints([10**100, 10**120, 42]))
```

## Documentation

- PyO3 official docs: https://pyo3.rs/
- num-bigint: https://docs.rs/num-bigint/
- Context7 docs for PyO3 can be fetched for more advanced usage.

## Performance

Tested on AMD Ryzen 7 3750H, single threaded

### Test runs on 192-bit field, 2^10 degree, log_inv_rho=2

Galios GF init took 14271.46 ms (14.2715 seconds)
Pyo3 NTT computation took 1.98 ms (0.0020 seconds) for degree 1024
Galois NTT computation took 316.24 ms (0.3162 seconds) for degree 1024
All tests passed
Galios GF init took 13936.39 ms (13.9364 seconds)
Pyo3 NTT computation took 1.87 ms (0.0019 seconds) for degree 1024
Galois NTT computation took 316.23 ms (0.3162 seconds) for degree 1024
All tests passed
Galios GF init took 14568.80 ms (14.5688 seconds)
Pyo3 NTT computation took 2.27 ms (0.0023 seconds) for degree 1024
Galois NTT computation took 340.69 ms (0.3407 seconds) for degree 1024
All tests passed
Galios GF init took 14585.52 ms (14.5855 seconds)
Pyo3 NTT computation took 1.93 ms (0.0019 seconds) for degree 1024
Galois NTT computation took 328.82 ms (0.3288 seconds) for degree 1024
All tests passed
Galios GF init took 14150.33 ms (14.1503 seconds)
Pyo3 NTT computation took 2.04 ms (0.0020 seconds) for degree 1024
Galois NTT computation took 332.58 ms (0.3326 seconds) for degree 1024
All tests passed
Galios GF init took 12970.85 ms (12.9708 seconds)
Pyo3 NTT computation took 2.28 ms (0.0023 seconds) for degree 1024
Galois NTT computation took 309.37 ms (0.3094 seconds) for degree 1024
All tests passed
Galios GF init took 14067.27 ms (14.0673 seconds)
Pyo3 NTT computation took 1.93 ms (0.0019 seconds) for degree 1024
Galois NTT computation took 377.30 ms (0.3773 seconds) for degree 1024
All tests passed
Galios GF init took 14558.43 ms (14.5584 seconds)
Pyo3 NTT computation took 2.55 ms (0.0026 seconds) for degree 1024
Galois NTT computation took 373.84 ms (0.3738 seconds) for degree 1024
All tests passed
Galios GF init took 15881.35 ms (15.8814 seconds)
Pyo3 NTT computation took 2.41 ms (0.0024 seconds) for degree 1024
Galois NTT computation took 387.45 ms (0.3875 seconds) for degree 1024
All tests passed
Galios GF init took 13913.38 ms (13.9134 seconds)
Pyo3 NTT computation took 2.39 ms (0.0024 seconds) for degree 1024
Galois NTT computation took 325.46 ms (0.3255 seconds) for degree 1024
All tests passed

### Test runs on 192-bit field, 2^20 degree, log_inv_rho=2

Galios GF init took 14569.86 ms (14.5699 seconds)
Pyo3 NTT computation took 3518.26 ms (3.5183 seconds) for degree 1048576
Galois NTT computation took 632778.07 ms (632.7781 seconds) for degree 1048576
All tests passed
Galios GF init took 15904.51 ms (15.9045 seconds)
Pyo3 NTT computation took 4024.09 ms (4.0241 seconds) for degree 1048576
Galois NTT computation took 700253.31 ms (700.2533 seconds) for degree 1048576
All tests passed
Galios GF init took 16290.68 ms (16.2907 seconds)
Pyo3 NTT computation took 3953.48 ms (3.9535 seconds) for degree 1048576
Galois NTT computation took 755896.42 ms (755.8964 seconds) for degree 1048576
All tests passed
Galios GF init took 16302.98 ms (16.3030 seconds)
Pyo3 NTT computation took 3864.99 ms (3.8650 seconds) for degree 1048576
Galois NTT computation took 644348.51 ms (644.3485 seconds) for degree 1048576
All tests passed
Galios GF init took 16276.21 ms (16.2762 seconds)
Pyo3 NTT computation took 3667.95 ms (3.6680 seconds) for degree 1048576
Galois NTT computation took 563493.55 ms (563.4936 seconds) for degree 1048576
All tests passed
Galios GF init took 16186.68 ms (16.1867 seconds)
Pyo3 NTT computation took 3551.98 ms (3.5520 seconds) for degree 1048576
Galois NTT computation took 690446.91 ms (690.4469 seconds) for degree 1048576
All tests passed
Galios GF init took 16495.25 ms (16.4952 seconds)
Pyo3 NTT computation took 4208.25 ms (4.2083 seconds) for degree 1048576
Galois NTT computation took 539848.06 ms (539.8481 seconds) for degree 1048576
All tests passed
Galios GF init took 11939.40 ms (11.9394 seconds)
Pyo3 NTT computation took 3230.18 ms (3.2302 seconds) for degree 1048576
Galois NTT computation took 534624.67 ms (534.6247 seconds) for degree 1048576
All tests passed
Galios GF init took 12541.84 ms (12.5418 seconds)
Pyo3 NTT computation took 3062.70 ms (3.0627 seconds) for degree 1048576
Galois NTT computation took 602849.44 ms (602.8494 seconds) for degree 1048576
All tests passed
Galios GF init took 13728.16 ms (13.7282 seconds)
Pyo3 NTT computation took 3759.34 ms (3.7593 seconds) for degree 1048576
Galois NTT computation took 587476.54 ms (587.4765 seconds) for degree 1048576
All tests passed
