# Does the generator change index-calculus cost? Factor base and relation finding

**Status: design (PLAN), not frozen.** Following the volcano-cost
convention, a `PROTOCOL.md` freezes N, arms, metrics and decision rules
before any measured run.

**Where this comes from.**

- `volcano-rho --instance-study` (`run-20261017T0415Z-instance`) showed
  that the generator and k do not change rho's cost: every |z| < 0.8.
- The volcano cost study (`research/isogeny-volcano-cost_20261017`, branch
  `aburan28/youthful-ride-6u51pp-volcano-v2`) found IC yield effects that
  rho does not show:
  - horizontal τ ≈ 2.2%;
  - coordinate τ ≈ 4.7% between isomorphic models;
  - an exploratory ℓ = 2 floor/crater yield drop of about −6.5%.
- The IC side of the generator question was never measured.

## 1. What the code says before any run

**Factor base.** `prime_factor_base` (`ic_boundary.rs:4396`) and
`PrimeAbscissaBase::build` never read the generator. The base is a function
of the curve model alone. So any generator effect on the factor-base step is
exactly zero, and the check is a digest comparison, not a statistic.

**Relation finding.** Every target source draws coefficients and builds
points from them:

- `Random`: `R = [a]G + [b]Q`.
- `Walk`: jumps `[a_j]G + [b_j]Q` and pooled restart offsets.
- `Stride` and `StrideRenew`: `R₀ + [k]S`.

Set `G' = [c]G` with `c` a unit mod `r`. Then `(a, b) ↦ (a·c⁻¹, b)` is a
bijection of the coefficient space. It maps the stream drawn with `G'` onto
the stream drawn with `G`, point for point.

- Consequence: the relation process has **the same law** under every
  generator of the order-`r` subgroup ⟨G⟩. This holds whenever `r² ∤ #E`,
  because ⟨G⟩ is then the unique subgroup of order `r`.
- With coupled seeds the relation rows are **identical**, and the
  factor-base logs scale by exactly `c⁻¹`.
- The one exception is the counted cost of the scalar multiplications in
  the target draw, which depends on the scalar's bits. Its expectation is
  equal across generators.

**What is not projected.** The prime base takes the smallest abscissae of
the whole group `E(F_p)`, not of ⟨G⟩. With `#E = h·r`, a target in ⟨G⟩ is a
sum `±P_i ± P_j` only if the cofactor classes `[r]P_i` cancel.

- So yield follows the base's class histogram. That histogram depends on the
  model and the group structure, and not on G.
- `exact_decomposition_ceiling` (`ic_boundary.rs:451`) already counts the
  class-cancelling multisets `N₀`.
- **Working hypothesis:** the coordinate and horizontal τ, and the ℓ = 2
  floor effect, are all `N₀` effects.

So the experiment has two jobs:

1. Confirm G-invariance, which is predicted exact in law and exact under
   coupled seeds.
2. Test the mechanism that does move yield: the subgroup and its cofactor
   classes.

## 2. Population

- **P1: the volcano classes.** The six ordinary classes of PROTOCOL v1
  (24-bit `p`), model 0, 4 curves per class: 24 curves.
  - ℓ = 2 classes have `h = 4`, split across `Z/4` and `Z/2 × Z/2` 2-parts.
  - The ℓ = 3 classes are cyclic.
- **P2: a cofactor ladder.** Fresh 24-bit curves with `r` of about 22 bits
  and `h ∈ {1, 2, 3, 4, 8, 16, 64}`, 8 curves per `h`, seeded and exported
  the same way.
  - `h = 1` is the negative control for everything in E3.
- **P3: the j = 0 class.** For the GLV arm only.

## 3. Generator arms (per curve, all public seeds)

| arm | generator | why |
|:--|:--|:--|
| `g0` | the exported generator | reference |
| `g0-aa` | `g0`, another measurement seed | A/A noise floor |
| `grand-1..8` | `[c_j]g0`, `c_j` uniform in `[2, r−1]` | the question |
| `gneg` | `−g0` | sign |
| `gsmall-2`, `gsmall-3` | `[2]g0`, `[3]g0` | small multipliers |
| `gminx` | the point of ⟨G⟩ with the smallest abscissa | a "structured" generator |
| `gfb` | `[h·(h⁻¹ mod r)]P₁`, the ⟨G⟩-projection of the first base point; when `h = 1` this is `P₁` itself | generator inside the base |
| `glam` (P3) | `[λ]g0`, the GLV eigenvalue | automorphism |

