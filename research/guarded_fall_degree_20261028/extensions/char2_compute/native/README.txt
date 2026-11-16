Native small-field characteristic-two certificate engine
=======================================================

This Rust crate implements genuine GF(2^m) using an explicitly checked binary
irreducible modulus, with m<=8. Coefficients are polynomial-bit encodings.
For example, in GF4 defined by x^2+x+1, element 2 squared is 3; this is not
integer arithmetic modulo 4.

Commands (after cargo build --release):
  char2-fall-degree field 19
  char2-fall-degree closure PROBLEM.json DEGREE --trace
  char2-fall-degree verify-lower PROBLEM.json CLOSURE.json
  char2-fall-degree verify-upper PROBLEM.json REFUTATION.json
  char2-fall-degree verify-result PROBLEM.json CLOSURE.json

The closure JSON includes its upper refutation under the "refutation" key
when --trace is set and 1 is derived. Extract that object for verify-upper,
or use verify-result directly on the complete closure output. Verification
commands exit nonzero when no valid certificate is present.
Certificates bind to the entire separately supplied canonical input problem.
The lower checker verifies a closed separating superspace and excludes 1;
it does not trust the producer's rank or contradiction flags. Upper replay
uses sparse polynomial arithmetic and checks every DAG operation and degree,
independently of the dense RREF producer.

All degrees are ordinary polynomial degrees. Boolean reduction and finite
field equations are never implicit. The 20,000-column cap is a logical
reference limit and does not promise a safe memory bound for dense matrices.

Development validation: cargo test passes four checks for field arithmetic,
certificate/replay soundness on a fixed GF4 Boolean toy, malformed-proof
rejection, input normalization, and the pinned GF16 cube-orbit fixture whose
exact last fall four is now proved independently. The latter checks exclusion
at degree3, refutation at degree4, and serialized sparse-provenance replay.
It is a known-answer correctness regression, not a scientific parameter
sweep. Repository policy routes scientific campaigns to the experiment backend.

gf16_subfield_fixture.json is a mathematically specified known-answer input.
It uses ordinary binary E:y^2+xy=x^3+1 over
GF16, with S=GF4^*={1,6,7}, and target (8,15) outside the subfield. The
polynomial model uses S3=(xy+xz+yz)^2+xyz+a6 and v*(x+y)+1. Its source
equations and field convention are explicit; the independently proved
cube-orbit theorem gives exact last fall degree four, now regression-checked.

The sibling Python files were written before target-repository discovery,
were never executed, and are historical implementation evidence only. This
Rust implementation supersedes them for all new execution.
