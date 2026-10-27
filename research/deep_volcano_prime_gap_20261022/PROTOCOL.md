# Frozen protocol: ECDLP across a large prime conductor gap on a deep 2-volcano

Frozen 2026-10-22, before any ECDLP measurement on the class below. The class
itself is constructed deterministically from the parameters in §2 (no outcome
data). A pilot on a different, 34-bit class (`--bits 34 --depth 4 --seed 7`)
exercised the pipeline before this freeze; its numbers are not evidence and
are not reported as results. The pilot changed one design choice, stated in
§6: the primary bootstrap resamples curves, not targets.

## 1. Question and claims

The request: show that ECDLP hardness is the same across a large prime
conductor gap on a deep isogeny volcano.

Two kinds of answer are kept apart.

- **Proved** (report §Theorems, Lean where feasible): what follows from
  isogeny transfer and from the group being isomorphic, with exact
  hypotheses. That includes the known exception (j = 0 and 1728 craters have
  extra automorphisms) and the part that is open (KKM 2011; Galbraith 2024
  Problem B).
- **Measured** (this protocol): whether the counted cost of the strongest
  available generic method (`rho.negation`; `D₀ = −11` gives `Aut = ±1` on
  every curve) differs between the two sides of the gap, with every stratum
  of both 2-volcanoes covered. A measurement can bound a difference at this
  scale; it cannot prove equality.

## 2. The class

`volcano-gap construct --bits 52 --depth 6 --d0 11 --max-cofactor 16384 --seed 20261022`
(deterministic: `N` descends from the largest admissible prime, and traces
are taken in order of `|t|`):

