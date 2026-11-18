# Native census continuation — frozen initial scope

Local checkpoints are explicitly approved by the user for this audit.
No paid external compute. Source review/proof background remains in
../theorem_audit_20261108/sources.json and PROOF_MAP.md. KB remains unavailable.

First run: native exact Semaev census, integer finite-field arithmetic only.
- S3: every nonsingular short model over Fp for p=5,7,11,13,17,19;
  every x tuple; y lifts in Fp². Ordinary binary models over F2 and F4,
  all coefficients a2 and nonzero a6, ambient F4 and F16.
- S4/S5: full sparse symbolic resultants on every short model over F5,F7
  and every ordinary binary model over F2,F4; every base x tuple.
- Degree target: degree2,4,8 in every coordinate for S3,S4,S5 respectively.
- Truth test: actual generalized Weierstrass addition in the ambient
  quadratic extension. All inputs affine; intermediate sums may be infinity.
- Resultants use fixed-size universal Sylvester identities, so specializations
  with vanishing leading coefficient keep the correct padded resultant.
- Controls: field inverses, curve membership after addition, inverse sums,
  full small group associativity controls before a census; fixed known S3
  zero and nonzero fixtures. Cross-check symbolic quadratic resultants against
  a separately computed Sylvester determinant over scalar coefficients.
- Success/falsification: retain aggregate domain counts, exact zeros and both
  discrepancy counts; any discrepancy records a curve/tuple witness and fails.
  Finite agreement is not a general proof; Lean statements retain exact domains.
- Resource limit: local single process, small field bounds above, initial
  deadline10minutes. Preserve timeout as incomplete, not a negative result.
- Artifact paths: native/{finite.hpp,semaev.cpp}, evidence/semaev-native.jsonl,
  evidence/semaev-native.stderr, code hashes and compiler identity.

Later items remain the original14-item objective. They need their own bounded
inputs/controls before execution, rather than silently expanding this first run.
