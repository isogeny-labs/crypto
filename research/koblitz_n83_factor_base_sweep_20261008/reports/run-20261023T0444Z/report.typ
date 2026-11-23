#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Sans",size:10pt)
= N83 K2048-c1: pre-search admission refusal
Run2026-10-23T04:44Z. Frozen source a591f1daa3f26628ac1dadb0561acd366ef61ca2.
One public N83 K2048 orbit-chain workload, m5, seed2026100801, one target;
requested1.5M conflicts /9000worker seconds,8GiB/swap0,one CPU quota,
core14/30 worker14. Original image, fixtures and panel; command ceiling10200,
lock queue600. See REPORT.md and the immutable raw card/spec/source archive.

#image("admission-flow.svg",width:100%)

The command ran once, exit1, no timeout. Whole-command12.87025458seconds is
recorded diagnostic accounting, n=1,no uncertainty; algorithm time is missing.
Worker CLI rejected its requested budget above7200seconds before SAT. Stdout
and memory-profile are empty and no run directory exists. Runner subsequently
reports an empty memory profile. Docker OOM=false; outer and systemd runtime
cleanup pass. The director verifies argv, relay and custody and recommends
respecification. It does not interpret a solver result. No search result exists.

== Correction within the original allowance
Native accounting binds the original receipt/config/outer/stderr and rounds
the entire refused command upward to13seconds. The same original K2048 slot
has8987seconds left. K4096 retains9000. The four-workload ceiling is unchanged:
6000 +6000 +13 +8987 +9000 =30000worker seconds. No transfer, automatic retry,
or new allowance. K2048-c2 requires a separately frozen and verified card.

Worker admission now accepts only the exact additional natural-orbit m5,
unordered pairs (K2048,1500000,8987) and (K4096,1500000,9000). Legacy admission
and SAT/arena code are unchanged. Validation:21exporter,43guard and7card tests
pass, including the actual CLI path and fail-closed refusal accounting.
The new worker hash is0cfb0d19 and guard hash e11f22b0; full identities and raw
validation streams are in verification/native-arena-reclamation-20261023.

== Limits and evidence
The corrected continuations have not run at this report freeze. They still
require native audit, full matched1Msearch-prefix comparison, all storage and
memory checks, and independent custody assessment. Prior accepted1Mresults
are preserved separately. No UNSAT, base selection, minimum-memory, speed,
fastest, recovery or rho claim. No comparable algorithm values exist; the
figure is a gate diagram. Canonical graphs listed in REPORT.md remain
unchanged. No eligible leaderboard or benchmark row follows from this failure.

Sources: REPORT.md; RESPEC-C2.md; admission-correction/budget.json;
admission-correction/c1-director-assessment.json. Raw archive and upload receipts
are indexed through store/index.json in setec-astronomy-artifacts.
