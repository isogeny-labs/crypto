# Full-D18 shared-root model passed; planted search censored

The [preregistered shared-root option](../SHARED_ROOT_PROTOCOL.md) was built
from clean source commit `1f04629635101e04ac662bef28ce589b92bc929b`.
The executable SHA-256 was
`674712410c07a291aced53a81783430a6785a115cc0b2d317ecb7f65d0493c48`.
A fresh GET of the complete 160,747,529-byte S3 object matched the pinned
SHA-256 and provider checksum, and its custody wrapper matched the committed
producer manifest. The [independent audit](audit.json) rehashed that object,
the source CSV and both raw run receipts.

| Observation | Per-representative default | Shared-root opt-in |
| --- | ---: | ---: |
| Common root bits | encoded 32,643 times per summand | 66 zero bits, one unit each |
| Domain clauses, m=4 | 11,238,656 | 2,621,168 |
| Total CNF clauses, public model | 11,424,742 | 2,807,254 |
| Total CNF clauses, planted control | 11,425,240 | 2,807,752 |
| SAT variables / XOR rows | 329,726 / 3,901 | 329,726 / 3,901 |

The difference is exactly **8,617,488 clauses** in each model, as predicted
by `4 × 66 × (32,643 − 1)`. These are representation counts for the same
factor base, not separate factor bases or a solver-runtime ratio. Small-field
tests checked the exact closure of both encodings, including shared zero and
one bits. The default encoding remains the standard path.

The shared-root public model imported the full base in 65.722 seconds and
built the circuit in 0.183 seconds; whole-process worker wall was 65.996
seconds. Its raw supervisor record shows 67.70 seconds real and a 4.194 GB
maximum resident set size. This is L0 shared-host evidence, not a matched
isolation comparison with the earlier default-model run.

The shared-root planted control imported the same base in 62.5 seconds and
built the pinned circuit in 0.178 seconds. SAT did not return before the
300-second internal whole-process cap. The durable status is
`UNKNOWN_wall_cap`; no SAT model, original-field relation, rank increment or
normal result receipt was produced. The outer 310-second supervisor reported
exit 124, 301.21 seconds real and 3.988 GB maximum resident set size.
Both control encodings are censored at the same nominal cap, so neither an
empirical solve-time ratio nor a factor-base winner follows. The retained
selection field remains `null`.

The remaining bottleneck is the planted SAT search or its root propagation,
not circuit construction alone. A next diagnostic should expose root
propagation and search counters under a frozen bound, then pass the planted
control before spending capacity on natural queries. Complete cold IC cost
still requires checked useful-rank growth, linear algebra, unseen-target
descent and matched same-target rho.
