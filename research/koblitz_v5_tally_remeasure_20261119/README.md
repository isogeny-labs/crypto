# v5 with its jump-index tally against strong on the small Koblitz curves (2026-11-19) `[pcc:unverified]`

Reference re-measurement for the owner decision of 2026-11-19
([`../koblitz_rho_reference_20261115/DECISION.md`](../koblitz_rho_reference_20261115/DECISION.md)): for Koblitz rows below
r = 2^38 the matched rho reference becomes min(`rho.signed_frobenius_strong`, `rho.signed_frobenius_v5`), on condition that v5
first tallies its per-step jump-index selection. Frozen in [`PROTOCOL.md`](PROTOCOL.md) before any data (registration commit
`9c525b01f5`, `REGISTRATION.sha256`, `registered-at.txt`). Every number is `[pcc:unverified]`: no validator has signed it.

## Source change (commit `61e4022114`)

`rho.signed_frobenius_v5` now reports `partition_hashes_uncharged` = one per walk step (the name and definition strong uses:
one partition hash per step), listed in `cost.unpriced`, so the §5a band prices it at w per step. Nothing else changes: the
unit test `v5_jump_index_tally_adds_one_counter_equal_to_steps_and_changes_nothing_else` checks, on four Koblitz curves x 6
targets, that the tallied report equals the pre-tally report (recovered scalar, phases, adds/doubles/scalar mults, GAE bits,
natives, work_native, field ops, every other counter) plus that counter, and that it equals `steps`. The look-ahead's rejected
candidates are not counted as selections: each is charged a full group addition (`look_ahead_additions`). Other tuned-walk
methods' reports are unchanged. A pre-tally v5 record is replayed without the counter (`v5_pre_tally_record` in
`ecbench::audit::compare_replay`); checked on an old session (`koblitz_k1_setup_w1_20261118` n29-r1, binary `a6a38904…`):
`audit OK: 1536 records, 1536 verified, 1152 replays reproduced, 0 problems`. 49 related library tests pass (release).

Binary `ecbench` sha256 `92ae9054d2c791c796fc9e2f1b01c53d2bc1584f6661c34e59957576003044cb`, built from a scratch worktree
equal to `61e4022114` (CARGO_BUILD_JOBS=1, own target dir, under the CPU lock; target dir deleted afterwards).

## Result

R1 soundness: all 8 sessions `verify --replay-all` OK (768 records, 576 replays each, 0 problems); every measured solve
verified; every v5 record (384 per curve, warm-ups included) carries the tally equal to `steps`; both A/A ratios are 1.000 on
every curve and weight.

R2, v5 / strong (paired by target, 95% = wider of compare's two intervals; 48 targets x 3 rounds):

| curve | slug | log2 r | w = 0 | w = 0.1 | w = 1 |
|:--|:--|--:|:--|:--|:--|
| a0-n13 | `icv1-f2m13-t181-515ee569` | 10.97 | 0.062 [0.051, 0.082] | 0.067 [0.055, 0.090] | 0.104 [0.076, 0.149] |
| a1-n29 | `icv1-f2m29-tm40309-30c52b96` | 15.37 | 0.077 [0.072, 0.082] | 0.085 [0.079, 0.091] | 0.143 [0.130, 0.157] |
| a1-n17 | `icv1-f2m17-tm101-00378d4e` | 16.00 | 0.097 [0.090, 0.103] | 0.108 [0.100, 0.115] | 0.181 [0.165, 0.197] |
| a0-n19 | `icv1-f2m19-t797-b6cf2467` | 17.00 | 0.110 [0.101, 0.119] | 0.122 [0.112, 0.133] | 0.205 [0.184, 0.226] |
| a0-n31 | `icv1-f2m31-tm90707-c95f16f5` | 20.46 | 0.174 [0.160, 0.191] | 0.194 [0.177, 0.213] | 0.312 [0.279, 0.347] |
| a0-n23 | `icv1-f2m23-t5197-69e76b73` | 21.00 | 0.198 [0.181, 0.216] | 0.219 [0.200, 0.239] | 0.334 [0.299, 0.368] |
| a1-n23 | `icv1-f2m23-tm5197-1f85e9e1` | 22.00 | 0.251 [0.227, 0.276] | 0.277 [0.250, 0.306] | 0.413 [0.369, 0.462] |
| a0-n37 | `icv1-f2m37-tm534059-32aad96b` | 27.78 | 0.574 [0.507, 0.644] | 0.606 [0.534, 0.681] | 0.729 [0.638, 0.827] |

v5 is below strong on every curve at every weight (upper end < 1). Against the untallied 2026-11-15 values (different seeds), v5/strong at w = 1
rises by about 0.02 (n29) to 0.23 (n37) (n37 0.504 -> 0.729, n23 a=1 0.310 -> 0.413, n29 0.119 -> 0.143).

R3 crossover: not reached at any weight up to log2 r 27.78. (Context, not this protocol: the 2026-11-15 study measured
v5 / strong 0.94 [0.84, 1.05] at log2 r 39.0 and 0.96 at 44.3 at w = 0, and its derived tally-charged point at w = 1 is 0.93
at 39.0: parity sits near 2^39, just above the decision's 2^38 boundary.)

R4 consistency at w = 0 with 2026-11-15 (fresh seeds): n29 0.077 vs 0.079, n17 0.097 vs 0.093, n31 0.174 vs 0.168, n23 a=1
0.251 vs 0.261, n37 0.574 vs 0.581: every pair's intervals overlap.

Figure: [`figures/fig1-v5-over-strong.svg`](figures/fig1-v5-over-strong.svg); report [`REPORT.pdf`](REPORT.pdf); data
`summary.json`, `results.json.gz`, `tables/`, `audits/`; raw sessions in the benchmark record
`benchmarks/records/20261119T1819Z-koblitz-v5-tally-remeasure.json` (S3 by SHA-256; not uploaded: store refused access).

## Use and non-claims

These ratios are context for leaderboard rows whose evidence has no in-session v5; they are never applied to such a row as a
multiplier. The change of reference is an accounting change, not an IC improvement; it lengthens the reported distance to rho
on every affected row. L0 shared VM; counted GAE are deterministic; wall time is not used.
