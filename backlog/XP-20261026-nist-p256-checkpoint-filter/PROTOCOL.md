# NIST checkpoint filtering: R06 regression control and R08 candidate

This protocol applies to future filtering of the saved checkpoints. R08's
collection and R07's older filtering result were already observed. They
are prior evidence, not newly preregistered results. No claim of unusual
curves, performance, matrix rank, a factor, or historical motive is tested.

## Question and frozen inputs

Using the identical pinned YAFU/msieve filter, what counts survive duplicate
and singleton removal for R08's 10,441,117 raw relations, and does the
native controller request more relations or produce matrix prerequisites?

First run the **reference**: R06's unmodified 4,592,399-row checkpoint.
Require exactly 4,518,304 unique relations, 92 singleton survivors, 0
surviving tracked large ideals and four passes, as observed in R07. If this
regression control fails, stop; do not interpret the candidate.
Then run the **candidate**: R08's unmodified 10,441,117-row checkpoint.
`reference-inputs.sha256` and `candidate-inputs.sha256` bind every copied
input file, including the original relation data, selected polynomial,
range ledger, executable and parameter tables. Filtering always uses a
new copy and may append free relations to that copy. Never resume collection
from a filter-modified copy. Check source hashes before and after each run.

Literal residual:

```
33492266050798185653682380062483799213318454801653963493134064721059735936944520907295931496614455858076048437184499090478420204888353
```

## Instrument, workload and limits

YAFU upstream c7d90283972820f99b3b4920bfc1eff05c1f6342 with the existing
two-line timeout correction; executable SHA-256
bc7680adbdd7450a68120e73ef8aa88d0336c8ffa141343f50966c595ca6bae9.
Two threads and two linear-algebra threads (the latter phase is not run),
seed 20261025, `-gnfs -xover 81 -R -nc1 -minrels 1`, native 600 seconds,
outer 660 seconds plus 15 seconds kill grace, 4 GiB virtual memory per
process, 1 GiB per file, core dumps disabled. `-nc1` selects filtering
only. `-minrels 1` admits this diagnostic before the ordinary collection
gate; it does not weaken duplicate, singleton or merging criteria.

One run per checkpoint, reference first, no warmups. No statistical
population, p-value, timing comparison or speedup is inferred. A/A, a
random null population and a planted matrix-positive arm are inapplicable
to this exact counter inventory. This does **not** validate a detector of
general matrix existence or support a negative security claim. The required
R06 replay tests the complete native filtering path against known counters.
Both arms have identical instruments and budgets; only the input checkpoint
changes. Host and layout are recorded; wall time remains descriptive L0.

Use a nonpreemptible local/cairn lab host with 2 CPUs, 16 GiB RAM and 24 GiB
disk, at least 5 GiB free before copying. This filter has no admitted
in-phase application restore. Do not put it on an interruptible worker that
silently restarts it. A cancelled/interrupted attempt is retained as
incomplete, never resumed as the same measurement. No infrastructure retries
are authorized by this protocol. Each arm's whole task has a 900-second
ceiling; the two-arm envelope is 1800 seconds. No paid capacity is authorized.

The rough planning estimate is 520 seconds total, not a measurement or
completion promise: R07's recorded native filtering interval was 85 seconds;
scaling that count by 10,441,117 / 4,592,399 gives about 193 seconds for the
candidate, with 240 seconds reserved for two copies and checks. Filtering
may scale differently. The candidate has not been timed. The hard ceilings,
not this planning model, govern execution.

## Gates and stopping

1. Merge and pin the implementation, freeze `protocol.json` and its analysis
   source before candidate filtering; validate both schemas and the backlog card.
2. Publish the checksum-bound full checkpoints with the required artifact
   transport and restore them into the configured input directories. No
   remote object is currently claimed. Verify all input hashes and counts.
3. Require the repository's S3 resume checkpoint gate before a run starts.
   Record the selected nonpreemptible host and its native dependencies.
4. Build the Rust counter checker; require its six failure/replay tests.
5. Run reference once. Stop on a changed count, missing completion, nonzero
   native exit, source mutation, output cap or timeout.
6. Run candidate once in a new directory, then stop. Do not collect more
   relations, change a threshold, start linear algebra or allocate another run.

## Analysis and verdicts

`inspect_filter.rs` is the frozen native analysis source. It inventories the
full native log, requires a complete duplicate/singleton pass, rejects joined
logs and verifies `valid + rejected = raw` and
`valid + free - duplicates = unique`. Source/prefix checksum verification is
separate in the runner. Its success is **counter accounting**, not independent
reimplementation of msieve, verification of every relation, or a rank proof.

- `completed_insufficient`: valid complete counters and an explicit native
  controller request for more relations. Applies only to these parameters.
- `matrix_prerequisites_need_review`: the filter returns without asking for
  more and creates its cycle/matrix prerequisite files. Inventory and retain
  them; a second reader must check them before any readiness claim.
- `incomplete_execution`: timeout, nonzero status, missing counters, mismatch,
  interruption or an unknown controller outcome. This is not a negative
  mathematical result. Preserve all files and do not retry until a new decision.

Never infer rank from a survivor count or a file's mere existence. Retain
all native counters, commands, exact source/binary hashes, host receipt,
stdout/stderr, input/prefix checks and failure statuses. Produce one unified
`experiment.export/v1` beside the raw records, one row per execution and
explicit flow counts. Set `wall_valid: false`; retain descriptive clocks
only. File a second-runner review and a report, graph and inspected PDF in a
result PR. No future output/export is fabricated in this submission.
