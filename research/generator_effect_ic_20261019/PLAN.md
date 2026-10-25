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
