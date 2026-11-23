# Preregistration: does an algebraic decomposition reduce the IC-vs-rho slope β? (2026-11-18)

**To be frozen (`experiment.protocol/v1`, commit pinned) before any claim data.**
Class: engineering/research-design. Incumbent: the orbit m = 2 MITM arm of
`research/ecbench_frobenius_orbit_vs_rho_20261014/`, β = 0.165 [0.117, 0.213].
No security claim. This document states what would be run; it does **not**
authorize a run (loading instructions is not authorization; the experiment
director freezes the route and the executor runs it unchanged).

## 1. Question and the discriminating statistic

Does replacing the m = 2 MITM pair-table decomposition with a structure that
can change p_IC (the r-exponent of IC cost) **reduce β = p_IC − p_rho**, or does
it move only the constant again?

The incumbent and every candidate are measured in the **same counted unit S =
GAE/√r** on the **same curves**, so for each arm A the quantity

> **D_A(r) = ln( S_A(curve) / S_incumbent(curve) )**, paired by curve,

removes rho entirely and isolates the IC-internal exponent. Its slope

> **δ_A = d D_A / d ln r = p_A − p_incumbent**

is **0 under the null** (A differs from the incumbent by a constant only) and
**< 0 iff A grows strictly slower in r** (A bends the exponent toward rho). δ_A
is measured by `ecbench trend` (the study's own WLS of ln-ratio on ln r / ln n)
run on the paired per-curve S_A/S_incumbent ratios. The verdict on the
incumbent-vs-rho β is read exactly as the study does, as a cross-check.

## 2. Arms (all `*_full`, all priced across the §5a band)

Per curve, interleaved arm order, derived per-target seeds, arm labels masked
during analysis (controlled-experiment discipline):

- **`rho-frob`** — reference (same as study).
- **`rho-frob-aa`** — A/A control for the reference (must contain 1).
- **`ic-orbit-mitm2`** (INCUMBENT) — `compact-orbit-scan` + `mitm-frobenius-counted:m=2`,
  `stride-renew`, the per-curve `c` chosen exactly as the study chose it.
- **`ic-orbit-mitm2-aa`** — A/A control for the incumbent (noise floor for D).
- **`ic-orbit-mitm3`** (CONTROL arm, known analytic answer) — same base,
  `m = 3` MITM pair-table (`PairSumTable`, `RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md`:
  "a 3-decomposition is |F| lookups"). Prediction δ = 0 (see §3). Validates the
  δ-instrument against a case whose exponent is analytically unchanged.
- **`ic-alg-m3`** (PRIMARY scientific arm) — same base, **algebraic m = 3
  decomposition**: Semaev S₄ Weil-restricted + root-finding engine
  (`DecompositionStrategy` / `koblitz_index_calculus.rs`), per-target cost
  `poly(d_reg)` rather than Θ(N). The only arm whose mechanism can make δ < 0.
- **`ic-alg-m3-aa`** — A/A control for the primary arm.

All arms emit every native counter (orbit construction, hash probes, Frobenius
maps, canonicalisations, field mul/sqr/inv, word XOR, row operations, and — for
the algebraic arm — Macaulay/row-reduction counters and the Semaev/root-finding
field operations). **Any arm that names a counter but does not emit its count is
reported as a lower bound and cannot be ranked (§5a).**

## 3. Preregistered predictions (with the mechanism, so each is falsifiable)

- **H-control (`ic-orbit-mitm3`): δ = 0, interval contains 0.** Derivation: the
  m = 3 MITM balances table-build c² against search r/c² · c = r/c (prob a target
  is a 3-sum ∝ N³/r, N relations needed ⇒ r/N² trials × N per trial = r/N), so
  total ≈ c² + r/c, optimum c ∝ r^{1/3}, **r^{2/3} again**. Same exponent as m = 2.
  If this arm's δ is resolvably ≠ 0, the δ-instrument is mis-calibrated and the
  primary arm's reading is void (fail-closed).
- **H-primary (`ic-alg-m3`): δ ≥ 0 (no slope reduction).** Rationale: literature
  (Diem: q = 2, n prime ⇒ expected-exponential) + repo first-fall/variable-count
  evidence. We predict the algebraic solve does **not** keep per-target cost
  sub-N as n grows, so the ratio to the incumbent is flat or rising in r.
- **H-AA: every `*-aa ÷ *` contains 1** (noise floor). A curve whose A/A misses 1
  is reported unreliable and excluded from the slope fit.

## 4. Falsification criterion (the frozen rule)

For the primary arm, using the WLS slope δ_primary of D over the panel with
cluster-bootstrap 95% intervals, inflated by √(χ²/dof) as the study does:

- **"Moves the slope" (success) iff the 95% upper bound of δ_primary < −0.03.**
  (A resolvable negative slope: the algebraic arm grows strictly slower in r than
  the incumbent, net of constants.)
- **"Constant only again" (null confirmed) iff the δ_primary interval contains 0
  or lies above it.**
- **Inconclusive** otherwise (upper bound in [−0.03, 0)); report the interval and
  the n-span shortfall, do not claim either way.
- **Gate:** the result is read only if (a) H-control passes (δ_control interval
  contains 0), (b) every retained curve's A/A contains 1, (c) no arm is a lower
  bound, and (d) the order holds across the §5a band (§6).

The −0.03 threshold is set below the incumbent β interval's lower edge margin
and above the achievable δ-resolution (§5). A true exponent crossing toward rho
would drive δ_primary toward −1/6 ≈ −0.167 (IC exponent → 1/2), unmistakable; a
half-exponent move (2/3 → 7/12) is δ = −1/12 ≈ −0.083, comfortably resolvable; a
pure constant gain is δ = 0, the predicted outcome.

## 5. Required n-span and why

The incumbent β was resolved to a 95% half-width ≈ 0.048 (SE ≈ 0.0245 after
χ²-inflation) from **8 curves spanning log₂ r 10.97 → 39.0** (ln r span ≈ 19.4,
SD(ln r) ≈ 6.5). The slope δ of the *paired* ratio D has much smaller per-point
noise than the ratio-to-rho: the per-curve S_A/S_incumbent interval is a few
percent (se_ln ≈ 0.03–0.06) versus the study's se_ln 0.08–0.33 on the
rho-paired ratios, because the two IC arms share seeds, targets and host and
cancel. Statistically SE(δ) ≈ se_ln / (√#pts · SD(ln r)); with 8 shared-seed
points this is ≈ 0.003 — far finer than needed.

**The binding uncertainty is model mis-specification, not statistics**: the
constant of D shifts with n, cofactor, and the integer rounding of `c` (the
`free_r_n` γ in the study "rests on the two large-cofactor small-r curves"). To
guard against an n-confound masquerading as an r-slope, the design therefore:

1. **Reuses the full 8-curve panel** n ∈ {13, 17, 19, 23, 29, 31, 37, 41}, a0/a1
   as in the study, so D inherits the study's n/cofactor coverage and the
   incumbent column choices are unchanged.
2. **Adds 2 larger prime-n small-cofactor points** (target n = 43 and n = 47,
   cofactor ≤ 4, to be drawn/registered as cold public targets via ecdb), widening
   the lever to ln r span ≳ 22 and giving **≥ 10 points**, so δ is fit with ln n
   in the model (as the study's preferred `free_r_n`) and the r-slope is separated
   from the n-trend.

**Required span: log₂ r from ≈ 11 to ≥ 43 (ln r ≥ 22), ≥ 10 curves, ≥ 4 distinct
small-cofactor prime n above n = 31**, so that δ resolves against the
[0.117, 0.213]-scale with a 2σ resolution of ≈ 0.03. Fewer points or a narrower
span cannot distinguish a half-exponent move from a constant and the experiment
should not be run under-powered.

## 6. Cost model and pricing (same unit as the study, §5a band)

- Counted unit: **S = GAE/√r**, GAE = group-addition equivalents, additions and
  doublings = 1 unit each, every other native counter priced from the per-host
  `full_prices.json` table (`ecbench price-units`, median of 15 passes).
- **§5a band: price every unpriced native counter of every arm — rho references
  included — at 0, 0.1 and 1 GAE/op**, applied identically to all arms. Field
  counters (Semaev/root-finding mulmods on the algebraic arm; linear-algebra
  mulmods) also priced at the per-host calibrated `w_cal` and 10·w_cal, with
  `w_cal` measured by `perf-ipc` on an isolated host where available, else
  best-of-N timing, recorded with host and method.
- **Rank arms (and read δ) only where the order/sign holds across the whole
  band.** If δ_primary's sign flips between w = 0 and w = 1, report
  **unresolved at the band**, never a win. (The study's own robustness check —
  natives at zero — must be reproduced for every arm here.)

## 7. Verification (unchanged from the lab contract)

Every record verifies; `ecbench verify --replay-all` replays identically;
A/A = 1 on every retained curve; no record a lower bound; `VALIDATION.json`
names the replay certificate, validator report and matched-rho reference with
the §5a band; resume checkpoint filled before any run. 16 fresh public targets ×
3 rounds (+1 warm-up) per curve, L0, as the study.

## 8. Compute to price this (for the caller)

Order-of-magnitude, to be priced precisely by the director before freezing:

- **MITM arms (`mitm2`, `mitm3`) + rho arms**: ≈ the original study's budget. The
  study ran 8 sessions / ~2.8k records on a shared 4-vCPU host at L0 with
  descriptive wall; per-solve op-counts ~ r^{2/3}/n, dominated by n = 41
  (~2^{26} ops/solve × 16 × 3). Adding n = 43, 47 roughly ×4 and ×8 the n = 41
  cell. Estimate: **a few core-hours to ~1 core-day**, well within toy budget.
- **Algebraic arm (`ic-alg-m3`)**: the expensive and risky one. Per-target cost
  is the Semaev-S₄ Weil-restriction + root-finding/Gröbner solve; at n = 43, 47
  the Boolean monomial state and the (m−2)n chaining unknowns may hit the
  container's ≤16 GB / no-PMU limits (`RESEARCH_IC_LEADS_ASSESSMENT_20261018.md`
  §3). A refusal there is itself informative (it is the predicted mechanism). Fit
  δ on the n where the algebraic arm completes; report the n-ceiling.
  Estimate: **~1–3 core-days**, contingent on the n-ceiling; isolab-eks CPU,
  not GPU.
- **Total counted GAE across the sweep** ≈ dominated by the ≥ n = 41 cells,
  ~2^{31}–2^{33} per large cell × (arms × 48 solves) ⇒ order **10^9–10^10 counted
  GAE**. The §5a native share moves the wall, not the op-count ranking.

**Recommendation on ordering:** freeze and run the **cheap control + incumbent +
rho arms first** (`mitm2`, `mitm3`, `rho-frob`, all A/A) to validate the
δ-instrument (H-control must give δ = 0) and reconfirm the incumbent β on the
extended n = 43, 47 span. Only if that passes, add the `ic-alg-m3` arm. This
spends the risky compute last and makes the primary reading interpretable.
