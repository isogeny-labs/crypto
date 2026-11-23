# Prime31 RR9: radical support reduction and a squarefree completeness barrier

2026-11-17 UTC · PR #4541 stacked on #4539 · **[pcc:unverified]**

## New algebraic statement: all multiplicities, low-degree test

Let (p>8), (H_B(X)=\prod_{P\in B}(X-x(P))) monic, squarefree and
split over F_p, and (M(X)) any monic degree-eight polynomial (including
repeated factors). Define (D=\gcd(M,M')) with monic normalization and
(S=M/D). Because \(\deg M=8<p\), for every monic irreducible \(q\)
occurring to exponent \(e\le8\), \(q\) occurs to exponent \(e-1\)
in \(\gcd(M,M')\). Hence \(S=\prod_{q\mid M}q\) is squarefree, and

```
                M | H_B^8   if and only if   S | H_B.
```

The original #4539 coefficient-only support predicate tests the left side.
The new native Rust implementation independently tests the right side using
field-polynomial Euclid, a derivative, exact quotient and remainder.
This requires neither a degree-248 polynomial nor exponentiation by eight,
and it preserves repeated signed leaves. Both are *given-M membership
predicates*; neither extracts M from a hidden target.

For the separable discriminant-nonzero branch, \(D=1\) and \(S=M\).
Therefore the entire support constraint reduces to **eight coefficients
of \(H_B\bmod M(A,V,R)\) being zero**, with \(H_B\) degree 31,
not \(H_B^8\) degree 248. This is the first targeted elimination branch
for quotient/F4, but the seven unknown normalized signed-chart coefficients
remain unsolved.

## New exact counting barrier (no random model)

Assume B has b distinct x coordinates, all y nonzero. An unordered
eight-leaf signed support is a multiset of eight points from \(\pm B\),
so the number of formal supports is \(\binom{2b+7}{8}\). In the
squarefree-x stratum one chooses eight distinct x values and independently
one of two signs for each:
\(N_{\rm sf}(b)=2^8\binom b8\) (zero when b<8).

All formal supports map to a group residual R, but multiple supports can
map to the same residual. Thus the number of distinct residuals covered
by the stratum is **at most** its support count, regardless of curve
arithmetic or distribution of residuals.

| b | All signed formal multisets | Squarefree signed supports | Maximum squarefree fraction of n=2,147,446,831 |
|---|---:|---:|---:|
| 4 | 6,435 | 0 | 0% |
| 8 | 490,314 | 256 | 0.0000119% |
| 31 | 8,361,453,672 | 2,019,513,600 | 94.0425% |

In particular, on the actual cofactor-one prime31 group, at least
**127,933,231 distinct residuals (5.9575% of the group)** have
**no squarefree eight-leaf signed decomposition** over a 31-x base.
This is a rigorous upper bound, not a probability approximation or
claim that all other residuals are covered. A complete single-residual
solver **cannot** rely exclusively on the squarefree branch. It must
include repeated-root strata or sound target randomization, with the
latter's cost included. B4 has *no* squarefree eight-leaf residual at all;
in B8, \(M|H_B\) and both are monic degree eight, forcing \(M=H_B\).
Consequently B8 provides a small *exact* seven-variable, eight-quadratic
planted-instance gate; it is not an unbiased proxy for B31 coverage.

The enumerator counts above were recomputed using exact integer
binomial arithmetic; they are elementary counts, not new runtime
measurements, and have no Lean or signed PCC certification.

## Rust implementation and prospective verification

Frozen prospective protocol:
[RR9_RADICAL_SUPPORT_PROTOCOL_20261117.md](RR9_RADICAL_SUPPORT_PROTOCOL_20261117.md).

Existing Rust code:
[prime31_rr9_evaluation.rs](../native/examples/prime31_rr9_evaluation.rs).
Implemented `rr9_poly_div_rem`, `rr9_poly_gcd_monic` and
`rr9_radical_support`. Same program retains the *independent* old power
gate and cross-checks new against old on **64 distinct positives,
64 target-preserving mutations, six repeated positives and six repeated
mutations**, all on the real prime31 curve. New pure-polynomial
controls cover partitions [8], [4,4], [5,2,1], [1^8] and omitted-root
negatives, a squarefree shortcut, noncanonical input and nonsquarefree
H rejection. Existing Hermite jet and original-group witness checks
remain unchanged.

**Source-specific CI: SUCCESS**, immutable source commit
`02e76e640aaf7407280de86ce81ba5f60a115347` on pinned
Rust 1.100.0, [Actions #44046676997](https://github.com/isogeny-labs/crypto/actions/runs/44046676997),
job `133913970897` at 2026-11-17 08:29 UTC. Logs show **40/40** native
library tests, **6/6** RR9 example tests (including the new radical
multiplicity control), and **64/64** distinct positive + **64/64**
distinct negative determinant-panel decisions. The example's on-curve
assertions additionally check all **70** valid support cases and **70**
target-preserving altered support negatives through the new radical
predicate alongside the old H^8 power predicate, plus original Hermite
repeated-root and ninth-jet controls. The [raw original-curve
128-case artifact](https://github.com/isogeny-labs/crypto/actions/runs/44046676997/artifacts/14312111559)
has artifact id `14312111559` (13,662 bytes). This is source-specific
producer CI only, **[pcc:unverified]**, not signed independent PCC or a
new complete-log benchmark.

## Cost frontier, prior searches, and next experiment

The matched historical #4532 producer-only 1024-Q Rust experiment measured
mixed 4+4 generic-oracle IC: **1,315.438ms setup + 906.035ms online =
2,221.474ms**, and cached generic BSGS (2^20 babies):
**103.840ms setup + 66.274ms online = 170.113ms** on that runner.
All 1,024 outputs per solver were individually replayed in that prior
study, but independent signed PCC was not achieved. It is not a newly
measured comparison. A purely local predicate improvement cannot
overcome a 1,315ms setup cost within a 170ms full-batch target. If setup
were **zero**, the current online 906ms must still fall by at least
5.3× to fit the full 170ms batch budget; with a 100ms setup budget the
online requirement is about 12.9×. Therefore the next experiment
must focus on a truly inexpensive **target-conditioned coefficient
extractor**, not more allocations or a faster supplied-certificate check.

Prior-lead review: #4466 Hermite/norm; #4539 signed chart;
#4228/#4349 Rust quotient, GCD/subresultants and roots;
#4373 anchored m8 F4 censored/negative;
#4484 scratch-buffer and #4486 linear-chord negative optimization gates;
#4513 complete mixed IC versus rho; #4532 strong cached generic BSGS;
crypto-autoresearcher #3301/#3305 alternative-Semaev negative heat maps.
Primary prior art: Amadori–Pintore–Sala 2018
(https://doi.org/10.1016/j.ffa.2018.01.009) and McGuire–Mueller 2017
(https://eprint.iacr.org/2017/1262.pdf). The classical gcd/squarefree
identity makes no cryptanalytic novelty claim. Research-KB MCP, checkout
wrapper and Conductor are not available in this runtime; the GitHub
paths/papers above are the explicit fallback.

**Priority 1:** On planted B8 public R, solve `M(A,V,R)=H_B` without
enumerating sign assignments, using the existing Rust F4/quotient work;
independently exhaust all 256 sign choices *only as a validator*, retain
both zero-fibre and off-fibre negatives. Show complete signed solution
sets, not a single planted witness.

**Priority 2:** Generalize to B31 squarefree unknown M with a monic
degree-23 cofactor L and the 31 coefficient equations
`M(A,V,R)*L=H_B` (mixed cubic/linear structure). Profile F4 matrix
size, cancellation and root recovery, and reject this lead if symbolic
elimination exceeds the cached-BSGS budget before complete target solving.
Add a separate repeated-x branch since squarefree-only completeness is
mathematically impossible for all residuals.

**Priority 3:** If the first two gates survive, recover fresh blind
unknown scalars in the original Rust solver on the true p=2^31-1 curve,
charging every failed target, setup, linear algebra and memory; compare
same-host, equal-precomputation shared BSGS, tuned rho and external
prime-field IC. This PR has **zero** new complete unknown-target ECDLP
recoveries and no measured solver cost reduction.
