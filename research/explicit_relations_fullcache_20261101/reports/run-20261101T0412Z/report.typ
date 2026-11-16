#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Full-cache queries: verified retained comparisons
Run fullcache-20261101-v2 · RTX 4090 · 2026-11-01 UTC

The uninstrumented full-cache specialization completed 33 verified recoveries.
On the completed matched sets, candidate cold time / mean of the two baseline
controls was 0.83252 and 0.82569 for the two small inputs (median of five rounds),
and 0.89898 for the first large input (one round). The first large candidate took
2,329.971 seconds; its baselines took 2,591.626 and 2,591.934 seconds.

RunPod recorded deletion of the worker during the second large input's first
baseline. That attempt is interrupted; its candidate and remaining baseline
were never started. No complete timing or comparison is assigned to that input.

#table(columns: 6, inset: 5pt,
  table.header([Stratum], [Planned], [Attempted], [Verified], [Interrupted], [Not started]),
  [52-bit], [30], [30], [30], [0], [0],
  [63-bit], [6], [4], [3], [1], [2],
)

#image("../../retained-analysis-v2/scaling.svg", width: 100%)
The plot shows all 33 completed executions. The interrupted attempt has no
complete clock and is not plotted. Baseline b and control x are the same binary.
Candidate v differs only by the full-cache specialization. All clocks are L0
shared-host descriptions; the large input has one matched round.

#let stats = csv("../../retained-analysis-v2/statistics.csv")
#table(columns: 7, inset: 4pt,
  table.header([Bits / seed], [Rounds], [Baseline s], [Candidate s], [Repeat s], [C / mean(B,A)], [A/A max %]),
  ..stats.slice(1).map(r => (
    [#r.at(0) / #r.at(1)], [#r.at(2)],
    [#calc.round(float(r.at(3)), digits: 3)],
    [#calc.round(float(r.at(5)), digits: 3)],
    [#calc.round(float(r.at(7)), digits: 3)],
    [#calc.round(float(r.at(9)), digits: 5)],
    [#calc.round(100 * float(r.at(10)), digits: 3)],
  )).flatten(),
)
Per-arm columns are median cold solver seconds. The ratio is the median of
per-round candidate / mean(b,x); A/A is the maximum absolute baseline difference
divided by the pair mean, displayed as a percentage. Minima and every paired row remain in
statistics.csv and ratios.csv. Cold solver clocks include all setup, relation
queries, linear algebra and in-process replay; fresh external replay is recorded
separately.

#pagebreak()
= Implementation, validation and interruption custody

The opt-in kernel directly reads the complete cached right table and converts
the target outside its traversal loop. It retains the original normalization,
lookup and minimum-winner semantics. The generic kernel remains the fallback
when the complete cache is unavailable. There are no added query-stage clocks,
fences or barriers. CUDA compilation reported 40 registers and a 56-byte stack
frame for the specialization versus 48 registers and 120 bytes for the generic
kernel, with no spills in either.

== Checks actually completed
All 29 driver tests and Clippy passed. Host and device affine arithmetic each
passed 37,584 cases. The direct query differential passed 315 cases (261 hits,
54 misses), including tails, infinity, duplicate rows and minimum winners.
The public backend differential covered forced full-cache refusal and fallback.
All six memcheck/racecheck/synccheck executions reported zero errors or hazards.
Both service/driver self-tests and the candidate PTX instrumentation exclusion
passed before measurements.

Each of the 33 completed recoveries passed fresh CPU certificate replay and
rejection of a scalar+1 certificate. Certificates agree byte-for-byte across
all completed matched arms and rounds; relation and attempt counts also agree.
The completed large set used 338 relations and 15,737 attempts per arm.

== Analysis status and complete accounting
The original frozen analyzer is unchanged. It correctly rejected the incomplete
large stratum and produced no whole-panel result. The separately named
#raw("analyze-retained.rs") was filed after the interruption, unblinded. It
preserves the original formulas and completed-cell checks, but adapts coverage
to the retained sets. Its outputs are descriptive, not preregistered analysis
outputs. A second execution reproduced them byte-for-byte. The source and output
hashes are in #raw("retained-analysis-v2/analysis-custody.json").

Both exports pass schema and referenced-file checks. The small export also
passes protocol coverage. The large export deliberately retains four coverage
findings for the infrastructure shortfall: four actual attempts versus six
planned cells, missing second-instance attempts for b/v, and differing observed
instance sets. Its interrupted row has a null clock. The two unstarted cells
remain in the protocol and interruption record; no fake attempt rows were added.

== Reproduce and locate the evidence
Source: #raw("0fe875d034b77fc9b9825af8217194aa2f4ccdee"). Clean checkout;
pinned CUDA 13.2 image, Rust 1.88.0, one RTX 4090 with 24,564 MiB framebuffer.
Command: #raw("bash research/explicit_relations_fullcache_20261101/run.sh").
Original 52/63-bit public fixtures and seeds 101/202; protocols
#raw("protocol-small-v2.json") and #raw("protocol-v2.json") were frozen before data.

Pod #raw("dde6p5u9kqsrkg") was deleted at 06:38:03 UTC, before controller cleanup.
The initiating reason is not in the audit record. Controller exit 93 and verified
absence are retained. Raw records, canonical exports, compiler/sanitizer logs,
certificate files and interruption receipts are under #raw("results-v2/").
Hydrate policy-selected logs with #raw("python3 tools/artifacts/artifacts.py pull research/explicit_relations_fullcache_20261101/results-v2").
The archive manifest identifies the retained S3 partial archive and every file's
hash. Archive SHA-256:
#raw("210a446bf2801a2c5f8be6dfbaedd15a97a9d2cff184abd8a0cb6e1de9f69bf2").

Version 1's stale-lock preflight failure remains in its separate report. Charges
of 180 seconds for v1 and 8,820 seconds for v2 leave 9,000 seconds in the original
worker budget. No replacement is launched while the external deletion is being
clarified. The second large input's complete matched comparison remains open.
