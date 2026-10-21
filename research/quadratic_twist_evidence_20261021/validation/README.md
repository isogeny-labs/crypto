# Validation and environment limitations

Base repository revision: `43e01b10e0e5ec6bb74a1cd24f283ef006da586d`.

- Native C++17 audit and semantic controls passed; receipts are under `audits/`.
- Compiler: GCC 14.2.0; OpenSSL 3.5.7. These are correctness checks, not timings.
- `conductor check --summary ... --scope path:research/quadratic_twist_evidence_20261021 ...`
  could not execute: `conductor` is not installed (exit 127). The referenced
  `.conductor/WORKFLOW.md` is absent from this base revision (GitHub returned 404).
  No shared-task reservation is claimed. The proposed diff uses a new study
  directory and a new scoped workflow, without changing existing implementations.
- `cargo test --release --lib` could not execute: no Cargo or Rust toolchain
  is installed in this environment (exit 127). No Rust library tests are claimed
  to have passed. The diff does not modify the Rust library or Cargo manifests.
- Lean is not installed. No checked Lean formalization is claimed.
- The active audit uses the installed native C++ toolchain. The archived Python
  code remains a source snapshot and is not executed in this repository.

These limitations remain explicit review/validation gaps. Opening this PR does
not declare them resolved or authorize bypassing repository merge gates.
