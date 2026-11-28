# PROTOCOL: sibling divisors D − M from the shared Cantor step in the genus 2/3 relation walk

Frozen 2026-11-23 before any data was taken (hash in `frozen.sha256`). Isolation L0, one shared
4-vCPU VM, Python only (see "Constraints"). Every measured number from this study is
`[pcc:unverified]`. Every number quoted from an earlier study keeps that study's standing.

## 0. Map of what the lab already has (read before choosing the lever)

- Pipeline: `src/cryptanalysis/genus_ladder.rs` (the lab's genus 2–5 hyperelliptic IC: walk
  `cur ← cur + M_j`, 16 jumps, restart every 4096 steps; parity prefilter χ(disc u); `x^p mod u`
  test; Cantor–Zassenhaus roots; Ferrari at g = 4; 2LP arm; SGE + coordinate Wiedemann).
  Counted unit: Cantor `F_p` multiplications + inversions (`cantor_mul`, `cantor_inv`), relation
  phase `rel_mul`, linear algebra `la_mul`; the wall-clock study prices an inversion as 10 mults.
- Best recorded g = 3 cell (`genus_ladder_wallclock_vs_rho_20261122`, p = 65537, 2LP + prefilter
  + SGE(8,3)): IC/rho wall 0.19–0.26 [pcc:unverified]. Per run: Cantor ≈ 219.7 mults and 5.0
  inversions per group operation, `rel_mul` ≈ 253 per candidate, LA ≈ 50 M with SGE.
- Root-finding levers already measured in the lab and therefore NOT repeated here:
  parity prefilter (exact, `genus_ladder` notes); Cardano–Lucas cubic test and Cardano roots at
  g = 3 (`ic_positive_control_lucas_20261116`: relation phase 0.45–0.52×, not yet ported to
  `genus_ladder` at g = 3 — a port, not a new lever); Ferrari at g = 4
  (`genus4_positive_control_20261116`); single/double large primes (`genus_ladder*`,
  `hec_native2lp`, `hec_sparse2lp`); bounded/forest/SGE LA variants (`genus_ladder_*`, `hec_*`).
- Product-tree batch `x^p − x` smoothness is predicted a loss at g ≤ 3 [derived, not measured]:
  per-candidate schoolbook squaring mod a degree-g `u` costs about 2g² mults per bit of p, a
  remainder tree costs at least M(kg)/k·log(kg) per bit per candidate, which exceeds 2g² for g ≤ 3
  for any fast M. Not measured; recorded so it is not re-proposed without a new argument.
- Codex's "Reproduce the genus 2–4 HEC comparison" (PR #4789, branch research/hec-tenfold-replay,
  not present on origin here): a replay. This study does not touch it.
- Sibling idea in the lab so far: elliptic only (`ic_ksum_sibling_20261118`, walk `sib`, shared
  inversion for x(S ± P)). Never applied to Cantor divisor sums (grep of `research/` and
  `src/cryptanalysis/` for sibling / add-sub in hec/genus files: no hit).

## 1. Question

In the lab's genus 2 and genus 3 relation walk, the step D_{i+1} = D_i + M_j computes
`xgcd(u_D, u_M)`, `u_D·u_M` and `e1·u_D`. The same three objects serve D_i − M_j = D_i + (u_M, −v_M),
because −M has the same `u`. Does emitting the sibling D_i − M_j as a second relation candidate
lower the counted Cantor cost per candidate, with the sibling's smoothness (full splitting of u)
rate unchanged, and what does that do to the counted cost of the whole 2LP arm and to the distance
to matched negation rho?

## 2. Instrument

`sibwalk.py`: a line-by-line Python port of `JCurve::cantor` / `reduce` and `Fk`
(`mul` ticks a.n·b.n, `scale` ticks a.n, `divrem` ticks db per row plus 1 per row for a non-monic
divisor, `inv` counts one inversion, `xgcd`/`monic` as in Rust). The sibling routine
`cantor_pm(D, M)` returns (D + M, D − M) and ticks only the work it actually performs:
shared `xgcd(u1,u2)`, `u1·u2`, `e1·u1`; then for each sign its own products
`(e1u1)·v2`, `(e1u1)·v1` (computed once), its own `rem` and its own `reduce`. In the rare
gcd(u1,u2) ≠ 1 branch it falls back to two full Cantor calls (both charged).
Counted cost of an operation: mults + 10 × inversions (the wall-clock study's price), and also
mults and inversions separately.

## 3. Arms and controls

- **A (incumbent walk):** forward steps only; one candidate per step.
- **B (sibling walk):** the same forward walk (identical D_i, paired), plus the sibling D_i − M_j
  per step, skipped when j_i = j_{i−1} (then D_i − M_j = D_{i−1}, a known duplicate). Charged:
  the full `cantor_pm` cost minus nothing (the forward result comes out of the same call).
- **N (negative control on the counter):** the sibling computed by an independent full
  `cantor(D, −M)`; its cost per sibling must equal A's cost per step within 2 % (the two inputs
  have the same shape), so any saving in B is from sharing, not from a counting artefact.
