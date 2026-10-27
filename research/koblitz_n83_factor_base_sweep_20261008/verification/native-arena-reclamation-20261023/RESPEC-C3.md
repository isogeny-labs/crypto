# Correct the assessed container-launch failure without extending search time

This supplement preserves PROTOCOL.md, CONTINUATION.md and RESPEC-C2.md. Root
explicitly authorizes one separately frozen K2048-c3 correction after the c2
assessment and infrastructure validation, under the user's existing instruction
to continue. It grants no automatic retry, extra search or transferred allowance.
K4096-c1 remains the original separately assessed continuation workload.

K2048-c2 at de389b7ad4f6fb8a911a1e31d0c3d8ff0626d6c0 ran one command,
one attempt, exit1, no card timeout. Its Docker detached-launch RPC hit the
original15-second limit (nested exit124). RESOURCE_READY was absent, no worker
search was released, and no search result or memory profile exists. Native
container removal and runtime cleanup pass; the director verifies custody
and classifies impeded launch, not completed search. See the separate
run-20261023T0510Z report and launch-correction/c2-director-assessment.json.

Recorded whole-command29.838953191seconds is conservatively charged as30
seconds inside the prior8987 allowance. Native accounting verifies the original
13-second receipt, single stopped c2 receipt, exact config, launch error,
missing release/search artifacts and complete cleanup, then emits:
**13 +30 =43 charged; 9000 -43 =8957 remaining**. A negative control with a
release marker is rejected without writing a budget. No new allowance is granted.
Total remains6000+6000+43+8957+9000=30000worker seconds. This now comprises
four workloads and up to six cards, including two assessed pre-search failures.
No further retry is included in this supplement. Unused time does not carry over.

The corrected K2048-c3 pair is exactly1.5Mconflicts/8957worker seconds,
command ceiling10157seconds (including queue<=600 and existing setup/cleanup
allowance). K4096-c1 remains1.5M/9000,command10200. The worker wrapper and
runner admit only those exact additional natural panel pairs; stale8987 K2048
is rejected by the new binaries. Frozen older binaries/cards remain untouched.
SAT/arena, library, exporter entry point and dependencies are unchanged.

Infrastructure correction: only the single Docker `run` RPC may now wait up to
60seconds inside the existing command envelope. Inspection already allowed60;
other Docker operations retain15seconds. No mutation is retried. Existing
startup barrier prevents worker release before memory/swap policy is verified.
Docker25.0.16/overlay2 health succeeds with no surviving task container. Host
I/O pressure was elevated when diagnosed (CPU pressure alone had passed); this
is an observation, not a proved cause. No daemon restart or peer-job stop was
performed. Require both CPU and I/O PSI some avg10<=5 before compilation and
dispatch; preserve refusals and wait for readiness rather than retry a worker.

New worker native-arena-admission-v3 SHA256
`8ab6f9cf7abe886f33e497bc052089f32fdbe5270b8eee2218242040fa2cc6b4`;
runner-arena-continuation-v5 SHA256
`efa7c80519a547cb13843ae84d777a269419cd668851af2d87fa0558ae6815a5`.
All21exporter,44runner and7native-card tests pass, including actual worker CLI
admission, adjacent/stale budget rejection and cumulative fail-closed accounting.
Native noncryptographic barrier smoke passes at the pinned image,8GiB/swap0,
core14: held before release, scope synchronization, zero swap after resource
reapplication, exit0 and cleanup. Prior unchanged-source library/model/CLI/panel
passes remain applicable. New source/build/test/smoke/accounting raw evidence
is published before timing; receipts are indexed in PUBLICATION.md.

Both new cards must bind this supplement, prior protocols/receipts, c2 assessment,
accepted1M evidence and exact new recipe/binaries. Original public query/inputs,
image, CPU reservation14/30,worker14,one CPU quota,8GiB/swap0,drift gate,lock,
cleanup,full native audit/memory/storage validation and matched original1M
search-prefix checks remain required. Later counters stay unpaired. No own
build/test/compression overlaps workers. Positive results require independent
verification; capUnknown implies neither UNSAT nor a base winner or rho gain.
No c3 or K4096 continuation has run at this supplement's freeze.
