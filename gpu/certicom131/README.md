# Native ECCp-131 and ECC2-131 arithmetic

This directory supplies correctness-first native arithmetic for the two
distinct 131-bit Certicom challenges:

- ECCp-131: five 32-bit limbs, Montgomery multiplication modulo the published
  prime, inversion by the fixed `p-2` exponent, and affine short-Weierstrass
  point operations.
- ECC2-131: three 64-bit polynomial-basis limbs modulo
  `x^131+x^13+x^2+x+1`, carry-less multiplication/reduction, inversion, and
  affine binary-Weierstrass point operations with the published nonzero `a`
  coefficient.

`test_host.cpp` and `test_cuda.cu` run the same stable verifier against all
frozen field and group vectors.  The CUDA test is intentionally a one-thread
correctness replay; no speed claim follows from it.

Run locally:

```sh
make host-test
```

On an RTX PRO 6000 with a CUDA toolkit supporting `sm_120`:

```sh
make cuda-test
```

The recovered `oracle131.py` is retained byte-for-byte as historical
provenance.  It is not invoked by the build, tests, or verifier.  See
`PROTOCOL.md` for frozen hashes, acceptance criteria, and claim boundaries.
