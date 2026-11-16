# Native N83 SAT boundary diagnostic

Frozen before execution. This continues the free-selector known-witness
problem after the retained K2048/K4096 one-query, 1,000-conflict diagnostics.
It is an L0 observability diagnostic, not a performance comparison or a
natural relation search. No cold-runtime winner, external-tool comparison,
rho ratio, asymptotic or key-recovery claim follows.

Question: does the original fixed-coordinate/free-selector control spend
its budget before or after initial level-zero propagation? Add opt-in
`N83_SAT_PHASE_TRACE=1` at solve entry/return, initial propagation begin/end,
CDCL entry, first conflict and each 1,000th conflict. Snapshots export
existing cumulative clause/XOR propagation, analysis, minimization and
reduction timers and counters. They are cumulative per solver instance;
elapsed milliseconds restart per solve call. A killed worker's snapshot is
not a terminal count. Initial propagation conflict returns UNSAT without
CDCL entry; a previously inconsistent solver can return without propagation.
Instrumentation changes neither clauses, assumptions, branch order nor caps.

Use the exact prior domains/fixture (SHA256
fc203db28eed0164e47b0c755c3d9a09f80df1acd5e4c7e09dfe172fa5f0a538),
primary a=0 polynomial x^83+x^45+x^2+x+1 curve,
subgroup order 2417851639230796216685689. Both controls retain five summands,
1182/866 representative domains, eight fixed x-coordinate blocks, original
order and 5,000 conflicts / 180 seconds wall each. Control A additionally
pins representative/phase units and must return its independently checked
model. Control B is the original free-selector worker, unchanged source;
only the linked solver has trace instrumentation. No selector pin is added
to B. Controls are distinct workloads; A does not discharge B's search gate.

Run A then B once each; no automatic retry. Additional container allocation
is 360 seconds, separate from all prior allocations and from compilation,
tests, audit, rendering and storage overhead. Freeze clean source commit,
exact native ELF workers/runner, protocol hash, full public inputs and
immutable image sha256:541096ae9b05d3b850341b78790e2ed31e94d5aa0ea086933c6ff0c09ffa2e63.
Use native `freeze` for A and `freeze-free-selector` for B, columns=0,
conflicts=5000, wall=180. Native run/card/check only; no Python harness.
Director compiles strict cards, primary relays the native prompt verbatim
to mechanical execution, director assesses the exact saved relay before
HEAD changes. Each run command timeout is 240 seconds; scoped output under
the already ignored `../native-phase-20261019-v2/exec/v4/` is fresh/write-once.
Bind every argv file and original input/binary/image identity. A is a gate
for B; B's expected censoring is recorded as a failed positive control, not
converted into success. Accept runner exit1 for B's completed censored
diagnostic only after native audit, cleanup and trace validation exist.

Guarded Docker: one CPU quota, 4 GiB RAM, zero swap, network none, 64 PIDs,
task UID, read-only root/source/input/worker, all capabilities dropped,
no-new-privileges, 64 MiB tmpfs; independent inner timeout and outer
supervision, final inspection, log custody, verified cleanup. No concurrent
builds/tests or other task workloads during these observations. Host is
not reserved; timings remain descriptive L0, n=1 each.

Audit all trace records as a valid single-call sequence or killed prefix,
with monotone timestamps/conflicts and terminal counters only at return.
No CDCL marker localizes the censoring to initial propagation; CDCL marker
localizes it after that boundary, without proving which subphase dominated.
Returned capped UNKNOWN requires matching native terminal counters; UNSAT,
invalid models, OOM and producer failures remain failures. If B returns SAT,
preserve it but fail closed pending independent decoded-selector replay
(the native auditor currently supports the fully pinned variant only).
Exported timers do not account for an in-progress call until it returns.
Retain every failure, raw trace and source; forge-order/time/counter tests
must fail. Full permitted serial release library and touched-example tests
must pass before pushing. Report plus diagram/graph and inspected PDF live
beside records, with benchmark copies. Existing full-cold scoreboards stay
unchanged unless this diagnostic establishes a new admitted cold result.
