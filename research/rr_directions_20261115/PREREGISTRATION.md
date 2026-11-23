# Preregistration: SAT against F4 on the quadratic Riemann–Roch system over GF(2^m)

Frozen 2026-11-15T02:45Z, before any panel data (one smoke run at m = 7, norm arm,
2 draws, exists and is not part of the panel). Standing of everything here:
`[pcc:unverified]`, local L0 runs on a shared 4-vCPU container.

## Question

The owner's direction: "Over F₂ⁿ the RR system is quadratic, so run XL/MutantXL and SAT
with native XOR clauses (CryptoMiniSat) against F4/F5. Quadratic systems are where SAT
tends to win and high-degree summation polynomials are where it fails." The repository has
SAT on the incidence (quadratic) form (`research/wdsat_rr_20260920/`: CryptoMiniSat 7/8 at
n = 9, WDSat cannot load it) but no Gröbner-basis measurement of the incidence form at
all; the `ic_rr_norm_ladder` measured a Macaulay refutation degree of the eliminated
degree-4 norm form only. The open item recorded there is "Nagao's first-fall-degree claim
on the incidence/auxiliary-variable form".

## Instrument

- Systems: `harness/rrsys.py` (`incidence`: Boolean degree 2, unknowns 3ℓ + 8m with three
  distinctness witnesses; `norm`: Boolean degree 3, unknowns 3ℓ + 2m), curve
  y² + xy = x³ + 1 over GF(2^m) in the polynomial basis, factor base x ∈ V with
  dim V = ℓ = ⌊(m+2)/3⌋.
- F4 arm: `examples/bool_f4_file.rs` → `pq_f4_f2::groebner_basis_f4` (Boolean ring,
  Gebauer–Möller, field pairs), built with `--features wide-vars`, `RAYON_NUM_THREADS=1`,
  `--solutions`.
- SAT arm: `harness/sat_solve.py` → pycryptosat 5.18 (CryptoMiniSat), one Tseitin variable
  per nonlinear monomial, one native XOR clause per polynomial, nothing else.
- Both arms receive the identical polynomial file. Every solution or model is decoded and
  checked by independent group arithmetic (`rrsys.decodeSolution`); the ground truth per
  draw is the group-arithmetic count of ordered factor-base triples summing to R.

## Cells and draws

m ∈ {7, 9, 11, 13}, ℓ = 3, 3, 4, 5; 4 planted and 4 unplanted draws per cell; seed
20261115; the same draw feeds both arms and both solvers. Budget 600 s per solve,
3 concurrent solves. If every m = 13 incidence F4 solve times out, m = 15 is not run for
F4 (it exceeds 128 unknowns anyway) and the SAT arm alone extends to m = 15, 17.

## Unit and metrics

- F4: `solving_degree` (highest step degree that added a basis element), `degree_reached`,
  `matrix_rows_max`, `word_xors`, wall; decided = finished within budget and not
  oversize; `n_solutions` must equal the ground truth (ordered, distinct-abscissa for
  incidence; ordered, all, for norm).
- SAT: decided (sat/unsat within budget), conflicts (solver's own counter), wall; a sat
  model must verify.
- Wall-clock is a practicality note (AGENTS.md §6); the counted figures are the result.

## Predictions, written before the run

P1. F4 solving degree on the incidence form is 4 at m = 7 and does not exceed 5 at m ≤ 13
    (a bounded first-fall degree at these sizes, as the Nagao-style auxiliary-variable form
    is argued to have).
P2. CryptoMiniSat decides every instance at m ≤ 13 on both forms; F4 times out on at least
    one incidence instance at m = 13.
P3. Where both finish, SAT wall is below F4 wall on the incidence form at every m ≥ 9, and
    the gap widens with m (the SAT-favours-quadratic claim).
P4. On the norm form F4 solving degree is 4 at m = 7 (smoke) and at most 5 at m ≤ 13.

## Falsifiers

- F1 (against the bounded-degree reading): incidence solving degree ≥ 6 at m = 13, or
  rising by ≥ 1 per step of m across 7 → 13.
- F2 (against the SAT-wins claim): F4 wall below SAT wall at m = 13 on the incidence form on
  the median draw.
- F3 (instrument): any F4 solution count different from the ground truth, or any SAT model
  that fails verification, voids the cell.

## Inadmissible

Changing the encoding, witnesses, budget or seed between arms; counting a timeout as a
refutation; any statement about n = 83 or 131; any end-to-end or rho comparison (this is a
stage diagnostic, AGENTS.md §2/§5).

## Amendment 1 (2026-11-15T02:50Z, additive, before the Rust panel ran)

- **Instrument of record** is `examples/rr_quadratic_panel.rs` (Rust, AGENTS.md
  implementation-language rule), not the Python harness; the Python passes already started
  (`runs/panel_20261115/`, 1 GiB F4 matrix cap) stay as a cross-check on the same cell
  definitions and are labelled as such.
- **Factor base definition.** 0 ∈ V always, and both polynomial systems admit the 2-torsion
  point T = (0, 1) as a summand (the smoke run found 3 verified extra norm solutions with
  x_i = 0 at m = 7). The base is therefore *every affine point with x ∈ V, T included*, and
  the ground truth is `truth_ordered_*_incl_2torsion`. The Python count was changed to match
  for future runs; its committed records before this amendment used the x ≠ 0 count.
- **F4 matrix cap.** The smoke runs hit the default cap `MAX_MATRIX_WORDS = 2^27` (1 GiB) on
  the incidence arm at m = 7, degree 4. The Rust panel runs with
  `F4_F2_MAX_MATRIX_WORDS = 2^30` (8 GiB; the container has 15 GiB) through a new,
  default-preserving environment override in `pq_f4_f2.rs`; the 600 s budget stands. A
  result that is still undecided then reports which of the two limits stopped it.
- **F4 budget overrun.** F4 checks its deadline between matrices, so a step can overrun it
  (85 s against a 60 s budget in the smoke run); reported wall is the true wall.

## Amendment 2 (2026-11-15T07:10Z, additive, recorded during the m ≥ 11 cells)

- **Incidence F4 at m ≥ 11 runs under the default 1 GiB cap** (no `F4_F2_MAX_MATRIX_WORDS`).
  Reason: the 8 GiB-cap processes were killed twice by the container's memory cgroup
  (dmesg: `Memory cgroup out of memory: Killed process … rr_quadratic_pa`, 6.7 GB RSS),
  and all four m = 11 incidence draws already measured under the 8 GiB cap were undecided
  at the 600 s budget. Under either cap the outcome recorded is "undecided", with the
  limit that stopped it (`oversize` with the refused matrix, or `timed_out`). The cap in
  force is recorded per row by run directory (`rust_panel_20261115_incf4`: m = 11 draws
  0–3 at 8 GiB are in `rust_panel_20261115`; draws 4–7 and m = 13 at 1 GiB).
- **Norm F4 at m = 13** keeps the 8 GiB cap and runs alone, with no concurrent build.
