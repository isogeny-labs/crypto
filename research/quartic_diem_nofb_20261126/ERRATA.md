# Errata: research/quartic_diem_nofb_20261126

Source: independent validator, validated commit cc7e5c5488a. Verdict: **validated with errata**; gate checker `valid`. Its structured report is `validation/validator-report.json` (errata E1–E5, process findings P1–P7), with prose in `validation/validator-report.md`.

The pinned files stay byte-equal: PROTOCOL.md, `seeds.json`, `analyze.py`, `run_*.sh`, `raw/` and `analysis.json`. No decision, verdict or `analysis.json` number changes. Every number is [pcc:unverified].

- **E1, the headline ratio.** The pooled held-out GAE₁ ratio is 0.8324606. It was rounded twice (0.83246 → 0.8325 → 0.833).
  - The correct figure is **0.832 [0.758, 0.907]**.
  - The wrong 0.833 appears at `RESULTS.md:32`, `RESULTS.md:52`, `reports/run-20261126T0333Z/report.typ:20` (and `report.pdf`), the bench record's `summary`, and the subject of commit cc7e5c5488a. The bench record's metric value is correct.
  - Any row cites 0.832.
- **E2, `RESULTS.md:60`.** The nofb interval at w = 0.1, p = 65537 should be [0.263, 0.275].
- **E3, `RESULTS.md:61`.** The nofb interval at w = 0.1, p = 131101 should be [0.184, 0.216].
- **E4, `RESULTS.md:69` and `report.typ:36`.** "About twice" / "about double" overstates the cost of a second attempt.
  - The 14 two-attempt nofb runs cost 1.50–1.90× the geometric mean of the same curve's single-attempt runs (GAE₁).
  - The one three-attempt run (131101, curve 177, seed 4) costs 2.25×.
  - The stated mechanism, attempt 1 leaving t undetermined, is confirmed on all 15 fallback pairs.
- **E5, `analysis.json:835` (written by `analyze.py:218`, comment at `:201`).** `"rule": "PROTOCOL.md section 6"` should name section 5. Section 6 is Outputs.

## What the validator found beyond the claims

- **Dominance on every counter.** nofb is no costlier than sge in `group_ops` and in all six native counters, on all 80 pairs. So the ranking holds at every w ≥ 0, including any calibrated weight.
- **Rescues were impossible, not only unobserved.**
  - The binary's uncounted `--quartic-diag` ran on every fallback curve × 4 seeds × 2 arms, 88 runs, with no committed field changed.
  - On all 32 failed attempts, t was undetermined: rank = rank without t < columns.
  - Attempts are also equal on all 80 pairs, which a rescue would have changed.

## Process findings (no number changes)

- **P1.** The raw data files are committed directly; S3 is unavailable here and the CI key is invalid (#5216 comment). The binary exists only at a scratch path.
- **P2.** No `w_cal` is recorded. The ranking does not depend on it, because of the component-wise dominance above.
- **P3.** RESULTS.md has no §3a cost chain. Its premise holds: only the fallback's own fields differ between the arms.
- **P4.** Latent weaknesses in the frozen `analyze.py`. None affects this study, because the grid is complete and every key exists.
  - C0 and the IC/rho loop read fields with `.get`.
  - Completeness is never checked against the `seeds.json` panel.
  - The identity check compares a subset of fields.
- **P5.** `seeds.json` gaps against `docs/experiment-seeds.md`. The pairing itself is correct.
  - `master_seed` is null.
  - Two seeded streams are not listed: the BSGS order computation (`quartic_ladder.rs:1241`) and the sampled factor base (`quartic_ladder.rs:1888`).
  - The solver stream is not keyed by instance; this is declared as inherited.
  - Derived seeds are not retained.
- **P6.** The bench record names commit 8bc5537a844 with `commit_dirty` set; it was opened 9 s before the freeze. The driver `run_all.sh` (not frozen) was committed 51 s after the run started.
- **P7.** The unit test never exercised a fallback. This was disclosed in the protocol, and the grid identity check covers the 15 fallback pairs.
