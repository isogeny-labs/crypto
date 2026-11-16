# EXP-R31-F — certified finite pair-abscissa elimination (2026-11-15)

**Scope:** algebraic preprocessing component for the existing **Rust** prime-field
index-calculus project. **[pcc:unverified]** producer arithmetic and focused CI
only; no new algebraic relation extraction, complete prime31 solve, cost win,
independent PCC signature or new complexity exponent is asserted.

## Starting point and prior-result review (before selecting this experiment)

The fixed Semaev S3 constraint was the first actual Rust F4 failure on all
16 naturally sampled public masks: 16/16 stopped on the *same* 26,618-entry
dense reducer under a 15,000-entry cap, before target-dependent S4 changed
the measured work. The two-tower minimal S3 ablation did not fix it
([EXP-R31-D](M8_NATURAL_F4_CI_OUTCOME_20261115.md)).
Seven first-collision arms, with the redundant Dickson tower collapsed,
still stopped on five of seven arms; their summed development F4 multiply-add
count increased from 538,689 to 732,473
([EXP-R31-E](DICKSON_FIRST_COLLISION_S3_20261115.md) and
[exact CI counters](DICKSON_COLLAPSED_S3_CI_EVIDENCE_20261115.json)).
Those numbers are *censored component* counters, not complete cold costs.

Before choosing this change I rechecked the following existing paths:

- [crypto #4077](https://github.com/isogeny-labs/crypto/pull/4077):
  useful independent rows and *whole-call* gamma/delta, not positive-witness
  counts or lone stage speedups, are the advancement metric.
- [crypto #4275](https://github.com/isogeny-labs/crypto/pull/4275):
  dense membership beat Dickson towers at small *planted-positive* cells
  but enumeration remained cheaper, so an expensive dense polynomial is
  not presumed an improvement.
- [crypto #4228](https://github.com/isogeny-labs/crypto/pull/4228):
  certified quotient solving, paired S3 and linear subresultants; this is
  a possible target-dependent post-elimination implementation, not a result
  already obtained on the present natural-prime31 panel.
- [crypto #3877](https://github.com/isogeny-labs/crypto/pull/3877):
  chained transfers still need an independently computable scalar-free
  point map and full cost, so they are not substituted for this oracle.
- [cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536):
  optimized auxiliary-map index calculus is already in Rust, but no
  same-target qualified full-cost advantage is reported there.
- [crypto-autoresearcher #3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305):
  the prior semaev-alternatives campaign and lead-atlas rules explicitly
  warn against toy-stage positives, uncertified closures and omitted rho
  totals. This remains an independent prior-lead register.
- [Petit–Kosters–Messeng, PKC 2016](https://doi.org/10.1007/978-3-662-49387-8_1)
  gives the established rational-map/Semaev prime-field IC framework;
  [Diem, Compositio 2011](https://doi.org/10.1112/S0010437X10005075)
  provides extension-field and special prime-field complexity regimes.
  The S3 quadratic and polynomial projection below are algebraic
  consequences of those established methods, **not a novelty claim**.

The shared research-KB MCP server and its repository wrapper were not
available in this runtime. This is a **bounded**, source-linked GitHub and
primary-literature review, not a comprehensive novelty clearance.

## Exact finite-field elimination theorem (on the fixed public curve)

The tested domain is the inherited ordinary cofactor-one curve

    p = 2147483647 = 2^31-1
    E: y^2 = x^3 + 847514846*x + 2087444724
    #E(Fp) = n = 2147446831 (prime)
    G = (625037715, 2100627251)

The deterministic depth-six Dickson preimage fibre has 64 distinct x values,
31 of which lift to canonical finite nonzero-y elliptic-curve points
P_i=(x_i,y_i). This is the same fixture as #4188; no smooth subgroup or
known-secret input is introduced.

Write S3(x,y,w) as a polynomial **quadratic in w**:

    a = (x-y)^2
    b = -2*((x+y)*(x*y + A) + 2*B)
    c = (x*y - A)^2 - 4*B*(x+y)
    S3(x,y,w) = a*w^2 + b*w + c

Let \`R_{ij}\` be the finite Fp roots of S3(x_i,x_j,w), including
i=j. For i<j, there are two roots: x(P_i+P_j), x(P_i-P_j).
For i=j, the quadratic degenerates to one finite root: x(2P_i);
the other sign choice produces the point at infinity and **has no
affine x-coordinate**. The field is of odd characteristic, the group
has prime odd order, and the tested P_i are nonzero, so these cases
have no extra zero/torsion exceptions.

Define the **monic field polynomial**

    H(W) = product_{0<=i<=j<31} product_{w in R_ij} (W-w) in Fp[W].

Then for *every* rational w in Fp,

    H(w)=0  <=>  some i<=j has S3(x_i,x_j,w)=0.

This is exact existential elimination of the two fixed Dickson S3 leaves
**for rational coordinates over this chosen factor base**. The equivalence
is immediate because Fp is a field and H is an explicit product of linear
factors. The construction also retains a preimage index for each w
so any *later* target-dependent S4 solution can reconstruct its contributing
point pair and check the full public point with y signs.

### Exact measured finite-instance certificate

| Item | Exact result |
|---|---:|
| Liftable canonical factor-base points | 31 |
| Unordered base pairs, including diagonal | 496 |
| Two-root distinct pairs | 465 |
| One-root diagonals | 31 |
| Algebraic S3 finite roots, with multiplicity | 961 |
| Distinct finite w roots | **961** |
| Degree of squarefree H | **961** |
| First-collision branch polynomial degrees, j=0..5 | 16, 20, 62, 124, 240, 468 |
| Diagonal branch polynomial degree | 31 |
| H leading coefficient | 1 |
| First six H coefficients, constant first | 1384764775, 1730047243, 2061394966, 644849095, 1634675601, 1780857477 |
| Sorted-root SHA-256 (u32 LE) | \`f83e7ecdd585af6d9457b054e61d66c957624a7cc5a9719b62ca92e4afe7aca6\` |
| Coefficient SHA-256 (u32 LE) | \`a1b9f09503c9eb08ff70bca1f7cf6dd7b9c575c924bc4f5530580f7ca4852e3a\` |

The collision-free preimage count is a *measured property of this specific
31-point base*: each w determines one unordered pair and one S3-root
index. It is not asserted for arbitrary curves or growing B.

Construction needs 496 field S3 quadratics and 961 rational roots. A
naive incremental monic-product builder uses exactly
\`961*962/2=462241\` modular coefficient multiplications for H
(excluding inversions, square roots, polynomial checks and other field
work). The seven branch products separately require 149,211 such
multiplications. These are transparent analytic operation counts,
**not a same-host runtime, a full oracle invocation cost, or evidence
that the result beats EC group enumeration**.

The independent Python replay reconstructs the curve base and all signed
group sums with a separate group-law implementation, checks every one of
the 961 S3 root abscissae, multiplies the seven factor polynomials,
and compares both complete SHA-256 hashes. The Rust example uses
the existing \`Arithmetic\`, \`Base::build\` and \`PRIME31\`, generates
candidate abscissae solely by the field S3 quadratic, and uses EC
addition only as a semantic validator **after** root construction.

## Actual algorithmic opportunity and limitation

Replace the **fixed** S3(x6,x7,w) plus two depth-six leaf towers in the
balanced m8 system with the exact rational condition H(w)=0. This is
an algebraically valid projection for finding x-coordinate solutions.
Full integer-index relation extraction still requires selecting a
preimage (i,j), sign-lifting, rejecting repeated indices if required,
and point-verifying against the original public R. Those are explicit
steps, not implicit consequences of H.

The existing packed Rust F4 configuration has a 12-degree bound, while
H has degree **961**. Therefore simply inserting H into that F4 ideal
is **not executable under the registered cap** and is not advertised
as a speedup. The two implementation alternatives to measure, without
pretending to have already measured them, are:

1. **Specialize w** to the 961 certified roots with cached provenance.
   This removes the two pair towers and S3 equation, leaving the six
   other depth-six blocks and two auxiliary variables u,v. First run
   all branches on a frozen, bounded diagnostic panel, recording every
   failed/successful specialization and at most one prior-known positive
   *validator*; never select w using a secret or test-only oracle.
2. **Certified quotient/CRT or streamed branch elimination:** represent
   the seven H_j factor sets, or a factored fixed quotient, without
   expanding degree-961 H into a degree-12 F4 system. Audit the
   projection/extension and back-substitution, and test actual F4
   target-dependent S4 work. Compare with #4228 and dense #4275,
   using the same frozen positive and negative natural masks.

The *first falsifier* is whether the target-dependent left/right S4 ideal
still oversizes under a six-block, two-free-variable specialization;
the earlier separate left S4 ablation also oversized when all eight
unused tower blocks were retained. A smaller ring is a hypothesis, not
a benchmark. Any method that searches all 961 w values must charge
every attempt and all fixed preprocessing once per cold target.

## Experiment protocol / promotion gates

- **R31-F1 (automated development test):** run the independent Python
  and existing-Rust certificate programs on this exact source head.
  Require the 31-point curve fixture, all 496 pair laws, 961 unique
  finite roots, seven factor polynomials with the exact degrees/hashes,
  256 nonmember controls, and no compilation/test failure.
- **R31-F2 (bounded true algebraic pilot):** preregister the same 16
  natural public masks from EXP-R31-D plus held-out masks. In the real
  Rust S4 solver try w-specialization and/or factored-quotient
  preprocessing with **all** branches, no generic pair-table fallback,
  complete call/censor/memory/root counters, and genuine signed outputs.
  Keep 12-degree, 15k dense/NNZ and five-second per-call controls for
  *development diagnostics*; request a separate EKS admission if a
  larger registered study is needed. The 961 specializations must
  never be silently discounted.
- **R31-F3 (complete cold IC):** only after natural algebraic extraction,
  collect projective-distinct independent rank on public targets, solve
  the modular base-log system and independently verify unknown Q's
  scalar. Include base generation, H construction, *every* F4,
  rejection, root, rank, LA, memory and final check. Compare against
  optimized same-curve Rust negation-map rho and BSGS using the
  registered ecbench/PCC accounting (whole-call gamma/delta #4077).
  Independent audit and growing-field holdouts are mandatory before
  any solver cost or complexity improvement claim.

No paid EKS execution, new leaderboard score or PR merge is authorized by
this *component* report. The implementation is intended for the existing
open [crypto PR #4188](https://github.com/isogeny-labs/crypto/pull/4188);
preserve historical producer data and the independent PCC gate.

## Reproduction

- [Rust certificate](examples/prime31_pair_elimination_poly.rs):
  \`cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_pair_elimination_poly\`
- [Independent Python certificate](replay_pair_elimination_poly.py):
  \`python3 research/prime31_ic_20261115/native/replay_pair_elimination_poly.py\`
- Development workflow:
  [prime31-algebraic-ic.yml](../../../.github/workflows/prime31-algebraic-ic.yml)

**Current verdict:** exact fixed-pair algebraic projection and collision-free
finite-root catalogue demonstrated by producer arithmetic; Rust exact-head
execution is separately gated by CI. It is a useful elimination component,
not yet a natural-target m8 oracle, a prime31 scalar solver, or a
cheaper-than-generic ECDLP result.


## First actual target-dependent S4 pilot after exact pair elimination

The focused [GitHub Actions run 43724705218](https://github.com/isogeny-labs/crypto/actions/runs/43724705218)
successfully compiled and ran the new \`--preeliminate-w-f4\` mode of the
[existing Rust balanced m8 example](examples/prime31_m8_balanced_f4_input.rs).
The producer output and source pin are frozen in
[M8_S4_AFTER_PAIR_ELIMINATION_CI_20261115.json](M8_S4_AFTER_PAIR_ELIMINATION_CI_20261115.json).
This is **developer CI**, not ecbench/PCC certification.

The experiment mathematically removes both fixed pair leaves and the fixed
S3 equation, specializes \`w\` to a certified rational field root, and reuses
the actual Rust \`f4_fp_tower.rs\` on the **remaining original three S4
constraints**. The quotient has 36 Dickson tower variables and two free
abscissae, rather than the former 48 and three. The eight frozen calls
include **one known-natural-positive witness used strictly after F4 to
validate polynomial soundness**, and seven preselected public-Q roots
(one from each of the six first-collision branches and the diagonal).
These do not constitute all 961 roots or a complete target oracle.

| Frozen arm | Degree reached | F4 muladds charged before stop | Outcome |
|---|---:|---:|---|
| Known natural-positive soundness control | 10 | 56,211 | Oversize / undetermined |
| Public Q, first collision j=0 | 10 | 56,211 | Oversize / undetermined |
| Public Q, j=1 | 10 | 56,211 | Oversize / undetermined |
| Public Q, j=2 | 10 | 56,211 | Oversize / undetermined |
| Public Q, j=3 | 10 | 56,211 | Oversize / undetermined |
| Public Q, j=4 | 10 | 56,211 | Oversize / undetermined |
| Public Q, j=5 | 10 | 56,211 | Oversize / undetermined |
| Public Q, diagonal | 10 | 56,211 | Oversize / undetermined |

**8/8 censored; 0 extracted algebraic relations, 0 recovered logarithms.**
The partial F4 multiply-add sum is **449,688** across these eight
diagnostic calls, excluding root generation, polynomial construction and
all other phases. The F4 report's \`max_dense_entries=10229\` is a
previously *accepted* reducer peak, **not** the size of the attempted
next reducer that triggered \`oversize\` under the 15,000 cap. The
~9–10 ms per-call engine clocks are not complete process timings.

**Interpretation and falsifier:** removing the fixed S3 bottleneck moved
the observed stop from degree 6 to degree 10 but did not make the remaining
S4 system soluble at the frozen cap, even for an independently witnessed
positive case. Full 961-way specialization under this same cap is therefore
**not a useful next step**: it would repeat an already-censored solver
configuration. The identical counters suggest (but do not by themselves
prove) an early target-independent S4 reducer; a pre-registered left/right
single-constraint ablation can verify the precise cause. Prioritize a
certified #4228 S4 quotient/extraction or a demonstrably stronger
external algebraic engine before considering complete specialization.

For the prior **all-eight-distinct-index** m8 benchmark, the diagonal
branch is a deliberately overinclusive algebraic soundness control:
its 31 values represent \`x(2P_i)\` and necessarily repeat a base index.
If distinctness is imposed, the correct fixed pair polynomial is
\`H_distinct = H / H_diagonal\`, exactly **degree 930** with branch factors
j=0..5. Distinctness filtering is not optional when claiming usable
eight-point relations. This refinement removes the 31 diagonal
specializations from any subsequent *complete distinct-index* search.

Even after this component improvement, no full cold prime31 scalar
recovery or cheaper-than-matched-rho result has been measured.


## S4 ablation: the remaining stop occurs in each side separately (Rust CI)

[Actual Rust CI run 43725152557](https://github.com/isogeny-labs/crypto/actions/runs/43725152557),
head \`7bdd55c0425048fb64c0b46bf432c6f03e9b72f3\`,
passed the focused development job. Its exact six reduced-ideal records are
preserved at [M8_S4_SIX_TOWER_ABLATION_CI_20261115.json](M8_S4_SIX_TOWER_ABLATION_CI_20261115.json).
The frozen natural-positive witness was checked against each ideal before
calling the same shared Rust F4. These **reduced ideals are diagnostic
subsystems, not usable m8 relation oracles**.

| Six-block original-curve S4 ideal | Degree reached | F4 muladds before stop | Outcome |
|---|---:|---:|---|
| Left S4 only | 10 | 14,129 | Oversize |
| Right S4 only | 10 | 14,129 | Oversize |
| Target S4 only | 0 | 0 | Basis, no extraction |
| Left + right | 10 | 56,211 | Oversize |
| Left + target | 10 | 14,129 | Oversize |
| Right + target | 10 | 14,129 | Oversize |

Each individual left/right S4 already hits the **15k reduced-matrix cap**.
All eight previously run full remaining-S4 specializations had the same
56,211 partial muladds. This isolates a **target-independent S4-side
bottleneck after removal of fixed S3**, not a dependence on the chosen w or
public target. No roots and no public discrete logs were extracted.

The next minimally confounded experiment is a **single S4 with only three
Dickson six-level towers plus one free abscissa**, removing the unused other
three blocks. If the single-block S4 is still oversize under identical limits,
the actionable next lever is the S4 resultant representation or a certified
quotient/#4228 or mature solver, not additional target/pair specializations.
No full-cost or rho advantage is inferred from these partial counters.


## Minimal S4 falsifier completed: three towers are sufficient to reproduce the stop

[Exact-source Rust development CI 43725436615](https://github.com/isogeny-labs/crypto/actions/runs/43725436615)
(success), head \`cb2213463ba7171be48dddc574ebb401cf1ebb51\`,
reused the unchanged original \`f4_fp_tower.rs\` and the same S4 resultant
to test each side with only **three active depth-six Dickson towers and
one free sum abscissa**. The natural-positive witness was reconstructed
and checked directly in each minimal quotient ring. [Machine-readable
CI evidence](MINIMAL_THREE_TOWER_S4_CI_20261115.json).

| Exact S4 subideal | Active towers | Free vars | Degree reached | Partial F4 muladds | Result |
|---|---:|---:|---:|---:|---|
| Six-block left-only (prior) | 6 | 2 | 10 | 14,129 | Oversize |
| **Minimal left-only** | **3** | **1** | **10** | **14,129** | **Oversize** |
| Six-block right-only (prior) | 6 | 2 | 10 | 14,129 | Oversize |
| **Minimal right-only** | **3** | **1** | **10** | **14,129** | **Oversize** |

Both minimal arms exactly reproduce the relevant partial counters, including
1,289 max columns and 2,571 *previously accepted* max dense entries. The
aborted reducer exceeds the separate frozen 15,000 limit; these maxima do
not report its size. All four partial ideals have known rational positive
solutions, checked before F4, so \`undetermined\` is not confused with a
refutation. The **S4 resultant plus three depth-six fibre blocks** is thus
itself the measured algebraic resource bottleneck at this F4 cap; six unused
towers are not its cause. This remains an implementation-specific diagnosis,
not a lower bound for all Gröbner methods.

**Practical research decision:** stop spending on all 930 admissible fixed
pair roots, Dickson-block stripping, or simply varying target masks under
the same F4 formulation/cap. The next experiment should change the
**three-point S4 encoding or reduction method**, preferably via the already
investigated certified quotient/cubic pairing of #4228 or independent
Singular/msolve F4. Before an end-to-end trial, a minimal S4 must return
actual finite-field solutions for the *positive* control and genuine negative
results on confirmed negative toys with all work counted. Only then restore
the full eight-summand oracle and cold rank/recovery benchmark. No new
complete prime31 logarithm has been obtained by this F4 path.
