# Protocol: ECDLP cost across whole isogeny classes (prime, binary, Koblitz)

Version 1, frozen 2026-10-18 before any measurement of the run it governs.
Instrument: `isoclass-rho` (`src/bin/isoclass_rho/`), commit recorded in the
run README. The `controlled-experiment` skill's §3 items follow in order.

## 1. Question and claims

**Question.** Carry one ECDLP instance `Q = kP` (`P` of prime order `n`) by
explicit, verified isogenies to many curves of its isogeny class over `F_q`,
spread over the class's endomorphism-ring strata and reached through
"easy" (small `ℓ`) and "hard" (large `ℓ`, large kernel field) isogenies, and
run the same Pollard rho on every curve. Does the curve, its stratum, its
level in any `ℓ`-volcano, the degree or the kernel field of the isogeny that
reached it, the generator of `⟨P⟩`, or the curve the instance was drawn on,
change the cost of solving?

**Null (theorem).** Over `F_q`, isogenous curves have the same `#E`, and an
isogeny of degree prime to `n` is an isomorphism on `⟨P⟩`; a generic walk
depends only on the group `⟨P⟩`, so its expected cost is the same on every
curve, for every generator and every source curve: `√(πn/2)` operations
(`×1/√(1 − 1/r)` for an `r`-adding walk, `×1/√|G|` when the walk is folded
by a group `G` of cheap maps).

**Claims this run can license.**

| claim | class | what establishes it |
|:--|:--|:--|
| `#E`, `n`, `t`, `D`, `D_K`, `f`, class numbers, class size | exact computation | BSGS count checked on eight points per curve; reduced-form count of `h(D_K)`; conductor formula, cross-checked by counting `h(f²D_K)` |
| every edge is a rational isogeny carrying the instance | exact computation | image on the codomain, `φ(P+Q) = φ(P)+φ(Q)`, `n·φ(P) = O ≠ φ(P)`; every rho solve returns the planted `k` |
| "curve X / stratum S / generator g / source s costs the same as the rest" | comparative | this protocol |
| "the automorphism / Frobenius fold on the crater costs `1/√|G|`" | comparative, positive control | this protocol |
| "an `ℓ`-isogeny of this size costs this much to find" | measurement | wall time per kernel, labelled wall |

Nothing here is an index-calculus improvement; §5a pricing does not apply
(rho against rho, one instrument). Not beating anything is not a failure.

## 2. Targets, null population, positive controls

Four classes, one root curve each (identities from `isoclass-rho search`,
seeds 11, 13, 42; the Koblitz curve by construction):

| id | field | root curve | `#E = h·n` | `t` | `D = f²·D_K` | class size `Σ_{d|f} h(d²D_K)` |
|:--|:--|:--|:--|:--|:--|:--|
| P1 | `F_p`, `p = 235212858169` | `a = 228505936335`, `b = 82673460332` | `9 · 26134762081` (n: 35 bits) | 68003 | `315² · (−8205107)`, `h(D_K) = 612` | 347,004 |
| P0 | `F_p`, `p = 217710133177` | `a = 0`, `b = 205801627284` (`j = 0`) | `28 · 7775395183` (n: 33 bits) | −931946 | `27792² · (−3)`, `27792 = 2⁴·3²·193` | 38,472 |
| B1 | `F_2^37`, `z³⁷+z⁶+z⁴+z+1` | `a = 89938361934`, `b = 61869205585` | `2 · 68719627817` (n: 37 bits) | −302161 | `833² · (−660703)`, `833 = 7²·17`, `h(D_K) = 296` | 327,080 |
| K | `F_2^41`, `z⁴¹+z³+1` | Koblitz `E_0: y² + xy = x³ + 1` | `4 · 549756390943` (n: 40 bits) | −2308219 | `703889² · (−7)`, `703889 = 409·1721` | computed by the walk |

Identities in P1/P0/K are decimal; binary coordinates are the integers whose
bits are the polynomial-basis coordinates in `F_2[z]/(poly)` with the
tool's first irreducible of that degree. The `search` records are in the
run directory.

