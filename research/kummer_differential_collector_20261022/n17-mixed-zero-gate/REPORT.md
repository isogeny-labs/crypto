# N17: differential-pair mixed-zero gate and chart-basis audit

The [exact 21-model CSV](results/gates.csv) and [native diagnostic](src/main.rs)
confirm `C1 != C2` and the basis identity

```text
sigma = 1 + lambda + mu + nu = -f4
C2[:,2] = -C1[:,2] - sigma*C1[:,3]
C2*(1,-a1,-a0,q3) = C1*(1,-a1,a0,q3+sigma*a0).
```

Both determinants are nonzero in every frozen model. On each model the
diagnostic additionally uses the valid branch-point divisor with
`u=(x-1)(x-lambda), v=0`; C1 inversion of its C2 projection gives exactly
the transformed vector. The earlier suspicion that N15 inverted the wrong
matrix is **withdrawn**. N15's 138,095 finite signed-lift rows, 414,285
xADD comparisons, and cost counters are unchanged. The distinct C1/C2
matrices matter for the untransformed degree-one vector in the N16
exceptional proof; the retained wrong-basis failures still diagnose that
specific error.

For cached theta xADD, with all four fixed-difference coordinates and
selected theta constants nonzero, the mixed result is the zero vector
exactly when `H(K(D))_i * H(K(D+G))_i = 0` for every `i`.
The existing 414,285 finite calls found no zero, but the all-state
mixed-zero locus has not been excluded by a geometric addition theorem.
The N11 differential pair has an exact signed-label invariant. N15's
candidate enumeration supplies at most six general conversions, twelve
square-root calls and thirteen candidate checks **conditional** on the
declared projection and torsion actions. N16 certifies the action for
declared charts across reduced-form branches, but a source-independent
geometric Kummer embedding proof remains open.

A constructive mixed-zero escape route is to lift `(K(D),K(D+G))` to the
unique signed `D`, compute `2D`, `2D+G`, and `2D+2G` in the Jacobian, and
project the two coordinates needed by the N11 branch. This route has three
Jacobian additions for those three signed points after a successful lift,
plus the lift, projection, validation, and setup costs. It is **not yet a
certified all-state fallback**: the source projection is action-assisted,
its geometric identity and totality are unproved, and the fallback has
not been integrated and independently checked at every transition. The
conditional N15 candidate count is not a measured cold cost for this path.

| Requirement | Status | Evidence or gap |
| --- | --- | --- |
| Exact C1/C2 basis identity | Verified, 21/21 models | CSV, native diagnostic, N16 matrix verifier |
| N11 signed pair invariant | Proved in N11 abstract odd-order group model | N11 protocol and report |
| N15 finite lift and xADD replay | Verified finite run, unchanged | 138,095 states; 414,285 xADD calls |
| All-state geometric projection and xADD/fallback | Open | No source-independent embedding/addition proof or integrated total fallback |
| New 1,260-target signed collector and matched rho cost | Not run; ratio `null` | Arithmetic gate and complete cold-cost protocol remain open |

The [graph](status-graph.svg) and [PDF](REPORT.pdf) depict evidence coverage
without treating `null` as failure or zero. The CSV schema repeats the
historical N15 finite-state count in each model row only as a provenance
field; do not sum that column. Blank proof/cost cells mean `null` (open or
unmeasured), not a demonstrated false statement. The [protocol](PROTOCOL.md) and
`SOURCES.sha256` bind the inputs. Reproduce from the repository root with:

```sh
CARGO_TARGET_DIR=/dev/shm/n17-target TMPDIR=/dev/shm \
  cargo run --offline --locked --release \
  --manifest-path research/kummer_differential_collector_20261022/n17-mixed-zero-gate/Cargo.toml \
  --bin diagnose
CARGO_TARGET_DIR=/dev/shm/n16-ideal-target TMPDIR=/dev/shm \
  cargo run --offline --locked --release \
  --manifest-path research/kummer_differential_collector_20261022/n16-action-certificate/Cargo.toml \
  --bin validate-exceptional-ideal
```
