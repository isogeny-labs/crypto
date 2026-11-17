# Protocol: Primo on Boolean index-calculus descent systems

Version 1, frozen 2026-11-14 before performance measurement.

## Question and scope

Does removing per-call temporary-file transport and the 5 ms completion-sampling interval make
Primo faster on the exact Boolean systems produced by the repository's binary-field index-calculus
descent stage, without changing any SAT/UNSAT result or accepted model?

This protocol licenses only a first-solution Boolean solver-stage comparison. It does not measure
relation collection, point lifting, linear algebra, individual logarithm recovery, a deployed
curve, an asymptotic change, or end-to-end index-calculus speed. Complete-enumeration engines are
used as correctness oracles and are not ranked as the same workload.

## Implementations and inputs

- Candidate: `primo-smt`, Primo 0.2.0 fed through stdin with prompt child-completion observation.
- Matched baseline: `primo-smt-file`, the prior temporary-file and 5 ms polling path retained in
  the same binary.
- First-solution reference: `sat-cdcl:sat_xor_encoding=native`.
- Correctness oracle: `exhaustive`, which enumerates the complete solution set.
- Primo source: `fcc274c23ff5b146cc43d80b1b0317faf91c7f08`; the executable hash is captured by the runner.
- Families: Koblitz (`K`) and random binary (`R`).
- Cells: `11:6:2`, `13:7:2`, `15:8:2`, and `13:5:3` (12, 14, 16, and 15 Boolean variables).
- Targets: 8 per family/cell, seed `233163335` (`0x0DE5_CE47`).
- Budget: 30 seconds per solver call. A budget result is retained and makes the performance claim
  incomplete for that target.

`ic descent` fingerprints every canonical Boolean system with BLAKE3, records every target and
repeat, rotates engine order each repeat, independently checks Primo SAT models against the
original equations, and compares each answer with the exhaustive solution set.

## Design and metrics

One two-target/one-repeat warmup is discarded. The measured run uses seven repeats. The primary
metric is paired `SystemSolver::solve` wall nanoseconds, including encoding and transport, over
identical `(family, cell, target, repeat)` inputs. Report medians and the paired baseline/candidate
ratio by cell and pooled; retain the full distribution. Solver-reported conflicts and decisions,
verdicts, solutions, input bytes, row counts, and system fingerprints are correctness and work
controls, not interchangeable timing units.

The legacy arm's odd and even repeat indices form the A/A timing control. The native-XOR CDCL arm
is the strongest directly integrated first-solution comparator; it is not expected to share
Primo's conflict count. The complete oracle establishes solution membership and UNSAT, not speed.

## Decision rules

The change is accepted only when:

1. every candidate and baseline target is decided in all repeats;
2. `disagreements` is empty and both Primo arms agree with the exhaustive oracle;
3. every Primo SAT result reports `model_checked=1` and `invalid_model=0`;
4. matched legacy/candidate verdicts, solutions, conflicts, and decisions are identical;
5. the candidate/legacy paired stage-time interval excludes 1 in the faster direction and the
   observed gain is larger than the legacy odd/even A/A interval.

A null, slowdown, timeout, invalid model, changed deterministic counter, or disagreement is
retained and reported. No cell, target, or repeat is removed after observing results. Wall results
from a non-L2 host are labelled L0 diagnostics and cannot satisfy the admitted speed claim.

## Execution and resource limits

The committed runner is:

```sh
research/primo_ic_performance_20261114/run_stage.sh OUTPUT_DIRECTORY
```

Before any run, `scripts/checkpoint.sh gate` must pass. The admitted run uses one c8a.2xlarge Spot
attempt, 4 vCPU, 4 GiB, a 900 second absolute deadline, no automatic retry, and a USD 2 ceiling
including build, idle time, storage, and transfer. Launch is forbidden until current daily and
monthly AWS budget headroom and pool availability are recorded. A local diagnostic has a 15 minute
deadline and never substitutes for L2.

## Analysis and publication obligations

Raw JSON, stdout/stderr, host metadata, binary/source hashes, failed attempts, and the checkpoint
receipt are retained. The result PR must include a `bench.record/v1`, progress report, graph and
PDF, validation record, benchmark index row, and a PCC measurement claim. Until CI signs the claim,
all quantitative prose is tagged `[pcc:unverified]`.
