# Instruction-to-solve cost-label admission

`crypto dataset cost-labels --out RUN` replays the run's native manifests and
scalar certificates, then prints one cost decision per completed cell. The
current checked PMU collector counts only a calling-thread phase. Its own
fixed policy declares whole-worker coverage and hardware independence false.
Consequently these local runs produce `instruction_to_solve: null`; phase
telemetry artifacts remain counted as diagnostics, not added or scaled into a
solve cost. `phase_artifact_count` counts both start and metrics JSON files;
it is neither a phase count nor a count of independent PMU reads. The output
schema is `crypto.ml-dataset.cost-label-coverage/v2`; saved v1 outputs remain
historical artifacts.

The admission checker requires an independently replayed complete solve and
scalar, a trusted whole-worker hardware-instructions capture, matching kernel
event ID at open/read, nonzero equal enabled/running times, completed readback,
and accounted worker and descendant lifetimes with no escaped descendant. A
future collector must establish the trusted replay input outside producer
JSON. Before admission, the independently derived expected binding must match
the counter's exact run digest, cell index, worker identity, and full solve
interval start/end; an absent, mismatched, or empty/invalid binding is rejected.
The expected binding must come from verified run/worker provenance rather than
being copied from the counter artifact. Its producer-consistency check alone
cannot establish hardware truth.
Multiplexed values are estimates, and missing or invalid evidence remains a
null label with a reason. No current artifact supplies the trusted whole-worker
proof, so this command does not admit one by interpreting a producer flag.

For the 48 local cells, run the command separately against
`/home/ec2-user/ecdlp/n24-n25-cross-family-20261101` and
`/home/ec2-user/ecdlp/n26-n31-cross-family-20261101`. Each output includes
per-cell status, phase artifact count, cost decision, and aggregate reasons.
The n24–25 native run's strict experiment-export mapping was rejected, so its
cost coverage is an integrity view only. This command performs local replay;
it starts no cryptographic trial or cloud job and writes no S3 objects.

Sources checked: the repository's `telemetry/checked/policy.json`,
`telemetry/checked/native.rs`, and `DESIGN.md`; Linux
[`perf_event_open(2)`](https://man7.org/linux/man-pages/man2/perf_event_open.2.html)
documents event IDs, `time_enabled`/`time_running`, inheritance and multiplexed
estimates. The kernel's [perf ring-buffer guide](https://www.kernel.org/doc/html/latest/userspace-api/perf_ring_buffer.html)
describes task versus per-CPU event scopes. Neither source makes a
calling-thread event a complete preexisting-thread/descendant worker count.
