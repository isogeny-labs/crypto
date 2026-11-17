# Actual unknown-coefficient rational solver: bounded native correctness baseline

The preceding prototype checked supplied norm-function witnesses. This continuation
actually finds unknown coefficients from curve, target and factor-base parameters,
then uses those decompositions in a complete tiny-curve direct mixed-target IC DLP
baseline. No source summands, generation scalar, norm witness or decomposition
table is passed to the relevant solver interface. The implementation remains
**an explicit exponential coefficient census, not a sub-exponential algorithm,
novel solver or performance result**.

## Implemented solver

For arity three write `A=X^2+uX+v`, `C=w`. The target equation gives

```text
v = y_R*w - x_R^2 - u*x_R.
U = (A^2 - (X^3+aX+b)*w^2)/(X-x_R).
```

The native candidate enumerates `(u,w) in F_p x F_p^*`; the input contains no
preferred pair. It requires the resulting monic cubic `U` to be squarefree and
`U(x_R)!=0`. It checks `X^(2^t)=c mod U` by repeated quotient-remainder squaring,
then extracts all three rational abscissas and recovers signs by `y=-A(x)/w`.
Every recovered point is rechecked against the curve and membership equation.
No point-table sum/join is used to find the coefficients.

All loop coefficients are in `F_p`. Consequently the implementation does not
mistake an algebraic-closure coefficient assignment for a rational answer. The
standalone bounded CLI validates a nonsingular, whole prime-order, non-anomalous
curve over a prime field `5<=p<=257`, a nonzero split coset and an on-curve affine
target. The unit suite below uses only the fixed `p=17` curve.

For the selected chart the coefficient enumeration is complete: a three-point
decomposition with finite distinct abscissas, no target-abscissa overlap and
nonzero constant `C` has a function in `L(4O)` of exactly this form, uniquely
normalized to monic `A`. Conversely the norm identity, membership and chart
conditions recover a divisor with sum `R`. This is the existing Riemann-Roch
correspondence, not a new decomposition theorem. Exceptional charts are excluded
explicitly; no nonvanishing chart-coverage or rank claim is made.

## Independent complete model-set reference

`point_oracle.cpp` is a separate executable with its own affine group-law code.
It enumerates the rational signed factor-base points and every unordered triple
with distinct abscissas different from the target's. It readds each triple and
keeps those summing to the target. It does not contain norm or polynomial
remainder logic. Both programs output the same canonical point-model format.

This supplies an independent implementation within the same development session,
not an external scientific replay or independent review. Comparing complete
models detects both missing and spurious answers; matching counts alone would
not suffice. The candidate never receives the oracle's models.

## Fixed correctness results

The frozen [PROTOCOL.md](PROTOCOL.md) specifies every affine target of
`E/F17: y^2=x^3+2x+2` for five declared domains. The curve has order 19, so this is
a prime-order, non-anomalous fixture. These are one-curve unit fixtures, not
independent field-size samples or a scaling experiment.

| `(t,c)` | Signed base points before target-chart exclusion | SAT targets | UNSAT targets | Complete models across targets |
|---|---:|---:|---:|---:|
| `(1,1)` | 2 | 0 | 18 | 0 |
| `(2,1)` | 4 | 0 | 18 | 0 |
| `(3,1)` | 6 | 6 | 12 | 6 |
| `(3,16)` | 10 | 18 | 0 | 58 |
| `(4,1)` | 16 | 18 | 0 | 336 |

All **90 complete rational model sets agree exactly**, including **42 SAT and
48 UNSAT** cases and **400 models** in total. Each candidate case visits exactly
272 coefficient pairs. `t=1,2` are deliberate too-small distinct-chart negatives.
Among the twelve `t=3,c=1` unsuccessful targets, six have fewer than three usable
abscissas after excluding the target's x; six retain all three usable abscissas
but have no selected-chart decomposition. These six are nonempty-chart natural
negatives in a domain that also has six successful targets. Every target is taken from the independent full
affine-point enumeration, not chosen by planting a decomposition.

Both executables reject malformed/extra-witness arguments, zero and nonsplit
cosets, invalid tower depth, off-curve targets, composite characteristic, singular
curves, composite-order curves and an anomalous prime-order curve. There are
**18 negative CLI checks**, all returning the declared error exit 2. Infinity has
no valid affine CLI representation and is rejected as malformed input; this is
not complete handling of infinity within a relation pipeline.

Retained [checks/checks.tsv](checks/checks.tsv) records every target, domain,
model count, coefficient-pair count, SAT/UNSAT status and number of usable
abscissas after target exclusion. The two full canonical
model files are retained separately and compare byte-for-byte. No elapsed times,
speedups, degree of regularity or security conclusions were collected.

## Reproduce

From the repository root:

```sh
bash research/prime_one_shot_20261029/continuation/solver/check.sh \
  /tmp/unknown-norm-development-check
```

The script builds both executables with C++17 and
`-O2 -Wall -Wextra -Wpedantic -Werror`, executes all fixed model-set comparisons
and negative controls, and fails on any mismatch. [build-receipt.txt](build-receipt.txt)
retains compiler, hashes, commands and exit statuses. `files.sha256` binds the
source and retained correctness outputs. Native binaries are outside Git.

For one actual unknown solve, without a witness:

```sh
/tmp/unknown-norm-development-check/norm_solver 17 2 2 3 1 0 6
```

