# Resume: native cubic cold benchmarks, 2026-11-05

## Goal
Run the actual native cubic elliptic-to-genus-three transfer benchmark on the
public p109 and p157 fixtures, preserving complete cold accounting, source rho
with and without negation, target IC/rho, A/A and actual L2 isolation.

## Lead / hypothesis
The full native transfer landed in PR #2893. Its correctness pilots do not
establish performance. The requested experiment measures the complete pipeline,
including construction, failed charts, inversion, recovery and source checks.
Tian arXiv:2012.07173 supplies the established cover construction; no new
mathematical or speed claim is made here.

## Spec
`research/cubic_extension_transfer_20261025/native_end_to_end/benchmark_20261105/launch-plan.json`
is an unfrozen proposal: p109/p157, eight fresh public targets per curve,
five measured rounds, five arms, one warmup per target/arm, one algorithm thread,
one proposed Spot L2 job and a proposed USD5 total ceiling. It is not a formal
experiment.protocol/v1 and does not authorize launch without budget admission.

## State
Branch `codex/cubic-benchmark-access-20261105` starts at
`369c1a2ad56cfd597313a5ef6a101616f234e15c` in
`/workspace/crypto-benchmark-20261105`. Preserve the separate old checkout
`/workspace/crypto-e2e` and its historical results.
The two read-only captures under `benchmark_20261105/preflight-attempt*/` record
access failures. Claim PCC-20261105-a6723f has unverified
standing. No scientific benchmark or cloud build was launched. [pcc:unverified]
Current daily/monthly balances and reservations are unknown. Native measurement
integration, strongest applicable source-reference review and protocol freeze
remain unfinished. Do not use system EKS nodes as alternate runners.

## Next steps
1. Resolve the pending request for an existing authorized IsoLab operator role
   or runner. Broad AWS keys are not stored in this checkout or checkpoint.
2. From this checkout, after the role is configured, run
   `research/cubic_extension_transfer_20261025/native_end_to_end/benchmark_20261105/capture-access.sh FRESH_DIRECTORY`.
   Preserve prior captures. Recheck actual budget balances, charged/reserved
   costs, live NodePool limits and namespace quota before paid work.
3. Finish the native per-execution measurement adapter and formal protocol,
   including A/A, source verification and failure preservation. Check the
   applicable reference and instrumentation scope; do not call partial target
   counts whole-pipeline GAE.
4. Push exact source/protocol/configuration, retain the check-experiment-git
   receipt, and run `bash scripts/checkpoint.sh gate`. Only after all launch
   gates pass, build one source-bound digest image and submit the immutable
   native custom-command request through the documented IsoLab route.
   Preserve the shared root resume: automatic review rejected replacing it.
   The task-specific backup alone does not establish a current launch gate;
   resolve the checkpoint workflow for this task before executing a benchmark.
5. Collect every raw result and actual L2/replay evidence. A successful Job or
   profile name alone does not admit performance results.

## Runs in flight
None. No scientific or build process is running for this benchmark request.

## Stop and success conditions
Stop paid launches when headroom is unknown or exhausted, or the finite total
ceiling cannot cover the attempt. Do not switch provider, raise pool caps or
substitute the default validation workload. Success requires the requested
complete experiment with checked answers and actual measurement admission;
honest failures remain part of the record.