**Walked primes.** P1: `ℓ ∈ {2,…,19}` everywhere and `{23,…,61}` at the
first three vertices (hard, horizontal: kernel fields up to `F_{p^30}`). P0:
`{2,3,5,7,11,13}` (2-volcano of depth 4, 3-volcano of depth 2; `ℓ = 193`
divides `f` but is out of reach of `Φ_193` in this session: stated as a
limit). B1: `{7, 17}` everywhere (volcanoes of depth 2 and 1) and `{23, 31}`
at the first three vertices. K: `{409, 1721}`, the only primes with any
rational isogeny off the crater (for odd `m`, every prime of the conductor
is `≡ ±1 (mod 8)`; `h(−7) = 1`, so the horizontal edges are loops), by the
torsion route (kernel abscissae in `F_{2^{41·204}}` and `F_{2^{41·215}}`),
three sampled kernels per vertex. The tool records which primes were
inactive and why.

**Null population (negative control).** The measured curves themselves
under the paired seeds are the population the null predicts to be
homogeneous; its spread is calibrated by (i) the isomorphic-model controls
(P1, P0: three random `(u⁴a, u⁶b)` models of the root; B1, K: the root in
three other polynomial bases of the field), which change every coordinate
and nothing else, and (ii) an A/A replica (the root under an independent
seed set). This is the right null: same instrument, same `⟨P⟩`, only the
representation differs.

**Positive controls (detector power).** Run through the same code path on
the root:
- `plant-neg`: the walk folded by negation, expected mean ratio `1/√2 =
  0.707` (strong effect);
