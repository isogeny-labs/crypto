# Fresh-Q EC2 Tier 1 worker panel

This protocol was frozen before the first timed arm on the separate
`c7i.4xlarge` instance `i-0dff25746b9b36970` in `us-east-2c`. The
[worker plan](worker-plan.json) has SHA-256
`5ca936ef68c63384b7f7fc5a982322a970b837dd42e979f21b28dcd705c724fc`.
The 32-point [fixture manifest](fixture-manifest.json) has SHA-256
`546fe0b842a2f559395c9ac5d74ced12b96ce366e41322387eb0baec5dcb3adc`.
The public worker source bundle has SHA-256
`a99b5c752cc3bb1c2e02d83482f127b8c76894b8afb4100b16de6e667227e75c`;
the Linux benchmark executable has SHA-256
`f19422b3d6567dd7d6dfb069b48051e9bc5b6439d33ad98c6b63b1e72d0ff4b4`.

Each point has a serial-versus-batch comparison, a serial A/A control and a
batch A/A control, each repeated four times. There are 384 pairs and 768
arms. Each arm runs 32 walkers with 12 distinguished-point bits, an 8,192
step cap, and a check-in every eight walkers. The serial width is one and
the batch width is 32. The clock charges target-dependent setup and walking.
The [runner](run_tier1_worker_batch.py) records every arm, exact-output
fields, child peak RSS, CPU and memory cgroup events, affinity, steal time,
local interrupts, and the complete stdout/stderr. The
[auditor](analyze_tier1_worker_batch.py) independently replays the receipts
against the frozen plan and requires every A/A pair within [0.90, 1.11],
complete exact-output parity, no disqualifying host event or memory increase,
and a Q-cluster bootstrap lower 95% bound above 1.05 for an integration
decision.

The run was launched as `rho-worker-freshq.service` at 2026-10-19 20:09:11
UTC, with its service on CPU 0 and timed children on CPU 4. Its active output
directory is `/home/ec2-user/rho/results/b9a2ed25901949fa8a888bf97117c1ed`.
Its [raw archive](first-raw.tar.gz) and [independent audit](first-audit.json)
preserve the complete result.

## Prospective full-width confirmation

The first plan's check-in every eight walkers limits the active lane group to
eight, even though the requested batch width is 32. A second
[plan](worker-fullwidth-plan.json), SHA-256
`4a5e38e2f1ed29bce319b34f7bc521cd91b82c231ad6c2e2bad7b933bda3d5a4`,
was frozen before its first arm. It retains all 32 Q points, 32 walkers,
four repetitions, exact-output fields and charged interval, and changes the
check-in interval to 32 so the batch lane can hold all 32 walkers. It
started only after the first panel completed, preserving one timed job at a
time. The [service journal](service-journal.txt) records the ordering.

Its [independent auditor](analyze_fullwidth.py) requires zero invalid arms,
complete exact replay and no disqualifying host or memory event. The A/A
policy, specified before this panel, permits at most two of 256 controls
outside [0.90, 1.11], none outside [0.80, 1.25], both mode medians within
[0.98, 1.02], and each Q-by-mode median within [0.95, 1.05]. The worker
integration threshold remains a Q-cluster bootstrap lower 95% bound above
1.05. Failed gates retain the raw result and leave the qualified aggregate
unknown.

## Completed Tier 1 results

The first panel completed all 768 arms and 384 exact-output pairs, with no
invalid arm, invalid pair, structural issue or memory failure. One frozen
batch A/A control (`q06`, repetition 1) had reference/candidate ratio
1.437258 outside [0.90, 1.11]. Its decision is `unknown_gate_failed`.
The raw archive SHA-256 is
`cebef51fc31edf69946a3365f6c69a300c1a235f2bc1ecaf1458f1b2d1904cee`.

The [full-width raw archive](fullwidth-raw.tar.gz), SHA-256
`78260d94fe31a1f13766a3ac5011f9a7beece8a2c16c084bcdb19889664ed479`,
contains all 768 arms and 384 pairs. One reference arm of the `q12` batch
A/A control in repetition 3 recorded one CPU-steal tick; the runner marked
that arm and pair invalid. The other 383 pairs were valid, and no memory
gate failed. Its [independent audit](fullwidth-audit.json) reports
`unknown_gate_failed` under the frozen all-arms host gate. For diagnosis
only, the 32-Q comparison median reference/candidate ratio was 4.4912
with Q-cluster 95% interval [4.4069, 4.6637]. Its qualified interval is
null. The [post-run script](postrun-audit-and-archive.sh) checked both
terminal summaries and ran the auditors only after both timed services
stopped.

## Production check-in confirmation, frozen but not dispatched

The worker's ordinary `LaneOptions` checks in every 64 walkers. A separate
[128-walker plan](worker-production-plan.json), SHA-256
`3caf3cd9c5b4aab90a53bdbdc44c9fe7f9f72920c28f336004cce135c36fbcfc`,
and [independent auditor](analyze_production.py) preserve the same 32 Q
points, four repetitions, exact-output and host gates, and prospective A/A
policy while exercising 128 walkers with a check-in every 64. It is a
deployment gate if the 32-walker full-width result supports changing default
routing. It was frozen before any of its own arms and has not been launched;
the current worker default remains serial.
