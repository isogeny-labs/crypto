# One shared F4 system for the 8 cofactor lifts (`pkm-semaev:lifts=2`) on the k = 20 Curve25519 miniature

Protocol: [PROTOCOL.md](PROTOCOL.md) (frozen before data; amendments listed in it were also written before any protocol cell ran).
Scope of every statement: **k = 20, m = 2, mode full, the five cells of lead 2** (isogeny t = 4, 5, 6 with guess 0; bits t = 5, 6 with guess 3), the unit **F4 elimination multiply-adds**, trace off except where stated. Component study of one oracle: no rho arm, no distance to rho, no ranking, no leaderboard row.

## Result in one paragraph

The shared system gives the same verdicts as the 8 separate systems and uses 1 F4 call per target (isogeny cells) where lifts=1 uses 8, but it costs **more multiply-adds, not fewer: 30.7x to 628x those of lifts=1 per target** on matched targets in all five cells, because its solving degree rises from 5 (or 6) to 8 (10 at t = 6) and its matrices have 3.1x to 13x the rows, 5.8x to 15x the columns and 19x to 330x the non-zeros. This is a null result for the question asked ("cheaper than 8 separate systems"): the coordinator hypothesis (cost between lifts=0 and lifts=1, degree rise of at most 1) is refuted in all five cells, and my own forecast (cost above lifts=1, degree rise of at least 2) holds in direction in all five; its size was badly underestimated (see D3). No improvement is claimed.

## What was built

- `src/cryptanalysis/ic_framework/pkm_semaev.rs`: `lifts=2`. With a non-trivial fiber (`h > 1`) the third abscissa is a free ring variable `X` (the engine's free variable, as chain mode's `u`) and the system is `{S_3(x_1, x_2, X) = 0, P_R(X) = 0}` plus the summand membership, one `f4_tower` call per guess branch, where `P_R(X) = prod (X - x(R+K))` over the distinct lift abscissae (same lift list and the same charged group additions as lifts=1). After a non-refuted system the grid of `V^2` is read for each lift abscissa in lifts=1's order and the first sum equal to its lift target is returned (the same relation as lifts=1). `h = 1` is the blind oracle. `lifts=2` needs `engine=f4`, `m=2`, no staircase stop. `trace=1` learns on a made-up `P_R` (fiber-size random roots from the fixed stream).
- Counters (all in `native`, unpriced like the existing `pkm_system_fp_mults_uncharged`; none in the F4 multiply-add column): `pkm_shared_poly_fp_mults_uncharged` (P_R construction, 36 multiplications per target at 8 roots), `pkm_shared_lifts`, `pkm_shared_systems`, `pkm_grid_fp_mults_uncharged` (up to 8 grids per non-refuted target), `pkm_lift_hits_nonidentity`. The lift additions are group operations in `ops`, identical to lifts=1.
- `src/bin/curve25519_family.rs`: `f4cost ... <lifts 0|1|2> ...`; new output fields `lifts_mode`, `solving_degree_mean`, `solver_extra` (max rows/cols/nnz), `covered_projected_points`, `coverage_projected`. `lifts` (bool) is unchanged for the old values.
- Design reasoning and the rejected alternatives (an 8-isogeny tower for `X`; a resultant) are in PROTOCOL.md.

## Validity checks

- Equal base size: for every cell the five-field base signature (columns, covered points, all-coset points, projected points) is identical across all arms, asserted by `analysis/make_tables.sh` (`analysis/base_check.txt`: one distinct signature per cell).
- lifts=0 and lifts=1 reproduce lead 2's counts exactly (multiply-adds per target 522870.56, 4180475.38, 3957252, ...; hits 0/1/0/0/0/4 as in its table), which is also a check that the instrument was not disturbed.
- Unit test, `cargo test --release --lib pkm_semaev`: see "Tests" below.
- Binary identity and the frozen-copy rule: see "Measurement".

## Measurement

Frozen binary `/home/user/frozen-bin/curve25519_family_leadD`, sha256 `838c087cd6c7bba8e7a1eff06f4ae25323d64caca86f353e3992aedb0df7eca0`, built (after `touch` of the changed sources) and copied inside one `flock` call. Identity check: a 3-target smoke run with `lifts=2` prints `"lifts_mode":2` and the `pkm_shared_*` counters, which only this branch emits. Driver `run_cells.sh`; raw `analysis/f4cost_shared.jsonl` (31 KB, sha256 `ea9b1f643f65dbc6feef4c81257b30068e3ca6600ced854ce7a4c65f288629b9`, 23 lines) and one JSON, empty stderr and status line per run. **All 23 runs finished (none failed or was killed).** Single process at a time on the shared 4-core box; the 1-minute load average before every run is in the table (2.05 to 7.26) and in the jsonl. Wall times are not compared. Local runs, as instructed, not isolab-eks.

Cap and truncation: lifts=2 hit the 600 s per-cell cap in three cells: isogeny t = 5 (181 of 200 targets), isogeny t = 6 (8 of 200), bits t = 6 (56 of 200); these are **truncated samples**, and, as the protocol's amendment states, lifts=0 and lifts=1 were re-run on exactly the same first n targets (rows "prefix"), so the comparison in those cells is paired. The number of targets that fit in 600 s depends on the load, so it is not itself a result.

### All runs

lifts = 0 blind; 1 = eight separate systems (lead 2); 2 = one shared system; 2+trace = shared with the F4 trace. `cov/r` is the exact fraction of the order-r subgroup that an ideal oracle of that arm sees (lifts=0: points that are a sum of two base points; lifts 1 and 2: points R with R+K such a sum for some K in E[8]). The last-but-one column is the per-relation cost at that exact coverage (multiply-adds per target divided by cov/r). The per-hit column is on observed hits (undefined at 0 hits, never imputed). Exact coverages (covered / projected / all-coset points; r = 131203 in every cell): isogeny t = 4: 18 / 162 / 163; t = 5: 72 / 648 / 649; t = 6: 172 / 1250 / 1251; bits t = 5: 42 / 242 / 243; bits t = 6: 162 / 1250 / 1251.

