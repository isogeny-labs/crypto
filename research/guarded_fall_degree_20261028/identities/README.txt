Repository identity records for the fall-degree proof
====================================================

These four identity.certificate/v1 records use the repository's unchanged
src/cryptanalysis/identity_certificate.rs contract. Each record has 32
statement-and-seed-derived evaluations over 2^61-1; the replay receipt also
checks 32 domain-separated extra points. The record contains a SHA-256-bound
reference to the checked FallDegreeIdentities.lean source. The record checker
does not invoke Lean and does not prove the full bounded-degree theorem.

Records
-------
odd_resultant_cancellation.json: the all-ring minus-two resultant identity.
char_two_resultant_integer_lift.json: the ALL-RING integer lift of the binary
  cancellation, including its explicit twice-the-correction term. This is
  not the characteristic-two-only equality evaluated in an odd field.
inverse_guard_divided_difference.json: the pure guard cancellation, obtained
  by specializing the referenced Lean theorem to gx=d*q and gy=0.
cubic_missing_root.json: the exact monic-cubic missing-root identity.

The binary lift's public Lean declaration is
FallDegreeIdentities.char_two_resultant_integer_lift. Its specialization to
characteristic two removes the correction using CharP R 2. A negative native
unit test removes that correction and confirms the resulting false claim is
rejected by the odd-field issuer.

Reproduce from ../native:
  cargo run --release --bin identity_records -- check
  cargo test --release --bin identity_records

To intentionally regenerate after reviewing a change to the formal source:
  cargo run --release --bin identity_records -- issue
This checks the current proof-source hash and the exact expected statements
before emitting replay.json. recheck.json is a separate serialized-file replay
receipt. These are native conformance checks, not performance measurements or
new field/curve experiments.