**Two target conventions:**

- **Fixed Q** (primary): the same point `Q`, so `k' = k·c⁻¹`. This is one
  instance with a different base.
- **Fixed k**: `Q' = [k]G'`. This is what `planted_shared` does today.

Both are predicted equivalent. Fixed Q needs explicit targets in the spec.

## 4. Experiments

### E0: exactness (no statistics; any deviation is a bug)

- **E0a.** The factor-base digest (points, columns, signs) is identical
  across all arms on every curve.
- **E0b.** Coupled seeds, under a new hook that rescales every drawn
  `a`-coefficient by `c⁻¹` for each arm `[c]g0`:
  - identical target-point stream;
  - identical relation-row digest;
  - identical rank trajectory;
  - recovered `k'` verified;
  - linear-algebra logs equal to `logs₀·c⁻¹ mod r`, entry by entry;
  - ΔS equal to the recomputed difference in scalar-multiplication ops,
    and nothing else.
- **E0c.** For each target source (`random`, `walk`, `stride`,
  `stride-renew`), E0b passes.

### E1: uncoupled generator equivalence (the measured question)

- **Arms and size:** all §3 arms on P1 (and P2 at `h = 1, 4`), 64 paired
  targets per arm and curve. About 24 × 15 × 64 ≈ 23k runs, roughly 35 min
  serial at the volcano study's per-run cost.
- **Metrics:**
  - F3 yield, `relations_found / targets_tried`;
  - rows to full rank, and duplicate or dependent rows;
  - F3s total S;
  - per-phase S: target draw, decomposition, linear algebra.
- **Statistics** (`volcano-stats` with cells = generators, strata =
  curves):
  - Cochran Q and max|z| with permutation nulls;
  - DerSimonian–Laird τ beside the null's 95th percentile;
  - each arm/`g0` ratio as a 90% interval.
- **Decision:** generator changes yield by less than ±2% if the pooled
  90% CI lies inside [0.98, 1.02]. A generator effect needs p < α/m and
  τ_gen above the A/A band.
- **Controls:**
  - planted `size=33` vs 32 (predicted 1.063);
  - `--plant-cell <grand-1>:1.03` on the real records, which must be
    detected or the run is inconclusive.
- **Prediction:** equivalent. τ_gen sits at its null and far below the
  coordinate τ of 4.7%.

### E2: generator inside the base (free column)

- **Arm:** `gfb` on `h = 1` curves.
- **Target law:** unchanged, so yield is predicted unchanged.
- **Linear algebra:** the known log `log P₁ = 1` removes one unknown. Only
  if the solver uses it do rows-to-solve fall from `B + 1` to `B`, a
  predicted −1/(B+1) ≈ −3% at B = 32.
- **First step:** check whether the pipeline anchors the column. If it does
  not, the prediction is 0. Implementing the anchor is an engineering arm
  with a ceiling of 1/(B+1).
- **Priority:** low. Bounded and small.

### E3: cofactor classes, where an effect is actually predicted

- **E3a (no new runs).** Recompute `N₀` for every one of the 800 workload
  curves of the volcano v1.1 sessions, then regress log yield on log
  `N₀/r`. Pre-registered:
  - slope is within [0.9, 1.1];
  - after adjustment, the coordinate τ and the horizontal τ fall to their
    A/A nulls;
  - the ℓ = 2 floor/crater contrast is ≥ 80% explained by `N₀`.
  - If these fail, the coordinate effect has another cause and E3b is
    re-planned.
- **E3b (new factor-base arms)** on P2, at equal B:
  - `prime-abscissa` (current);
  - `subgroup=filter`: the B smallest abscissae whose point lies in ⟨G⟩.
    This scans about h× more abscissae and costs one `[r]` check per
    candidate, all charged.
  - `subgroup=project`: `{[h]P_i}`, deduplicated.
  - **Prediction:** yield ratio filter/current equals `N₀(filter)/N₀(current)`,
    about h for evenly spread classes. At `h = 1` the arms coincide
    exactly: same digest, ratio 1.000.
