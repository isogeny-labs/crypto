Native bounded-degree certificate replay
=======================================

This Rust crate ports the historical proof_trace.py replay contract. It
normalizes the separately supplied expected input, reproduces its canonical
SHA-256 binding, reconstructs every polynomial from elementary proof nodes,
checks every ordinary total-degree bound, and requires the selected goal to
equal exactly one. No closure search, row reduction, Python process, or
recorded polynomial value is trusted or invoked.

Development checks:
  cargo test --release
  cargo run --release -- verify tests/fixtures/p101_problem.json tests/fixtures/p101_proof.json

The two committed fixture pairs preserve the existing p101 constructive
proof (391 nodes) and characteristic-five repeated-root edge proof. These
are regression tests of a known contract, not a new research sweep or a
performance comparison.

Arithmetic and limits
---------------------
Moduli are explicit integers in 2..=u64::MAX. Modular sums/products use u128
intermediates. The checker does not certify primality: its accepted identity
is valid even over Z/pZ; a field last-fall-degree interpretation presumes p
is prime. It is not a GF(2^m) checker and never interprets modulus 2^m as a
field. Negative input coefficients must fit i64; nonnegative coefficients
must fit u64, and are reduced modulo p. Exponents are nonnegative u32 values.
Floating-point numbers and larger integers are rejected explicitly. The JSON
parser intentionally disables its arbitrary-precision feature, whose reserved
object representation can coerce an object into a number. Polynomial metadata is ignored by
the canonical binding, matching the historical contract.

Fail-closed reference limits are 256 variables, 1,000,000 nodes, 1,000,000
terms per polynomial and a degree bound of 1,000,000. These are logical
resource guards, not memory-capacity promises. Invalid DAG references,
noncanonical trace coefficients, unsupported operations and non-unit goals
are rejected. Every node is checked, including nodes unused by the goal.

Primary implementation documentation consulted
---------------------------------------------
  https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html
  https://docs.rs/sha2/0.10.9/sha2/
The source mathematical/replay contract is preserved in proof_trace.py and
instance.py in the parent research snapshot. The native checker is not a
Lean proof and does not establish an asymptotic degree theorem by itself.
