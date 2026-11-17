#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Full-cache queries: second large matched recovery
Run fullcache-20261101-v3-funded · RTX 4090 · 2026-11-01 UTC

#let data = csv("../../results-v3/ratios.csv").at(1)
#let baseline = float(data.at(5))
#let candidate = float(data.at(6))
#let repeat = float(data.at(7))
#let ratio = float(data.at(3))
#let spread = float(data.at(4))
The second original 63-bit public input (seed202) completed all three recovery
executions on one fresh RTX 4090. Every certificate passed fresh CPU replay and
scalar+1 rejection; certificates and relation/attempt counts matched across arms.
The candidate took #calc.round(candidate, digits: 3) seconds, compared with
#calc.round(repeat, digits: 3) and #calc.round(baseline, digits: 3) seconds for the
identical baseline executable before and after it. Candidate divided by baseline
mean was #(calc.round(ratio, digits: 6)). The observed cold-time reduction was
#(calc.round(100 * (1 - ratio), digits: 2))%. Baseline repeat spread was
#calc.round(100 * spread, digits: 3) percent.

#table(columns: 5, inset: 6pt,
 table.header([Run], [Arm], [Order], [Cold solver s], [Outcome]),
 [63-202-0-x], [Baseline repeat], [First], [#calc.round(repeat, digits: 3)], [Verified],
 [63-202-0-v], [Full-cache candidate], [Second], [#calc.round(candidate, digits: 3)], [Verified],
 [63-202-0-b], [Baseline], [Third], [#calc.round(baseline, digits: 3)], [Verified],
)

#image("../../results-v3/scaling.svg", width: 100%)
The plot contains the three new executions on pod k1mjc0sf1gpzvq. Cold solver
clocks include target construction, startup, curve validation, base construction,
relation queries, elimination, in-process replay, serialization and shutdown.
Additional fresh CPU replay is recorded separately. One round for one fixed
input; shared-host L0 descriptive clocks. No profiler was attached.

#pagebreak()
= Checks, history and reproducibility

== Frozen setup and checks
Version 3 was frozen before new data. It selects the original seed202 input in
x/v/b order and retains the exact GPU/driver code, resource limits, checks and
analysis formulas. The original worker was externally deleted; all three arms
were restarted on this host. No old-host clock is substituted or pooled.
The query specialization directly accesses the complete cached right table and
reuses target conversion. Original normalization, lookup and minimum-winner
semantics remain unchanged.

Each new recovery used 344 relations and 13,733 attempts.
All 29 driver tests and Clippy passed. Host and device arithmetic each passed
37,584 cases. The direct query differential passed 315 cases, and the backend
check covered full-cache refusal and fallback. Six sanitizer executions reported
zero errors/hazards. Service/driver self-tests and the PTX check for unwanted
profiling instructions passed before the recovery runs.

The original public fixture was regenerated and compared byte-for-byte. Each
successful recovery required a full right cache and nonempty filter. Tables had
549,596,520 left rows and 444,762,500 cached right rows. Conservative admission
was 14,920,341,256 bytes; this is an admission estimate, not measured peak memory.

== Preserved history
Version 1 failed preflight before any recovery execution. Version 2 completed
30 small recoveries and the three-arm first large comparison, then lost its pod
during the second input's baseline. That interrupted execution has no complete
clock; two further version 2 slots were never started. The first version 3
submission returned HTTP 402 before allocation and consumed no GPU time.

The new version 3 set has three planned, attempted and verified executions.
Together with version 2 there are 36 verified recoveries and one interrupted
execution across 37 actual recovery attempts. The two unstarted version 2 slots
remain recorded. Both original large inputs now have complete within-host
matched comparisons, on different hosts. The original whole-panel analyzer and
its incomplete-version 2 refusal remain unchanged.

== Locate and reproduce
Protocol: #raw("protocol-v3.json"), hash
#raw("ebffb8127816d9b5b58785cc391b6a7cf3882718bbb5c0bc33e8e92c6f60d34a").
Instrument: #raw("64901ebb2182790f0629ed637b507aea4da48bee"), clean source.
Command: #raw("bash research/explicit_relations_fullcache_20261101/run-v3.sh").
Source archive SHA256:
#raw("3424947660018b11d1b6bb1d601ad671b79b25f1aa29ccdbb2db95ae59273713").

Results and canonical export: #raw("results-v3/"). Analysis source:
#raw("analyze-v3.rs"). Hashes, raw archive location and every producer file are
in #raw("results-v3/archive-manifest.json"). Hydrate large logs with
#raw("python3 tools/artifacts/artifacts.py pull research/explicit_relations_fullcache_20261101/results-v3").

The single worker was bounded at 9,000 seconds and the controller at 9,300 seconds,
within the unused original budget. Request through verified cleanup took 3,780.073 seconds. The conservative
charge was 3,840 seconds, leaving 5,160 seconds of the original worker budget.
No GPU remains running. Budget and verified deletion receipts are retained
with the final results. This report is rendered only after successful
collection, replay and export checks, and after raw records are published.
