// Bind this presentation template to a completed local packet, without copying data.
#let render(s, flow, p, plot) = {
 set page(paper: "a4", margin: 15mm)
 set text(font: "DejaVu Sans", size: 9pt)
 set par(justify: true)
 heading(level: 1, "Sparse quotient-graph extraction: p" + str(p))
 [16 November 2026 · Native Rust · Public known-answer cubic-extension systems]
 parbreak()
 [Source #raw(s.source_revision.slice(0, 9)). Retained
 #s.retained_execution_batches of #s.planned_execution_batches planned batches;
 #s.attempted_execution_batches starts and
 #s.in_flight_batches_without_completion unfinished started batches.
 Invalid completed batches: #s.invalid_or_capped_batches. One process attempt,
 zero retries. [pcc:unverified]]
 plot
 [Both arms compute complete native F4 bases, finite quotients and coordinate
 triples, including empty sets. All basis/border/matrix construction, failed
 sparse attempts and dense fallback are inside the timed phase. Shared symbolic
 preparation, public residual generation and dense qualification are outside
 the phase and retained separately. This is a coordinate-engine comparison.]
 parbreak()
 let cases = s.cases.filter(r => r.p == p)
 let fmt(v) = if v == none { "Missing" } else { str(calc.round(v, digits: 3)) }
 table(columns: (0.8fr, 0.8fr, 1.3fr, 0.9fr, 1fr), inset: 3pt,
 [seed / r], [Krylov / dense], [95% interval], [Local A/A], [Fast / dense fallback],
 ..cases.map(r => {
  let metric = r.complete_krylov_over_dense
  let work = r.first_measured_krylov
  (
   [#r.seed / #r.ordinal],
   [#if metric == none { [Missing] } else { fmt(metric.geometric_mean) }],
   [#if metric == none { [Missing] } else { [#fmt(metric.bootstrap_95.at(0))–#fmt(metric.bootstrap_95.at(1))] }],
   [#if r.eligible_local_controlled_comparison { [Pass] } else { [Rejected] }],
   [#if work == none { [Missing] } else { [#work.at(3).krylov.completed / #work.at(3).krylov.dense_fallbacks] }],
  )
 }).flatten())
 parbreak()
 [Ratios are geometric means with deterministic 10000 paired-round bootstrap
 intervals on fixed systems, not independent-curve inference. Each batch must
 return exactly the dense qualifier's canonical triples; both A/A signatures,
 intra-batch consistency and the frozen reciprocal1.10 timing tolerance are
 required. Rejected/missing cases remain diagnostic. [pcc:unverified]]
 parbreak()
 [Method: Faugere–Mou, #link("https://arxiv.org/pdf/1304.1238")[Sparse FGLM algorithms],
 section3.1, Theorem3.1 and Algorithm2. Weighted transpose-graph walks supply
 moments; full-degree BM, nonzero constant coefficient, exact vector recurrence,
 checked inverse and Hankel coordinate equations guard the fast path. Failed
 checks invoke the charged dense extractor. [pcc:unverified]]
 parbreak()
 [Host: local AMD EPYC9V74, CPU0, one pinned CPU, 4CPU quota, 8GiB virtual-memory
 cap and 900second process ceiling. Native operation counters remain partial;
 cold wall is the comparison unit. Strongest external tools and independent
 signed replay/recompute are absent. No admitted benchmark/leaderboard result,
 standard lab admission or canonical encrypted-schema validation is asserted.
 Raw/copied binaries/checkpoints stay in the authorized local custody location
 and hash recorded in #raw("local-evidence.json"). [pcc:unverified]]
}
