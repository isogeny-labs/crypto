# T4 seed-selection replication across k (F4 engine check)

Run date 2026-11-03. The plan is `PROTOCOL.md`, frozen in commit
`2b330f250a1bca382b81023d95d556fccd7078d3`. It was executed unchanged as
`scripts/run.sh`. On disk, `PROTOCOL.md` hashes to sha256 `3adc1efa…d057` and
`scripts/run.sh` to `af58d7f4…5b25`. Git was not run (lead rule), so I did not
re-check these hashes against the freeze commit. The binary is
`/home/user/ctarget/release/curve25519_family`, sha256 `7a438b1d…98be`, the
protocol hash; `run.sh` checked it and all five census hashes before starting.

**Verdict (P5): replicates.** P1–P4 held in all 4 primary cells (k = 16, 18,
22, 24 at t = 6, G = 4) and in the secondary cell (k = 24, t = 5, G = 3). The
control (P0) reproduced the published k = 20 counts exactly. At fixed
per-target cost, the J-selected isogeny seed cuts the F4 mul-adds per relation
by 3.20x (k16), 2.43x (k18), 2.10x (k22) and 1.41x (k24 t6), plus 2.50x in the
secondary cell. This is a component (F4 phase) result only. Rho remains ahead
by many orders of magnitude. See "Distance remaining".

## Validity gate V

All 12 runs exited with rc = 0, each printed one JSON line, and every stderr
file is empty. Every run had `capped=false`, `targets_done=100`,
`undecided=0` and `hits_verified = hits`. No run was repeated. Wall time for
the whole script was 43 s (logged only, no timing claim). One unrelated
sibling process (`f4cost 20 bits@546312 …`, another lead) was running on the
machine at the time. This lead ran only one process at a time.

## Results (verbatim from `data/*.json`)

| cell | seed | columns | covered_points | coverage_exact | solving_degree_max | poly_terms_per_specialisation | f4_muladds | muladds_per_target | muladds_per_relation_at_exact_coverage | hits / verified | calls_per_target | max_cols_max / max_rows_max |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| k20 t6 G4 (C0) | 0 | 25 | 172 | 0.0013109456338650793 | 5 | 16.0 | 71009400 | 710094.0 | 541665483.0348837 | 0 / 0 | 16.0 | 66 / 105 |
| k20 t6 G4 (C0) | 1093 | 46 | 528 | 0.004024298224888151 | 5 | 16.0 | 70384751 | 703847.51 | 174899441.01236743 | 1 / 1 | 15.86 | 144 / 186 |
| k16 t6 G4 | 0 | 28 | 198 | 0.024119868437081252 | 5 | 16.0 | 69501215 | 695012.15 | 28814922.926010102 | 4 / 4 | 15.66 | 144 / 186 |
| k16 t6 G4 | 1381 | 40 | 614 | 0.07479595565842369 | 5 | 16.0 | 67383541 | 673835.41 | 9008981.890374592 | 8 / 8 | 15.18 | 144 / 186 |
| k18 t6 G4 | 0 | 30 | 230 | 0.006993644905281722 | 5 | 16.0 | 71009190 | 710091.9 | 101533879.63173914 | 0 / 0 | 16.0 | 66 / 105 |
| k18 t6 G4 | 1277 | 45 | 554 | 0.016845562076200322 | 5 | 16.0 | 70522595 | 705225.95 | 41864198.22680505 | 1 / 1 | 15.89 | 144 / 186 |
| k22 t6 G4 | 0 | 32 | 270 | 0.0005145493595766212 | 5 | 16.0 | 71009600 | 710096.0 | 1380034756.2074072 | 0 / 0 | 16.0 | 66 / 105 |
| k22 t6 G4 | 1070 | 46 | 568 | 0.0010824593934797067 | 5 | 16.0 | 71009497 | 710094.97 | 656001485.3927288 | 0 / 0 | 16.0 | 66 / 105 |
| k24 t6 G4 | 0 | 39 | 364 | 0.00017355242260587057 | 5 | 16.0 | 71009600 | 710096.0 | 4091536086.5494504 | 0 / 0 | 16.0 | 66 / 105 |
| k24 t6 G4 | 1490 | 41 | 514 | 0.0002450712780753227 | 5 | 16.0 | 71009600 | 710096.0 | 2897508045.7276263 | 0 / 0 | 16.0 | 66 / 105 |
| k24 t5 G3 (sec.) | 0 | 17 | 76 | 3.623622010452242e-05 | 5 | 16.0 | 17711200 | 177112.0 | 4887706264.31579 | 0 / 0 | 8.0 | 54 / 66 |
| k24 t5 G3 (sec.) | 797 | 24 | 190 | 9.059055026130606e-05 | 5 | 16.0 | 17711200 | 177112.0 | 1955082505.7263157 | 0 / 0 | 8.0 | 54 / 66 |

