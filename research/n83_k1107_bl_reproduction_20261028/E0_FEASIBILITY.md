# Why the §8a E₀ m = 83 gate cannot run with the current IC pipeline

AGENTS.md §8a names `E₀: y² + xy = x³ + 1` over GF(2^83) as the
high-fidelity gate, with group order `4 · 2417851639230796216685689`.
Every measured N83 IC row in this repository uses the legacy curve
`EC1N83Ckb1a1haacb69ee9f37` (`y² + xy = x³ + x² + 1`). Its prime subgroup
has `r = 8569786107849059` and cofactor 1128547018. This note records why
that substitution exists. It is *derived* from repository formulas, not
measured.

## 1. A code barrier: the wide pipeline's scalar field is one word

`examples/koblitz_orbit_dlp_fast.rs` (wide module, `run`) computes
`r = curve.subgroup_order.to_u64().unwrap()`. On E₀,
r = 2417851639230796216685689 ≈ 2^81.0 exceeds `u64::MAX`, so the process
panics before any IC stage. The relation matrix, column logs, descent and
scalar check all work modulo that `u64`.
`research/ecbench_m83_ic_20261009/PROTOCOL.md` ("Native wide-pipeline
dependency map") lists the same one-word limits in `ic_boundary`,
`ic_framework::InstanceCtx`, `ic_framework::linalg` and `CountedGroup::key`.
Widening those to `u128` or checked big integers is necessary for E₀.

## 2. An algorithmic barrier: relation yield at r ≈ 2^81

Widening alone does not make the gate reachable. The current relation
search is the 4-sum, or m = 3, pair table. Its yield model in
`RESEARCH_ECC2K130_IC_FEASIBILITY.md` §6–7 is `probes/relation ≈ r/(n²K²)`
for n = 83 and K folded columns.

| curve | r | K | probes per relation | relations (≈ K) | total probes |
|:--|--:|--:|--:|--:|--:|
| legacy a = 1 (measured baseline) | 8.57·10¹⁵ ≈ 2^52.9 | 1107 | 1.0·10⁶ | 1.1·10³ | 1.1·10⁹ |
| E₀ (§8a gate) | 2.42·10²⁴ ≈ 2^81.0 | 1107 | 2.9·10¹⁴ | 1.1·10³ | 3.2·10¹⁷ |

The legacy figure is consistent with the measured roughly 114 s cold
process (`research/n83_cold_base_confirm_20261021/RESULT.md`).

The matched rho reference on E₀ is the signed-Frobenius walk with
A = 2n = 166 usable automorphisms. Its expected cost is
√(πr/(2A)) ≈ 1.5·10¹¹ ≈ 2^37.1 steps, and that needs no precomputation.
So the current IC on E₀ needs about 2·10⁶ times as many probes as rho
needs steps, before counting the cost of a probe.

Raising K does not close this gap. Total probes scale as r/(n²K). Matching
rho's 1.5·10¹¹ would need K ≈ r/(n² · 1.5·10¹¹) ≈ 2.3·10⁹ columns. That
size rules out the linear algebra on any host this program uses.

The same section's design note for a 6-sum extraction gives
`45·r/(n⁴K⁴)` ≈ 1.5·10⁶ probes per relation on E₀ at K = 1107. That is the
legacy curve's regime. The 6-sum kernel is not built. The note itself
calls the S_{k+1} step "the structural obstacle", and
`research/ic_techniques_20261012/sixsum_extraction/` measured the kernels
far above that model at n = 41.

## 3. What follows

- A complete-DLP E₀ m = 83 comparison is unreachable for the current
  pipeline on any host. This is a cost result, not a host limitation.
  Under §8a, high-fidelity ECC2K-130 improvement therefore stays
  **unestablished**, and the m = 83 comparison stays **missing**.
- Two prerequisites, in order:
  1. The u128 widening of §1, cross-checked against the one-word path
     wherever both run, as the m = 83 protocol requires.
  2. A relation search with higher arity than the 4-sum whose measured
     probes per relation are near the 6-sum model.

  Only then does a budgeted E₀ run make sense.
- The legacy-curve reproduction in this directory is the run this pipeline
  can make. It is **not** a §8a gate result.