- **A/A:** arm A rerun with a second walk seed on the same curves; difference of split rates and
  costs gives the noise floor.
- **Positive control (calibration):** arm A's mean mults and inversions per forward step on the
  lab's own published curves must match the lab's recorded `cantor_mul/group_ops` and
  `cantor_inv/group_ops` within 3 % (g = 3, p = 65537: 219.7 and 5.00; g = 2, p = 524309 and
  65537 from `genus_ladder_scale_20261115`). If it fails, the counts are reported but no
  projection is made.
- **Correctness:** every sibling must equal `cantor(D, negate(M))` exactly (all of them, arm N
  provides it), and for 200 sampled smooth siblings per cell the tracked coefficients must satisfy
  ca·G + cb·T = D (integer coefficients, T = k·G with known k) and the sum of the root places
  must equal D.

## 4. Cells (public synthetic curves already published by the lab)

- g = 3, p = 65537: the 6 curves of `genus_ladder_wallclock_vs_rho_20261122/raw/run_65537.jsonl`.
- g = 2, p = 65537 and p = 524309: the 3 curves per prime of
  `genus_ladder_scale_20261115/scale_g2{,b}.jsonl`.
- Per curve: one walk of N = 40,000 forward steps for g = 3 and 60,000 for g = 2 (Python speed),
  16 jumps α_j G + β_j T with short random coefficients, restarts ignored (they are the same in
  both arms per forward step). Walk seeds derived as sha256("rootfind_genus23|curve_seed|arm|s").
  A/A uses s = 2, all others s = 1.

## 5. Metrics

- c = (counted cost of `cantor_pm`) / (counted cost of `cantor`) − 1 averaged over steps where a
  sibling is emitted: the marginal price of a sibling as a fraction of a forward step.
- Walk counted cost per candidate: A: cost/step; B: (Σ cantor_pm cost) / (steps + siblings).
- Candidate rate (deg u = g) and full-split rate of u, forward vs sibling, difference with a
  95 % Wald interval pooled over curves; duplicate rate of siblings against all forward points
  and siblings of the same walk.
- Derived projection (arithmetic on lab rows, not a measurement): with the same relations needed
  and an unchanged per-candidate yield, the 2LP arm's Cantor cost scales by
  ρ_w = (1 + q·(1 + c)) / ((1 + q)·(1 + c_A)) where q = 15/16 is the sibling emission rate
  and costs are per forward step, i.e. ρ_w = cost per candidate B / cost per candidate A; `rel_mul`
  and `la_mul` unchanged. Applied to each lab row (cantor_mul + 10·cantor_inv) to give the
  projected arm total and the projected IC / matched-rho ratio in the same unit.

## 6. Predictions (with falsifiers)

- **P1 (sharing is real):** c ∈ [0.40, 0.80] at g = 2 and g = 3. Falsifier: c > 0.85 at both
  genera → the lever is not worth a port (verdict: null).
- **P2 (siblings are ordinary candidates):** sibling full-split rate equals forward full-split rate:
  the 95 % interval of the difference contains 0 and its half-width is < 15 % of the rate.
  Falsifier: interval excludes 0 with sibling lower.
- **P3 (no hidden duplicates):** sibling duplicate rate < 0.1 % after the j-repeat skip.
- **P4 (walk saving):** walk counted cost per candidate B/A ∈ [0.70, 0.90].
- **P5 (end to end, derived):** projected 2LP arm total ratio at g = 3, p = 65537 (with SGE) in
  [0.85, 0.95]; at g = 2, p = 524309 in [0.80, 0.93]. Falsifier for "worth porting": ≥ 0.97.
- **P6 (rho):** the lever does not apply to rho (a rho walk needs one deterministic successor);
  the IC/rho counted ratio falls by the same factor as P5, no more.
- **N control:** arm N's sibling cost / A's step cost ∈ [0.98, 1.02].

## 7. Stopping rule and decision

Fixed sample (section 4); one pass; no re-runs except to fix a crash, which is recorded in
SESSION_LOG.md. Verdict: **supported (component)** if P1, P2, P3, P4 and the N control hold;
**null** if P1 or P4 fails; **refuted** if P2 fails. P5 is a projection and is reported as such;
it is not an end-to-end measurement and cannot be promoted to a leaderboard row.

## 8. Constraints recorded before data

- No Rust build: `df -h /` showed 752 MB free (< 800 MB, the workflow's stop line), so the
  Rust pipeline is not rebuilt and no end-to-end Rust solve with siblings is run. The Python port
  measures counts, not wall time; no wall-time claim is made.
- No AWS/EKS/S3, no research-kb binary in this container (as in the survey), L0 only.
- No end-to-end DLP solve with siblings (LA not run); the end-to-end figure is a derived projection.
