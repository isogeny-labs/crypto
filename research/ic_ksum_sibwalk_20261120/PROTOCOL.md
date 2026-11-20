---
title: "Protocol: a sibling trial walk (walk_sib=m) for the k-sum Table B arm"
date: 2026-11-20
status: registered before any confirmatory run (hashes in REGISTRATION.sha256)
---

# Protocol: sibling trial walk for the k-sum arm on Table B `[pcc:unverified]`

**Track: generic decomposition (bounded by sqrt(r); cannot beat rho or BSGS).** A constant-factor lever on the trial walk;
no exponent changes.

## 1. Incumbent, profile, lever

**Incumbent (best per row).** `research/ic_ksum_prefix_joint_20261118` (VAL-20261118-facd3e), registered best:
`ic.pipeline`, `oracle = ksum:m=k,prune=1,split=1[,prefix=1]`, `targets = stride-renew-joint`, the producer's (k, N);
`prefix=1` on P20, B15, B18, B21, B24 (where it held), not on P22, P24, B27.

**Profile (prior data).** Set-up 28-57 % of S at w = 0, relation trials 43-67 %
(`research/ic_ksum_retune_20261118`); a trial is one stride addition plus two calibration-priced native operations (the
guard probe, priced as a lookup, and the table lookup).

**Lever (`walk_sib=m`, new, commit 39d3d20d72; `TargetSource::StrideRenewSib`).** The stride-renew walk presents
`R_{j+1} = R_j + S`, renewing `S <- S + T` after each relation. With `walk_sib=m` the same progression is formed in blocks
of `2m + 1`: from the anchor `A` (the start, or the target a relation came from) the block is
`C - [m]S, ..., C - [1]S, C, C + [1]S, ..., C + [m]S` with `C = A + [m+1]S` (one addition). Walking the minus side, pair
`i` is formed from ONE inversion (`x_sibling_pair_counted`): `C + [i]S` in full (kept for the plus side) and the abscissa
of `C - [i]S`, charged **one group addition plus one `sibling_second_points`** (the convention of main's `sib`, the
Koblitz `walk_sib` and the layout pair table; priced at `w` by the band). The abscissa is probed by one counted lookup of
the table key `x + 1` (`KSumTable::probe_x`); a key hit is always a decomposition (`+-R` is stored), and then the point is
formed by one more counted addition and goes through the guard and the oracle like any target. The multiples
`[2]S..[m+1]S` cost `m` counted operations each time a drawn or renewed stride starts a block. A relation drops the rest
of its block (plus points already paid for are wasted, counted as `walk_sibling_unused_plus`).

**Soundness.** Every presented point is `[a]G + [b]Q` with known coefficients (tracked mod r); hits are decomposed and
verified by the incumbent's `probe` (k - 1 confirming additions) and enter the same elimination; the answer is checked
`[d]G = Q`. The progression, the trials, the relations, the rows and the answer are **those of the incumbent, run for
run**: an abscissa that does not decompose is not guarded (no full point exists), but the oracle is deterministic, so a
decomposing repeat's earlier copy also decomposed, was formed in full and guarded, and is skipped exactly as before.

**Unit test** (`cargo test --release --locked --lib`): `walk_sib_on_stride_renew_keeps_the_targets_and_counts_the_pairs`
(prime k = 4, 5 and binary k = 4; stride-renew and stride-renew-joint; prefix 0/1; m = 1, 3, 8): absent and
`walk_sib=0` identical to the incumbent; on, same targets, relations, rank, answer, scalar multiplications, oracle
set-up and stride counters, and the relation phase's additions plus doublings equal exactly
`inc - stride_steps + pairs + 2 full_pairs + centres + lifts + multiple_ops`. 26/26 ksum, stride and Koblitz-sibling
tests pass.

## 2. Count model and pilot (not data)

