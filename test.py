from fast_ntt_pyo3 import ntt
import json
import time
import galois

FIELD192 = 4787605948707450321761805915146316350821882368518086721537


def ntt_galios(GF, poly_coeff, size):
    return galois.ntt(poly_coeff, size, GF.order)


def test_ntt(poly_coeffs_file, evals_file, poly_degree, log_rho_inv):
    with open(evals_file, "r") as f:
        evals = json.load(f)

    test_evals = [int(evals[i]) for i in range(len(evals))]

    with open(poly_coeffs_file, "r") as f:
        poly_coeffs = json.load(f)
    test_poly_coeffs = [int(poly_coeffs[i]) for i in range(len(poly_coeffs))]

    start_time = time.time()
    GF = galois.GF(FIELD192)
    end_time = time.time()

    duration = end_time - start_time
    print(f"Galios GF init took {duration * 1000:.2f} ms ({duration:.4f} seconds)")

    start_time = time.time()
    result = ntt(poly_degree, log_rho_inv, test_poly_coeffs)
    end_time = time.time()

    duration = end_time - start_time
    print(
        f"Pyo3 NTT computation took {duration * 1000:.2f} ms ({duration:.4f} seconds) for degree {poly_degree}"
    )
    start_time = time.time()
    result2 = ntt_galios(GF, test_poly_coeffs, poly_degree * (1 << log_rho_inv))
    end_time = time.time()
    duration = end_time - start_time
    print(
        f"Galois NTT computation took {duration * 1000:.2f} ms ({duration:.4f} seconds) for degree {poly_degree}"
    )

    assert result == test_evals
    assert (result2 == GF(test_evals)).all()


if __name__ == "__main__":
    # test_ntt("./test/poly_coeffs_2pow10.json", "./test/evals_2pow10.json", 1024, 2)
    test_ntt("./test/poly_coeffs_2pow20.json", "./test/evals_2pow20.json", 2**20, 2)

    print("All tests passed")
