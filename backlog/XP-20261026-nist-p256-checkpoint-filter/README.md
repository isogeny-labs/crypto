# Filter the completed NIST P-256 checkpoint

**Submission status: blocked; no job has been queued or run.** This is a
reviewable experiment definition in Git. It is distinct from a live cairn
task, a queue receipt, or approval to launch a paid machine.

The completed [R08 report](../../research/nist_skipped_curve_dissection_20261021/factorization/rounds/R08-20261025/reports/completion-20261026T2122Z/REPORT.md)
explains the numbers. In brief, 10,441,117 is the raw row count in the
newest checkpoint; 92 was the survivor count from filtering an older
4,592,399-row input. The newest file has not been filtered. This experiment
will measure its native filtering counts and preserve the controller's
next-state decision.

## Submitted records

- [card.json](card.json): `backlog.card/v1`, one paired lab task, blocked
  prerequisites, resources, commands and verifier.
- [PROTOCOL.md](PROTOCOL.md) and [protocol.json](protocol.json): prospective
  `experiment.protocol/v1`; exact inputs, both arms, control rationale,
  metrics, budgets, exclusions and interpretation rules.
- [experiment tracker](../../experiments/specs/EXP-NIST-FILTER-001.yaml):
  draft `crypto.experiment/v1` for `expspec ls` / `validate`. It requires
  a committed coordinator decision and promotion before `expspec next`
  can issue execution cards. The PR itself is ready for review.
- [reference-inputs.sha256](reference-inputs.sha256) and
  [candidate-inputs.sha256](candidate-inputs.sha256): exact checkpoint manifests.
- [run_pair.sh](run_pair.sh), [run_filter.sh](run_filter.sh),
  [inspect_filter.rs](inspect_filter.rs), [verify_pair.sh](verify_pair.sh):
  thin native orchestration, counter analysis and replay. No Python research
  implementation or new factoring algorithm is introduced.

The reference arm must reproduce R07's native filtering inventory on the
unchanged R06 source: 4,518,304 unique rows and 92 singleton survivors. A
failed reference stops the pair. The candidate is the completed R08 source.
Both use the same executable and settings; only the checkpoint changes.
No survivor ratio is forecast from R07. No matrix rank is inferred.

## What must happen before execution

1. Merge this implementation and bind `card.code.commit` and the tracker
   to that merged commit. The prospective protocol hash and analysis source
   are frozen before candidate filtering; do not silently edit them.
2. Complete the existing [artifact publication problem](../../problems/nist-factorization-raw-archive-publication.md).
   Upload the original R06/R08 full archives through `tools/artifacts`,
   verify remote readback, and commit actual `.artifacts.json` entries.
   Both are currently local-only; a SHA-256 alone is not a download location.
3. Choose the nonpreemptible local/cairn lab environment and restore the
   original checkpoints. The default paths are
   `.cache/nist-filter-inputs/R06/state` and
   `.cache/nist-filter-inputs/R08/state`; these are conventions, not claims
   that the data is in a fresh checkout. Never use the R07 filtering copy.
4. Build the checker, run its tests, validate the card and tracker, complete
   `scripts/checkpoint.sh init` / `push` / `gate` under AGENTS.md, then
   promote the card to ready and the tracker with a committed decision.
   Select a cairn lab and identity using the [backlog runbook](../README.md).
5. Publish and claim the lab task through cairn, run it once, fetch its
   outputs and return them in a result PR. Merging this definition does not
   claim a task or execute it.

## Exact runner interface

From the repository root, on the prepared host:

```sh
CARD=backlog/XP-20261026-nist-p256-checkpoint-filter
rustc --edition=2021 --test "$CARD/inspect_filter.rs" -o /tmp/nist-filter-tests
/tmp/nist-filter-tests
rustc --edition=2021 -O "$CARD/inspect_filter.rs" -o /tmp/nist-inspect-filter
export NIST_FILTER_CHECKER=/tmp/nist-inspect-filter
export GNFS_BIN_DIR=/opt/yafu/factor/lasieve5_64/bin
export NIST_R06_STATE=.cache/nist-filter-inputs/R06/state
export NIST_R08_STATE=.cache/nist-filter-inputs/R08/state
bash "$CARD/run_filter.sh" --preflight-only reference "$NIST_R06_STATE" unused
bash "$CARD/run_filter.sh" --preflight-only candidate "$NIST_R08_STATE" unused
# After the prerequisite gates and a lab lease, run with a new output path:
bash "$CARD/run_pair.sh" /results/nist-filter-NEW-RUN-ID
bash "$CARD/verify_pair.sh" /results/nist-filter-NEW-RUN-ID
```

All names above are task-specific. `--preflight-only` verifies bytes and
line counts and launches no filter. Each native run uses a new output
directory, checks source and copied bytes, preserves native output/status,
then inventories the complete native log. The source relation files remain
unchanged; generated free relations stay only in the filtering copies.

## Cost and stopping

Request 2 CPUs, 16 GiB RAM, 24 GiB disk and at least 5 GiB free before each
copy. Each native filter has a 600-second budget and outer 660 seconds plus
15-second grace; each whole arm has a 900-second ceiling. The pair is capped
at 1800 seconds. Rounded reservations of one wall hour and one core hour
cover 2 × 900 seconds × 2 CPUs; they are not factoring completion estimates.
The protocol separately labels its rough 520-second planning model.

Funding is 0 because this submission is blocked and unqueued; it has no
priority over funded cards. No paid or Spot host is authorized. Filtering
has no admitted in-phase restore. Retain an interruption as incomplete and
stop; any later allocation needs a separate recorded decision.

Done means: a passing reference, a completed candidate or preserved failed
attempt, unchanged input proofs, all raw logs and native artifacts, a unified
`experiment.export/v1` with two planned rows and actual flow counts, a
second-runner review, and an inspected report/graph/PDF in `research/` by PR.
A useful matrix or a factor is not guaranteed. Until execution, there are
no candidate counts and no populated export.
