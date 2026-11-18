#import "../local-report-template.typ": render
#let s = json("summary.json")
#let plan = json("runtime-plan.json")
#let fmt(x) = str(calc.round(x, digits: 3))
#for (index, oracle) in ("SymmetricPairOnly", "PreparedSymmetricPairOnly").enumerate() {
  let subset = s + (strata: s.strata.filter(row => row.oracle == oracle),)
  render(summary: subset, plan: plan,
    figure: image("ratios-" + oracle + ".svg", width: 100%),
    title: if index == 0 { "Epoch control: reference S3" } else { "Epoch control: prepared S3" },
    run: "local-prepared-short-20261115-06", duration: "23.191",
    findings: [All streams and scalars match across arithmetic implementations.
    Preparation and stack polynomial work are charged. Rejected controls stay
    grey and cannot support acceptance. No tested graph interval reaches rho.
    The final page separates direct prepared/reference timing from the
    decomposable-abscissa mechanism counters. [pcc:unverified]])
  pagebreak()
}
#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 9.5pt)
= Arithmetic comparison and mechanism counters
#raw("local-prepared-short-20261115-06") · Source #raw(s.source_revision.slice(0, 9)) · Sampler #raw(s.sampler)

Direct paired graph-wall comparison on identical inputs. Ratios below 1 cost
less; an accepted timing comparison requires both oracles' local controls.
These extra summaries were added after the calls to describe the predeclared
controls. Original acceptance strata compare identically with the first
analysis and are retained in original-summary.json. [pcc:unverified]

#table(columns: (1fr, 1fr, 1.4fr, 1fr), inset: 4pt,
 [p / seed], [Prepared / reference], [95% interval], [Both A/A],
 ..s.oracle_comparisons.map(r => (
   [#r.p / #r.seed], [#fmt(r.candidate_graph_wall_over_reference.geometric_mean)],
   [#fmt(r.candidate_graph_wall_over_reference.bootstrap_95.at(0))–#fmt(r.candidate_graph_wall_over_reference.bootstrap_95.at(1))],
   [#if r.eligible_local_controlled_comparison { [Pass] } else { [Rejected] }],
 )).flatten())

== Repeated decomposable abscissae
The first measured reference-oracle graph row per target supplies these
deterministic counters. The paired prepared stream is identical. A repeated
abscissa identifies a repeated point up to sign; the counter stores no
coefficients and never supplies recovery equations. [pcc:unverified]

#table(columns: (1fr, 1fr, 1fr, 1fr, 1.5fr), inset: 4pt,
 [p / seed], [Residuals], [Distinct x], [Repeated x], [First repeat at recovery],
 ..s.relation_mechanism.targets.map(r => (
  [#r.p / #r.seed], [#r.residuals],
  [#r.relation_point_stats.distinct_abscissae], [#r.relation_point_stats.repeated_abscissae],
  [#if r.first_repeat_at_recovery { [Yes] } else { [No] }],
 )).flatten())

#s.relation_mechanism.first_repeat_at_recovery_count of 9 target/sampler cases
finish at their first repeated decomposable x. Across short 06 and long 07,
this happens in 17 of 18 cases. This supports the coalescing-trail hypothesis,
not a new independent IC density or complexity conclusion. The exception is
short 06 p127/seed47, which recovers with 23 distinct decomposable x and no
repeat. All original/rejected outcomes remain retained. [pcc:unverified]

The mechanism control follows van Oorschot–Wiener, author preprint dated
23 September 1996, sections 4.1/5.1:
#link("https://people.scs.carleton.ca/~paulv/papers/JoC97.pdf")[Parallel Collision Search with Cryptanalytic Applications].
The next independent lead is the native F4 solver for three-point S4;
its basis phase must be measured before integrating quotient/root recovery.
[pcc:unverified]
