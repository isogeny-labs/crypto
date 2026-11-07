# Post-run analysis readiness — no data analysis executed

Target run: pencil-p739-20261030-a1. This document and templates are preparation,
not observed output. No solver, replay trial or production aggregation was run.
Sources inspected: frozen protocolv5; native_price README and aggregate contract;
native_canonical_export README, lib.rs and output_contract.rs; pinned controller
cloud.rs verify_benchmark and computation export paths.

## Fetch and retain actual evidence

Retain actual terminal controller state, supervisor status, failure receipt if
present, CodeBuild start/end/status/logs, attempt Pod/instance identities and
intervals, checkpoint generation manifests and termination/fencing evidence.
The recovery prefix is recovery/runs/pencil-p739-20261030-a1/.
When present, computation-result.json binds computation.tar by key, version,
SHA256 and size. Download that exact version and verify bytes before extraction.
Computation archive contains /work/compute/session evidence; discover actual
member paths from the archive listing. Preserve the native replay record. These
computation clocks are always invalid, including a run with zero restores.

benchmark-result.json identifies the separate fresh benchmark epoch. Its payload
prefix is runs/{actual_benchmark_job}/{actual_pod_uid}/. Retain downloaded
SHA256SUMS and every listed file; validate safe relative names, exact file-set
completeness and each hash. The controller uses the maintained verifier after
these checks. Preserve its actual result, session identity, binary/source/build
identity, records.jsonl, original spec and plan, isolation/host metadata, native
verification and resource observations. Do not substitute controller-success
status for downloaded artifact verification. If archives or records are absent,
retain that absence; do not manufacture empty successful sessions.

## Separate computation, fresh measurements and prior failed attempt

Do not concatenate computation and fresh-benchmark records into one paired
session. The canonical measured input uses the actual fresh session only.
Computation records/verification are separate evidence and resource observations.
Retain the original failed run pencil-p739-20261029, its null PID/status,162.316s
controller interval and conservative180s nonrefundable reservation. Remaining
admission allowance14220 is not a measured resource total. Report allocated
lifetimes separately; no dollar/vCPU conversion or zero-cost inference.

Execution history is explicit per observed record_id: attempts>=1, resumed>=0,
nonempty hash-bound evidence, and checkpoint metadata with generation if resumed.
Use actual epoch/attempt evidence; do not label every fresh measured record as
attempts2 solely because an earlier infrastructure run failed before a PID.
Retain that earlier attempt in the resource sidecar. Unknown history remains
unknown and blocks canonical export; do not coerce it to zero restores.

## Fixed denominator and commands after evidence arrives

Use exactly gate-p739.json plan hashbc57819e43862f5d9c9aee0f13b983fec186b2dd14dbc8b921d8999acc4b6776.
All60 targets,1080 slots,180 warmups and300 measured pairs remain expected.
The analyzer receives records:null when the whole fresh session is absent; it
emits missing rows/null ratios. Never replace the denominator with survivors.
Duplicate/retried records require explicit retained attempt selection, not silent
last-row wins. Prior attempt resources remain attached in every report.

After replacing pending bindings with verified existing paths/hashes, native
commands are:

```
native_price/target/release/analyze_cohort analysis-manifest.json --p739-gate60 > analysis.json
native_price/target/release/analyze_cohort analysis-manifest.json --p739-gate60 --summary > summary.json
native_canonical_export/target/release/pencil-native-canonical-export package-request.json --out export.json
native_canonical_export/target/release/pencil-native-canonical-export check-export export.json --protocol FROZEN_PROTOCOL.json
```

Capture each exit status/stdout/stderr. Incomplete export may be written with a
nonzero semantic-shortfall exit; preserve it. Schema/history/identity failures
emit no export. Native canonical packaging performs source-linked deterministic
analysis replay, not a new solver trial. It exports wall_valid:false; downstream
review must assess cold/online A/A and actual L2 evidence. Cold field is
 time.method_call_wall_ns; online requires exact five phase names/sum and explicit
clean diagnostics. Never substitute solve_wall_ns or infer clock qualification.

## Concrete binding blockers discovered before aggregation

Frozenv5 analysis_build.receipt references generic bindings.json, but exporter
requires pencil-native-analysis-build/v1 with build_exit_code0,
source_files_match_committrue, source_revision, mapping_sha256, sources[] and
binaries[]. analysis_build.binaries omit required byte sizes. Separately,
finite_cost_model.analyzer_binaries_sha256 retains older binary hashes although
analysis_build points to gate-capable binaries. These are metadata blockers;
leave frozenv5 intact and make a transparent analysis-provenance amendment with
unchanged measured methods, seeds, data and estimands. No successful canonical
packaging claim is possible until corrected.

Independent finite coverage review remains a separate price-admission input.
Software scope review is not a completed nine-category coverage review. An
absent review must remain null; native prices stay null with explicit blockers.
