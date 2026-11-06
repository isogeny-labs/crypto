# Protocol: toy falsifier for the `θ₂` Coppersmith ceiling on `S₃`

**Frozen 2026-10-23, before the first data point of the run it governs.**
Study directory: `research/coppersmith_s3_theta2_20261023/`.
Instrument: `examples/coppersmith_s3_theta2_falsifier.rs` (Rust; Python only plots).
Pre-registered in [`../notes/ecdlp-general/RESEARCH_LATTICE_ECDLP_ROUTES.md`](../notes/ecdlp-general/RESEARCH_LATTICE_ECDLP_ROUTES.md) §7 item 1.

## 1. Question and the claims each outcome licenses

The note's §6 derives, for Coppersmith-type lattices on Semaev's
`S₃(x₁, x₂, x(R)) ≡ 0 (mod p)` with a small-`x` factor base `x(P_i) ≤ B = p^β`,
a root-region ceiling `∏ X_i ≤ p^{θ₂}`, `θ₂ = 1/3`, i.e. `β ≤ 1/6` per variable
as the multiplicity `ℓ → ∞`, and at finite multiplicity the determinant
condition of the Jochemsz–May basic box lattice gives

| `ℓ` | dim | `β_det(ℓ)` per variable |
|---:|---:|---:|
| 1 | 9 | 1/18 = 0.0556 |
| 2 | 25 | 1/10 = 0.1000 |
| 3 | 49 | 5/42 = 0.1190 |

(These equal the "theory column" that `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md`
§3.2 and `research/lit2026_w10_s5_coppersmith_20261011/` §5.1 computed independently.)
Rho parity for the two-summand route needs `β ≥ 1/4` (`θ ≥ 1/2`); §6 says it
is never reached.

Claim classes (controlled-experiment skill §1):

- `β_det(ℓ)` and `θ₂`: **heuristic estimate** (a Coppersmith determinant
  argument). This protocol tests them.
- "Recovery stops below `β = 1/4` at every size tested": **comparative /
  statistical**, established only by this experiment with its controls.
- Nothing here is a speedup, an improvement, or a security statement.

Outcomes:

- **Refuted.** If the pipeline recovers the planted decomposition in at least
  5 of 10 trials in any cell with `β > 1/4`, the §6 ceiling is wrong and the
  note is corrected (a dated correction, the original sentence kept).
- **Closed at measured level.** If recovery is 0 of 10 in every cell with
  `β ≥ 0.26` at every size and multiplicity, and the positive controls pass,
  the small-`x` Coppersmith route is closed at the measured level for
  `ℓ ≤ 3`, in addition to the derivation.
- **Inconclusive.** Anything between, or a failed positive control: reported
  as such, with the cells.

## 2. Instances, grid and sample size

- Primes: the largest prime below `2^bits` for `bits ∈ {24, 32, 40, 56}`
  (`u64` field arithmetic). One random curve `y² = x³ + ax + b` per size,
  `a, b` uniform in `[1, p)` with `4a³ + 27b² ≠ 0`, drawn from
  `StdRng(seed ^ (bits << 8))`, `seed = 20261023`.
- Multiplicity `ℓ ∈ {1, 2, 3}` (lattice dimension `(2ℓ+1)²`: 9, 25, 49).
- `β ∈ {0.04, 0.06, …, 0.34}` (16 values), `B = ⌊p^β⌋` clamped to `[1, p−1]`.
- `N = 10` trials per cell `(bits, ℓ, β)`; trial seed
  `seed·1000003 + bits·10⁶ + ℓ·10⁵ + i_β·10³ + t`. Each trial draws two
  points with `1 ≤ x ≤ B` (uniform `x`, random sign of `y`), redrawing while
  `P₁ + P₂ = O`, and sets `z = x(P₁ + P₂)`.
- 4 × 3 × 16 = 192 planted cells, 1920 trials.

