# Protocol: building leads L1–L4 of `koblitz_ic_new_leads_20261018` (frozen 2026-10-24, before any measurement)

The leads are proposed in
[`research/koblitz_ic_new_leads_20261018/README.md`](../koblitz_ic_new_leads_20261018/README.md) §2.
This protocol fixes, before any run, what is measured, on which records,
and how each outcome is read. All four leads are built natively in Rust.

- **L2:** `src/cryptanalysis/partial_enlargement.rs` and `examples/pe_closure.rs`.
- **L1, L3, L4:** `src/cryptanalysis/koblitz_ic_leads.rs` and
  `examples/koblitz_ic_leads_bench.rs`.
- **Instrument commit:** the binaries are built from the commit that adds
  this file. `runs/binary.sha256` records their hashes.

## L2: partial-enlargement closure

**Engine.** `pe_closure(system, n_vars, D, mode, max_cols, max_rows)`.

- **What it computes.** The degree-D MutantXL closure. This is the space
  `ReducedBasis::from_system_closed_full` builds, explored in one of two
  orders.
- **`full`.** The dense engine's order: the whole degree-D Macaulay matrix,
  then every fall each round.
- **`partial`.** Seed at degree D − 1, then expand the low rows in batches
  of 64. Lowest leading degree goes first, then the smallest leading
  variable. The run stops at `1`.
- **Unit tests.** Both modes agree with the dense engine on verdict and
  saturated rank (`both_modes_agree_with_the_dense_closure`), and never
  refute a planted solution.

**G1. Correctness gate (degree 4).**

- **Draws.** The 16 unsatisfiable draws whose dense closure refuted at
  degree 4 in `research/dreg_closure_driver_20261016/runs/`:
  - m = 3 (22, 6): draws 0–3;
  - m = 3 (24, 6): draws 0–3;
  - m = 4 (11, 5): draws 8, 67, 107, 143;
  - m = 4 (12, 5): draws 24, 26, 47, 52.
- **Run.** `partial` mode on every draw.
- **Pass.** Every draw refutes at degree 4. One draw that saturates without
  refuting is an instrument failure, and stops the study.
- **Soundness control.** The satisfiable draws m = 4 (11, 5) draws 0 and 1
  must saturate without refuting.

**P1. Primary metric (degree 4).**

- **Metric.** `peak_matrix_rows`: the most rows one elimination held. Its
  bytes are `peak_matrix_rows · ⌈columns/64⌉ · 8`.
- **Draws.** All 16, in `partial` mode.
- **Reference.** `full` mode on the four m = 3 (22, 6) draws and the four
  m = 4 (11, 5) draws, through the same engine, as the A/A reference for
  the peak.
- **Falsifier** (from the lead). Peak rows at refutation are at least
  0.5 × columns on 3 or more of a cell's 4 draws. Then the lead is dead for
  that cell.
- **Payoff.** Peak rows below 0.25 × columns.
- **Wall time.** Recorded, descriptive only. The host is a shared 4-vCPU,
  15 GB container at isolation L0.

**C1. The decisive cell (degree 5).**

- **Draws.** m = 3 (19, 7), the unsatisfiable draws 0, 3, 5, 7 of
  `research/dreg_closure_scale_20261016/draws/m3-19-7.jsonl`. Draw 0 is the
  one the dense closure could not finish at 169 GB
  (`research/dreg_closure_m4l6_20261018/RESULTS.md`, C1).
- **Run.** `partial` mode at degree 5, batch 64, row cap 100,000. That is
  about 9.5 GB at 760,099 columns, inside the container's memory. The time
  limit is 4 hours per draw.
- **Outcomes, per draw:**
  - **refuted at 5:** the closure degree of that draw is 5;
  - **saturated without refuting:** it is at least 6;
  - **capped or timed out:** unmeasured. This is never evidence about the
    degree.
- **Soundness control.** Satisfiable draw 1 at degree 5, under the same
  cap. Any refutation is an instrument failure. A cap is expected, because
  saturating the full closure exceeds the cap.
- **Reading for the ECC2K-130 rows.** Refutation at 5 on the measured draws
  supports the degree-5 rows of `RESEARCH_ECC2K130_DESCENT_DEGREE.md` at
  ℓ = 7. Saturation without refutation supports a rise at ℓ = 7. Either way
  it is one cell (m = 3, ℓ = 7), not m = 4.

## L1: λ-closure

- **Run.** `koblitz_ic_leads_bench lambda`: both Koblitz curves over
  F_{2^11}, F_{2^13}, F_{2^17} and F_{2^19}, with 12 random subspaces each
  (seed 20261024).
- **Pass.** All three identities hold on every curve and subspace.
- **Classification.** A closure, not an improvement.

## L3: Agreeing–Gluing on the S₃ chain

- **Run.** `koblitz_ic_leads_bench gluing`: m = 3 at ℓ = 3–10 with n = 3ℓ,
  and m = 4 at ℓ = 3–7 with n = 4ℓ. Four draws per cell, seed 20261024.
- **Correctness.** The two solvers' solution counts must agree on every
  draw. The unit test checks both against `chained_solution_count`.
- **Metrics.**
  - root solves and probes for each solver;
  - the glued table size;
  - the fraction of partial solutions the agreeing step discards.
- **Reading.** If Agreeing–Gluing's root solves fit 2^{2ℓ} at both m = 3 and
  m = 4, it is the pair-table meet-in-the-middle already measured
  (`binary_mitm_oracle_*`). That outcome is classed accounting. A fit below
  2^{2ℓ} at m = 3 would be new.

## L4: solving-degree bounds

- **Run.** `koblitz_ic_leads_bench bounds`: the Caminata–Gorla Macaulay
  bound (Steiner, ToSC 2024(1), Corollary 2.9 and Theorem 3.2) for every
  measured cell, read off a built system, plus the ECC2K-130 shape.
- **Falsifier** (from the lead). The bound at a measured cell is at least
  ℓ + 4. Then L4 closes as uninformative.

## What none of this does

- No group-operation count, leaderboard row or distance to rho is measured
  or changed. L2 is an instrument; L1 and L4 are closures or bounds; L3 is
  an equivalence check.
- No result here is a speedup claim. `benchmarks/` receives no row.