- **Gain test:** any yield gain must also show in total S and survive the
  §5a band against `rho-neg` before it counts. Otherwise it is an
  accounting result.
- **G-invariance inside E3:** E3b uses `g0` and `grand-1` only. The
  prediction is no interaction.

### E4: j = 0 (P3)

- **Arms:** `g0`, `glam` and `grand-1..4`, under `glv-orbit` and
  `prime-abscissa`.
- **Prediction:** equivalent. λ fixes ⟨G⟩, and the orbit base is built
  without G.

## 5. Order and stopping

1. E0 (instrument). If E0 fails, fix and re-run E0 only.
2. E3a (free, on existing data).
3. E1.
4. E3b.
5. E2 and E4 last, optional.

Freeze `PROTOCOL.md` after E0 and before E1. A changed arm, N or margin
after that is a new labelled version.

## 6. Tooling needed

| change | where | for |
|:--|:--|:--|
| emit generator arms as spec workloads (`gx, gy` override) and explicit-Q targets | `volcano-rho --ecbench --generators <list>` | E1, E2, E4 |
| `coeff_scale=c` target hook: rescale drawn `a` by `c⁻¹` | `ic_boundary.rs` target loop (fresh, jumps, pool, stride) | E0b, E0c |
| cells = generators | `volcano-stats` | E1 |
| `N₀` per workload from records | `volcano-stats` or an `ic bench` export of `exact_decomposition_ceiling` | E3a |
| `prime-abscissa:subgroup=filter\|project` | `PrimeAbscissaBase` | E3b |
| cofactor-ladder curve export | `volcano-rho` search with fixed `h` | P2 |

## 7. Fixed wording

- "generator changes *metric* by less than ±2% (90% CI [lo, hi]) on these
  curves at 24 bits"
- "relation streams under coupled seeds are identical on N/N curves"
- "yield follows the base's cofactor classes: slope …, residual τ … (null …)"

Scope: 24-bit prime fields, `prime-abscissa` and `glv-orbit` bases, MITM
oracle. No security statement.

---

# The fixed generator and index calculus: a base built from G, reusable across every key

**Status: design (PLAN), not frozen.** `PROTOCOL.md` freezes N, arms,
metrics and decision rules before any measured run, per the volcano-cost
convention.

**The question (owner, 2026-10-19).** On P-192 and P-224 the generator `G`
is fixed by the standard, and every public key is `Q = [k]G`. Does fixing
`G` give the attacker anything in index calculus: either something known
about a factor base, or a factor base built from `G` that serves every
target in `⟨G⟩` at once?

## 1. What is already settled by the code and by algebra

**A. The abscissa base ignores G.** `prime_factor_base`
(`ic_boundary.rs:4396`) and `PrimeAbscissaBase::build` read the curve
model only. The base is the same point set for every generator.

**B. The relation stream is G-invariant in law.** Every target source
draws `R = [a]G + [b]Q` (or jumps and strides built from such points).
Under `G' = [c]G`, `(a, b) ↦ (a·c⁻¹, b)` is a bijection of coefficient
space, so the stream has the same law, the rows are identical under
coupled seeds, and the solved logs scale by `c⁻¹`. On a prime-order curve
(cofactor 1, P-192 and P-224) there is only one subgroup, so this covers
every generator. **So no statistic of the abscissa base or of its
relation matrix carries information about G.** The one-off checks for
this are in §4, E0.

**C. What fixing G does give: amortisation.** The logs of a factor base are
taken base `G`. With `G` fixed by the standard they are computed once per
curve and reused for every key. The repository already measures this
split (`ic_single_target_20260930`: reusable set-up against online
interval, break-even target counts 7–37 at 2^37–2^47 on Koblitz curves,
2.0–8.0× above the Bernstein–Lange generic-precomputation model).

**D. The oracle in use does not exploit the abscissae.** `MitmOracle`
(`plugins.rs:1033`) is a table of pair sums probed per target. It never
uses the algebraic description "x is small". Only a Semaev/Gröbner oracle
does. So, for the MITM pipeline, the abscissa base is an arbitrary point
set of size B, and it can be swapped for any other set of size B without
changing the relation law, except through the cofactor classes (§1E).

