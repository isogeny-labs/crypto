# Koblitz n23 m3..15 enumeration: 26-cell run report

All 26 frozen cells have terminal restricted S3 custody. The same public
field, curve, generator and target were used at every arity within a twist;
the quadratic-twist public core differs. Native enumeration completed its
finite domain at m3 and m4 on both twists. Every m5..15 cell reached the
preregistered 30-second native cap. The four completed cells verified no
decomposition in their fixed signed factor bases; no cell promoted a PDP
witness or an independently recovered ECDLP scalar.

| Arity | Untwisted | Quadratic twist | Interpretation |
|---:|---|---|---|
| 3 | verified | verified | 512 complete candidates, 584 group-add calls per cell; exhaustive fixed-base negative |
| 4 | verified | verified | 4,096 complete candidates, 4,680 group-add calls per cell; exhaustive fixed-base negative |
| 5..15 | timeout at each m | timeout at each m | resource-conditioned censored native prefix; no decomposition verdict |

The [official export](../experiment-export.json) passed the pinned checker
with zero findings: 26 planned and attempted, four completed, 22 not
completed, zero excluded, interrupted or resumed. Its SHA-256 is
`525509d3d62b6c8fe99d94a32394d7a24f71748971400009e496507d44835262`.
The [independent audit](../AUDIT.json) rehashed all 26 restricted archives
(8,849,802 bytes) and 685 native artifacts. It checked the native manifest,
frozen axes, factor-base/solver status binding, Koblitz Frobenius recurrence
and quadratic-twist order-certificate types, F2 degree 23 with coefficient
field degree 1, and the matched public core across all 13 arities. Public-core
SHA-256 values are `65876a64c2a92c1bac6a51c1e916694c05726e11a14a132d7e9e17c4af8a08e0`
untwisted and `db4cbbe375b2a4318520a47b0afc40b617f3ce530e1eddf603780a8666fc3557`
twisted. The [status chart](outcomes.svg) shows the full denominator.

[Per-cell Git checkpoints](../checkpoints/) bind every archive and custody
pointer to producer S3 PUT and fresh GET/hash comparison. A
[separate fresh S3 readback](../S3-READBACK-SAMPLE.json) independently checked
the first and last archives. The protocol and plan were frozen and committed
before any data at `346b4ff2d8ced995fd36cfcacfffd8791782335e`; protocol
body SHA-256 is
`9bd53b54e48aaddcb0e80b5d5ecb4967b73a3ae7f41f299b31a63588b4c06de2`,
protocol file SHA-256 is
`fe51504cb5afb702e986027464ff747302cc9941eb6aa7d48d452a48ae7d5faf`,
and plan SHA-256 is
`6b97af5a3955fce532c40c862f66150b8a727a1ff77f13656ede5c83d499b1a0`.
The clean native source is `c41bec150e16e4c4f9a00634002b633b821de262`
and binary SHA-256 is
`b8e0881ec5e46f60c23e6bbb49ad7c3677b9168199f5787b68d89169bfaaa77e`.

[Cost observations](../cost-observations.jsonl) contain 82 phase telemetry
records. The completed cells retain exact native group-add and candidate
counters. Some phase records contain raw calling-thread hardware readings,
but none verifies whole-worker instruction coverage, so there are zero
instruction-to-solve training labels. Wrapper clocks are diagnostic; this
fixed-population study makes no timing-effect comparison. An
[unrelated release compile](../HOST-CONTENTION.json) was observed during
later cells. The timeouts are retained as resource-conditioned censored
outcomes, without attributing an individual timeout to that process.

Index 18 had one sealed native timeout attempt. Its controller was interrupted
during the custody-pointer client; both content-addressed S3 objects were
already present and were fresh-GET byte checked. The
[reconciliation receipt](../S3-INTERRUPTED-CUSTODY-18.json) records zero new
PUTs and zero new native attempts. An
[operational controller amendment](../CONTROLLER-AMENDMENT.json) replaced
repeated Git-object reads that stalled under host I/O with the recorded
[pre-data commit binding](../PRE-DATA-BINDING.json). It retained the same
protocol, plan, native binary, limits, verifier and one-attempt markers.
The S3 store description for each cell carries a stale “m4” prose string
from the controller template; immutable logical names, public records,
manifests and this report identify the actual m3..15 axes.

The finite-domain enumerator emits and fsyncs each visited transition. At
factor-base size 8, the fixed 30-second cap allowed complete traces through
m4 and censored all higher arities. A separately frozen binary/prime wave can
use a smaller fixed base and a charged transition cap to preserve more
verified or budget-exhausted method artifacts while keeping all m3..15/twist
cells. That is a design implication from these observations, not a result for
the unrun families. For algebraic context, [Shantz and Teske's
characteristic-two study](https://eprint.iacr.org/2013/596.pdf) distinguishes
experimental Weil-descent/Gröbner behavior from proved complexity; this run
measures the native bounded enumeration adapter only.
