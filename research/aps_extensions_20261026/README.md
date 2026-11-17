# APS single-relation baseline and extensions

**2026-10-26. Status: bounded native correctness implementation; performance validation open.**

The request is to track the partitioned summation-polynomial idea and further
extensions, implement them in Crypto, and test whether they work. This study
implements the paper's single-relation route and five extension mechanisms.
The correctness checks use public tiny fixtures. They do not reproduce the
paper's 11–22-bit benchmark ladder or establish a practical improvement.

| Requirement | Delivery | Remaining acceptance criterion |
| --- | --- | --- |
| Track ideas and related work | [TRACKER.md](TRACKER.md), [SOURCES.md](SOURCES.md) | Carry the results into the next sibling lead-atlas review |
| Native paper baseline | `aps_single_relation`, random labels, partitioned S3 and joined-S3 S4 | Reproduce larger parameter ladder with every phase charged |
| Implement extensions | Fixed coefficient labels, unordered divisor and two-pair encodings, automorphism orbits, covering partitions, expanded/split/hybrid solving | Optimize and compare under a frozen protocol |
| Validate actual relation solving | Exact solution-set comparisons against point addition; checked DLP recovery with no fallback | Independent transcript replay and broader curve coverage |
| Formal and algebraic checks | [Lean coefficient lemmas](lean/Coefficients.lean); native S3 identity certificate | Elliptic-curve bridge and implementation correspondence are not formalized |
| Performance and scaling assessment | [Panel design](PROTOCOL_PLAN.md) | Complete accounting, freeze protocol, S3 checkpoint, controlled execution and external solver comparison |

## What is implemented

The native module is [aps_single_relation.rs](../../src/cryptanalysis/aps_single_relation.rs).
It reuses [f4_fp](../../src/cryptanalysis/f4_fp.rs) and
[prime_fast](../../src/cryptanalysis/prime_fast.rs). The
[example](../../examples/aps_single_relation_check.rs) exposes the public
fixtures and writes retained attempts, labels, coordinates, algebra reports,
full curve identities, and verified answers as JSON.

The pipeline is:

1. Construct points `R = uG + vQ` with known labels, without giving the
   relation solver the planted scalar.
2. Require distinct abscissae and build the exact membership constraints.
3. Solve the requested algebraic system with native F4 and finite-field
   substitution. A degree bound or timeout is recorded; an unresolved
   search is never reported as complete.
4. Decode coordinates, try point signs, and check the zero sum by curve
   addition. Accumulate `AG + BQ = O`, reject `B = 0`, compute `d = -A/B`
   modulo the prime subgroup order, and verify `dG = Q`.

There is no log table, decomposition oracle, generic-DLP fallback, or
planted-scalar argument in that path. The exhaustive point oracle belongs
to the tests, outside the solver. It is independent of polynomial construction
and uses the same native curve arithmetic; it is not a separately implemented
elliptic-curve backend.

The field is explicitly bounded by `MAX_FIELD = 65521`. Sampling and DLP
attempts also have caps. The substitution backend can enumerate field values;
its success on these fixtures provides no evidence that enumeration disappears
at large sizes.

## The five extensions

**APS-E1 — choose informative coefficient labels.** For odd arity, points
sampled as `uG + Q` have an odd signed coefficient sum. With `n > m`, it
cannot vanish modulo `n`. For even arity, one point from each partition can
use weights `1,2,4,...`; require `n > sum(weights)`. Canonical sign changes
are applied to both labels. Regrouping an even-arity base into unordered
blocks, or expanding automorphism orbits, does not inherit this guarantee;
the recovery check remains mandatory. These choices change construction
and relation informativeness; no faster polynomial solving is inferred.

**APS-E2 — encode unordered selections.** For the squarefree polynomial
`f(T) = product(T-v)`, require a monic cubic
`H(T) = T^3-e1*T^2+e2*T-e3` to divide `f`. The three remainder coefficients
vanish exactly when `H` selects three distinct coordinates from the base.
The established symmetric S3 rewrite is
`(e2-a)^2 - 4*e1*(e3+b) = 0`. The decoder requires three distinct rational
base roots. A second encoding selects two unordered pairs from two disjoint
blocks and joins them through an auxiliary abscissa. This removes permutation
copies without imposing one particular three-way partition. The remainder
equations can be much harder to solve; degree reduction is not claimed.

**APS-E3 — use automorphism orbit bases.** On the public `j=0` fixture,
`phi(x,y)=(beta*x,y)` and the verified subgroup eigenvalue `lambda` propagate
known labels through whole orbits. An orbit's vanishing polynomial is sparse.
The intrinsic relation `R+phi(R)+phi^2(R)=O` has `A=B=0` and reveals nothing;
the tests explicitly reject it and recover the fixture answer from an
informative relation across orbit blocks. Related GLV work already exists
here; see the source map. Sparse equations alone do not imply cheaper F4.

