# Bounded-memory Boolean Macaulay passes: execution record

Eight fixed-degree passes on actual descended systems completed. Four small
controls agree exactly with an independent dense bit-matrix RREF. The n=17,
m=4, ell=6 passes processed 31,110 rows with final rank 30,888 and a 1,048,576
byte pivot-cache payload limit. This is a row-reduction infrastructure result.
It does not measure iterative closure, solving degree, full F4, or ECDLP cost.

The [run report](reports/REPORT.md), [PDF](reports/report.pdf),
[graph](reports/passes.svg), and [machine records](summary.json) preserve the
results and limitations. Protocols were written before their respective trials:
[original](protocol.json), [large continuation](protocol-large-continuation.json).

| Requirement | Execution status | Evidence / remaining gap |
|---|---|---|
| Native disk pivot reduction and bounded cache | Verified on the stated passes | `src/disk_macaulay.cpp`, eight pass records; 512 MiB address-space ceiling enforced by native `wait4` launcher |
| Actual descended Macaulay inputs | Executed | Exact repository ANFs, field structures, targets, subgroup orders and polynomial-basis membership in `inputs/*.anf.json`; each input and binary hashed |
| Independent exact control equality | Verified on n=5 and n=9, two policies | Dense packed-bit oracle uses different representation/elimination loops; unique RREFs compared exactly |
| Cache eviction, zero-cache, checkpoint/restart | Verified on n=5 control | Same rank and independent RREF; pauses at row 31, resumes from retained state |
| Larger n=15 and n=17 passes | Completed, independently unverified | Dense reference exceeded its explicit 256 MiB construction budget; large continuations deliberately remain diagnostic and do not discharge that validation gate |
| Fail-closed budgets and input binding | Verified on stated controls | Truncated stream, generated-row disk cap, restart SHA mismatch and unsupported ell=7 layout refused; raw stderr retained |
| Full Boolean F4 / MutantXL closure | Not implemented | No critical-pair/closure scheduler or integration into the incumbent solver |
| Original frontier ell=6/7, guessed ell=8/9 closures | Open | Chosen n=17/ell=6 is a diagnostic cell, not a substitute for the canonical frozen frontier suite; m=4, n=19/ell=7 requires 66 variables and the current exporter is capped at 64. The later remote m=3, n=19/ell=7 MutantXL attempt had 40 variables and stopped at its 80 GB address-space cap; these are different cells. |
| Distributed / GPU reduction | Not implemented | No GPU or configured cloud runtime; this is a serial disk-backed implementation |
| Legal pivot policy hooks and traces | Executed | `first` and minimum incoming weight are legal choices among rows with the same leading column; all decisions have row/lead/weight/action trace records |
| Learned column/pivot policy with heldout transfer | Not attempted | No model, frozen train/calibration/heldout split, training charge or heldout efficacy experiment; deterministic hooks only |
| End-to-end or fastest-engine improvement | Not claimed | Shared host, isolation tier 0; no matched external engine or full pipeline/rho comparison |

Later campaign work is reported separately: [the restricted learned selector](../learned_pivot/README.md) evaluates whole-pass policy selection, and [the execution matrix](../EXECUTION.md) links the remote GPU and MutantXL attempts. Those continuations do not add learned pivot control, a GPU implementation or full closure to this standalone reducer.

## Reproduce

Build commands are in [build_commands.txt](build_commands.txt). The exporter was
compiled directly against the root agent's already-built library to avoid the
shared Cargo lock. Exact binary and dependency digests are retained. Root source
base is `e063d98475589d9aba7ae265a8cba830927e3156` with the root agent's four
library build repairs; no core source is modified by this study.

`run_trials.sh` and `run_continuations.sh` contain the executed commands. They use
fixed fresh output directories and refuse to overwrite them. For a replay, copy
the study to a new run name, restore compressed records, and build the binaries.
Every original `.rows`, `.bin` and `.jsonl` record is stored losslessly as gzip;
`gzip -dk path.gz` restores its original basename and the recorded uncompressed
SHA256. Do not execute the scripts over the existing input/run paths.

The exact row family is generator-times-squarefree-multiplier, with multiplier
degree at most D minus the generator's input degree, and Boolean collisions
cancelled. This matches the traversal in the pinned repository's
`koblitz_groebner::visit_macaulay_rows_counted`. The sparse disk journal keeps
forward echelon pivots, not a fully reduced basis; the small oracle independently
computes dense RREF to certify equality. Leading masks use degree descending and
ascending integer mask within each degree (repository grevlex convention).

## Memory and checkpoint limits

The pivot cache bounds **row payload**, with explicit LRU eviction. Index and LRU
objects add overhead; the metadata guard estimates 128 bytes per pivot and the
whole process also has a hard 536,870,912-byte address-space ceiling. The pivot
journal is capped at 1 GiB and each trace at 64 MiB. Row-size guards admit at
most 16 MiB of terms; an XOR's temporary vector reservation can use up to
32 MiB before that guard, within the process-wide address-space ceiling.
The wall budget is checked during row reduction. These are engineering bounds,
not extrapolations of frontier memory requirements.

Checkpoints flush the journal before atomically renaming metadata and record the
input SHA256, policy, cache size, next input offset, journal boundary and counters.
Restart truncates incomplete journal suffixes. This study checked process restart
and input identity. It did not establish power-loss durability, `fsync` semantics,
or resistance to arbitrary journal corruption. Journals and original failure
records are copied into fresh directories for large continuation runs.

Counters include completed checkpoint history. Work rolled back after the last
checkpoint and index reconstruction traffic are not included in the cumulative
logical pivot counters. Native per-process CPU/wall/RSS records retain the
observed costs of failed attempts and continuations; no total-cost speed ratio is
claimed. Build, input export/generation and verification phases have separate
records rather than being folded into a solver denominator.

No canonical IC leaderboard or solving-degree graph changes: this study adds no
admitted pipeline gain, solving degree or security finding. A full closure
scheduler with exact certified replay on the canonical frontier inputs is the
next implementation requirement. Model training and heldout policy transfer are
separate outstanding requirements.
