# What counts: below rho, smaller exponent, subexponential, and transfers

Owner rule, 2026-11-15; revision 2 (same day) after the red-team review in
`docs/ic/reviews/SUCCESS_CRITERIA-redteam-20261115.md`; revision 3 (owner, same
day): the construction of a map is not charged (§5, rule 1). This file defines the
long-term claims of the index-calculus program: what "below rho" means at one
size, what an exponent claim needs, what a subexponential claim needs, which
sizes support which claim, and how a transfer to another group (a hyperelliptic
or other Jacobian, a finite field, an isogenous curve) is accepted.

It sits on top of AGENTS.md "Success criterion", "Tracks of index calculus",
"The rule: boundary, table, ratio" (§1 to §15), "IC measurement rules",
`docs/VALIDATION_GATE.md` and `docs/bounds/README.md`, and changes none of
them. **For tier claims the cold end-to-end interval of section 1 governs**,
as AGENTS.md "IC measurement rules" now also states (owner, 2026-11-15); the
online interval is reported beside it, never instead of it. Any other disagreement is
fixed in this file by the PR that finds it, and no claim relies on the
disagreement in the meantime.

The success criterion does not change: progress is iterative, and every tier is
reported as the **distance remaining** to the next, never as a gate on the work.
Everything in this file is a definition, `[pcc:unverified]` until cited by a
verified claim.

## 1. The instance, the unit and the reference

- **Instance** `(E / F_q, P, Q)`, named by its ICV1 slug (§11). `q = p^m`;
  `#E(F_q) = h · r`, `r` prime, `h` the cofactor; `P` of order `r`.
- **Public target.** `Q` is derived by hashing a beacon-derived seed to the
  curve and clearing the cofactor, so no party knows `log_P Q` before the solve
  (`target_kind: public`, AGENTS.md §12). `P` is the published generator or is
  derived the same way. Planted targets serve controls and T0 only.
- **Size** `b = log2 r`. Field bits, `m`, cofactor and the number of polynomial
  variables are reported separately (§8b).
- **Cost** in group-addition equivalents on `E` (GAE), every phase priced,
  read across the §5a band (0, 0.1, 1 GAE per unpriced native operation).
  Headline unit `S = total GAE / √r` (§2).
- **One target, cold.** Every tier claim uses the cold end-to-end interval:
  from the instance to a verified `[k]P = Q`, with all target-independent
  setup (factor base, relation collection, linear algebra, solver setup)
  charged in full to that one target. Constructing a map (an auxiliary curve,
  its isogenies, a cover or other transfer) is not charged; §5, rule 1. This is the
  end-to-end `S` of "Success criterion" and §8. The online interval of "IC
  measurement rules" is a separate row labelled `online-only`; it never
  supports T1 to T3. A cost amortized over `t` targets is a separate row that
  names `t`, and is never the headline.
- **No uncounted work.** A tier claim lists every process and library the run
  invoked, phase by phase, each with an operation count priced by the band or
  by a pinned, versioned conversion. A lower-bound record cannot support T1 to
  T3 at any weight. Lower bounds include a record marked `bounded`, an
  algebraic solver whose work was taken out of the total (`ic.pipeline`), and
  a transfer arm with `total_gae: 0` (`jacobian_image.*`).
- **Memory.** Peak memory is reported per size, or "memory unknown". A claim
  better on operations and worse on memory is written "T1 (ops), trade on
  memory" (docs/bounds/README.md §5).
- **The rho reference** is, at each weight of the band and at `w_cal`, the
  **cheapest** measured configuration among the registered rho variants on the
  same instance and targets: plain, negation, and negation with the Frobenius
  or other automorphism fold, each with and without its setup variants, run
  cold in the same binary, with unpriced work priced the same way (§5a). An
  automorphism is any efficiently computable map acting on `⟨P⟩` as a scalar
  of small multiplicative order: negation, Frobenius on Koblitz and other
  subfield curves, the order-4 and order-6 maps at `j = 1728` and `j = 0`;
  `γ` is the order of the group they generate. The idealized expected cost is
  `√(π r / (2γ))` additions (`S ≈ 0.886` with negation alone; `S ≈ 1.3` in §2
  is plain rho and is never the reference). Measured constants have been
  1.06× to 2.22× that floor at lab sizes, so rho is measured wherever section
  3 says it must be.
