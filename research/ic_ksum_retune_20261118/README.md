# Retuning the k-sum (k, N) for the post-split cost curve, Table B: a null `[pcc:unverified]`

**Standing: counted cost at L0, `[pcc:unverified]`, no pcc claim, no validator report; no leaderboard or `benchmarks/README.md`
row.** Track: generic decomposition (bounded by sqrt(r); cannot beat rho or BSGS).

## Outcome

- **Lever.** Re-size the incumbent `-psr` arm (`ksum:m=k,prune=1,split=1`, `targets=stride-renew`) of
  `research/ic_ksum_split_20261118` (VAL-20261118-1e81cb) after split and stride-renew made the set-up cheaper. The rule
  (`rule.py`) is computed from r and the field family alone: minimise `build(N, k) + (N + 0.6) r / (2T) + w (T + 2 trials)`,
  `T = 2^(k-1) C(N, k)`, over k = 3..7, at the band middle w = 0.1 (R1).
- **Step 1, profile of the `-psr` arms (prior data).** T is exact, a trial hits with probability 2T/r (fitted 1.93-2.04 x T/r on
  16 arm-rows), relations = N + 0.6, set-up adds reproduced exactly; the set-up is 28-57 % of S and the trials 43-67 %. The
  producer's N, inherited from the prune study, already sits at or next to the model's optimum on every row: because T moves
  by 1.5-2.5x per step of N, split's lower cost per entry does not move the optimum by a whole step.
- **Pilot** (fresh seeds, 92 (k, N) arms, 64 targets): the model ranks the arms as observed (its optimum = the empirical best
  on 6/8 rows; `model.svg`). The rule changes (k, N) only on P24, B24, B27 (R0), and B27's change loses at w = 1
  (1.061 [1.000, 1.126]); R1 (w = 0.1 objective) changes P24, B18, B24.
- **Confirmatory (registered fdc66e8622, fresh seeds, 3 x 144 pairs per row): the registered rule holds on no row.**
  - B24 (k5 N9 -> k6 N9): **0.9685** [0.9424, 0.9954; bootstrap 0.9365, 1.0031] at w = 0, **0.9556** [0.9290, 0.9831] at
    w = 0.1, **0.9117** [0.8833, 0.9410] at w = 1; it fails because replicate r2 is 1.000 at w = 0 (upper 1.062) and the
    pooled bootstrap upper at w = 0 is 1.003. Cheaper by the point estimate at every weight, and the pooled interval excludes
    1 at w = 0.1 and 1, but that is not the registered test.
  - P24 (k5 N11 -> k6 N10): 1.002 / 1.002 / 1.003, no detectable change.
  - B18 (k4 N7 -> k5 N7): 1.024 [1.003, 1.045; bootstrap 0.9965, 1.053] / 1.017 / 0.991; not "significantly worse" under the
    registered definition (smaller of the two lower ends 0.9965), but costlier at w = 0.
  - The five other rows keep the producer's (k, N): ratio 1 by construction.
- **Distance remaining to rho-neg:** unchanged at the registered level, **2.78-4.97 at w = 0** (split study, best arm per
  row). On this study's own targets the B24 point estimate moves 4.23 -> 4.10 (w = 0), 4.52 -> 4.32 (w = 0.1),
  5.88 -> 5.36 (w = 1); P24 4.52 -> 4.53; B18 2.90 -> 2.97. Rho is ahead on every row at every weight.
- **Controls.** A/A `rho-neg-aa / rho-neg` = 1.0000 everywhere; 9/9 sessions `verify --replay-all` audit OK (6,912 records,
  5,184 replays, 0 problems); pilot 8/8 audit OK. All runs verified.
- **Reading.** For this pipeline, base sizing is not a lever left on Table B: the remaining cost is
  `~sqrt(2(N + 0.6))` (the T-against-trials balance at the optimum) plus the fixed walk set-up, and the discrete choices of
  (k, N) already sit within a few per cent of it. B24 is the one row where k = 6 gains, by 3-9 % depending on w; a powered
  replication could confirm that alone.

## Deviations

Local L0 instead of isolab-eks; no S3 checkpoint or raw upload (no AWS); `.resume/RESUME.md` not rewritten (another lane's);
Conductor and research-kb unavailable. Binary reused (no code change): the split study's validated
`808511395723…ffe161`. Rule revised after the pilot (R1, before the confirmatory data, recorded in PROTOCOL.md section 2).
Amendment A1: evaluator I/O fix after data. Raw sessions outside Git: `confirm-sessions.tar.gz` (sha256 `7ab0ef94…1f26`) and
`pilot-sessions.tar.gz` (`a2ba7a4a…0ba1`) in the producer's scratchpad; per-file digests in `data/sessions.sha256`.

## Files

`PROTOCOL.md`, `RULE.md`, `rule.py`, `make_specs.py`, `specs/`, `pilot/` (specs, `analyse.py`, `pilot_table.md`,
`pilot_summary.json`), `run_pilot.sh`, `run_all.sh`, `analysis/` (evaluate, summary, plot), `results.json`, `results.md`,
`summary.json`, `data/` (run log, verify summaries, digests), `ratios.svg`, `model.svg`, `REPORT.pdf`.
