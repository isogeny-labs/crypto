# Protocol: selective quotient cuts in the symmetric Curve25519 remainder circuit

**Frozen on 2026-11-25 before implementing cut policies or observing any
selective-arm solver row.**

Study id: `EXP-C25519-SELCUT-001`. Parent task revision: `a7145dc78b`.
The instrument and analyzer are native Rust in the Isogeny Labs `crypto`
repository. No JavaScript is used.

## 1. Question and prior result

The active objective is a classical prime-field ECDLP algorithm with expected
total work `r^o(1)`. This study is only a relation-system stage diagnostic.

The completed predecessor
`research/curve25519_symmetric_remainder_20261125/` compared direct remainder
equations with an explicit quotient at every multiplication in the binary
computation of `X^d mod q`. At `d=19`, the all-quotient arm lowered maximum
productive degree from 5 to 4 on 8/8 random targets, but its paired F4-work
ratio was `1.2864624565` and peak-RSS ratio reached `2.4275`. It failed its
frozen gate and stopped before scaling.

For a cubic residue `A=a0+a1 X+a2 X^2`, direct reduction of `A B` can contain
the quartic coefficient product `a2 b2 e1 e3`. A multiplication by `X` has
`b2=0`, so it is already at most quadratic after direct reduction. The initial
square of the constant residue `X` is also already quadratic. Only later
residue-squaring operations can create the quartic direct equations.

**Question.** Can retaining quotients only at the last two nonlinear
residue-squaring operations preserve the observed one-degree F4 reduction
without the all-quotient variable and memory penalty?

## 2. Fixed instance, targets, and engine

- Curve: the committed `curve25519_family_instance(20)`, short-Weierstrass
  `y^2 = x^3 + 63,700 x + 872,826` over `p=1,048,573`, prime subgroup order
  `r=131,203`, generator `(349,837,122)`.
- Factor base: `x = 349,833 + 3 w`, `w in mu_d`, with distinct roots only.
- Qualification order: `d=19`; decomposition arity `m=3`.
- Relation equation: the same symmetrized `S4` and cubic root polynomial
  `q(X)=X^3-e1 X^2+e2 X-e3` as the predecessor.
- Targets: the exact predecessor seed `20261125`, 8 random subgroup targets
  followed by 4 planted distinct-root decompositions. Scalar and affine point
  must match the predecessor rows byte for byte.
- Solver: repository `f4_fp`, grevlex, release mode, one process,
  `RAYON_NUM_THREADS=1`, degree ceiling 24, 300-second per-system cap.
- Primary unit: counted F4 `F_p` multiply-adds (`field_ops`). Secondary units:
  maximum productive degree, rows, columns, steps, exact membership-build
  coefficient operations, peak RSS, and descriptive wall time.

No trace learning, guessing, target reuse, or post-data cut selection is
allowed.

## 3. Semantic cut policies

Construct the standard left-to-right binary schedule beginning at residue
`X`. Classify an operation as a **nonlinear square** when it squares a prior
unknown residue. The initial constant square `X*X` and every multiply-by-`X`
operation are **non-beneficial**: their direct coefficient equations are
already at most quadratic.

At `d=19` the schedule is

```text
step 0: X^1  -> X^2   square, initial/non-beneficial
step 1: X^2  -> X^4   nonlinear square
step 2: X^4  -> X^8   nonlinear square
step 3: X^8  -> X^9   multiply by X, non-beneficial
step 4: X^9  -> X^18  nonlinear square
step 5: X^18 -> X^19  multiply by X, non-beneficial
```

Every explicit cut uses the predecessor's five coefficient equations and two
quotient variables. Every uncut step uses the predecessor's three direct
remainder equations. Intermediate residue variables are identical in all
arms.

The six frozen arms are:

