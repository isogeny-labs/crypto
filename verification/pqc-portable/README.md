# PQC correctness and portability checks

This standalone Cargo package loads the complete production ML-KEM, ML-DSA,
SHA-3, scalar/four-way Keccak and RNG modules directly by path. No algorithm,
dependency or RNG is replaced. The modules' own tests include NIST ACVP known
answers, fast/reference differential tests, malformed-input checks and available
SIMD backend checks. Changes to those modules are picked up automatically.

From the repository root:

```sh
cargo test --release --locked --manifest-path verification/pqc-portable/Cargo.toml
cargo check --release --tests --locked \
  --manifest-path verification/pqc-portable/Cargo.toml \
  --target aarch64-unknown-linux-gnu
```

Install the desired standard-library target with `rustup target add TARGET`
before cross-checking. A check compiles test code but does not link or execute
tests on that target. Run the first command on each native target to execute
its tests and available hardware backends. In particular, forcing the portable
backend on x86 is not an ARM hardware test.

This package checks the PQC modules independently of the rest of the library;
it does not replace the repository's full-library test requirement. It is not
a benchmark, leakage test, formal proof, or production-security certification.
The RNG still uses the actual `OsRng` backend and retains its platform limits.