| quantity | value |
|:--|:--|
| `p` | 4497080234220953 (52 bits) |
| `t` | −5051274 |
| `t² − 4p` | `−11 · (2⁶ · 631409)²` |
| fundamental discriminant `D₀` | −11 (class number 1; 2 inert) |
| Frobenius conductor `f` | `2⁶ · N`, `N = 631409` prime |
| gap prime `N` | 631409 ≈ 77·p^¼ (`log_p N ≈ 0.371`; KKM's regime `p^¼ < N < p^½`) |
| 2-volcano depth | 6 |
| `#E` | 4497080239272228 = 2388 · `n` |
| `n` | 1883199430181 (prime, ≈ 2^40.78) |
| `t + 2 ≡ 0 (mod N)` | yes, so `N² ∣ #E^twist` and every cross-gap kernel is twist-rational |

Order proof: a point `P ≠ O` with `[n]P = O` has order `n`, and since
`n > 4√p` the Hasse interval holds exactly one multiple of `n`, which is
`2388·n`.

**Sides.** `S1` = conductor `2^a` (`a = 0..6`), one 2-volcano with crater
`j(−11) = −32768`. `S2` = conductor `N·2^a`: `h(O_N) = N − 1 = 631408`
further 2-volcanoes of the same shape (`(−11/N) = (N/11) = +1`, since
`N ≡ 9 (mod 11)`; inert 2 gives one-vertex craters). Of the `N + 1`
`N`-isogenies from the S1 crater, 2 are horizontal (endomorphisms back to
`j(−11)`) and `N − 1` descend to S2. Prediction from the class-number formula: each
volcano has `1, 3, 6, 12, 24, 48, 96` curves on levels `0..6` (191 curves).

**Roots and transport.**

- S1 root: the `j(−11)` curve in the twist with trace `t`.
- S2 root: the codomain of a degree-`N` Vélu isogeny `φ` from the S1 root,
  with kernel `⟨K⟩` for `K` of order `N` on the quadratic twist. `φ` is
  checked to be a homomorphism on 8 random pairs, `φ(P)` has order `n`, and
  the dual `ψ` (from the S2 root's one twist-rational `N`-line) reaches
  `j(−11)` with `ψ∘φ = ±[N]`.
- One generator `P` on the S1 root, `φ(P)` on the S2 root. `volcano-rho
  --generator` carries each down its 2-volcano by verified Vélu 2-isogenies.
  `planted_shared` targets give the same `k` on all 382 curves.

**Side certificates.** Every curve and model is certified by
`volcano-gap certify`:

- S1: the twist's `N`-torsion has rank 2; Frobenius is `−1` on `E[N]`.
- S2: the twist's `N`-part is cyclic, proved by a witness point.

Plus the generator on the curve with order `n`, `#E` killing random points,
and no `j` shared between the sides.

## 3. Arms (ecbench, one binary, counted units)

| spec | arm | method | workloads |
|:--|:--|:--|:--|
| `spec.json` | `rho-neg` | `rho.negation` (reference; the strongest rho for `Aut = ±1`) | all 382 curves × 2 models (walked + one isomorphic rescaling) × 6 targets = 4584 runs |
| `spec-aa.json` | `rho-neg-aa` | `rho.negation`, measurement seed `⊕ 0xAA00000000` | 382 curves × model 0 × 4 targets = 1528 runs |
| `spec-bsgs.json` | `bsgs-neg` | `bsgs.negation` | 382 × model 0 × 2 targets = 764 runs |

Measurement block: rounds 1, **warmup 0** (the counted cost is deterministic;
warmups only matter for timing, and timing here is L0, descriptive),
order shuffle (S1 and S2 workloads interleaved in one session), seed
20261022, isolation L0 (shared 4-vCPU VM), timeout 600 s. The three sessions
run concurrently, one per core.

## 4. Metric

`S = GAE / √r` per run (`cost.s`), `r = n`, the same for every curve. The
§5a band at 0, 0.1 and 1 GAE per unpriced native operation
(`canonicalisations_uncharged` for `rho.negation`) is reported for both sides
with `ecbench band`. Both sides run the same method, so the band is
reported, not used to rank.

## 5. Primary statistic and decision rule

`G = mean S(S2) / mean S(S1)` over all `rho-neg` runs (levels 0..6, both
models, targets 0..5). Both sides have the same level composition, which is
checked against the prediction in §2.

- **Interval:** curve-cluster bootstrap, 4,000 replicates: curves resampled
  within each (side, level), each with all its runs. 90% and 95% percentile
  intervals.
- **Test:** permutation p for `|ln G|`, 10,000 permutations. Side labels are
  shuffled among the curves of each level.
- **Seed** 20261022 (`volcano-stats --vertical stratified --gap 0:1`).

Decision, fixed wording:

- **Equivalent within ±5%:** the 90% interval lies inside [0.95, 1.05]
  (two one-sided tests at α = 0.05), *and* the planted controls in §7 are
  detected.
- **Different:** the 95% interval excludes 1 and p < 0.05. If the effect is
  inside ±5%, the reading is "a detectable difference smaller than the
  margin".
- **Inconclusive:** anything else, or any failed control.

Power from the pilot variance: 90% half-width ≈ ±2% at 2292 runs per side,
so a true `G` within ±3% should read as equivalent.

## 6. Secondary (reported, not decisive)

- Per-level `G_l`, levels 0..6, with 95% intervals. Seven intervals: one
  missing 1 is not a finding. Level 0 has one curve per side and is wide.
- The crossed bootstrap (one shared draw of target indices, then curves). It
  is conservative for rho because it counts run noise twice. **Pilot
  change:** this was the planned primary until the pilot showed it about
  doubles the variance. The decision uses the curve bootstrap; both are
  reported.
- Within each side, floor over crater (`--vertical stratified`), and the
  horizontal and model Cochran Q with permutation nulls (as in
  `research/isogeny-volcano-cost_20261017`).
- The A/A arm's `G` and the `rho-neg` against `rho-neg-aa` pair.

## 7. Controls

| control | prediction | failure means |
|:--|:--|:--|
| A/A (`rho-neg-aa`): its own `G` | 95% CI contains 1 | instrument noise is miscalibrated: inconclusive |
| BSGS exactness | for each target, one cost on every curve of both sides (`max_distinct_cost_per_target = 1` and `G = 1` exactly) | transport or pairing is broken: the run is void |
| planted `--plant-class 1:1.05` (S2 × 1.05) | 95% CI excludes 1, p < 0.05 | the design cannot see 5%: equivalence is not claimed |
| planted `--plant-class 1:0.95` | 95% CI excludes 1, p < 0.05 | the same |
| isomorphic models (model 1 vs 0) | model τ inside its permutation null | coordinate effect: reported, as in the 2026-10-17 study |
| walk counts | `[1, 3, 6, 12, 24, 48, 96]` on both sides | the volcano is not the predicted one: the run is void |
| side certificates | all curves certified, no shared `j` | the run is void |
| audit | `ecbench verify --replay 24` on every session: all replays exact | the session is void |

## 8. What is not claimed

- No security statement about any deployed curve.
- No claim beyond this class, `p ≈ 2^52`, `D₀ = −11`, `rho.negation` and
  BSGS. Index calculus is not run: at `n ≈ 2^41` it is out of reach on this
  host, and earlier volcano studies found it 30–60× above rho at 24 bits.
- The class is special: `t ≡ −2 (mod N)` makes the cross-gap isogeny cheap
  (`O(N)` over `F_p`). Rho never uses an isogeny, so this does not enter
  the measurement. It does enter the transfer bound, which is reported as
  measured for this class only. The general case is the theorem section's
  job.
- An equivalence verdict bounds the difference of one method's mean cost at
  ±5% at this size. It does not prove hardness equal for every algorithm.
  That part is the transfer theorem, with its additive cost.

## Erratum (written as the sessions started, committed 08:53 UTC while they ran; before any result was read)

`1 + 3 + 6 + 12 + 24 + 48 + 96 = 190`, not 191. So each side has 190
curves, and the run counts are 380 × 2 × 6 = 4560 (`spec.json`),
380 × 4 = 1520 (A/A) and 380 × 2 = 760 (BSGS). The walk found exactly
190 per side with the predicted level counts. Nothing else changes.

A second typo, found by the validator after the run: §3 writes the A/A
measurement seed as `⊕ 0xAA00000000`. The code (`volcano-gap spec`) and
`spec-aa.json` use `0xA_A000_0000` = `0xAA0000000`, the same constant
the 2026-10-17 volcano-cost study used. The design is unaffected: the A/A
arm only needs a seed different from the primary's.

## 9. Reproduce

```
cargo build --release --bin volcano-gap --bin volcano-rho --bin ecbench --bin volcano-stats
research/deep_volcano_prime_gap_20261022/run_pipeline.sh build research/deep_volcano_prime_gap_20261022/run 52 6 16384 20261022 2 6 4 2
research/deep_volcano_prime_gap_20261022/run_pipeline.sh run   research/deep_volcano_prime_gap_20261022/run
research/deep_volcano_prime_gap_20261022/run_pipeline.sh audit research/deep_volcano_prime_gap_20261022/run 24
target/release/volcano-stats --records run/sessions/spec/records.jsonl,run/sessions/spec-aa/records.jsonl,run/sessions/spec-bsgs/records.jsonl \
  --curves run/export-s1/curves.jsonl,run/export-s2/curves.jsonl --vertical stratified --gap 0:1 \
  --boot 4000 --perm 10000 --seed 20261022 --pair rho-neg:rho-neg-aa --out analysis.json
```

plus the two planted runs (`--plant-class 1:1.05`, `--plant-class 1:0.95`).
