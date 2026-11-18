# E18 replay amendment and E19 explicit curves, version 2

Date: 2026-10-23. Freeze this amendment and its source before running it.
The [version-1 protocol](GLUING_PROTOCOL_V1.md) and every initial output are
retained. Its native torsion constructors finished, but the GP torsion
replay used a conflicting `image` function name and two function definitions
on one line. GP printed errors while still exiting zero and printing its
final marker. Those external checks are invalid; a marker or exit code alone
does not admit them. Native construction is not changed by this repair.

## E18 corrected replay

Emit two braced GP functions on separate lines under distinct task-specific
names. Repeat the four native constructions into new `v2` directories and
run their newly emitted independent checks. Compare their coordinates,
matrices and graph certificates with version 1; only the checker source may
change. Retain both versions and all GP errors. A known `new maximum stack
size` warning is not a mathematical error; the native adjudicator must
explicitly classify it and reject every other error message. The earlier
transport metadata that rejected any `***` line is conservative, not the
scientific verifier. Original per-curve 1,800-second/4-GiB/one-thread caps and
all degree, negative-control, actual-point and pairing requirements remain.

## E19 explicit geometry constructor

Use the same four positive sources and the same published input models.
Implement Broeker–Howe–Lauter–Stevenhagen, arXiv:1403.6911, Algorithm 5.4
and Appendix A. This is prior art, not a new gluing algorithm. The family is

    t*y^2 = (x^3+3*a*x+2*b)*(2*d*x^3+3*c*x^2+1),
    12*a*c+16*b*d=1, (a^3+b^2)*(c^3+d^2)*t != 0.

Solve its two j-invariant equations over the exact base field. All four
inputs have j distinct from 0 and 1728. Compute a resultant in the a=b
chart, extract every rational root and specialized gcd, and also inspect
the c=d chart through the parameter swap. Preserve zero, exceptional and
duplicate candidates; do not count parameter presentations as independent
curves. Check both square classes of t against actual short-model
isomorphisms to E and its standard twist. Matching j alone is insufficient.

Use PARI/GP 2.17.2 as the native constructor. Rust BigUint independently
reconstructs the curve and both rational maps, checks squarefreeness and
genus, proves each cleared-denominator polynomial identity by comparing all
coefficients, verifies degree three and separability, checks the declared
target isomorphisms, and rejects a deliberately shifted-x map. Count every
Fp point on toy models (including infinity) and check the predicted p+1;
do not attempt NIST point counting or any NIST DLP. Emit exact coefficients,
all constructor parameters and both map presentations.

The construction input generator and checker are native Rust commands:

```sh
RUNNER followup geometry-input FIXTURES FRESH_INPUT_DIR
gp -f -q FRESH_INPUT_DIR/SOURCE_NAME.gp
RUNNER followup geometry-verify FIXTURES FRESH_CHECK_DIR CONSTRUCTOR.stdout
```

Pin the runner and emitted GP input before execution. Use one thread,
4 GiB address space, and 3,600 seconds per source constructor/checker. This
source-level cap is a bounded pilot inside the original per-anti-isometry
program; exhausting it leaves the omitted cells open. Preserve all failed
resultants, syntax errors, timeouts, partial output and rejected models.

An explicit smooth curve and degree-three maps advance E19, but do not by
themselves identify which E18 graph induces the polarization. Actual
divisor-class pullback/norm, exceptional charts, subgroup image/order,
scalar recovery, independent arbitrary-class arithmetic, formal bridges
and complete cost remain separate obligations. Do not mark the whole E19
experiment complete from the equations alone. This pilot makes no speed,
novelty, source-DLP or asymptotic claim.