**Resolution.** The decision rule is a per-cell count, not a rank test. With
`N = 10`, a cell at 0/10 bounds the recovery probability by 0.31 (one-sided
Clopper–Pearson, 95%). The refutation criterion (≥ 5/10 in a cell with
`β > 1/4`) has, under a true per-trial recovery probability of 0.05,
probability `≈ 6·10⁻⁵` per cell; over the 60 cells with `β > 1/4` the family
false-refutation probability is below `4·10⁻³`. That is the only test family.

## 3. Controls

- **Positive control (detector power).** For each `(bits, ℓ)`, the cell with
  the smallest grid `β` such that `B ≥ 4` must recover in at least 8 of 10
  trials, *provided* that `β ≤ β_det(ℓ) + 0.02`; where no grid `β` satisfies
  both (`ℓ = 1` at 24 and 32 bits: `β_det = 0.056` gives `B < 4`), the
  `(bits, ℓ)` row has no positive control and is excluded from the verdict.
  A failed positive control makes its row *inconclusive*, not "closed".
- **Negative control.** For each `(bits, ℓ)`, 10 trials of the same pipeline
  on `z = x(R)` for a uniformly random point `R` (no planted small
  decomposition) at `β = 0.30`. Expected: no genuine pair reported (the
  chance that a random `R` has a two-summand decomposition with `x ≤ B` is
  about `B²/(2p) = p^{−0.4}/2`). Raw candidates that fail `S₃ ≡ 0 (mod p)`
  are counted separately and are not findings.
- **Same instrument on every arm:** one binary, one LLL (`lll_reduce_hp`,
  `δ = 0.99`), one extraction routine, one `ROWS = 6`, same seeds policy.

## 4. Metrics (per trial, raw rows in `trials.jsonl`)

- `recovered`: the pipeline's reported set of genuine pairs (integer common
  roots of two reduced polynomials among the first six LLL rows, verified
  against `S₃ ≡ 0 mod p`) contains `{x(P₁), x(P₂)}`. **The decision metric.**
  The planted root is not used to select rows or pairs.
- `recovered_hg`: as above, restricted to pairs of rows that satisfy
  Howgrave-Graham (`‖v‖ < p^ℓ/√dim`), the rigorous sufficient condition.
- `hg_first_two`, `rows_vanish` (number of reduced rows vanishing at the
  planted root over `Z`): diagnostics only; the second uses the root.
- `lll_ms`, `extract_ms`, `pairs_tried`, `pairs_dependent`, `candidates`.

## 5. Decision rule

As in §1, on `recovered` only. Cells are reported in full; no cell, size,
`β` or `ℓ` is dropped after the run. Changing `N`, the grid, the metric or
the thresholds after seeing data is a new, dated protocol version, and both
are reported.

## 6. Pre-flight runtime (dry run, 2026-10-23, this container, 1 thread)

Dry run: `--bits 24,56 --ell 1,2,3 --beta 0.06:0.30:0.08 --trials 2 --neg-trials 2`,
27 s wall. Per trial: `ℓ = 1`: 1–5 ms LLL; `ℓ = 2`: 10–420 ms; `ℓ = 3`:
0.1–6 s (largest at the smallest `β`, where the basis is far from reduced);
extraction ≤ 100 ms, rising to ≈ 45 ms per pair at `B = 1.1·10⁵`. Estimate
for the full grid: `ℓ = 3` ≈ 640 trials × ≈ 2 s ≈ 20–25 min, `ℓ = 2` ≈ 3 min,
`ℓ = 1` < 1 min, negative controls ≈ 2 min: **≈ 30 min** total. The dry run's
positive-control cells (`β = 0.06`) recovered 12/12; its `β ≥ 0.22` cells
recovered 0/24; the negative control reported no genuine pair in 12/12
trials (raw candidate `(0, 0)` in every one, which the filter of §4 removes).

## 7. What this does not test

Only the Jochemsz–May basic box lattice of the single polynomial `S₃`, at
`ℓ ≤ 3`, with the repository's LLL (no BKZ). Extended strategies, the
symmetric `(e₁, e₂)` rewriting and `S₄` were measured in Probe C and W10 and
are not re-run here. A measured ceiling at `ℓ ≤ 3` is evidence for the
derivation's finite-`ℓ` rows, not a proof of the `ℓ → ∞` limit.
