# Preregistration: does the pre-solve `malloc_trim(0)` remove the allocator confound from solve wall time without changing counts?

Frozen before any data of this study was taken (see `git log` for this file's
commit and the sessions' first `started` timestamps). Written 2026-10-23 for
aburan28/crypto PR #2455 (branch `ecbench/fix-runner`). Classification per the
`controlled-experiment` skill: the counts claim is an **exact comparison** (record
fields are equal or not); the wall-time claims are **comparative / statistical**
and are descriptive on this host (see "Host").

## Question

PR #2455 makes the measured child call `malloc_trim(0)` after the workload is
built and before any clock opens (docs/ecbench/README.md section 5, "Allocator
state inherited from set-up"). The PR text says that before the change glibc's
`malloc_consolidate`, triggered by fastbins left behind by set-up, landed inside
the timed solve (smoke-size rho/BSGS solves read 6-8 ms, after the change
0.13-0.37 ms; the IC online window on claim-koblitz fell about 15% at the
median), and that counts are unchanged.

Question: on the same specs and seeds, does the branch binary (with the trim)
read a lower solve wall time than the merge-base binary (without it), at the
sizes the PR names, while every recorded count stays bit for bit identical?

## Arms (binary)

| arm | build | source |
|:--|:--|:--|
| `base` | `cargo build --release --bin ecbench`, own `CARGO_TARGET_DIR` | `git merge-base HEAD origin/main` = `3a1ea541d964d6ef9099300a9ab512d55eaa073b` (exported with `git archive`, no local changes) |
| `fix` | `cargo build --release --bin ecbench` | branch `ecbench/fix-runner` at `63fede816dcacc012580ce37fd40b4ad850ad867` (clean tree) |
| `fix-notrim` (exploratory, H4) | a third build in a scratch copy of the `fix` source tree (never committed to the PR) in which the single statement `facts.trim_result = Some(unsafe { libc::malloc_trim(0) });` in `prepare_allocator` (src/cryptanalysis/ecbench/record.rs) is replaced by `facts.trim_result = Some(0);`; every other line is the PR's. The patch is saved as `analysis/notrim.patch`. (`LD_PRELOAD` was considered and rejected: the runner starts the child with `env_clear()`.) | isolates the trim from the other changes the PR makes (grading v3, isolation, PMU group). Built last, reusing the base `CARGO_TARGET_DIR` (so no fourth target dir) after the base binary has been copied out |

All builds are made under `flock /home/user/wt/.cargo.lock`. Both binaries'
SHA-256 are recorded in `BINARIES.txt` and in each session's host capsule.
Only the binary differs between arms; spec, seeds, flags and host are the same.
Caveat stated in advance: the PR changes more than the trim (grading, isolation
probing, PMU, session writing); `base` vs `fix` therefore measures the PR, and
only `fix-notrim` speaks to the trim itself.

## Specs (frozen)

| key | spec | sha256 | executions |
|:--|:--|:--|--:|
| smoke | `docs/ecbench/specs/smoke-prime.json` (committed, unchanged) | `684c9aeb...79cc2`, full value in `SPEC_SHA256.txt` | 72 (9 arms x 2 targets x (3 rounds + 1 warm-up)) |
| claim | `specs/claim-koblitz-r10.json` (NEW file in this study; the committed `claim-koblitz.json` has 1 round, so n = 2 per arm; this copy changes only `rounds` 1 -> 10 and the label/description) | `c23b5202...52392` | 66 (3 arms x 2 targets x (10 rounds + 1 warm-up)) |

`claim-koblitz.json` is the cheapest committed `claim-koblitz*` spec (Koblitz
a=1, n=17, 2 public targets). Round count 10 is even so the `alternate` order
balances. `smoke-prime.json` has 3 measured rounds (odd; `ecbench plan` warns
that the alternate order cannot balance); it is used unchanged, the imbalance is
identical in both binaries, and n is raised by repetition (below), not by
editing a committed spec.

## Design

Each (spec, binary) is run twice, in an ABBA order to cancel linear drift of a
shared host: `base`, `fix`, `fix` (rev), `base` (rev). Same spec file, same
spec seeds, `ecbench run --wait --allow-busy --cpus auto --quiet` for every
session. The reverse-order replicates are separate sessions
(`<arm>-<key>-rev`). The pooled sample of an arm is the two sessions of one
binary together, over measured (non-warm-up) records that verified: smoke
n = 2 targets x 3 rounds x 2 sessions = 12 per (arm, binary), claim
n = 2 x 10 x 2 = 40 per (arm, binary) minus any unverified record. The exploratory `fix-notrim` arm is run once
per spec after the four ABBA sessions, same flags.

Session names: `base-smoke`, `fix-smoke`, `fix-smoke-rev`, `base-smoke-rev`,
`base-claim`, `fix-claim`, `fix-claim-rev`, `base-claim-rev`,
`fix-notrim-smoke`, `fix-notrim-claim`. Sessions are evidence: never edited
after a run. A failed or interrupted session stays and is reported.

## Host

One shared 4-vCPU virtual machine (Intel Xeon 2.10 GHz, `ecbench host` class
ECBENV3 for the `fix` binary), with other agents running cargo builds and
tests at the same time (load average 4-6 when this was written). Runs use
`--cpus auto --allow-busy`: the harness reserves a whole core away from CPU 0
but a busy preflight is recorded and every run stays below its requested L2
where the harness says so. Consequences, fixed now:

* **Wall figures are descriptive.** The isolation level each record earned is
  reported; no wall-clock statement is made as a test of significance, and
  `outside_noise`/`admitted` are quoted as the harness records them.
* The `fix` binary (grading v3) refuses to admit wall time from a solve under
  1 ms; sub-millisecond `fix` smoke solves are therefore expected to be
  "not admitted" by the grader even where the number is recorded.
* The two binaries run in different sessions, so `ecbench compare` refuses
  wall time across them by design; the wall comparison below is a summary of
  recorded `time.solve_wall_ns` and `online.wall_ns` values by `jq`
  (`analysis/summarise.sh`), not an `ecbench compare` result.
* Operation counts do not depend on the host.

## Outcomes

Primary (wall, descriptive): per (binary, arm) over verified measured records:
n, median, min, quartiles and max of `time.solve_wall_ns`; median of
`time.solve_wall_ns / cost.total_gae` (ns per GAE, per record); for the claim
spec also the median of `online.wall_ns` of `ic` and `rho-strong` (the IC and
strong-rho online windows). Ratio = base median / fix median.

Control (counts): for every (arm, workload target, round, warm-up flag) present
in both binaries' sessions: `outcome.status`, `outcome.recovered`,
`algorithm_seed`, `cost.total_gae`, `cost.unpriced`, `phases[].adds / doubles /
scalar_mults / gae / native`, `counters`, `run_id`. Compared two ways:
(a) `ecbench compare --dir <fix> --b-dir <base> --a X --b X` for each arm X
(ratio of S, `same_seeds`), (b) a `jq` field diff of the records
(`analysis/counts_diff.sh`). Also: every `fix` record carries an `allocator`
block with `pre_solve_trim: true`; `base` records carry none.

Audit: `ecbench verify --dir D --replay-all --exit-code` on every session with
the `fix` binary; receipts stored beside the study.

## Predictions and what would falsify them

* **H0 counts identical.** Every field listed under "Control" is equal for
  every matched record across binaries. Falsified by any single difference
  (then the PR changes what is counted and the "counts unchanged" statement is
  wrong, whatever the wall time says). A missing partner record or a record not
  `verified` in one binary is reported, not silently dropped.
* **H1 smoke wall.** On `smoke`, for the eight non-IC arms (`rho-neg`,
  `rho-plain`, `rho-frozen`, `bsgs`, `bsgs-il`, `bsgs-neg`, `kangaroo`,
  `rho-neg-aa`) the pooled fix median `time.solve_wall_ns` is below the pooled
  base median. Supported if at least 7 of 8 arms have base/fix >= 2 and a fix
  median below 1 ms; falsified if fewer than 4 of 8 arms have base/fix > 1 (the
  confound the PR describes is not there, or the trim does not remove it);
  anything in between is reported as partial. `ic-mitm` is reported, with no
  threshold.
* **H2 claim wall.** On `claim`, the pooled median of `online.wall_ns` of `ic`
  is lower in `fix` than in `base`. Supported if base/fix > 1 in each of the two
  order replicates separately and at least 70% of pooled `fix` values lie below
  the pooled `base` median; falsified if the pooled median ratio is <= 1.00;
  otherwise inconclusive. The PR's "about 15% lower" is quoted for comparison
  and is not a threshold. The same readout for `rho-strong` and for
  `solve_wall_ns` of all three arms is reported without a verdict.
* **H3 audit.** Every session passes `ecbench verify --replay-all --exit-code`
  and, with `--replay 0 --allow-interrupted`, the CI form. Failure of either is
  a failed study, reported as such.
* **H4 attribution (exploratory).** On `smoke`, if the trim causes the drop,
  `fix-notrim` pooled medians sit near `base` (base / fix-notrim between 0.7 and
  1.4 for at least 6 of the 8 non-IC arms) and far above `fix`. If instead
  `fix-notrim` sits near `fix`, the wall drop comes from another change in the
  PR, and that is the finding. No statement about the trim itself is made
  unless this arm ran.
* **A/A noise floor.** In `smoke`, `rho-neg` and `rho-neg-aa` are the same
  method; |median ratio - 1| per binary is reported as the noise floor of the
  per-arm medians. A base/fix ratio within that floor is "not distinguished
  from noise".

## What does not count

Changing specs, seeds, flags, the arm list, thresholds or the pooling rule after
seeing data; dropping a session or record that is awkward. A change is a labelled
amendment (`AMENDMENT_n.md`) and both versions are reported. Wall time is never
the headline of an ecbench result (AGENTS.md section 6); it is quoted as a
practicality note about the harness.

## Scope of any conclusion

Two sizes (a 20-bit prime-order curve, Koblitz a=1 n=17), one shared 4-vCPU
VM, glibc allocator, one pair of commits. It says nothing about other
allocators, larger solves where `malloc_consolidate` is negligible, or hosts
that earn L3. It is not an index-calculus improvement and does not move any gap
to rho: it is a measurement-instrument correction.

## Evidence plan (validation gate)

`docs/VALIDATION_GATE.md` asks for an *independent* replay certificate and an
*independent* validator report. This study is run, replayed and summarised on one
host by one agent. If `independent: true` cannot honestly be asserted, no
`benchmarks/README.md` row is added and the study is kept with the gap stated.
