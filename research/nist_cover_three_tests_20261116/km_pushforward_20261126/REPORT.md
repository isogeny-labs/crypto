---
title: "P-192: callable elliptic pushforward from opaque KM input"
date: "2026-11-26"
geometry: margin=0.65in
fontsize: 10pt
header-includes: |
  \makeatletter
  \renewcommand{\maketitle}{\begin{center}\Large\@title\par\small\@date\end{center}\vspace{0.5em}}
  \makeatother
---

The backward map $B=f_*$ is now callable directly from a valid rational KM
class space. The external facade accepts a JSON object containing only its
$22\times85$ evaluation coordinates and returns an elliptic point. It
normalizes infinity support, extracts an equivalent degree-three divisor,
checks re-encoding in the actual target Jacobian, then applies the actual
degree-six curve map in separate OpenSSL arithmetic. A literal single-key
input completed that entire path and returned $[6]G$ for the retained public
$F(G)$ class. [pcc:unverified]

All twelve frozen end-to-end cases pass, including nonzero classes, infinity
multiplicities zero through three, a special Riemann--Roch space of dimension
two, repeated divisors and a triple at the retained affine pole. The
standalone expectation-free map returns the same points for every case.
Three mutations per case reject, and an off-ambient class input rejects with
its explicitly expected exit 1. Successful workers and wrappers exit zero.
[pcc:unverified]

| Fixed family | Cases | Required outcome |
|---|---:|---|
| Principal representations with $mH_\infty$ | 4 | Detect $m=0,1,2,3$; result $O$ |
| Nonzero class, infinity multiplicities 1 and 2 | 2 | Same independently evaluated point difference |
| Actual public $F(G)$ and its KM double | 2 | Fresh $[6]G$ and $[12]G$ |
| Special effective divisor | 1 | RR dimension 2 and correct elliptic image |
| Triple and double-plus-single reference supports | 2 | Exact multiplicities and correct image |
| Triple at the original affine pole | 1 | Exact triple retained; pole evaluator reached |

![The blue stages use the retained GMP target arithmetic; the green stages use the separate OpenSSL curve and elliptic backend. The external interface supplies only class coordinates.](flow.pdf){width=98%}

## The input and output contracts

The supplied matrix represents a **valid** effective-degree-12 divisor $D$
through $W_D=L(9H_{56}-D)$, and hence the class $[D-3H_{56}]$. The
coefficient field and evaluation ordering are the fixed P-192 model's.
Canonical coefficients, dimension, ambient membership, normalization and
actual KM equalities are checked. Arbitrary 22-dimensional subspaces are
outside this valid-RR-input contract. [pcc:unverified]

The adapter accesses only `class_space`; it does not use a generating scalar
or support decomposition. The original test fixtures additionally hold
expectations for the separate checker. The final
[class-only invocation](evidence/class-only-v1/) removes those fields from
the supplied bytes: its JSON has exactly one key, `class_space`. The actual
map executable accepts the resulting cycle certificate and fixed curve
context, with no expected-answer argument. [pcc:unverified]

The output is a canonical elliptic point, including the identity flag. This
connects general valid rational class input to the morphism $f_*$ used in
the [actual-map composition proof](../composition_verification_20261126/REPORT.md):
$f_*\phi\pi^*=[6]$ on the stated smooth models. The older affine
$\Gamma^t$ implementation is not used to obtain the answer. [pcc:unverified]

\clearpage

## From the input space to the elliptic point

The first stage reuses the retained infinity normalizer and opaque
Riemann--Roch decoder. Normalization detects the input representation's
infinity multiplicity from reconstructed numerator degrees and returns an
equivalent finite representation. The decoder then finds an effective
degree-three divisor $E$ with

$$[E-E_{\rm ref}]=[D-3H_{56}].$$

It retains the complete intersection, projective chart, separating
coordinate, repeated support and re-encoded class. Equality with the
**original supplied class** is required in actual GMP target arithmetic.
The RR derivation and base-point-free division are the existing
Khuri-Makdisi construction; the wrapper imports its original guarded
sources rather than changing their algorithms. [pcc:unverified]

