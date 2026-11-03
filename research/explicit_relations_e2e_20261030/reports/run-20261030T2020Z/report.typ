#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= V16: completed work reported, archive lost
The worker console reported six 52-bit completions and one 63-bit completion.
The pod disappeared before the final archive was uploaded. These are retained
console observations; certificates could not be collected or independently
audited here. No audited recovery rows are manufactured from these messages.

#image("stages.svg",width:100%)

== Recorded work
All eight query variants completed memcheck/racecheck/synccheck preflight;
host arithmetic/filter checks and 26 Rust tests passed in the console.
The six 52-bit CELL messages reported total seconds:
#table(columns:(1fr,1fr,1fr,1fr),[Seed],[Baseline b],[Filter u],[Repeat x],
[101],[14.922],[4.904],[4.906],[202],[17.726],[5.454],[5.453])
The 63-bit seed101 message reported 1148.950 seconds, oracle1146618.757783ms
and linear2025.645493ms. Seed202 outcome is unknown. These rounded values
are copied from console messages, not reconstructed rows. Protocol clocks
are L0, wall_valid=false. No statistical timing claim is made. The diagram
shows the interruption; a recovery plot is omitted because its certificate
and row archive is missing.

== Failure and custody
Provider GET returned404; cleanup DELETE also returned404. A fresh complete
inventory contained no task pods. Controller exit22. Worker exit, final GPU
telemetry, TABLE/cache sizes, certificates and final archive hash are unavailable.
Cause of disappearance is unknown. S3 listing showed only source-checkout.tgz.

Pod `omb2jpo4ntoq5g` started2026-10-30T20:20:06UTC inUS-NC-1 atUSD2.49/hour
(provider rate, not final bill), using one RTX PRO6000BlackwellServerEdition.
Source: `23dc8a61ca928e47237f8fb5a3a8d87228649628`.
Protocol SHA256:
#text(size:8pt)[`fe424abdd265c0ded0c191edf1c36810d86a87163502a1497c2da43ffece378f`].
Command: `bash research/explicit_relations_e2e_20261030/run-filter.sh`.
m8; seeds101/202; rows2^32, work2^34, chunk2^20, columns16384, trials16;
52/63-bit solve caps180/7200s, replay60, worker18000/controller18300.

Raw records were pushed in `43d32aa4c087` before this report.
`interrupted-v16/interruption.json` and its hash-store manifest bind the exact
console bytes, receipts and scripts. The full SSE remains under
`validation/ecbench-explicit-x/e2e-20261030-v16/logs-interrupted.sse` in the
approved artifact bucket. V16's one-attempt execution is closed.

== Collection improvement
V17 keeps its frozen source, targets and bounds. A dated collection addendum
adds partial artifact uploads every60s and reconciles failed status reads
against complete inventories. Copies are evidence backups, not restored
solver trials. Host I/O is additional background work; all arms retain L0
clocks. Final certificates still require replay. V17 compares8,10,12slots
to reduce relation counts.