- **Track.** Every claim names its track ("Tracks of index calculus").
  A T2 or T3 candidate in the generic-decomposition track contradicts the
  generic-group lower bound `Ω(√r)` and is treated as an accounting error
  until the uncounted group work is found. A transfer is reported in the track
  of its destination solver.

## 2. Preregistration and multiplicity

Every T1 to T3 claim is preceded by a preregistration, committed and recorded
as a pcc claim before the first measured run, naming:

- the **family**: a rule that produces instances from a size and a beacon seed,
  with its density among all curves of that field size; the path (how `q`,
  `m`, the cofactor bound and the method's parameters grow with `b`); the
  sizes;
- the method and its parameters as functions of size; the track;
- targets per size; the estimator and interval (section 3, T1); the weights;
- the exponent margin `δ` (T2), the held-out rule, the numeric criterion of
  each heuristic test (T3), and the acceptance and abandonment conditions (§4);
- the **family of attempts**: every path, variant and instance set the thread
  has preregistered for the same tier.

Instances and targets are derived from an `ecbench challenge commit`
commitment and a later beacon (docs/bounds/README.md §3.4). Every drawn
instance is reported, and each passes the weakness checks (embedding degree
above a declared bound, not anomalous, subfield and cover structure disclosed,
§8b); where a known non-generic attack applies, it is an extra reference arm
(§15). Intervals are Holm-corrected over the family of attempts (family-wise
0.05), and every attempt, failed or abandoned, is listed with its outcome. The
number of targets is fixed in advance; adding targets after seeing an interval
starts a new attempt.

## 3. The four claim tiers

### T0. Gain over the IC baseline

"Success criterion": lower verified cost than the frozen baseline on the same
instances, at component or pipeline scope, classified by §3 and written as a
§3a cost chain, with the ratio to matched rho as the distance remaining. T0 is
open at every size, the `m = 83` gate (§8a) and the challenge included.

### T1. Below matched rho at a stated size

1. **Runs.** Cold, one public target per run, every phase priced, on the
   preregistered number of targets (at least 32 below `b = 64`, at least 8
   above), each answer verified `[k]P = Q` and replayed. Each target is a
   separate cold IC run with its own precomputation seed (no shared setup).
   Every launched target counts: failures, timeouts and restarts are charged
   at their consumed cost, and the success probability is reported.
2. **Estimator.** `Σ C_IC / Σ C_rho` over the targets, with the two-stage
   bootstrap of docs/bounds/README.md (2,000 resamples, recorded seed);
   `tail.large` reported for both arms, and a large tail requires a
   replication session.
3. **T1 (band)**: the interval lies wholly below 1 at every weight of the §5a
   band. This reading governs the leaderboard (§5a).
   **T1 (calibrated)**, reported as "ranked at calibrated price": both arms
   carry a calibration for the same host (`w_cal` by `perf-ipc` on an isolated
   host, the addition time taken from the rho reference's own per-step time on
   that host, so batched inversions are counted as rho pays them); the interval
   lies below 1 at `w = 0`, `w_cal` and `10·w_cal`; every non-field counter is
   priced at each band weight (`ecbench band --w-cal`). Where no calibration
   exists for a field family (today `ecbench wcal` covers prime fields only),
   the band alone applies (§5a). T1 (calibrated) without T1 (band) is reported
   with both verdicts and is never written as plain T1.
4. **Growing sizes.** The ratio is reported at the claimed size and at no fewer
   than two smaller sizes on the same preregistered path, with its trend
   ("Success criterion", §2, §15). It need not be below 1 at the smaller
   sizes. The claim says nothing about larger sizes.
5. **Controls**, in the same session: an A/A comparison of the rho reference
   against itself with independent seeds (its interval must contain 1); a
   positive control whose expected win is known; a negative control expected
   not to win (a fixed `n = 2` Weil restriction, or a decomposition oracle on
   a random base).
6. **Evidence.** The validation gate (`VALIDATION.json`: replay certificate,
   validator report, matched rho with the band); an audit receipt from another
   host class that replayed both arms (§12); pcc standing `verified`; and
   `validator-breakthrough` on the frozen evidence.

The IC positive control (`research/ic_positive_control_20261112/`, genus-3
Jacobians, `[pcc:unverified]`) is why the two T1 readings exist: below rho in
group operations from `p = 257`, unresolved across the band, and below 1 at an
informational calibrated price only at `p = 65,537`. It is a Jacobian DLP, not
an instance of section 1, and would be T1 at neither reading. It is cited as
the reason for the calibrated reading, not as a T1 result.

### T2. Smaller exponent than rho

The cost grows more slowly than rho's along a declared family path. This is
still exponential in `b`.

1. **Runs.** T1 items 1, 2, 5 and 6 at each size. T1 item 3 (ratio below 1) is
   not required: T2 is about the exponent, and the ratio at each size is the
   distance remaining.
2. **Path.** Declared in the preregistration. The cofactor is fixed or bounded
   by a declared constant, with `log2 h` reported per size. The fit is
   reported against both `ln r` and `ln q^m`, and the claim holds only if both
   satisfy item 4. The rho arm is compared with its own model along the path,
   `√(π r / (2γ(b)))`, not with a constant `α = 1/2` (on a Koblitz path
   `γ = 2m` grows with size).
3. **Sizes.** At least six sizes, spanning at least 24 bits, the smallest at
   `b ≥ 32` and so the largest at `b ≥ 56`, all inside one bounds tier
   (`docs/bounds/README.md` §3.1 refuses fits across tiers).
4. **The fit**: `ln C = ln K + α · ln r` by `ecbench bound fit`, for the total
   and for every phase separately, at every band weight and at `w_cal` where
   it exists. The claimed exponent `α*` is the largest of the phases' upper 95%
   limits; a phase with work on fewer than four sizes has no interval and
   blocks the claim. T2 requires `α*` below rho's fitted exponent along the
   path by at least `δ` at every weight. For every phase, the size where its
   fit projects it to reach half the total is computed and reported.
5. **The analysis.** The method's own derived cost (for example `Õ(q^{2-2/n})`
   for summation-polynomial IC over `F_{q^n}` with fixed `n`, which equals
   rho's exponent at `n = 2`) is written next to the fit with its assumptions.
   A fitted exponent below the derived one, by more than the fit's interval, is
   unexplained (a fixed cost, a phase not yet dominant, a counting gap), and
   the claim is held until it is accounted for.
6. **Held-out size.** After the fit is frozen, an instance is derived from the
   hash of the frozen fit and a later beacon, at least 8 bits above the largest
   fitted size and at or beyond every phase's projected crossover (item 4)
   below `b = 128`. The fit tool's own 95% prediction interval is used without
   widening, at every weight. The size is chosen so that interval excludes the
   rho-exponent model refitted to the same data; if no feasible size achieves
   that, the held-out test cannot test the claim and T2 is not reached.
   A measurement outside the interval falsifies the claim on that path.
7. **Review.** `red-team-breakthrough` and `validator-breakthrough`.
8. **Families with no known sub-1/2 algorithm.** On general prime-field curves
   and prime-degree binary fields, no algorithm with exponent below 1/2 is
   known; a T2 claim there also meets T3 items 2 and 3.

For the ECC2K-130 reference family (`E_0`, prime `m`, `#E = 4r`), only
`m = 41, 83, 97` fall in `32 ≤ b < 100`. A T2 claim toward ECC2K-130 states its
relaxation (Koblitz `E_0` or `E_1` with prime `m` and a declared cofactor bound,
or another stated family) and is a result for that relaxed family (§8b).

### T3. Subexponential

The cost is subexponential in `N` (the group size, or `q^m`, stated) along the
family: `N^{o(1)}`, in the form the analysis proves, for example
`exp(O((ln N)^a (ln ln N)^{1-a}))` or `exp(O((ln N)^a))` with `a < 1`, or
quasi-polynomial `exp(O((ln ln N)^2))`. The common case is `L_N[a, c]` with
`0 < a < 1`.

Measurement cannot establish T3: an exponential model with polylog factors,
`K · r^α (ln r)^k`, matches noiseless `L[1/3]` and `L[1/2]` curves to about 1%
at a held-out size 8 bits out. T3 rests on the argument; data can only falsify
it. Required, all of:

1. **The family and its parametrisation** as functions of `N`: field, degree,
   genus, factor-base size, decomposition degree. A fixed-`n` family is not
   subexponential; Diem-type results need `n` and `log q` to grow together in a
   stated way.
2. **The complexity statement**, either proved or as a heuristic analysis whose
   every heuristic is named as a hypothesis (degree of regularity or first fall
   degree, smoothness probability, matrix rank). A heuristic analysis is
   reported as heuristic. First-fall-degree arguments for binary curves have
   been disputed in the literature and need explicit per-size evidence here.
3. **Each heuristic tested** at every measured size against a numeric
   criterion fixed in the preregistration (for example observed solving degree
   at most the forecast; relation yield within a factor 2 of the predicted
   smoothness probability, with an interval; matrix rank deficit at most a
   declared bound; `dreg-triage` for the forecasts). A failure at a size
   withdraws the claim until the heuristic is repaired.
4. **Data**, at least eight sizes on the path: the measured cost against the
   analysis's prediction, with its explicit treatment of the `o(1)` term, at
   every size and at a held-out size at least 8 bits above the largest fitted;
   beside it, a fit of the exponential competitor `K · r^α (ln r)^k`.
   Agreement is consistency, not evidence of T3.
5. **Lean**: the complexity theorem, or its conditional form with the
   heuristics as explicit hypotheses (`lean-instance-gate`). Partial
   formalization is reported as partial.
6. **Review** as in T2.

## 4. Sizes

Names below are this file's bands, by `b`; the bounds tool's tiers (`toy`,
`medium`, `crypto`) are by field bits and are reported beside them.

| band | `b = log2 r` | supports | rho reference |
| --- | --- | --- | --- |
| B0 | `b < 32` | T0; controls; debugging | measured |
| B1 | `32 ≤ b < 64` | T0, T1, T2/T3 fit sizes | measured, at least 32 targets |
| B2 | `64 ≤ b < 100` | T0, T1, T2/T3 fit and held-out sizes | measured while expected rho cost per target is at most `2^40` GAE (about `b ≤ 80` with negation), at least 8 targets; above that, extrapolated as below |
| B3 | `b ≥ 100` | T0, T1, held-out sizes | extrapolated |
| challenge | the exact instance (ECC2K-130: `E_0` over `GF(2^131)`, `#E = 4r`, `b ≈ 129`) | T0; target-specific T1 | extrapolated |

- **Extrapolated rho**: `√(π r / (2γ))` times the **low end** of the measured
  constant's 95% interval at the largest measured size on the same family and
  `γ`, labelled "rho model (extrapolated, bound ECBND…)", so the comparison
  errs in rho's favour. T1 item 1 then applies to the IC arm only, and the
  ratio interval comes from the IC interval alone.
- **At the challenge instance**, one verified cold IC solve, checked against a
  preregistered cost prediction with its interval, replaces the target count.
- **Fidelity gates still apply.** Binary Koblitz work toward ECC2K-130 passes
  the `m = 83` gate (§8a) before any claim of surviving scale; `m = 31, 51,
  53` are exploratory and their differences are disclosed (§8b). The IC side
  is always measured, never extrapolated, for a T1 claim; IC extrapolation is
  only the labelled prediction a held-out test checks (§6).

## 5. Transfers

A transfer moves the problem to another group where it is cheaper, solves it
there and brings the logarithm back. It is a valid route to every tier, and a
transfer that costs less than rho on `E` counts like a direct IC result. The
table lists known routes with the complexity of the destination solver; every
entry is to be confirmed with `research-kb` before it is quoted, and the
comparison is always with rho on `E`.

| route | applies to | destination cost (heuristic unless noted) | compare with |
| --- | --- | --- | --- |
| Weil descent / GHS: `E / F_{q^n}` to a Jacobian of genus `g` over `F_q` | curves with a suitable cover; `g` is often about `2^{n-1}`, `n` only for special curves | `Õ(q^{2-2/g})` for fixed small `g` (double large primes; constants exponential in `g`); `L_{q^g}[1/2]` when `g` grows faster than `log q` | rho on `E`, about `q^{n/2}` |
| genus-3 hyperelliptic to plane quartic (Smith's isogeny), as a second stage after a cover | a fraction of genus-3 hyperelliptic curves in odd characteristic | `Õ(q)` (Diem / Diem–Thomé) | rho on `E` |
| pairing (MOV / Frey–Rück) | embedding degree `k` small | finite-field DLP in `F_{q^k}`: quasi-polynomial in small characteristic; NFS-family `L_{q^k}[1/3]` otherwise, variant by `k` | rho on `E` |
| anomalous (Smart; Satoh–Araki; Semaev) | `r = char F_q`, e.g. `#E(F_p) = p` | polynomial time | rho on `E` |
| cover or isogeny walk to a weaker curve, then one of the above | a computable map to a curve the next route applies to | map cost plus the next route's cost | rho on `E` |

Koblitz curves are called "anomalous binary curves", but Smart's attack does not
apply to ECC2K-130 (`r ≠ 2`; `#E = 4r`). For ECC2K-130, `m = 131` is prime, so
descents that need an intermediate subfield do not apply directly; any route is
shown on that exact curve (§8b).

Every transfer claim records and charges:

1. **The map**: its evaluation on `P` and `Q` is charged in every cold row.
   Its construction is not part of the success criteria (owner, 2026-11-15):
   finding an auxiliary curve, its isogenies, a cover or any other
   precomputation that depends only on the field, curve, subgroup or base
   point. The construction is not charged. Its method and measured cost are
   reported beside the row, so it can be reproduced. Anything that depends on
   `Q` is evaluation and stays charged.
2. **A correctness certificate**: the map is a homomorphism on `⟨P⟩`, injective
   there (the image of `P` has order `r`), checked by code independent of the
   producer; the answer is verified on `E` (`[k]P = Q`), not only on the
   destination group.
3. **Unit conversion**: each destination-group operation type converted by its
   own measured factor against the rho reference's per-step time on the same
   host, read at 0.5×, 1× and 2× of each factor; ranking requires the order to
   hold at all three.
4. **Applicability**, preregistered: the exact class of curves the route
   applies to, checked on the exact instance, with its density among curves of
   that size. A result on a weak subfamily is a result for that subfamily.
5. **Literature status**: the best known complexity for that subfamily. A
   transfer whose subfamily is already known to be weak at that tier
   (MOV-weak, anomalous, GHS-weak, a fixed-`n` Weil restriction) is reported as
   a reproduction for that subfamily, not as a new tier.
6. **The chain** as a §3a cost chain (transfer, solve, return), the interface
   (instance, target, unit) unchanged from first line to last.

Skills: `ecdlp-surfaces`, `curve-algebra`, `isogeny-volcano`; existence,
construction, subgroup action and evaluation cost are separate claims
(`docs/endomorphism-rules.md`).

## 6. What does not count

AGENTS.md §6 applies in full. In addition:

- a tier claim on the online-only interval, or with any uncounted phase, at
  any weight;
- a claim without a preregistration that predates its first measured run, or
  whose targets were planted or added after seeing an interval;
- a reference other than the cheapest registered rho at each weight;
- an exponent from fewer than six sizes, under 24 bits, a path changed between
  sizes, a fit across bounds tiers, or a total-cost slope with a steeper phase
  (T2);
- a subexponential claim from data alone, from a solver stage, or with an
  unnamed or untested heuristic (T3);
- a weak-instance or weak-subfamily result stated for the family; a known
  weak-class reproduction stated as a new tier;
- a transfer whose map construction, evaluation or conversion is not charged
  on every instance, or whose answer is verified only on the destination;
- a held-out prediction written after the held-out instance was drawn.

## 7. How a claim is written

In this order:

1. tier (T1 band or calibrated); track; preregistration id; family, path,
   sizes, and the family of attempts with every outcome;
2. the cost chain (§3a), each phase's cost, the dominant phase, memory;
3. the ratio to the cheapest rho with its interval at the band and at `w_cal`,
   at the claimed and two smaller sizes (T1); the per-phase fits, `α*` and
   rho's path model (T2); the analysis, its heuristics and their per-size tests
   (T3); controls;
4. the held-out prediction and outcome (T2, T3);
5. for a transfer: the map, certificate, conversions, applicability, density
   and literature status;
6. evidence: `VALIDATION.json`, replay and cross-host audit receipts, validator
   report, pcc claim id with standing `verified`, Lean status;
7. the distance remaining to the next tier.

The checklist [`success_criteria.json`](success_criteria.json) carries the same
requirements with numeric thresholds.

Sources checked: research-kb was not available to the author or the reviewer.
The section 5 complexities and the literature remarks in T2 and T3 are marked
for confirmation with `research-kb` before they are quoted in a paper or
result.