Derived ratios. In each cell, rho_T = MPT(S*)/MPT(0), rho_R = MPR(S*)/MPR(0),
and c = seed0_cov/sel_cov from the census.

| cell | rho_T | rho_R | c (pred.) | P4 band [0.9c, 1.1c] | reduction 1/rho_R |
|---|---|---|---|---|---|
| k20 (C0) | 0.9912 | 0.3229 | 0.3258 | [0.2932, 0.3583] | 3.10x |
| k16 | 0.9695 | 0.3126 | 0.3225 | [0.2902, 0.3547] | 3.20x |
| k18 | 0.9931 | 0.4123 | 0.4152 | [0.3736, 0.4567] | 2.43x |
| k22 | 1.000 (0.999999) | 0.4754 | 0.4754 | [0.4278, 0.5229] | 2.10x |
| k24 t6 | 1.000 | 0.7082 | 0.7082 | [0.6374, 0.7790] | 1.41x |
| k24 t5 (sec.) | 1.000 | 0.4000 | 0.4000 | [0.3600, 0.4400] | 2.50x |

## Predictions

| # | prediction (frozen rule) | outcome | evidence |
|---|---|---|---|
| P0 | Control C0 reproduces the published k20 fields exactly | **held** | seed 0: 71009400 / 172 / 5 / 16.0 / 0. Seed 1093: 70384751 / 528 / 5 / 16.0 / 1. A full-JSON diff against `curve25519_isogeny_seed_20261103/engine/*.json` differs only in `wall_s`. |
| P1 | Engine coverage equals census (exact integer; coverage_exact within 1e-12) | **held**, 4/4 primary + secondary | All 10 non-control runs match the census integers. \|Δ coverage_exact\| = 0.0 in every run. |
| P2 | Same solving_degree_max and poly_terms_per_specialisation within each cell | **held**, 4/4 + secondary | degree 5 and terms 16.0 on both arms in every cell. |
| P3 | 0.90 ≤ rho_T ≤ 1.10 | **held**, 4/4 + secondary | 0.9695, 0.9931, 1.000, 1.000; secondary 1.000 |
| P4 | 0.90c ≤ rho_R ≤ 1.10c | **held**, 4/4 + secondary | 0.3126, 0.4123, 0.4754, 0.7082; secondary 0.4000 (each inside its band, table above) |
| P5 | Replicates if P1–P4 hold in all 4 primary cells | **held: replicates** | 4/4 primary cells pass P1–P4. The secondary cell also passes, but it is scored separately and does not enter P5. |

## Exploratory (not preregistered)

- **Hits.** These are counts only, not scored: k16 4 vs 8, k18 0 vs 1, k20
  0 vs 1, and 0 vs 0 elsewhere. These are solver-reported (`hits_verified` = `hits`);
  no decomposition certificates exist in the artifacts, so they were not independently
  re-verified.
- **Why rho_T sits slightly below 1.** Rho_T < 1 occurs in the cells where hits occurred
  (k16, k18, k20). There `calls_per_target` drops below the full tower
  (16.0 at t6 G4, 8.0 at t5 G3), for example k16 15.66 vs 15.18. This looks
  like an early stop once a target is decomposed. The exception is k22, where
  rho_T = 0.99999855 < 1 with 0 hits in both arms and calls_per_target 16.0 on both, so
  the early-stop explanation does not cover k22. Among no-hit runs MPT is 710096.0 on both
  arms at k22 s0 and k24, but not in all of them: k18 s0 is 710091.9, k20 s0 is 710094.0 and
  k22 s1070 is 710094.97. The per-target cost depends on the seed only approximately: within
  no-hit cells the arms differ by up to 1.45e-6 relative (k22: 1.03 mul-adds per target). The
  larger column count (e.g. 40 vs 28 at k16) does not change the F4 system. A larger
  `max_cols_max`/`max_rows_max` (144/186 vs 66/105) appears only in runs with
  hits, consistent with a successful decomposition producing a bigger final
  system. This is a reading of the counts and was not tested.
- **The c computed from engine values** (instead of the census) equals rho_R/rho_T
  identically here, because P1 holds.

## Costs

- Counted F4 mul-adds, as above. The seed search is **not priced here**. The
  2000-seed census per (k, t) was paid in `curve25519_isogeny_seed_20261103`.
  At k = 20, t = 6 it cost about 1.35·10⁷ group operations, which is more than
  one rho solve. Any end-to-end figure must charge or amortise it explicitly.
- Native uncharged work (`pkm_system_fp_mults_uncharged`) is present in the
  JSON and is not priced in this component check. A §5a cross-method
  comparison would price it at 0, 0.1 and 1 GAE.

## Distance remaining