The fixed plane quartic's infinity intersection is one closed point of
degree four. A rational effective divisor of degree three cannot contain
it. Thus the extracted $E$ is affine even when the input representation
contains infinity support. This is the existing model-specific reason the
degree-three interface can use finite coordinates. [pcc:unverified]

The independent OpenSSL stage checks the full separated algebra

$$R=\mathbb F_p[z]/u(z),\qquad x=x(z),\quad y=y(z),\quad\deg u=3,$$

including $z=x+\lambda y$ and the actual quartic equation modulo the whole
$u$. It factors $u$, replays its complete factor product with multiplicities
and checks each factor's irreducibility. For each closed point $P$ of
degree $d$ and multiplicity $m_P$, it computes the actual $f(P)$ and all
Frobenius conjugates. The result is

$$B([E-E_{\rm ref}])=
 \sum_P m_P\sum_{j=0}^{d-1}f(P)^{(p^j)}
 -\sum_{Q\in E_{\rm ref}}f(Q).$$

Every trace must descend to $\mathbb F_p$. The three reference coordinates
are compared with the original frozen pool and formal reference, and their
images are freshly evaluated. Exact support and power-basis-rank checks
bind the repeated-divisor cases to their original complete cycles.
[pcc:unverified]

Where the rational-function denominators are units, the evaluator uses
their exact values. At a denominator zero it lifts that point's own smooth
affine chart and evaluates Laurent ratios at fixed precision
$4\max\deg+1=45$. It checks either finite elliptic coordinates or pole
orders $-2e,-3e$ and the leading elliptic equation. The triple-pole case
actually reaches this path; its multiplicity three remains in the divisor
algebra. [pcc:unverified]

The arithmetic separation is explicit: GMP performs RR extraction and KM
class equalities; OpenSSL checks the decoded curve algebra, map evaluation,
multiplicities, elliptic traces and test expectations. This does not supply
a second implementation of the whole KM group law or an independent
signature. Smooth-model and standard divisor pushforward hypotheses remain
as in the prior reports. [pcc:unverified]

\clearpage

## Calling and reproducing the interface

From the repository root, qualify fresh build trees:

```bash
study=research/nist_cover_three_tests_20261116/km_pushforward_20261126
bash "$study/run.sh" evidence/replay-main-01
bash "$study/run-eval.sh" evidence/replay-eval-01 evidence/replay-main-01
```

The first command builds the fixture preparer, external class extractor and
separate checker, then executes the fixed panel. The second builds the
standalone $f_*$ executable from the exact already-tested evaluator
functions. Its explicit second argument selects the fresh extraction tree;
compiled binaries are local and must be built in a new checkout.

For a supplied valid `class.json`, call:

```bash
bash "$study/evaluate-class.sh" \
  "$study/evidence/replay-main-01" \
  "$study/evidence/replay-eval-01" \
  class.json "$study/evidence/new-query-01"
```

The new output directory must not exist. `result.json` contains the elliptic
image; `extracted.json` contains the complete RR certificate. The facade
snapshots the supplied input, records both native stages and their exits,
and hashes all fixed inputs, source and binary subjects before and after
calls. It accepts no expected answer or source scalar. [pcc:unverified]

To reproduce the final literal class-only qualification from those fresh
builds, use another fresh directory:

```bash
bash "$study/check-class-only.sh" \
  "$study/evidence/replay-main-01" \
  "$study/evidence/replay-eval-01" \
  "$study/evidence/replay-class-only-01"
```

This selects the already prepared public matrix as JSON metadata, removes
all other fields, invokes the full facade and compares the result natively
with the preceding separately checked public result. Python is used only
for JSON bookkeeping and CPU-affinity selection; every field, divisor,
curve-map and elliptic calculation is native C++. [pcc:unverified]

## Bounds and control scope

The retained RR schedules use projective-chart parameters 0 through 24 and
shears 0 through 308. The factor splitter retains shifts 0 through 1023.
Any exhaustion rejects and is not converted into a valid answer. There is
no larger schedule, alternate selected point or higher jet precision after
observing a failure.