It returns the unique selected-chart model
`(9,16),(13,7),(16,4)`. The independent oracle finds the same model by point
addition. This is a tiny exact decomposition solve, not a discrete-log recovery.

## Complexity and limits

The coefficient loop alone has `p*(p-1)` iterations. Constant-degree norm/gcd and
`t` remainder-squaring steps are performed per candidate. Root extraction here
also scans `F_p` for each accepted polynomial; curve-order admission enumerates
all `(x,y)` pairs. A generous field-work bound is
`O(p^2*(t+1)*log p + p*M)` for `M` retained models, plus output, with finite-degree
constants suppressed. In particular, `divrem` currently computes the leading
coefficient inverse by Fermat powering on each division, even for a monic `U`;
the safe bound therefore includes a `log p` factor for every membership layer.
No sub-exponential dependence on `log p` follows. Even
replacing the root scan by a faster factoring routine would leave the `p^2`
coefficient census. This solver is the explicit baseline a genuinely implicit
solver must surpass; it must not be marketed as the missing breakthrough.

Only arity three is implemented. There is no unknown-coefficient solver for
growing arity here, no full exceptional-chart coverage, no general-curve DLP
implementation, no comparative performance admission and no general solving-degree
bound. The parent research goal remains unresolved.

Before implementation the PKM 2016 primary text sections 3.1-3.3 and the prior
norm/remainder prototype were read. They supply the low-degree membership and
Riemann-Roch interface while leaving complexity open. The protocol was written
before these correctness checks; its hash is retained. `conductor check` returned
command-not-found, as already documented by the parent. Local development tests
are kept separate from the repository's prescribed EKS scientific measurements.

## Complete tiny-curve direct IC path

The subsequent direct-IC addendum in [PROTOCOL.md](PROTOCOL.md) was written before
its checks. `direct_ic.cpp` builds the canonical signed factor base cold, samples
labels `a,b`, computes `R=[a]P+[b]Q`, passes **only public parameters and R** to the
PDP, and binds the canonical point decomposition to a row of factor-base
coefficients C. The fixed generator is `P=(5,1)`, order 19. The `(t,c)=(3,1)` base
has B=3 canonical sign orbits; no factor-base logarithms are computed or supplied.

After B+1=4 accepted rows, a standalone left-null routine takes only C and produces
nonzero w. It does not receive a,b,Q or any expected scalar. The recovery step
uses `k=-(w.a)/(w.b) mod 19`. A zero denominator is retained as a failed batch and
restarts with fresh samples. There is **no full-rank-B gate**. Each returned
scalar is checked by `[k]P=Q` inside the solver.

Run the standalone actual DLP interface without supplying a logarithm:

```sh
bash research/prime_one_shot_20261029/continuation/solver/check_direct.sh \
  /tmp/direct-ic-development-check
/tmp/direct-ic-development-check/direct_ic --solve 6 3 1234
```

The last command receives only Q coordinates and a development RNG seed; it
recovers scalar 2 and verifies the group equation. It uses no rho, BSGS, target-log
scan, precomputed base logs or cross-target retained setup.

All 18 nonzero targets are verified in both forms: an internal point-only solver
API whose fixture generation is outside the solver, and separate executable
calls `--solve Qx Qy 1234`. Ground-truth scalars are used only by the test harness
after each solve. Four malformed direct-interface inputs are rejected. Separate
linear-algebra controls cover ranks 0,1,2,3 and denominator-zero rejection.

For the 18 coordinate-seeded internal cases, final C ranks are 1 on 2 targets, 2 on
10 targets and 3 on 6 targets: **12 of 18 successful solves do not have full base rank**.
They collectively use 230 attempts: 13 identity misses, 145 unsuccessful PDP calls,
and 72 accepted rows. All 217 nonidentity PDP calls are charged, including misses,
for 59,024 coefficient pairs. Each solve starts cold with 289 curve-point admission
checks and 9 canonical membership tests. The CSV also retains root field-candidate
scans, point-addition calls by setup/generation/relation/verification phase,
matrix and recovery modular reductions, all batches and failed denominators.
These are explicit diagnostic counters, not a unified field-operation price.

Neither 18-target seed panel happened to produce a natural zero-denominator batch;
the synthetic zero-denominator rejection control passes. Deterministic development
seeds and rejection mapping are used for reproducibility; these fixtures do not
prove the ideal independent-uniform label distribution or validate its predicted
probability statistically. The rank-free uniform-label lemma belongs to the
separate mathematical analysis, not to this empirical test.

[checks/direct-checks.tsv](checks/direct-checks.tsv) retains every internal target
and counter; [checks/direct-cli.tsv](checks/direct-cli.tsv) retains the separate
point-only executable checks. Accepted point relations and their a,b,C labels are
preserved in [checks/direct-relations.tsv](checks/direct-relations.tsv); null
vectors, alpha,beta and retry statuses are in
[checks/direct-kernels.tsv](checks/direct-kernels.tsv). Thus the reduction can be
replayed independently without knowing a hidden source scalar. Build and direct
test output are retained, including an initial namespace compile error corrected
without weakening strict flags.

This supplies complete fixed-fixture IC plumbing, not a better asymptotic solver.
The exponential PDP is still the dominant unresolved issue. The checks establish
no speed advantage and no sub-exponential ECDLP algorithm.