This is an improvement over the seed-0 isogeny incumbent inside the F4 phase.
It is not a solve, a speed claim or a leaderboard row. There is no same-unit
matched rho row for a component result, so the gap is given as context. The
F4 cost per relation with the selected seed is about 9.0·10⁶ (k16),
4.2·10⁷ (k18), 1.7·10⁸ (k20), 6.6·10⁸ (k22) and 2.9·10⁹ (k24 t6) mul-adds.
A whole rho solve needs on the order of √r group operations: about 91, 181,
362, 724 and 1448. At k = 20, rho-neg measured (completed T5 session ECBS1h405f20219036, ecbench/band.json in
`research/curve25519_isogeny_seed_20261103`, commit ea506e1b7f) S = 1.447 / 1.547 / 2.448 ·√r at
w = 0 / 0.1 / 1, with √r = 362.2, i.e. about 524 / 560 / 887 group operations (GAE), so
S = 1.45–2.45·√r across the §5a band. One relation alone costs about 10⁵–10⁶ times more counted
work than a complete rho solve, before the many relations and the linear
algebra that a solve needs. The gap also widens with k, because MPR grows
about 3.8–4.7x per 2 bits while √r grows 2x. A 1.4–3.2x constant cut does not
change the order across the §5a band. **Rho remains ahead.**

## Deviations

- `data/SHA256SUMS` was written by `run.sh` (the `*.json` and `*.stderr`
  files). I verified it with `sha256sum -c` and then appended one line for
  `data/run.log`. The protocol lists run.log as an output, but the frozen
  script does not hash it.
- The protocol mentions a later graph and PDF (research-visuals). This lead's
  task asked only for this README and the hash list, so no figure or PDF was
  made. That remains an open item.
- Conductor, S3 resume checkpoints and the EKS route were not used. The lead
  instructions fix a local, single-process run with no git.
- A sibling lead's f4cost process was running at the same time (see V). Every
  quantity is a deterministic count, and C0 reproduced exactly, so the
  sibling process had no effect on the counts.

## Files

- `data/<label>.json`, `data/<label>.stderr` (12 runs)
- `data/run.log`
- `data/SHA256SUMS`

## Validator errata (applied)

Source: `validator-report.json`. No prediction verdict changed.

- E1: k16 rho_R corrected from 0.3127 to 0.3126 (9008981.89 / 28814922.93 = 0.31265) in the
  derived-ratios table and the P4 row. The P4 verdict (band [0.2902, 0.3547]) and the 3.20x
  reduction are unchanged.
- E2: "Distance remaining" k = 20 rho-neg line replaced with the completed T5 session values
  (S = 1.447 / 1.547 / 2.448 at w = 0 / 0.1 / 1, sqrt(r) = 362.2, about 524 / 560 / 887 GAE,
  session ECBS1h405f20219036). The old "547-936" range (S = 1.510-2.584) matched no artifact.
  Conclusion unchanged: MPR(S*) at k20 divided by the rho-neg band is 2.0e5 to 3.3e5.
- E3: disclosed, not fixable here (text kept). The README uses sqrt(r) (a heuristic, not a
  measurement) as the rho reference at k16, k18, k22 and k24, and a non-frozen figure at k20
  (now the T5 values of E2), which the protocol did not commit to; this is a deviation from
  PROTOCOL.md's "matched rho reference for that k". The comparison of F4 field mul-adds per
  relation against rho group operations is unit-mixed (no measured mul-add-per-group-op
  factor). Recomputed MPR(S*)/sqrt(r) = 9.9e4, 2.3e5, 4.8e5, 9.1e5 and 2.0e6 for k16..k24;
  a same-unit gap would be smaller by the field multiplications per group operation. "Rho
  remains ahead" holds by orders of magnitude in any case. No band computation is shown here
  for the §5a claim; the T5 band supports it at k20 only.
- E4: Exploratory statements corrected (rho_T < 1 not only in hit cells: k22; "MPT 710096.0
  on both arms" not true for all no-hit runs; seed independence only approximate, up to
  1.45e-6 relative). The max_cols_max/max_rows_max statement was checked and is true.
- E5: Hits are cited as solver-reported `hits_verified` = `hits`, not independently
  verified relations (no certificates in the artifacts; unscored). Disclosed limitation:
  `coverage_exact` comes from the same binary in the census and the engine, so P1 is an
  internal consistency check between two code paths, not an independent recomputation.
- E6: disclosed, not fixable here. No graph or PDF (research-visuals) was produced, and
  README.md and data/ are uncommitted, so the result is bound only by the hashes in the
  validator report. Text kept ("Deviations" lists the graph/PDF as open).
- E7: "MPR grows about 4-5x per 2 bits" changed to "about 3.8-4.7x" (S* steps 4.65, 4.18,
  3.75, 4.42).