**Model** (`analysis/model.py`, fed only by the incumbent arm's records): segments between renewals/restarts geometric
with the record's mean length `L/(renewals + restarts)`; per segment of length `s` (relation at `s`), centres
`ceil(s/(2m+1))`, pairs `b m + min(q+1, m)` (`b` full blocks, `q` the last slot), a lift when `q < m`, and `m` multiple
operations per renewed stride. `dG = -L + centres + pairs + lifts + multiples`; native work
`dN = +pairs (sibling points) - (pairs - lifts) (guard probes not made) + lifts (second lookup) = 2 lifts`. Predicted ratio
`(P_inc(w) + dG + w dN) / P_inc(w)`, `P_inc(w)` the incumbent's band price per run (`ecbench band`).

**Pilot** (8 sessions, fresh seeds `target_seed = 20261120500 + i`, `seed = 20261120550 + i`, 64 targets, 1 round; arms
rho-neg, inc, off, sib4, sib8, sib16; record `benchmarks/records/20261120T0342Z-ksum-sibwalk-pilot.json`; 8/8
`verify --replay-all --exit-code` rc 0). `off / inc` = 1.000000 on every row and weight. Measured `sib_m / inc` against the
model (`pilot/pilot_table.txt`): agreement within 0.001 on the six larger rows and within 0.009 on B15/B18 at every m and
weight; e.g. sib8 at w = 0/0.1/1: P20 0.798/0.825/0.921, P24 0.761/0.792/0.905, B24 0.708/0.750/0.891, B27
0.781/0.809/0.911; B15 sib16 1.10 (too large an m for B15's short segments, as the model predicts).

**Registered m (rule fixed before the confirmatory data):** `m = argmin` over {1, 2, 3, 4, 6, 8, 10, 12, 16, 20, 24, 32} of
the model's ratio at w = 0.1 on the pilot's incumbent records (`M.json`): P20 6, P22 8, P24 12, B15 2, B18 3, B21 6,
B24 10, B27 12.

## 3. Confirmatory design

**Rows.** All eight Table B rows, the incumbent's (k, N) and options per row.

**Sessions.** 3 replicates per row, 24 sessions (`specs/<row>-r<rep>.json`, `make_specs.py confirm M.json`). Curves,
max_trials, timeout, L0 and order `alternate` from the prefix-joint study's spec of the same name; **targets_per_curve 48**,
3 measured rounds, 1 warm-up (432 pairs a row, 144 a replicate). **Fresh seeds:** `target_seed = 20261120300 + idx`,
`seed = 20261120400 + idx`, `idx = 10 * (row index) + rep` (not used by any earlier study or this pilot). Arms: `rho-neg`
(reference), `rho-neg-aa` (control), `inc` (incumbent best), `off` (`inc` + `walk_sib=0`, default-off control), `cand`
(`inc` + `walk_sib=m_row`); on P22, P24, B27 also `inc-prefix` (`inc` + `prefix=1`, test T3). Same targets and algorithm
seed per round.

**Binary.** `ecbench`, `cargo build --release --locked --bin ecbench` in a clean scratch worktree of 39d3d20d72, own
`CARGO_TARGET_DIR`, `CARGO_BUILD_JOBS=1`, under the scratchpad CPU `flock`; sha256 in `BINARY.sha256`
(`ff3172218ab6…f844`), checked by `run_all.sh` with this registration's hashes; `strings` shows `stride_renew_joint_sib`,
`walk_sibling_multiple_ops` and the `walk_sib` parameter text.

**Route.** Local, `--cpus none`, L0, counted cost only, one session at a time under the CPU `flock`, inside
`tools/benchrec.py run`; `ecbench verify --replay-all --exit-code` on every session.

## 4. Registered tests

**T1 (the lever), per row.** `R_w = cost(cand) / cost(inc)`, paired by target and round:
`ecbench compare --dir <the 3 sessions> --a inc --b cand --weight w --resamples 4000 --seed 20261120` (ops ratio of
totals), and the same per replicate. Upper end = the larger of the linearised-t and bootstrap 95 % upper ends. A row
**holds at w** if the upper end is below 1 at that w, pooled and in each replicate, and `1 - upper > max_w |R_aa(w) - 1|`
(`R_aa = rho-neg-aa / rho-neg`). A row **holds** if it holds at w = 0, 0.1 and 1. **Significantly worse:** pooled lower
end above 1 at any weight. **Verdict:** an improvement where at least one row holds and none is significantly worse; rows
that hold only at w <= 0.1 are reported as such, not as holding.

**T3 (secondary, fresh test of prefix=1 on P22, P24, B27).** Same rule with (inc, inc-prefix). Power is low (144 pairs a
replicate against effects of 2-3 %), so a failure is not evidence against prefix=1.

**Correctness (registered, `analysis/correctness.py`).** For every (session, round, target): `off` equals `inc` in every
record field but identifiers and method parameters; `cand` equals `inc` in outcome, recovered log, targets tried,
relations, rank, oracle set-up, scalar multiplications and the stride counters, and its relation-phase adds + doubles
equal the count identity exactly. Any mismatch invalidates T1 on that row.

**Secondary (reported, not gated).** Distance remaining `S(arm)/S(rho-neg)` per weight for inc and cand; model prediction
against measurement.

**Multiplicity.** Eight T1 row tests; Holm across rows reported beside the verdict when they differ.

**Predictions (fixed now, `M.json`).** T1 holds on all 8 rows. Predicted `cand / inc` at w = 0 / 0.1 / 1:
P20 0.798/0.825/0.920, P22 0.782/0.811/0.913, P24 0.760/0.791/0.905, B15 0.920/0.931/0.971, B18 0.853/0.873/0.944,
B21 0.795/0.822/0.919, B24 0.709/0.751/0.891, B27 0.779/0.808/0.911 (each within +-0.02). T3: unlikely to hold
per replicate on any of the three rows.

*Sources checked* (research-kb MCP and the lead atlas are unavailable in this container):
`research/ic_ksum_prefix_joint_20261118/` (README, PROTOCOL, results.md, specs), `research/ic_ksum_retune_20261118/README.md`,
`research/koblitz_k1_sibwalk_20261118/` (commit 217638524d, `TargetSource::StrideGeneratorSib`),
`src/cryptanalysis/ic_boundary.rs` (`collect_and_solve_relations_scaled_touching`, `KSumTable`, `x_sibling_pair`),
`src/cryptanalysis/ecbench/band.rs` (`SIBLING_WORK_COUNTERS`), `docs/ic/calibration.json` (`ns_per_lookup`).