| arm | explicit quotient cuts at `d=19` | purpose |
|:--|:--|:--|
| D | none | direct baseline, exact predecessor reproduction |
| L | `{4}` | last nonlinear square only; nested ablation |
| B | `{2,4}` | **primary**: last two nonlinear squares, four added variables |
| N | `{1,2,4}` | all nonlinear squares; mechanism upper bound |
| U | `{0,3,5}` | non-beneficial-cut negative control, same added variables as N |
| A | `{0,1,2,3,4,5}` | all-quotient predecessor reproduction |

Only B can qualify this study. L, N, U, and A are diagnostic controls; a
post-hoc winner among them cannot replace B.

## 4. Correctness and reproduction gates

Before cost interpretation:

1. Every arm's planted values satisfy every equation and re-add to the exact
   target group point; invalid returned relations equal zero.
2. For all unordered distinct triples from `mu_19`, every arm's membership
   equations accept the same elementary-symmetric coefficients.
3. The predecessor tiny-curve exhaustive positive/negative truth panel passes
   unchanged for D, B, N, U, and A.
4. D reproduces every predecessor direct row's target, `field_ops`, degrees,
   rows, columns, steps, and verdict exactly. A does the same for every
   predecessor all-quotient row.
5. U is a structural negative control. If it lowers maximum productive degree
   below D on more than 2 of 8 random pairs, the cut classification is called
   invalid and B cannot qualify.

Any failure makes the instrument invalid. Failures and partial rows are
retained; no cut, target, seed, or threshold is changed after a row exists.

## 5. Qualification and stop rule

Run 12 rows per arm at `d=19`, 72 rows total, in target-major order D/L/B/N/U/A.
A timeout, allocation failure, memory kill, or degree-ceiling result is
censored and never assigned a completed work value.

For B/D over random targets completed by both arms, compute the geometric mean
of paired `field_ops` ratios and a descriptive 95% paired bootstrap interval
with 10,000 resamples and seed `20261125`.

B qualifies for the gated scaling stage only if all conditions hold:

1. every correctness and reproduction gate passes;
2. B has no greater censoring fraction than D and at least 6 random pairs
   complete in both arms;
3. the upper 95% endpoint of the paired B/D work ratio is **below 0.75**;
4. B's maximum productive degree is at most 4 on at least 6 of 8 random pairs;
5. B/D peak RSS is at most **1.25** on every completed random pair;
6. B/D membership-build coefficient operations are at most **1.25** on every
   completed random pair; and
7. U's negative-control degree condition in section 4 holds.

Missing any condition closes B before larger orders or `m=4`. A favorable L
or N row is mechanistic evidence only and requires a separately frozen study.

## 6. Gated scaling stage

Only after B passes section 5 may D and B run at
`d in {7,9,21,27,57}`, with the same target counts, seed, caps, and semantic
policy: B cuts the last two nonlinear squares, or every nonlinear square when
fewer than two exist.

At every order B must retain the section 5 correctness, work-ratio, degree,
and resource limits. Fit `ln(B/D)` on `ln(d)` over completed cell geometric
means; call the advantage strengthening only if the upper two-sided 95%
interval on that slope is below zero. These finite cells remain descriptive.

No `m=4` experiment is authorized here. It needs a separate protocol. A total
ECDLP claim additionally needs relation probability, enough relations, setup,
filtering, sparse linear algebra, individual logarithms, and target recovery
all bounded by `r^o(1)`.

## 7. Evidence and research receipt

Retain exact revisions, compile-repair provenance, toolchain, host limits,
commands, binary hashes, per-target JSON, failures, native analyzer source,
paired CSV, editable SVG/PNG, Typst/PDF, complete hashes, and
`experiment.export/v1`. Results remain `[pcc:unverified]` unless separately
registered and signed.

The required research-KB wrapper was present, but its catalog executable and
a sibling `crypto-autoresearcher` checkout were unavailable. Exact-mechanism,
negative-result, application, and adjacent-mechanism fallback searches were
run over this repository. They found the predecessor all-direct/all-quotient
study and general square-and-multiply implementations, but no selective
quotient cut-set experiment. This is not a novelty claim; cross-repository
novelty remains unresolved.
