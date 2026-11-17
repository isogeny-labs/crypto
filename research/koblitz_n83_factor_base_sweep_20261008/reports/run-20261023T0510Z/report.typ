#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Sans",size:10pt)
= N83 K2048-c2: Docker launch impeded
Run 23 October 2026, about 05:10 UTC. Source de389b7ad4f6fb8a911a1e31d0c3d8ff0626d6c0.
Public N83 K2048 orbit chain, m5, seed 2026100801, one target. Requested 1.5M conflicts,
8987 worker seconds, command ceiling 10187, lock queue at most 600 seconds.
Original image and inputs; 8 GiB/swap 0, one CPU quota, core reservation 14/30, worker 14.

#image("launch-flow.svg",width:100%)

One command ran once. Whole-command 29.838953191 seconds is diagnostic accounting,
n=1, no uncertainty estimate; algorithm time is missing. The Docker launch RPC hit
its 15-second limit (nested exit 124). Command exit 1, no card timeout. The worker
was never released: RESOURCE_READY absent; no run or memory-profile exists.
Native container removal, cleanup and systemd runtime cleanup pass. The director
verifies exact argv, frozen binds, source, receipt and verbatim relay, and classifies
impeded launch. No search result is admitted.

== Charge the failure inside the original allowance
Native fail-closed accounting rounds the entire command upward to 30 seconds.
Prior charge 13 + additional 30 = 43; original 9000 - 43 = 8957 seconds remain.
K4096 retains 9000. Total: 6000 + 6000 + 43 + 8957 + 9000 = 30000 worker seconds.
No transfer, new allowance or automatic retry. A negative release-marker control
rejects accounting without creating an output budget.

== Validated readiness correction
Docker health reports 25.0.16, overlay2, no surviving task containers. I/O pressure
was elevated during diagnosis, not a proved cause. No daemon restart or peer stop.
The single Docker run RPC may wait 60 seconds inside the existing command envelope,
never retried; other mutation limits remain 15. Both CPU and I/O PSI some avg10
must be at most 5 before compile/dispatch. Worker time is not extended.

All 21 exporter, 44 runner and 7 native-card tests pass. The noncryptographic native
barrier smoke verifies hold-before-release, scope synchronization, swap zero after
resource reapplication, worker exit 0 and cleanup on the pinned image/core 14.
SAT/arena/library/dependencies are unchanged. New exact pairs: K2048 (1.5M,8957)
and K4096 (1.5M,9000). The corrected searches have not run at this report freeze.

== Evidence and limits
REPORT.md links the raw-backed native budget, director assessment and RESPEC-C3.md.
The full failure/card/source/binds/streams and correction tests/smoke are uploaded
through store/index.json to setec-astronomy-artifacts. Diagram used because no
comparable algorithm measurements exist. All-page PDF inspection is recorded.
Canonical graphs listed in REPORT.md are unchanged. No algorithm speed, UNSAT,
base winner, minimum memory, recovery or rho result follows from this failed launch.