- `plant-r4`: a 4-adding walk, expected `√(1−1/32)/√(1−1/4) = 1.137` (weak
  effect, Teske's `1/√(1 − 1/r)`);
- `fold-aut` on P0 (order-6 automorphism group of `j = 0`, expected
  `1/√6 = 0.408`) and `fold-frob`, `fold-frobneg` on K (Frobenius orbits of
  size 41 and 82, expected `0.156` and `0.110`): the structural, curve-
  specific effects the question is about, present by construction on the
  crater and absent (not computable) off it.

## 3. Instrument and metrics

`isoclass-rho measure` on the recorded `walk.json`: single-walker
32-adding distinguished-point rho, `θ = 2⁻⁸`, no negation map unless
folded, restart after `40·2⁸` DP-free steps; trial `s` uses seed `S ^ s` on
every arm; all (arm, seed) jobs shuffled and run on 4 threads. Metrics per
solve: group operations (`iters`, primary), restarts, distinguished points,
fruitless-cycle escapes, wall ns. Per arm: n, min, max, mean, median,
p10/p25/p75/p90, sd, CV, skewness, excess kurtosis, geometric mean, normal
and bootstrap 95 % intervals of the mean, ns per operation, mean ms, mean
end-to-end ms (transport of the instance to the curve + solve), ratio to
`√(πn/2)`, ratio to the pooled mean. Transport per curve: tree edges, `Σ
log₂ ℓ`, kernel field degrees, kernel-finding and push ms. Wall times are
indicative (shared 4-vCPU VM); operations are the unit of comparison.

Modes and budgets (frozen):

| class | `class` | `generators` | `shift` |
|:--|:--|:--|:--|
| P1 | every measured curve × 100 seeds; 3 iso controls, A/A, 2 plants | 6 curves spread over the strata × 100 generators × 25 seeds | 5 sources (root + 4) × every measured curve × 25 seeds |
| P0 | same + `fold-aut` | 4 curves × 100 generators × 25 seeds | 4 sources × all × 25 |
| B1 | same (3 representation controls) | 6 curves × 64 generators × 25 seeds | 5 sources × all × 25 |
| K | same + `fold-frob`, `fold-frobneg` | 3 curves × 64 generators × 25 seeds | 3 sources × all × 25 |

Generators: `P_i = r_i·P`, `Q_i = r_i·Q` with `r_i` uniform in `[1, n)`
from the seed, the same `k` everywhere. Shift: an instance `(P_s, k·P_s)`
drawn on source curve `s` and carried to every measured curve through
the dual edges up to the root and the tree edges down.

## 4. Decision rules (fixed before data)

- **R1, per curve (class mode).** Arm `i` against the pooled solves of the
  other measured curves: Welch `z`. Family `m` = number of measured curves;
  `α = 0.05`; a curve is *more extreme than the family* if `|z| > z_crit`
  with `z_crit` the two-sided normal quantile at `α/m`. Otherwise the
  verdict is *within the control distribution*. Reported alongside: KS `D`
  and `p`, Mann–Whitney `z`, rank of the arm mean among arm means.
- **R2, across curves.** Kruskal–Wallis over the measured curves and
  one-way ANOVA with the between-curve variance ratio `var(arm means) /
  (pooled var / T)`; heterogeneity is *detected* if KW `p < 0.05`, else *not
  detected*. The A/A replica and the controls must fall inside the R1 band
  (if not: the instrument drifted; the run is reported invalid).
- **R3, by stratum / level / distance / parent `ℓ` / kernel degree.** Pooled
  groups, Welch `z` against the rest, family = number of groups of that
  kind, same `α`.
- **R4, positive controls.** The 95 % interval of `plant-neg` must exclude 1
  and contain `0.707 ± 0.05`; `fold-aut` / `fold-frob(neg)` must exclude 1
  and lie within ±15 % of `1/√|G|`; otherwise the detector failed and every
  null verdict of that class is *inconclusive*. `plant-r4` is reported with
  its interval against 1.137 (expected `|z| ≈ 2.6` at `T = 100`: a 14 %
  effect sits near the single-arm threshold, which is what it calibrates).
- **R5, generators.** Per curve: KW across generators, variance ratio, and
  Bonferroni-flagged generators (family = curves × generators); Spearman
  correlation of generator means with `r`, `bits(r)`, `popcount(r)`, `x(P_i)`
  (reported, `|ρ| > 0.3` with `p < 0.05/4` counts as a correlation).
- **R6, shift.** KW by target and by source, `η²` of each factor;
  Bonferroni-flagged cells (family = sources × targets).
- **Not allowed after data:** changing `T`, the generator count, the seeds,
  the walked primes, the measured set, the folds or the thresholds. Any such
  change is a dated amendment and both versions are reported.

**Power check (`2/(N+1) < α/m` analogue for `z`).** Single solve sd ≈
0.52 × mean (rho's cost distribution). Per-curve mean at `T = 100`: sd ≈
5.2 % of the mean; with `m ≈ 100` curves, `z_crit ≈ 3.5`, so a per-curve
effect of about 18 % is detectable, a stratum effect (8 curves pooled) of
about 6.5 %, a whole-class shift (100 curves) of 1.8 %. The strong plant
(29 %) is detected per arm (`z ≈ 5.6`); the weak plant (14 %) is not
expected to pass the per-arm threshold. Generators at `T = 25`: per-
generator sd ≈ 10 %, family 600 → `z_crit ≈ 3.9`, detectable per-generator
effect ≈ 40 %; a spread across generators is tested by the variance ratio
and KW (power to see a 10 % between-generator sd: variance ratio ≈ 2).

**Pre-flight runtime (measured in the dry runs on this host, 4 vCPU Xeon
2.10 GHz).** Solve: P1 ≈ 0.12 s, P0 ≈ 0.07 s, B1 ≈ 0.10 s, K ≈ 0.5 s (cold
estimate from 40-bit `n` and 0.5 µs/op). Fixed costs: walks P1 ≈ 2 min
(Φ_ℓ up to 61: 25 s), P0 ≈ 1 min, B1 ≈ 3 min (ℓ = 23, 31 kernels 1.3–1.7 s
each), K ≈ 30 min (each 409-kernel ≈ 1 min, each 1721-kernel ≈ 2 min).
Per-sample: P1 class 106 arms × 100 ≈ 5.5 min wall, generators 15,000
solves ≈ 7.5 min, shift ≈ 6.5 min; P0 ≈ 3 + 3 + 3 min; B1 ≈ 3 + 5 + 3 min;
K ≈ 4 + 10 + 2 min. Total ≈ 1.5 h wall including the walks. Dry runs (5
seeds, 4 generators, scratch output): all solves correct in all four
classes; `fold-aut` 0.33×, `plant-neg` 0.55–0.84× at 5 seeds; no
heterogeneity at that resolution; nothing about the targets disclosed
beyond the identities above.

## 5. Execution

Walks are executed by a Sonnet-tier agent from the frozen command card in
the run README (owner's instruction for this task), verbatim; measurements
by the main session; every command, exit status and record is kept.
Protocol hash (sha256 of this file at freeze) is recorded in the run
README before the first measurement.