**E. Cofactor classes.** On curves with `#E = h·r`, the abscissa base is
drawn from the whole group; a target in `⟨G⟩` decomposes only when the
summands' classes `[r]P_i` cancel (`exact_decomposition_ceiling`,
`ic_boundary.rs:451`). A base built inside `⟨G⟩` has all classes zero.
Experiment E3a (PR #2288, aburan28/crypto) tested this count against the
volcano study's yield effects on existing data. On P-192 and P-224 `h = 1`,
so this is a toy-curve artefact to control for, not a feature of the
target curves.

## 2. The object to test: the known-log base

Let `F_G(B) = { ±[i]G : 1 ≤ i ≤ B }`.

- **Every log is known for free**: `log_G([i]G) = i`. There is no linear
  algebra and no set-up of `B + 1` relations. One relation decomposes one
  target: `[a]G + [b]Q = Σ ±[i_j]G` gives `k` at once.
- **Construction** is `B` additions (incremental), charged.
- **It lies in `⟨G⟩` by construction**, so on cofactor-`h` toy curves it
  realises the full `N₀` ceiling where the abscissa base loses a factor
  up to `h` (§1E).
- **Yield prediction (MITM, m summands):** identical to any other
  point set of size B in `⟨G⟩`, because the MITM oracle only tests set
  membership: success probability ≈ `C(2B, m) / r` per trial, as for the
  abscissa base at `h = 1`.
- **What it is, honestly:** with an MITM oracle this is a generic
  precomputation algorithm (a table of `B` known points, online cost
  `≈ r / N_m` lookups per target). It is the Bernstein–Lange shape with a
  different table, and it cannot beat `√r` without `B` growing, so there is
  no security claim and no exponent moves. Under the CLAUDE.md success
  criterion it is still a legitimate row if, at equal `B`, it removes the
  linear-algebra phase and the `B + 1`-relation set-up from the IC
  baseline's S, provided the saving survives the §5a band.
- **What it loses:** a Semaev oracle cannot use it, since `{[i]G}` has no
  polynomial description. So it is a result about the MITM pipeline, and
  it says which part of the current IC cost is the base's algebra (none)
  and which part is the table.

The precise claim to measure: **at equal B and equal oracle, `F_G(B)`
matches the abscissa base's online yield and removes its set-up and
linear algebra.** The natural follow-up is whether the current IC
baseline's advantage over rho, where it has one, is actually a
precomputation advantage any point set would have.

## 3. Population

- **P0: prime-order curves (the target shape).** 24-, 32- and 40-bit
  `y² = x³ − 3x + b` with prime `#E` (`a = −3` as NIST), 8 per size,
  exported by `volcano-rho`'s search with `--max-cofactor 1`, with a
  random generator per curve and, as an arm, the lexicographically
  smallest-x point as "the standard's G".
- **P1: the cofactor ladder.** 24-bit, `h ∈ {1, 2, 4, 8, 16}`, 8 curves
  each, for the `N₀` prediction of §1E.
- **P2: P-192 and P-224 themselves,** for the free checks of E5 only.
  No ECDLP run.

## 4. Experiments

### E0: G carries no information (exact checks, no statistics)

- **E0a.** Factor-base digest of `prime-abscissa:size=B` is identical
  across generators `g0, [c]g0, −g0` on every curve.
- **E0b.** Coupled seeds with an `a ↦ a·c⁻¹` hook in the target loop:
  identical target stream, identical relation-row digest, logs equal to
  `logs₀·c⁻¹` entry by entry, `k` recovered, and ΔS equal to the
  recomputed scalar-multiplication difference and nothing else.
- **E0c.** E0b for each target source (`random`, `walk`, `stride`,
  `stride-renew`).
- Any deviation is a bug, not a finding.

### E1: the known-log base against the abscissa base (primary)

- **Arms,** per curve of P0, equal `B` at the family optimum
  `≈ (#E)^{1/3}`, `mitm:negation_folded=1`, `m = 2` and `m = 3`:
  - `ic-abscissa`: `prime-abscissa:size=B`, set-up + online (current
    baseline).
  - `ic-knownlog`: `known-log:size=B`, construction + online.
  - `ic-abscissa-online`: the baseline with set-up excluded, the
    multi-target limit.
  - `rho-neg`: the matched rho reference.
  - A/A on `ic-knownlog`.
- **Targets:** 64 `planted_shared` per curve and arm; additionally a
  **multi-target block** of `T ∈ {1, 4, 16, 64}` keys under one `G`,
  reporting cost per key.
- **Metrics:**
  - F3 online yield, `relations_found / targets_tried`;
  - per-phase S: base construction, table build, target draw, table
    probes, linear algebra, total;
  - cost per key at each `T`;
  - S against `rho-neg` through `ecbench band` at 0, 0.1, 1 GAE.
- **Pre-registered predictions:**
  - online yield `knownlog / abscissa = 1.00`, 90% CI inside
    `[0.97, 1.03]` on P0 (`h = 1`);
  - linear-algebra phase of `ic-knownlog` is exactly 0 ops;
  - total S: `knownlog / abscissa < 1`, by the share set-up and LA hold
    in the baseline (measured in the dry run, reported before freezing);
  - at `T = 1`, `knownlog` is at or below `abscissa`; as `T` grows both
    tend to the online limit and the ratio tends to 1.
- **Decision:** improvement if the 90% CI of total S ratio lies below 1
  at every band weight, and the online yield CI contains 1. Otherwise
  "accounting, no improvement".
- **Controls:** planted `size = B + 1` vs `B` (predicted `((B+1)/B)²`);
  `--plant-cell knownlog:1.03` on the real records must be detected.

### E2: cofactor classes, where the two bases should separate (P1)

- Same arms as E1 at `h ∈ {1, 2, 4, 8, 16}`.
- **Prediction:** `knownlog / abscissa` online yield equals
  `N₀(knownlog) / N₀(abscissa)`, about `h` for evenly spread classes, and
  exactly 1.000 at `h = 1` (the control that ties E2 to E1).
- This is the §1E mechanism on fresh data. It says nothing about P-192 or
  P-224 (`h = 1`) and is reported as such.

### E3: does a Semaev oracle change the picture? (secondary)

- On P0 at 24 bits only, where the repository's prime-field Semaev/F4
  path runs: `ic-abscissa` with the summation-polynomial oracle against
  `ic-knownlog` with MITM, at equal `B`.
- **Prediction:** the Semaev arm's decomposition cost grows slower in `B`
  than the table probe; the comparison says at what `B` the algebraic
  base starts to pay for its linear algebra. No decision rule; reported
  as a crossover estimate with intervals.

### E4: "the standard's G" versus a random G (P0)

- Arms `gstd` (smallest-x point) and `grand` under both bases.
- **Prediction:** equivalent within ±2% on every metric, by §1B. This is
  the direct answer to "does the standard's choice of G help": it does
  not, in any measurable way, for either base.

### E5: free checks on P-192 and P-224 (no ECDLP run)

- **E5a. Small-multiple membership.** For `B = 2^20` and `2^24`, build
  `{[i]G}` and record the x-coordinates' sizes, Hamming weights and
  smallest-x element; compare with `B` random points' statistics. A
  match (predicted) says nothing in the first `2^24` multiples singles
  `G` out. Ties to slot S4 of the trapdoor program (E4a there: rigid
  derivation tests).
- **E5b. Known-log base cost at scale, by model only.** Project E1's fitted
  exponents to `r ≈ 2^192`: table size `B`, online probes per key, and the
  break-even `T` against rho, at each band weight. Reported as the
  distance remaining, not as a feasibility claim.

## 5. Order and stopping

1. E0 (instrument).
2. E1 dry run on two 24-bit curves to measure the set-up/LA share, then
   freeze `PROTOCOL.md`.
3. E1, E4 (same sessions), E2.
4. E3, E5 last; E3 optional.

## 6. Tooling

| change | where | for |
|:--|:--|:--|
| `known-log:size=B` factor base: `±[i]G`, `col_of = i − 1`, `coef_of = ±1`, via `FactorBase::from_column_map`; a `known_logs` vector the solver reads so the linear-algebra stage is skipped | `PrimeAbscissaBase`'s sibling in `plugins.rs`; the binary test helper `known_log_base` (`large_prime.rs:703`) is the model | E1–E4 |
| the pipeline reads `known_logs` and goes straight from one relation to `k` | `ic_run.rs` / `stages.rs` | E1 |
| `coeff_scale=c` hook in the target loop | `ic_boundary.rs` target match arms | E0 |
| `--generators` and explicit-Q targets, and `--max-cofactor 1` prime-order export | `volcano-rho --ecbench` | E1, E4 |
| cells = base / generator; `--n0` (from PR #2288) | `volcano-stats` | E1, E2 |
| multiple of `G` scan with x-statistics | `examples/` | E5a |

## 7. Fixed wording

- "at equal B the known-log base matches the abscissa base's online yield
  (ratio r, 90% CI [lo, hi]) and removes set-up and linear algebra; total S
  ratio s [lo, hi] at w = 0/0.1/1"
- "the generator changes *metric* by less than ±2% (90% CI) on these
  curves"
- "no security statement; a generic precomputation shape; no exponent
  moved"

Scope: 24–40-bit prime-order curves with `a = −3`, MITM and (E3) Semaev
oracles, the `prime-abscissa` and `known-log` bases. Nothing about
P-192/P-224 beyond E5's free checks and a model projection.

## Amendment 1 (2026-10-19, after the E1 dry run; PR #2297, `DRYRUN.md`)

The dry run falsified §2's equal-yield prediction, and the reason is
elementary: `{±[i]G : i ≤ B}` is an arithmetic progression, so its pair
sums are the multiples `[j]G` with `|j| ≤ 2B`, about `4B + 1` distinct
points instead of about `2B²`. Measured yield ratio known-log / abscissa was
0.0097 [0.0074, 0.0130] and 0.0092 [0.0072, 0.0126] on the two curves, and
`(4B + 1)/r` matches the measured hit rate on both. The base as written is
a baby-step table, and the dry run says so.

Two other readings change the plan:

- the baseline's set-up plus linear algebra is 0–2.5% of total S; the
  pair table is 51–77%. There was never a phase worth removing.
- at ±3% on yield, 64 targets per curve gives about ±30%; about 3,000 are
  needed.

**Changes to the experiments.**

1. **Sumset law replaces equal yield.** For any known-log base
   `{±[s_i]G}` the `m`-sum yield is `|mΣ S| / r` where `S = {±s_i}` and
   `mΣ` is the `m`-fold sumset. The progression is the minimum
   (`|2Σ| = 4B + 1`); a generic set gives `|2Σ| ≈ 2B²`. This is the
   pre-registered prediction for every known-log arm, checked against the
   measured hit rate.
2. **New arm `known-log:scalars=random`:** `s_i` uniform in `[1, r − 1]`,
   built by `B` scalar multiplications (charged) or by a chained
   pseudo-random increment walk `P_{i+1} = P_i + [d_i]G` with `d_i` from a
   small precomputed set (charged as additions). Predicted yield equals the
   abscissa base's at equal `B` on prime-order curves, with linear algebra
   at 0. This is the object §2 meant; it is Bernstein–Lange's table with
   known logs, and the honest framing in §2 stands.
3. **Comparator added:** `bsgs.negation` at equal memory, since the
   progression base is a BSGS table and the random-scalar base is a
   randomised one. Both known-log arms are reported against rho and BSGS
   through the §5a band.
4. **Sample size:** 3,000 targets per curve on P0 for the yield metric;
   64 remain for the per-phase S readings.
5. **E1's decision rule** now reads: improvement only if total S of
   `known-log:scalars=random` is below the abscissa baseline's at every
   band weight and below `bsgs.negation` at equal memory. Given the 0–2.5%
   set-up share, the prediction is "accounting, no improvement" at `T = 1`;
   the multi-target block is where any difference can appear.

The remaining experiments (E0, E2–E5) are unchanged. `PROTOCOL.md` is
frozen after this amendment's dry run of arm 2 on the same two curves.

## Appendix: the earlier draft (2026-10-19, superseded)

The first draft of this file asked whether changing the generator changes
IC cost on the isogeny-volcano toy curves, in the manner of
`volcano-rho --instance-study` for rho. Its E0/E1 are kept above; its E3a
ran as PR #2288 (cofactor classes explain the volcano study's IC yield
effects). Its E2 (a generator inside the base gives one free column) is the
`B = 1` case of §2 and is subsumed.