**APS-E4 — cover the excluded triples.** A deterministic bit-based
three-color separating family covers every distinct triple without looking
at its relation status. For a base of size `s`, it uses at most
`2*ceil(log2(s))*(ceil(log2(s))-1)` colorings before deduplication. This is
an `O(log^2 s)` construction, not an optimal or balanced perfect-hash family.
`solve_cover` shares a deadline across all systems and deduplicates their
decoded coordinate sets. Coverage does not imply a lower total cost.

**APS-E5 — change the solver presentation.** Expanded S4, the paper's
joined-S3 representation, and first-coordinate hybrid guessing use the same
base and checked relation path. Hybrid branches share one deadline. The
joined-S3 presentation is already in APS §3.4; the work here is an explicit
native comparison interface and correctness checks, not a novelty claim.

## Evidence and limits

The final locked release checks pass 15 module and four fixture-runner tests.
The separate isolated review build also passes those tests. They cover
complete solution sets, informative and uninformative relations, every
encoding's DLP route, label integrity, malformed inputs, covering-family
coverage (sizes 3–32 and 64), and deadlines. Integration checks and the
example's tests are recorded in [validation/README.md](validation/README.md).
The full library suite remains pending on an unchanged isogeny-walk test;
seven pre-existing strict root lint failures are recorded there separately.

Fixtures are `y^2=x^3+59x+6` over F101 (`n=107`, `G=(78,4)`, public `d=37`),
`y^2=x^3+166x+158` over F211 (`n=227`, `G=(155,121)`, public `d=73`), and
`y^2=x^3+5` over F103 (`n=97`, `G=(2,42)`, public `d=17`). Point-order checks
use scalar multiplication that does not reduce by the declared order.
The runner records their canonical ICV1 identities.

The [core-only Lean proof](lean/Coefficients.lean) passes the repository
proof gate: 16 audited theorems including generated equation lemmas, zero
failing and zero trusted declarations. Its eight named lemmas prove parity,
bounds for signed ones, and modular nonvanishing under stated bounds. For
signed powers, the magnitude bound is a hypothesis; the elliptic-curve
bridge and Rust correspondence remain tested rather than formalized.

The native certificate [identity.json](identity.json),
`IDC1h32f0b3e300bbf951`, checks the established symmetric S3 identity over
the certificate format's field, including a second independent test seed.
It is not a formal proof in every characteristic. The certificate and receipts
are linked in the validation record.

F4's `solving_degree` is the existing backend's top-level productive degree,
not a maximum over the substitution tree or the final basis degree. Its
field-multiplication counter covers F4 work only and can interleave under
concurrent solves. Setup, expression construction, point arithmetic,
root decoding, duplicate handling and verification are not fully priced.
No counter here is reported as a complete attack cost.

There is no measured speedup, rho comparison, fitted exponent, leaderboard
promotion, or conclusion about cryptographic-size curves. The implementation
does not reproduce APS's `m=5` experiments. Arbitrary multiplicative-coset,
trace, rational-map and isogeny-map bases are follow-ups, not completed arms.

## Reproduce correctness and proofs

```sh
cargo test --release --locked --lib aps_single_relation -- --test-threads=2
cargo test --release --locked --example aps_single_relation_check
tools/lean/proof-gate.sh research/aps_extensions_20261026/lean/Coefficients.lean
```

With the pinned Lean toolchain installed, the proof needs only `Std`.
To regenerate only the algebraic certificate (no experiment):

```sh
CHECKPOINT_SKIP='identity certificate generation only; no experiment' \
  cargo run --release --locked --example aps_single_relation_check -- \
  certificate --output research/aps_extensions_20261026/identity.json
```

The `solve` subcommand is a bounded diagnostic runner, not an ecbench export.
Before using it in an experiment, complete the frozen protocol and S3
checkpoint gates. Its built-in target scalar is used only to construct and
check a public fixture; the relation solver receives `(E,G,Q,n)`.

## Next work and workflow

[TRACKER.md](TRACKER.md) and the blocked
[backlog card](../../backlog/XP-20261026-aps-complete-cost-panel/README.md)
preserve the larger validation request and explicit
acceptance criteria. The next rung is complete accounting and transcript
replay, followed by the controlled panel in [PROTOCOL_PLAN.md](PROTOCOL_PLAN.md).
That document is a design, not a frozen protocol or an executed experiment.

Conductor was installed and a scoped task was claimed using the repository's
documented container-local setup ([docs/CONDUCTOR.md](../../docs/CONDUCTOR.md)).
It coordinates this checkout only. No shared endpoint was supplied. The S3
upload credential is absent, and no benchmark experiment was launched.
Ordinary builds, fixed correctness tests and proof checking continued.

Diagram and PDF: [pipeline.svg](pipeline.svg), [report.pdf](report.pdf),
editable sources [pipeline.dot](pipeline.dot) and [report.tex](report.tex).
Existing scoreboards, performance graphs and leaderboard rows are unchanged
because this study makes no performance result eligible for promotion.
