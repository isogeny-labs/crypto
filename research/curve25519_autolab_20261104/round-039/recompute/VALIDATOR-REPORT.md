# Validator report: Curve25519 autolab round 39 (k = 28, tw2l-x20-j16-n2 vs wwalkGg-x09-sr-n2)

Validated artifact set: repo /home/user/crypto, branch ic-autolab-20261117, commit d317b8728ef2243f9eff26fa25e55b9710bb36e2 (clean tree; `git status` empty). Evidence dir `research/curve25519_autolab_20261104/round-039/`. Claim PCC-20261117-15fb13 (standing: unverified; I did not and cannot change it).

Disposition: **validated** for the stated claim (counted GAE, k = 28, 1024 pairs, w = 0 / 0.1 / 1, band sel < inc), with the caveats in "Findings to carry forward". This is a numeric validation only; no wall-time claim is supported (see F1).

Scripts (mine, independent of the producer's): `/tmp/claude-0/-home-user/e375a883-d595-5f04-8ef9-a57bb8288ce5/scratchpad/v39/` : recompute.py (band rule + linearised-t), recompute2.py (bitop, wall, phases), cert.py (own EC arithmetic), acct.py (accounting identities), null.py (null calibration, permutation), train.py. Copy of the session for ecbench runs: .../v39/hold (records.jsonl sha256 equals the committed file). Validator replay receipt: .../v39/verify_validator.json.

## 1. Freeze, hashes, preregistration, exposures: PASS (one caveat)
- PROTOCOL.md at commit 7a1cd931e3 hashes to MANIFEST `protocol_sha256` 2c0511ae...; HEAD PROTOCOL.md differs by exactly the appended selection amendment (diff checked; the protocol permits it).
- MANIFEST `panel_sha256` f3f863b9... and `spec_sha256.train-k28` f8606752... match disk; train spec, panel and MANIFEST are byte-identical at HEAD and at the freeze commit.
- MANIFEST has no holdout spec hash (by design: spec "written at selection"). specs/holdout-k28.json sha256 e1bc85d9... equals the amendment, selection.json, run-holdout.json and RESULT.json.
- Commit order: PREDICTION-R39.md 8330552a17 (06:27:16Z) < freeze 7a1cd931e3 (07:20:13Z) < data (session start 15:11:15Z train, 15:11:34Z holdout; committed d11b9b367e 15:13:56Z). Prediction file unchanged since.
- Caveat: the holdout spec file itself was first committed after the holdout ran. Its content is the train spec plus exactly the frozen design deltas (diffed: arms inc-aa/rho-neg-aa added, 8 rounds, 128 targets, order alternate, target_seed 2026149928 = MANIFEST holdout seed, measurement seed 2026149935); no other field differs.
- Exposures: pre-freeze ledger (7a1cd931e3^) has 136 exposure seeds; 2026149928 and 2026149428 are in none, no [seed, seed+targets) range overlaps, and ecbench derives public targets by hashing (seed, index), so seed-disjointness is the right test. `git grep` finds the two seeds only in round-039 and the post-round ledger.

## 2. Holdout records: PASS
- 5760 records = 5 arms x 1152 (1024 measured + 128 warm-up): inc, sel, inc-aa, rho-neg, rho-neg-aa. All 5760 `verified`, `matches_target` true; session.json `status_counts {verified: 5760}`, records_sha256 3347420e... matches file.
- 128 workloads x 8 rounds (+ round 0 warm-up); every (workload, round) has all 5 arms; inc and sel share the algorithm_seed in every pair; seeds differ per round (so rounds are independent draws, not repeats).
- `binary_sha256` 581b3c6f1db5... in all 5760 records = run-holdout.json `ecbench_sha256` = sha256 of /home/user/crypto/target/release/ecbench. (Build provenance: see F2.)
- Arm labels: inc and inc-aa share method_sha256 3b0ad333...; rho-neg/rho-neg-aa share efea4e0e...; sel is 4fb5154a... with oracle `wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`. Order alternation balanced (512 / 512 measured). No duplicate record/run ids or seq.
- Certificates: for all 5760 records I recomputed recovered*G == Q with my own affine arithmetic (p = 268435157, order r = 33557809 verified prime, Q in the order-r subgroup, on the curve): 0 failures.
- Audit: `ecbench verify --replay 48 --replay-seed 3939393939` on my copy, on a different host/env class than the producer (auditor env ECBENV3h820fe8f25cdc, hw ECBHW1h0832b230440b; run was ECBENV3h761e6ce4dd1a): 48 replays identical, 0 problems. Producer receipts (replay 8 sha 0a3b6584...; replay 64 in recompute/) also consistent.

## 3. Recompute sel/inc: PASS (exact agreement)
Own implementation of the band.rs rule (priced S(w) = (G - C + w (U + n_C)) / sqrt r, C = 0 since no pinned calibration; U = cost.unpriced counts looked up in counters, then phase `native[name]` or `native[base]`; table work = `work_native.table_insert_attempts` only behind the `counted:` marker) and of compare.rs/stats.rs (workload clusters, z_k = (b - R a)/mean a, t(127) on the log scale; own Student-t via incomplete beta, checked against the NIST table):

| w | sel/inc (mine) | 95% (mine) | ecbench compare | RESULT.json |
|--:|--:|:--|:--|:--|
| 0 | 0.943876 | [0.918840, 0.969595] | 0.9439 [0.919, 0.970] | 0.943876 [0.91884, 0.96959] |
| 0.1 | 0.945370 | [0.920198, 0.971231] | 0.9454 [0.920, 0.971] | 0.94537 |
| 1 | 0.952272 | [0.926468, 0.978795] | 0.9523 [0.926, 0.979] | 0.95227 |

- `ecbench band --ranks` (default) on my copy: text and JSON identical to analysis/band-holdout-k28.{txt,json}; `sel < inc`; inc-aa/inc exactly 1.000000 at all w.
- Mean S (w = 0 / 0.1 / 1): inc 1.9030 / 2.0834 / 3.7069, sel 1.7962 / 1.9696 / 3.5300, rho-neg 1.0371 / 1.1344 / 2.0096; distance sel/rho 1.732 / 1.736 / 1.757 (RESULT.json agrees).
- Reading `--no-price-table-work` (pre-2026-11-08): sel/inc 0.9439 / 0.9253 / 0.7869, and sel < rho at w = 1 (0.8975); that reading omits the counted table inserts and would flatter sel and the IC arms against rho. The producer used the default (table work priced); I confirm it. Per L-546d37ca7750 I did not trust the producer script (paired_ratio.py reads cost.s, w = 0 only).
- Cross-checks: bitop/v1 `work.bitops_per_sqrt_r` sel/inc = 0.9453 [0.920, 0.971]; train means recomputed (rho 0.969, inc 1.873, sel 1.776 at w = 0) equal the amendment/selection.json; the train sel/inc is 0.948, so a challenger exists under the frozen rule (one candidate only, no selection freedom).
- Train->holdout independence: seeds differ; ledger confirms.

## 4. Oracle string, arm name, incumbent: PASS
- ledger.json at d11b9b367e^ (pre-ratchet) and at 7a1cd931e3 and 7a1cd931e3^: k = 28 incumbent `wwalkGg-x09-sr-n2`, oracle `wwalk:x=0.9,g=1,grow=2,seed=run`, since round 33; this equals the `inc` arm of specs/holdout-k28.json (factor base, oracle, targets stride-generator identical).
- Ledger post-ratchet arm `tw2l-x20-j16-n2` (family tw2l-x20-j16) oracle `wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run` = spec `sel` = selection.json = records. Evidence block carries the right session id, spec sha, receipt sha, binary sha.
- PCC claim file: canonical-JSON sha256 ea0596c9... equals the ledger entry (seq 1358); statement numbers match RESULT.json and my recompute. `pcc.py ledger verify` prints BROKEN, but all 151 reported lines are older claims lacking `claim_type`; none concerns 15fb13 (not caused by this round).

## 5. Sabotage / selection bias / accounting: no sabotage found; caveats below
- Accounting symmetry. Per-record identities (all 1152 sel, 1152 inc): total_gae = sum of phase GAE; for sel, charged adds/doubles = factor base + verify + tw_walk_adds + tw_setup_adds + tw_lazy_check_adds (and the matching doubles) with zero residual; `table_insert_attempts` = `tw_steps` exactly. For inc, relations adds = lookups plus a small positive offset (14 to 26 and up in the values I printed; 195 distinct offsets over 1152 records). So both arms do ~one native op per charged add. Mean native work: inc lookups 2221.7 + inserts 8195.6; sel lookups 0 (not in its unpriced list because its probe phase is gone) + inserts 10000.9. The only difference in `cost.unpriced` between inc and sel is `lookups_uncharged` (inc only), and it is priced at w > 0 and counted in the band; it is not hiding sel work. Lazy certification costs sel an extra 36 GAE (14 adds + 22 doubles) beside the standard verify phase, i.e. conservative toward inc. `tw_lazy_failures` = 0, `dropped_rows` = 0, no zero rows, in all 1152 sel runs.
- Null calibration: round-vs-round comparison of the same arm (different seeds): inc and sel each have 3 of 28 round pairs excluding 1 (non-independent pairs, nominal 5%): mild possible undercoverage of the workload-clustered interval. Conservative alternatives still exclude 1: round-clustered (K = 8) 0.944 [0.904, 0.986] at w = 0 and 0.952 [0.911, 0.996] at w = 1 (upper bound close to 1); workload-cluster sign-flip permutation p ~ 1e-5 (w = 0), 3e-4 (w = 1); both half-samples of workloads are below 1 (0.928, 0.960); per-round ratios range 0.869 to 1.024.
- The A/A control is degenerate (same seed, deterministic counts: exactly 1.0000) so it demonstrates reproducibility, not noise calibration; the producer says so.

## Findings to carry forward (none changes the numeric claim)
- F1 (disclosure gap): the same session shows sel slower in wall time: solve_wall sel/inc = 1.2356 [1.209, 1.262] (my recompute equals ecbench's "end-to-end wall 1.2356"), admitted ns/GAE 1.309, process wall 1.103. The counted-GAE gain (5.6 percent) does not transfer to wall time on this host/binary; round-039/README.md says "no wall-time claim" but does not report this (RESULT.json compare_verdict does). Any use of the win beyond counted GAE needs the wall result stated beside it. (Because every step is charged one GAE, the extra wall per GAE is work the GAE unit does not see; the band at w = 1 already shows the gain shrinking from 0.944 to 0.952.)
- F2 (reproducibility binding): session.json for the holdout has git_commit 7a1cd931e3 with git_dirty true; the binary (sha 581b3c6f...) has mtime 2026-11-14 08:11 and carries no build commit. `src/` differs substantially between the date before that build and 7a1cd931e3 (ic_boundary.rs +2285 lines, ic_framework +7201, band.rs +1016). I could not tie the binary to the frozen source and did not rebuild. Counts are self-consistent (replay on two host classes, independent certificates), so the numbers stand for this binary; "same source as the commit" is unproven.
- F3 (protocol text): PROTOCOL.md states every candidate is "a configuration never tried at its size in this loop". tw2l-x20-j16-n2 at k = 28 is in ledger trials for rounds 36 (train and holdout) and 37 (train and holdout, null 0.967 [0.927, 1.008]); PREDICTION-R39.md discloses it (fourth look, stop only on null). The fresh seeds make this round's test valid, and the effect is significant even after a 4-look Bonferroni; the repeat-until-pass pattern is nonetheless a selection-process risk that a ratchet on a single success inherits.
- F4 (holdout spec first committed with the results): see section 1 caveat.
- F5 (target-walk classification): sel's whole target walk is built in `oracle_setup` and shows up in `work.reusable_bitops` (436.7M of 439.1M) with `online` wall 41 us; per_target numbers for sel must not be used for amortised or online claims. The S claim (total GAE per run) is unaffected.
- F6 (process): owner waivers disclosed in the README (CHECKPOINT_SKIP, no benchrec record, local route). Not a measurement defect; the round remains development evidence with no VALIDATION.json, so no leaderboard or benchmarks/README.md row.

## Not done
- No rebuild of ecbench from the frozen commit; no run of any experiment launcher or `ecbench run`. No edit under research/, pcc/ or docs/ (so lessons.py outcome/test records were not written; applied: L-546d37ca7750 band recompute in both readings, L-99ee952350b2 real-key indexing in my scripts; proposed candidate lesson: a counted-GAE gain with a wall-time loss in the same compare output should be stated in the round README).