The off-ambient input changes one evaluation coordinate and is required to
fail in the adapter. The separate checker changes a decoded ordinate,
omits a complete-support factor and changes the elliptic answer by adding
nonzero public $G$. All intended rejections occur. The twelve ordinary
cases, the standalone evaluator, explicit-tree replay and class-only facade
all pass. [pcc:unverified]

\clearpage

## Complete executed costs

All four rows are nonoverlapping complete local wrappers. Their total is
**189.715 diagnostic wall seconds**. Each includes every nested build,
preparation, extraction, evaluation, comparison and facade call executed
inside it. Nested intervals must not be added again. [pcc:unverified]

| Wrapper | Wall (s) | User (s) | System (s) |
|---|---:|---:|---:|
| Twelve-case end-to-end, attempt-v1 | 141.819 | 139.004 | 2.455 |
| Standalone interface, eval-v1 | 19.955 | 19.234 | 0.696 |
| Explicit extraction-tree replay, eval-v2 | 20.266 | 19.462 | 0.748 |
| Literal class-only facade, class-only-v1 | 7.675 | 7.585 | 0.073 |

The initial wrapper includes 36.548 seconds of strict builds and 105.073
seconds of native calls, with the remaining wrapper interval covering its
bookkeeping. Per-case extraction and separate checking intervals are:
[pcc:unverified]

| Case | Extraction wall (s) | Checking wall (s) |
|---|---:|---:|
| Principal $m=0$ | 6.743 | 0.293 |
| Principal $m=1$ | 6.955 | 0.304 |
| Principal $m=2$ | 6.870 | 0.304 |
| Principal $m=3$ | 6.696 | 0.283 |
| Nonzero $m=1$ | 7.984 | 0.259 |
| Nonzero $m=2$ | 8.084 | 0.276 |
| Public $F(G)$ | 7.334 | 0.257 |
| KM double of $F(G)$ | 7.529 | 0.306 |
| Special RR dimension two | 7.249 | 0.375 |
| Triple reference | 7.391 | 0.242 |
| Double plus single reference | 7.274 | 0.340 |
| Triple pole | 7.860 | 5.735 |

The final facade is a nested 7.608-second API wrapper: extraction 7.353
seconds and pushforward 0.226 seconds. Its native comparison takes 0.005
seconds; all are included in the final 7.675-second row. Full native setup,
normalization, RR and map/trace phase clocks and all exact inputs/results
accompany the [cost summary](evidence/admin-v1/cost-summary.json).
[pcc:unverified]

Every native process retains one allowed CPU, 1 GiB address space, a
300-second strict-build cap and a 600-second worker cap. Calls run
sequentially. All ordinary exits are zero; the malformed-space call exits
one as required. No native build failed, mathematical expectation failed,
timeout occurred or limit increased. The replay-path correction and prose
lint stops are retained in the journal. [pcc:unverified]

Peak RSS, literature, reporting, coordination and publication costs lie
outside these wrapper diagnostics. Historical original synthesis remains
unpriced: six inspected stage-result records and the quartic model have no clock fields;
one elapsed value in the Prym input does not price the complete construction.
No historical duration is inferred from timestamps. Earlier relation/rank,
projective-map and composition evidence remains immutable. The explicit
coordinate-screen/matched-rho waiver persists; no new PCC ID or performance
comparison is added. [pcc:unverified]

Sources: retained opaque-KM/normalization/boundary protocols and exact native
sources; Khuri-Makdisi,
[*Linear algebra algorithms for divisors on an algebraic curve*](https://arxiv.org/html/math/0105182v3),
sections 2 and 5; Galbraith's corrected
[chapters 7](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch7.pdf) and
[10](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch10.pdf);
[PR #5255](https://github.com/isogeny-labs/crypto/pull/5255) and
[PR #5256](https://github.com/isogeny-labs/crypto/pull/5256). The shared KB
remains unavailable as previously recorded. Established methods are reused;
no novelty or signed verification is asserted. [pcc:unverified]
