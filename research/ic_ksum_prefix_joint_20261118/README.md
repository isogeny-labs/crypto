# Prefix inserts (L1) and a joint set-up (L2) for the k-sum `-psr` arm, Table B `[pcc:unverified]`

**Standing: counted cost at L0, `[pcc:unverified]`, no pcc claim, no validator report or `VALIDATION.json`; no leaderboard or
`benchmarks/README.md` row (artifact store down).** Track: generic decomposition (bounded by sqrt(r); cannot beat rho or BSGS).

## Outcome

- **Incumbent.** The `-psr` arm of `research/ic_ksum_split_20261118` (VAL-20261118-1e81cb): `ksum:m=k,prune=1,split=1`,
  `targets=stride-renew`, producer (k, N) (the retune was a null).
- **L1, `prefix=1` (new):** the build also keys the partial sums of fewer than k summands it forms anyway (base points, lower
  levels, split's pair sums), only under keys no k-sum took. No added group operation; one counted insert attempt each; prefix
  entries 10-13 % of T on the pilot. **Holds on 5 of 8 rows** (P20, B15, B18, B21, B24; registered all-weights, per-replicate
  rule): `prefix / inc` = **0.931-0.959 at w = 0**, 0.931-0.961 at w = 0.1, 0.931-0.969 at w = 1. On P22 (0.967), P24 (0.970) and
  B27 (0.980) the pooled interval is below 1 at every weight but one replicate's upper end reaches 1.0002 / 1.0002 / 1.0077 at
  w = 1, so the registered rule fails there; no row is significantly worse.
- **L2, `targets=stride-renew-joint` (new):** the three set-up `[a]G + [b]Q` (stride, renewal step, start) each by one joint
  double-and-add over `G + Q` (Shamir's trick, the existing `joint_mul`), same draws and points. **Holds on all 8 rows**:
  `joint / inc` = **0.938 (B15), 0.951 (B18), 0.987-0.997 elsewhere at w = 0**; 0.973-0.999 at w = 1; intervals +/- 0.0001-0.0006.
  Correctness: 12,288 joint/inc pairs (and 12,288 prefix-joint/prefix) identical in outcome, log, trials, relations, rank,
  every relation-phase counter and the oracle set-up; 22-102 GAE saved a run, scalar multiplications 6 -> 3 (8 -> 4 after a stride restart).
- **Both (`prefix-joint / inc`, secondary):** 0.887 (B18), 0.897 (B15), 0.922 (B24), 0.937-0.941 (B21, P20), 0.959-0.977 (P22, P24,
  B27) at w = 0; the levers are additive (L1 given L2 and L2 given L1 equal the marginal ratios to within 0.003).
- **Distance remaining to matched rho-neg** (this study's targets; registered best per row = prefix-joint on the five L1 rows,
  joint on P22, P24, B27): w = 0 **2.76-5.01 -> 2.48-4.99** (B15 2.76 -> 2.48, B18 2.86 -> 2.54, P20 3.88 -> 3.65, B21 3.87 ->
  3.63, B24 4.46 -> 4.11, P22 4.36 -> 4.33, P24 4.74 -> 4.72, B27 5.01 -> 4.99); w = 0.1 2.90-5.24 -> 2.63-5.23; w = 1 3.62-6.33 ->
  3.41-6.33. Rho is ahead on every row at every weight.
- **Controls.** A/A `rho-neg-aa / rho-neg` = 1.0000 everywhere; 24/24 sessions `verify --replay-all --exit-code` OK (73,728
  records, 55,296 replays reproduced, 0 problems); pilot 8/8 OK. Holm across rows on the pooled intervals passes for every row of
  both levers (smallest z 4.6, B27 L1); the per-replicate rule is the binding one.
- **Not an accounting change.** L1 adds entries the build already paid for and charges each insert; L2 removes doublings and
  additions the set-up actually performed, every remaining one counted. Both are constant factors (L2's saving is a fixed
  ~3.75 log2 r GAE against sqrt(r); L1 a ~10 % larger table at the same cost); no exponent changes.
- **Predictions (registered):** L2 all 8 rows within +/- 0.01 of the pilot: met (B15 0.938 vs 0.94, B18 0.951 vs 0.95, others
  within 0.001). L1 0.91-0.97 holding on most of P20, P22, B15, B18, B21, B24 and no detectable change on P24/B27: P22 failed
  (one replicate), P24 and B27 measured 0.970 / 0.980 with pooled intervals below 1.

## Deviations

Local L0 instead of isolab-eks; no S3 checkpoint or raw upload (no AWS); `.resume/RESUME.md` not rewritten (another lane's);
Conductor and research-kb unavailable. Code and study developed on a side branch (`ksum-prefix-wip`, scratchpad worktree)
while another agent merged `main` into the lane branch, then cherry-picked; the binary was built from that branch's commit
ea46319d35 (lane HEAD 276fe1a6ae + the code commit), in its own target directory, `CARGO_BUILD_JOBS=1`. Confirmatory
targets_per_curve set to 128 after the pilot (drafted 96), before the registration. `results.json` (132 KB) is committed gzipped.
Raw sessions outside Git: `kp-confirm-sessions.tar.gz` (sha256 `5cb867b8…10dd`) in the producer's scratchpad; per-file digests
in `data/sessions.sha256`; pilot raw deleted after summarising (`pilot/sessions.sha256`).

## Files

`PROTOCOL.md` (registered bc3122e8dc), `REGISTRATION.sha256`, `BINARY.sha256`, `make_specs.py`, `specs/`, `pilot/` (specs,
`analyse.py`, `pilot_table.md`, `pilot_summary.json`, `correctness.json`), `run_pilot.sh`, `run_all.sh`, `analysis/` (evaluate,
correctness, verify_summary, summary, plot, report), `results.json.gz`, `results.md`, `summary.json`, `data/` (run log, verify
summaries, correctness, digests), `ratios.svg`, `gap.svg`, `REPORT.pdf`.

Full table: `results.md`.
