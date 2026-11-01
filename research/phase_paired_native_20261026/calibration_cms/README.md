# Pilot runs of the CryptoMiniSat follow-up (non-frozen targets and tiny instances)

Raw receipts of what was run before `../PROTOCOL_ADDENDUM_2.md` was written, to see the pinned binary's
output format, exit codes, seed behaviour and speed, and to exercise the producer's new code paths. They
use the calibration manifest `../calibration/calib_32_manifest.json` (SHA-256
`8d0d9f23e5913e540ad5077186f78adec25ea6c3dbf32e78c31c48e2f6116e5a`, targets that are not in the frozen
manifest) or no manifest at all (tiny instances). Earlier builds of the producer wrote them, so the
`producer_sha256` of each header names a build that is not the final one. None of them enters a result
table, and all of their CPU counts against the 60-minute cap.

| file | what it ran | CPU s (from the file) |
|---|---|---|
| `selftest_try.jsonl` | `cms-selftest`, 100 random CNF+XOR instances (29 SAT, 71 UNSAT) and four hand-written files | 1.19 |
| `pilot_check.jsonl` | `cms-check` on `b0` and `a0` of the calibration set: fidelity of the export (native search from the original and from the DIMACS round trip, 20,000 conflicts, both arms) and the positive control (`b0`, both arms, same-seed rerun) | 46.16 |
| `pilot_run1.jsonl` | `cms-run`, `b0` arm A, conflict budget 5,000: CryptoMiniSat stops with `INDETERMINATE`, exit code 15, 5,002 conflicts. The producer then refused the exit code (it expected 0) and stopped with status `error`, which is how the exit code was learned | 0.15 |
| `pilot_run2.jsonl` | `cms-run`, arm A on calibration `a0` (UNSAT, 666,390 conflicts, 25.4 s) and `b0` (SAT, 70,864 conflicts, 2.0 s) | 27.42 |
| `mini_controls.jsonl`, `mini_run.jsonl`, `mini_run2.jsonl` | a mini end-to-end pass on the six calibration targets with tiny budgets (3,000 conflicts, 0.1 s backstop) to exercise the stage records, the cap stop, the skipped receipts and the replay | 4.15, 3.70, 0.84 |

Runs that left no receipt file, counted in the ledger by hand:

- the first `cms-selftest` (60 instances, a less dense family): 0.40 s;
- four direct invocations of the binary on the arm A `b0` calibration instance with `--random 0, 1, 2, 3`
  (`--maxconfl 3000000 --maxtime 120 --threads 1 --verb 1`): CryptoMiniSat reported 235,123, 278,401, 74,042
  and 96,951 conflicts and 7.81, 9.69, 2.50 and 2.84 s, 22.84 s in all. So `--random` changes the search a
  great deal (the same heavy tail the native study saw across phase seeds);
- one `cms-run` of the mini pass with a roomy cap (4.07 s), whose file was overwritten;
- the unit-test runs of both examples (native solver and exhaustive pair matching, a few seconds each).

The pilot ledger is the 83.6 s of the table, plus 0.4, 22.8 and 4.1 s above, plus an allowance of 60 s for
the test runs: **180 s** (rounded up), entered as `--cpu-prior 180` in the main run.
