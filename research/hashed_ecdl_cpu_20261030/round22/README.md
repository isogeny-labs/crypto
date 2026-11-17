# Round22 complete CPU stage diagnostic panel

Frozen and native-validated, not executed. No launch is authorized by this directory.
Protocol hash: 88c80abe26f704055292a5b8bfade5baa91a7f86167f4f4a5222d1817fafc1de.
The question is which instrumented stages contribute to complete recovery cost,
with ordinary controls exposing profiler overhead. This is descriptive L0
candidate-selection work, not isolated tenfold acceptance.

## Arms and workload

u/x retain the original baseline binary at 2bd4588fba603ba1787e1d8ad82f084ef461d7cf.
v/w use the candidate built from 972d0d2fed4d1a40ec1a0b4bd2b504e76ec98942,
ordinary batch64/batch1. y/z use the exact same candidate executable with
--profile-stages at batch64/batch1. Legacy bundle basenames previous/inverse
are byte-identical copies of candidate, not different algorithm revisions.
y/v and z/w isolate the added clocks; they do not measure a solver improvement.

All three round21 fixtures, seeds111/222/333, limits16 trials/1,000,000 work,
one software thread, one warmup and nine measured repetitions per cell remain.
Six arms produce180 executions:18 warmups and162 measured. The runner retains
the same interleaved/reversed schedule. Independent replay, corrupted-scalar
rejection and byte-identical full certificates are mandatory across every arm.

## Accounting and analysis

The ordinary complete interval and original baseline remain present.
Profiled rows use L0-profile; ordinary rows use L0. The analyzer accepts each
only in its declared arm. All60 stage rows must have matching identities and
complete/external-replay values. Missing rows, extra stage lines, NaN or negative
values fail validation. Stage summaries retain min/median/max, n9.

Targets includes fixed-pair setup, target multiplication and accepted relation
checking. Linear includes matrix construction, dependency and scalar extraction.
Internal replay is distinct from the mandatory external replay. Stage clocks
are not an exhaustive partition. Do not sum their medians or use profiled
latency as uninstrumented speed evidence. No historical timing rows are pooled.

The native exporter supports round22, attaches stage artifacts and counters
only to profiled rows, and retains wall_valid=false with a profiler blocker.
The report follows raw publication if an authorized attempt produces data.

## Preparation and custody

The candidate source was archived from the pinned commit, with standalone CPU
and tensor crates, shared hashed_ecdl.rs, and three archived certificate fixtures.
candidate-source.sha256 binds every archived file. Candidate builds/tests use
offline locked release Cargo and the standalone manifest; no root/CUDA build.
The original baseline was copied from the verified round21 bundle. Analyzer
is compiled with rustc -O from this directory. Artifacts are under
/home/ec2-user/hashed-ecdl-cpu-round22. All three candidate materializations name
the same source/binary hashes. The freeze helper uses the established native
canonical protocol library; protocol-check.json retains its validation result.

The runner requires EPHEMERAL_OUT and performs no cloud submission. A future
single attempt would request2vCPUs/4GiB/300seconds/zero retries and preserve the
one-node cap. Any additional attempt or autoscaling needs bounded admission;
round21's consumed authorization is not reused.

## Validation and handoff

46 pinned candidate library tests pass. Native synthetic tests verify complete
coverage, certificate equality, missing/corrupt stage rejection, export counters
and no overwrites. Synthetic records are software tests, not study observations.
Strict native Clippy, formatting and shell syntax are checked.

Initial test invocation lacked PANEL_STUDY/ANALYZER and failed before analyzer
execution; native-tests-missing-env.log preserves it. Initial protocol metadata
used boolean flags and a non-schema provenance value; corrected to string flags
and retrieved provenance before native validation. No affected data existed.

The source review and prior evidence are listed in protocol literature:
stage-profile/README.md, ReductionStats/solve_reduction and round21 summaries.
No new arithmetic or novelty claim is made. Next: obtain explicit bounded run
admission before creating any job. No input upload or job submission occurred.

The integration head 972d0d2fed4d incorporates the earlier registry-policy repair.
Its root check and Clippy jobs subsequently failed on mismatched delimiters in
examples/koblitz_orbit_dlp_fast.rs. The standalone CPU and native-panel builds
pass; that unrelated integration failure is not represented as a passing gate.
