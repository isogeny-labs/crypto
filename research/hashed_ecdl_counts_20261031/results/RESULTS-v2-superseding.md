# Counted-S v2: superseding results record

Supersedes the wording of `results/RESULTS.md` (merged in PR #3131, immutable) wherever the two differ. The data are unchanged: `decision.json` (sha256 eb5d791a8c632c09991a2096f44f1c2b247b0191ac529bec560e5d680777630f, unaltered). The independent validator report is `../VALIDATOR.md`. Status stays **unvalidated for the gate**: no `VALIDATION.json`, no leaderboard row, no `benchmarks/` row.

## Corrections to RESULTS.md

| # | RESULTS.md said | Corrected statement | Evidence |
|---|---|---|---|
| F1 | The gap to rho widens "roughly 4x per 2 bits". | The tensor-m2 / rho-neg S ratio grows about **5.1x per 2 bits**: log-log slope of ratio vs log2 r is 1.17 (linearised-t 95% CI 1.00 to 1.34; bootstrap 1.10 to 1.24). Successive 2-bit steps are 6.3x, 6.3x, 4.0x. The 4x figure holds only for tensor-m2's own median S (slope 1.04, inside the pre-registered P2 window). The ratio grows faster because rho's own median S falls from 3.1 to 1.9 (P3 slope 0.37, bootstrap 0.28 to 0.43, inside the 0.35 to 0.65 window). The qualitative claim, a widening gap, stands. | VALIDATOR.md check 5, `validator/slope.py` |
| F3 | Results are under `.../20261031/v2/`. | The v2 prefix holds only session C-16, the binaries and the receipts. Sessions B and C-10-14 are under `s3://setec-astronomy-artifacts/validation/hashed-ecdl-counts/20261031/snapshot-1730/sessions/`. | VALIDATOR.md |
| F4 | (decision.json `descriptive_note`) "includes exhausted pairs at the S they spent". | For the 17 affected cells the field covers verified pairs only. `decision.json` is left as produced; this record is the footnote DECISION.md required. | VALIDATOR.md check 4 |
| F5 | "No tested arm beats rho.negation in counted S at any size." | Too universal. Correct: **no complete cell shows an arm cheaper than rho or ic-mitm at the four tested sizes.** native-source and native-width never produced a verified execution (0 of 64); their exhausted rows spent only 1.0 to 5.9x rho's S at w=0 (0.99x once) because almost all of their work is unpriced, so they are unknown, not worse. | VALIDATOR.md check 6 |
| F6 | "ic-mitm 14.8 down to 5.2 x rho". | That is the w=0 column only. At w=1 it is 12.6 down to 5.0. | decision.json |
| F7 | P4 supported_within_scope. | True, and rests on **3 of 16** native cells (native-selectors fp10, native-partial fp10 and fp12). | decision.json P4 |

## Caveats RESULTS.md omitted

- **A/A is vacuous for counts.** Arm and A/A share one seed (the determinism rule), so identical S proves reproducibility, not the noise floor.
- **One round, 8 targets per cell, power.passes = false.** Intervals are linearised-t on a single round; the Holm family is 32 tests.
- **L0 isolation.** Counted units only. No wall-time claim, and the L2 gate was not used.
- **C-16 ran on a different host class** from C-10-14 and B. Counts are host independent; nothing wall-time is compared across them.
- **rho reference.** The comparison uses rho-neg. Track B also ran `rho.negation_v2`, up to 11% cheaper than rho-neg at fp16 (0.889 at w=0). No ranking changes, but a gate descriptor for B must use v2 or state this.
- **Binary provenance is unproven.** All three sessions record `git_commit = null`. The original C-10-14 (679cdc2f...) and B (a133714d...) binaries no longer exist. The replay binaries are producer rebuilds: C (5cec72d2..., rustc 1.97.0, source 4df337e) and B (603c6c86..., rustc 1.99.1, musl, `--no-default-features`, source 3ae1ac2bd). The original B was a glibc, rustc 1.97.0 build, so "not bit-identical" understates the difference. Replay checks scalar, exhaustion flag, bit-exact total and per-phase GAE, counters, factor base and field ops, and all 416 executions reproduced across the different toolchain, libc and feature set. The counts are therefore properties of the algorithm, but the binary-to-commit chain of custody is not established. 3ae1ac2bd is reachable only from branch `codex/hashed-ecdl-ecbench-e2e`; `pipeline.sh` (the only record of the C-16 command and B build flags) is in S3, not git.
- Unchanged and still true: finite-size fallback only (root_u = 0), so the recursive subexponential kernel was never exercised; native-arm S is a lower bound.

## What the validator confirmed (no change to the numbers)

Frozen bytes match the approval record (protocol hash 8a794feb...). `--replay-all` reproduced 96 + 288 + 32 executions with 0 problems. [d]G = Q holds on all 306 verified rows in independent Python. All 45 tensor/rho and tensor/ic-mitm ratios recompute from `records.jsonl` to a relative difference of at most 3.3e-16 against `decision.json`. The 15 complete cells and their p-values reproduce; the largest p is 3.96e-5 against a Bonferroni level of 1.56e-3. P1 holds on 4 cells, P4 on 3.

## Why a VALIDATION.json is not issued

Nothing here is headed for the leaderboard or `benchmarks/README.md` (every ratio is above 1), so the gate does not apply to recording this null result. A future row would first need: raw sessions in git (about 3.4 MB); a replay certificate in gate schema naming the rebuilt binaries; a validator-report.json; a rho-reference decision (kind negation, v2 for B); the B source commit pinned with `pipeline.sh` and build receipts in git; and sessions B and C-10-14 mirrored under `v2/` in S3.

## Not checked

Checkpoint and disk execution preconditions (only the 96/32/288 plan counts and `rustc -vV` from host.json were verifiable).
