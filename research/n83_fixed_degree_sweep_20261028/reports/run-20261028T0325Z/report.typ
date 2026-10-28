#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
#let rows = json("../../evidence/observations.json")
#let protocol = json("../../protocol.json")
#let terminal = json("../../terminal.json")
#let complete = rows.filter(r => r.status == "complete").len()
#let time = rows.filter(r => r.diagnostic_reason == "time_limit").len()
#let memory = rows.filter(r => r.diagnostic_reason == "memory_limit").len()
#let finite = rows.filter(r => r.degree_of_regularity_top != none).len()
#let result_label(r) = if r.status == "complete" { str(r.basis_max_degree) } else if r.exit_code == 14 { "memory" } else if r.status == "resource_limit" { "time" } else { r.status }
= N83 regularity with fixed factor bases
#text(fill: rgb("287d9c"))[ell = 27 · m = 2–9 · Koblitz, binary and prime · 28 October 2026]

Observations only. This is a bounded S3 decomposition-system experiment, not a review verdict or an end-to-end index-calculus result.

All #rows.len() attempts ran: #complete completed computations, #time process time limits and #memory memory-limit exits. The number of completed finite top-form regularity values is #finite. A resource limit supplies no degree bound.

#image("degree-grid.svg", width: 100%)

== The fixed factor bases
Each curve reuses exactly one saved ell27 base for all eight m values. Every input pins the manifest hash; every planted point was found by full (x,y) lookup in a hash-verified stored shard. The three manifests are in ../../evidence/factor-base-index.json. The original balanced-size run is separate historical evidence.

There is one planted target and one independently lifted natural target per cell. The S3-chain equations encode the candidate x-domain with27 Boolean selectors per factor coordinate; they do not encode the entire liftable-point lookup set. Any recovered relation needs rational lifts and consistent signs. Full affine curves are used without subgroup filtering.

== What was measured
Top-form regularity means one plus the last nonzero Hilbert degree of the quotient by the highest homogeneous components, only when zero-dimensional. Maximum final affine basis degree is a separate output. Intermediate solving degree and first fall are not measured here. Each input is processed in both modes.

#pagebreak()
== Completed affine basis degrees and limits
P/N denotes planted/natural. A zero basis degree denotes the unit ideal for the completed inconsistent natural cases. It is not a regularity value.

#table(columns: (auto, 1fr, 1fr, 1fr), inset: 5pt,
 [m], [Koblitz P / N], [Binary P / N], [Prime P / N],
 ..range(2,10).map(m => (
   [#m],
   ..("koblitz","binary","prime").map(f => {
     let pair = ("planted","natural").map(t => result_label(rows.find(r => r.m == m and r.family == f and r.target_kind == t and r.mode == "affine")))
     [#pair.join(" / ")]
   })
 )).flatten())

== Controls, execution and provenance
Five native tests passed, including field arithmetic, curve/S3 consistency and stored-record positive/negative lookups. Eight exact small controls completed through the same Singular driver. The producer checks completed affine bases by input reduction and the planted witness. No independent scientific replay certificate is claimed.

Runtime: Singular4.3.1 in the pinned container on the existing Spot workspace; four workers, each limited to3GiB virtual memory and40 seconds plus5 seconds kill grace per solver. Input expansion and witness checking are inside that limit. Container ceiling:4CPUs,16GiB,1800 seconds. No automatic retries. Host uploads preserve completed attempts every30 seconds. EKS worker storage authorization was unavailable in the preceding run; no permission or quota change was made.

Recorded start: #terminal.started_at; finish: #terminal.finished_at. Wall times are diagnostic and are not used for a performance claim. Source, mounted binary, runtime image and actual container limits are recorded separately.

#text(size: 8pt)[Protocol SHA-256: #protocol.freeze.protocol_sha256\ Source revision: #protocol.arms.at(0).instrument.commit]

== Scope, evidence and remaining gap
The prior algebraic derivation for the prime top-form presentation predicts positive dimension for m≥3 independently of ell. This is distinct from a completed solver observation and does not resolve affine solving degree. See the historical ALGEBRA.md and the current problem record.

Raw data, logs and the reproducible archive are described in ../../evidence/storage.json. The unified ../../experiment.export.json records all96 outcomes without excluding failures. ../../validation/ contains software and file checks. The editable figure generator is ../../src/bin/report.rs; counts and cells come from saved result rows.

Primary method source: #link("https://arxiv.org/html/1503.08001")[Kosters and Yeo, Notes on summation polynomials]. Singular definitions follow the prior checked manual and same-version exact controls; a current manual fetch was unavailable. This report does not replace unresolved regularity values with affine-basis or first-fall values.
