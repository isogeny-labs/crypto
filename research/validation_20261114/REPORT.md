# Validation of the O3 run and the w_cal wiring (2026-11-14)

Validator V's report, transcribed by the main session (the validator could not
write Markdown reports from its harness). Standing `[pcc:unverified]` for both
targets: neither has a pcc claim or keyring signature. Validated from origin/main
`224624a91a`, building only from `git archive` exports of the commit under test.
Scripts are in `scripts/`, outputs in `out/`, the checklist in `PROGRESS.md`.

| Target | Verdict |
|---|---|
| A: `research/full_jacobian_ic_20261113` (PR #4071) | admitted with caveats; `VALIDATION.json` written, `scripts/validation_gate.py evidence` reports `valid` |
| B: ecbench w_cal wiring (PR #4070) | admitted with caveats; tooling, so no `VALIDATION.json` |

## A: whole-Jacobian IC against the negation rho on E1

Every number, class and certificate checks, and "unresolved" is what the
frozen rule gives. Eight reporting errors were found; none changes a cell, a
class or the verdict. They are corrected in the run's `ERRATA.md`
(`RESULTS.md` stays as validated).

- **Binding.** PR #4071 = `c88c7b4e0c`, `008a5cf3fb`, `f199720572`; merged in
  `cc075763d7`. Target files are byte-equal to HEAD. Raw hashes match the
  record (`e65114ea…`, `bfd3f60d…`); `PROTOCOL.md` matches `frozen.sha256`.
  The record binds `28da34a03b` with `commit_dirty: true`; the clippy fixes at
  17:39Z came after the run (17:29–17:36Z).
- **Freeze before data: not established from git.** Protocol, hash and data
  entered in one commit after the run. Scratch evidence agrees with the
  protocol's own account, but the smoke run had shown the counters at p = 211
  and 419 (2 of 7 stage-1 primes) before the predictions were frozen.
- **Recompute:** 2,057 values, 0 mismatches (`out/recompute_fj.json`),
  including 64 log-log fits with an independent t quantile; all 635
  `tables.md` cells pass. Exact-t lower ends: −0.0698 (A), −0.6540 (B).
- **Replay:** the full recorded command, all 9 primes: 3,017 non-timing values
  and 135 calibrated order classes, 0 differences (`out/replay_compare.json`).
- **Certificates, independent arithmetic** (`check_fj_certs.py`, own E1 group
  law and own Cantor, with a negative control): 360/360 IC logs certify on E1
  and on J_C; 736/736 rho logs; 468 per-run means equal the json; point counts
  and `#J = #E1·#E2` (p ≤ 3203) agree; `[#J]` kills G', Z and random divisors.
- **Matched rho** is the strongest available: tuned negation rho, same
  process; j(E1) is never 0 or 1728.
- **Band and calibrated:** all 36 IC cells above at 0/0.1/1 GAE and at
  0/w_cal/10·w_cal.

Findings: (1) F6 failed on 3 of 176 runs; (2) the smallest w = 0 ratio is B's
17.96, not A's 18.7; (3) three secondary fits were 9-prime fits, unlabelled
(stage-1: 0.238, 0.074, −0.033); (4) "0 bad relations" was not checked;
(5) additions are not on the band; (6) protocol seeds differ from the code's
fixed formulas; (7) IC arms are lower bounds marked `lower_bound: false`, and a
Cantor step is counted as one E1 addition though it costs about 4.3–5.7, so
ratios are understated (conservative for "above"); (8) p = 102,407 missing from
`stage2_not_run`, the killed first run absent from the record, raw files
`uploaded: false`.

Not checked by the validator [pcc:unverified]: the freeze ordering, the `rho_J` context runs, the
`selection_skips` counts, the S3 copies of the raw files.

## B: ecbench w_cal wiring

The w_cal files are unchanged between the #4070 merge `97a8e0d841` and HEAD;
the pre-#4070 commit is `ec56f03dd5`.

- **B1 tests:** 67 pass, 6 fail. Five fail only because the S3-held
  `research/curve25519_f1_targetwalk_20261108/sessions/k20-s0/records.jsonl`
  is absent (four in band, one in compare); the sixth was the validator's
  symlink overlay and passes with real copies. The five were not run with data.
- **B2 before/after on committed sessions without w_cal:** 21 of 23 outputs
  byte-identical; the other 2 are verify receipts differing only in
  `audited_unix_ms` and `auditor_binary_sha256`. All stderr identical; no
  `calibrated` key appears; 3 bound fits equal the committed records.
- **B3 hand-check** with a sidecar w_cal of 0.0285 (scratch copy, hand-check
  value only): all 9 grid cells to 1e-12, the order classes, ranked flag and
  band pair reproduce.
- **B4 class scan:** 883 committed `records.jsonl` and 633 source literals; 0
  multiplication-like counters outside the field class and its exclusions.

Findings: (1) `comparison_id` ignores w_cal; (2) the procedure-2 `challenge`
calibrated block has no test and no committed verdict to exercise it;
(3) `read_wcal` runs for procedure-1 verdicts, so a bad sidecar fails them
needlessly; (4) 797 store-only `records.jsonl` are invisible to the class test
and scan; (5) pre-existing drift, not from #4070: the committed procedure-1
verdict for `prime-toy-reference-e1` no longer re-derives under either binary
(`ci_method` "linearised_t" vs "cluster"), contradicting `docs/bounds/refit.sh`.

## Caveats

Toy sizes (p ≤ 51,203), one shared VM with no isolation, stage 2 with one
instance and two IC seeds per prime. A negative, unresolved result: not an
improvement [pcc:unverified] and not a leaderboard row.
