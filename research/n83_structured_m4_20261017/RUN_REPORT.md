# Retained structured N83 base: bounded m=4 query and m=5 control

**Run date:** 2026-10-17 UTC. **Protocol:** [PROTOCOL.md](PROTOCOL.md),
committed before either query. **Curve:**
`icv1-f2m83-tm6151469093347-debefd74`. The 2,001-column retained
dimension-12 signed-Frobenius object imported with 332,166 points and
166,083 distinct x-coordinates; both the macOS and Linux native import
receipts passed. The [import report](IMPORT_REPORT.md) covers those checks.

## Frozen execution

The public corpus, validation file, object and manifest hashes are in the
protocol and checked by the worker. The worker/library source used for both
queries is commit `1b00cf8a3450540aacd7778b6b2115276f46a81c`;
the native arm64 Linux executable SHA-256 is
`a26a3aeda02c70cf732c9219f4eb09cb8be3a65afd8f03cad74f86bd0e9bc477`.
The source changed only in study files after this build. The arm64 Linux
build used the protocol's pinned offline image and completed in the
[build log](linux-build.stderr.log). The same Linux executable served the
two arms sequentially. Each container had one CPU, 4,294,967,296 bytes
of memory, zero swap, no network, all capabilities dropped and a 960-second
outer cap. The worker checked the cgroup limits and enforced its own
900-second process cap. The shared macOS/Docker host was L0; the K=16,627
exporter build overlapped part of the m=5 arm.

The worker arguments after `/worker` were identical except for the mode and
fresh output directory:

```text
run4|run /input/manifest.json /input/object.jsonl.gz /corpus.json
  /validation.json 900 8 5000 32 /out/result
```

Both commands used `ChainedS3FrobeniusOrbit`, seed `2026100901`, direct
relations disabled, `max_variables=150000` and
`max_domain_clauses=12000000`. The container image was
`sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31`.
Inputs were read-only mounts; output mounts were distinct. The exact
[m=4 config](run4/config.json) and [m=5 config](run5/config.json) retain
the worker's source and input hashes.

## Observed results

| Arm | Completed trials | Completed SAT conflicts | Worker disposition | Accepted relations | Rank observed | Worker wall | Container exit / OOM |
| --- | ---: | ---: | --- | ---: | ---: | ---: | --- |
| m=4 | 8 of 8 | 40,000 | `UNKNOWN_solver_cap`; all eight calls `Unknown` | 0 within cap | 0 of 2,002 columns | 784.331 s L0 | 0 / false |
| m=5 | 4 of 8; next trial interrupted | 20,000 | `UNKNOWN_budget`; each completed call `Unknown` | 0 in four completed trials | unobserved | 900.062 s cap | 124 / false |

The [m=4 result](run4/result.json) contains all eight coefficient pairs,
query points, attempt dispositions, relation/rank receipts and cgroup peak
of **487,276,544 bytes**. Import took 7.527 s within its process wall;
relation collection took 775.571 s and the empty-matrix linear-algebra
path 0.005 s. The [event stream](run4/events.jsonl) and
[container state](run4.container-state.json) confirm that all eight trials
completed before the cap, with zero SAT models and zero invalid models.
There were no accepted relation rows to replay independently.

The [m=5 cap receipt](run5/cap.json) records the worker's own 900-second
deadline. Its [events](run5/events.jsonl) show four completed calls and
20,000 conflicts; the next call has no completion event and its conflicts
are unknown. The [container state](run5.container-state.json) gives exit
124 without OOM. The cap occurred before `result.json`, rank, cgroup peak
and per-phase summaries could be written. Do not treat the unfinished trial
as a negative relation, a refutation, or a zero-cost call.

The [diagram](run-diagram.svg) shows the two result paths. The m=4 and m=5
arms ask different decomposition questions. Their wall times are descriptive
and not a speed ratio. Both use a single retained base and one public
target corpus; neither supplies natural relation yield or useful rank beyond
these caps. The small four-summand uniform-target support ceiling for a
roughly 2,000-column base is a separate cardinality calculation in the
[prebuilt-base review](../n83_prebuilt_factor_base_20261016/REVIEW.md),
not an explanation proved by the capped SAT outcomes.

## Verification and limits

The source branch's release example tests passed 2/2 and both native import
checks passed. The original m=4 library source had a 4,216-test release-lib
pass before the later mainline merge. The current merged head has an
inherited library compilation obstruction documented in
`problems/current-main-library-build-20261017.md`; a current-head test pass
is pending that repair. The Linux query binary is tied to the earlier
source commit and was not rebuilt from the obstructed head.

This is a stage diagnostic. There is no full cold IC cost, A/A noise
control, same-point rho reference, external equal-output solver timing or
independent-host replay. No base-quality winner or pipeline improvement is
selected. The canonical scoreboard and progress charts have no new
admissible rank or end-to-end data and remain unchanged. The next two
measurements are a retained K=2,048 explicit-orbit m=4 comparator on the
same public corpus and a capacity/query gate for a prebuilt K=16,627 base,
including a degree-83 planted relation positive control.
