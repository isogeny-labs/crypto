#let s=json("summary.json")
#let e=json("local-evidence.json")
#let fmt(v)=str(calc.round(v,digits:3))
#let eligible=s.strata.filter(r=>r.eligible_local_controlled_comparison)
#let gains=eligible.filter(r=>r.candidate_over_reference.bootstrap_95.at(1) < 1)
#let losses=eligible.filter(r=>r.candidate_over_reference.bootstrap_95.at(0) > 1)
#let rejected=s.strata.filter(r=>not r.eligible_local_controlled_comparison)
#for (i,p) in (67,127,1039).enumerate() {
 if i>0 {pagebreak()}
 set page(paper:"a4",margin:15mm)
 set text(font:"DejaVu Sans",size:9pt)
 heading(level:1,[Pair-dependency S4 certificate: p=#p])
 [16 November 2026 · Public cubic-extension fixtures · Complete-coordinate phase]
 parbreak()
 [#s.completed_batch_executions of #s.planned_batch_executions batches complete
 in #e.outer_wall_seconds seconds; #s.phase_counts.warmup warmups and
 #s.phase_counts.measurement measured batches retained. Source
 #raw(s.source_revision.slice(0,9)), exit #e.exit_code, one attempt, zero retries.
 Outputs and protocol integrity are checked by the native analyzer. [pcc:unverified]]
 image("ratios-p"+str(p)+".svg",width:100%)
 [Cold pair-graph trace / fresh F4 wall ratios, geometric means with 95%
 paired-round bootstrap intervals. Gray cases are rejected diagnostics.
 Each batch contains eight distinct residuals and starts with a new pool.
 [pcc:unverified]]
 parbreak()
 table(columns:(0.9fr,0.6fr,1.2fr,0.6fr,0.8fr,1fr),inset:3pt,
 [Seed / batch],[Ratio],[95% interval],[A/A],[Learn / accept],[Cert. products],
 ..s.strata.filter(r=>r.p==p).map(r=>(
 [#r.seed / #r.ordinal], [#fmt(r.candidate_over_reference.geometric_mean)],
 [#fmt(r.candidate_over_reference.bootstrap_95.at(0))–#fmt(r.candidate_over_reference.bootstrap_95.at(1))],
 [#if r.eligible_local_controlled_comparison {[Pass]}else{[Reject]}],
 [#r.first_measured_trace_stats.pool.learned / #r.first_measured_trace_stats.pool.accepted],
 [#r.first_measured_trace_stats.pool.certification_multiplications],
 )).flatten())
 parbreak()
 [Across all fields: #eligible.len() controlled comparisons, #gains.len() gains
 (upper interval below 1), #losses.len() losses (lower interval above 1),
 #(eligible.len()-gains.len()-losses.len()) inconclusive; #rejected.len() rejected.
 Largest-field controlled gains with accepted/pruned replay:
 #s.largest_field_controlled_gains_with_replay. [pcc:unverified]]
 parbreak()
 [The frozen full-IC viability condition evaluates to
 #raw(repr(s.allows_later_frozen_full_ic)). A passing condition permits a separately
 frozen trial; a failing condition rules out that follow-up for this version.
 Ratios are not composed with earlier trials. [pcc:unverified]]
 parbreak()
 [Charged: pool/graph construction/destruction, first learning, all input and
 direct pair reductions, product/chain/witness checks, rejected replay,
 fallback and full coordinate extraction. Shared outside phase: curve/symbolic
 #fmt(s.shared_cost_ns.curve_and_symbolic_once_per_instance/1e9)s, residuals
 #fmt(s.shared_cost_ns.residual_fixture_construction/1e9)s, dense qualification
 #fmt(s.shared_cost_ns.dense_qualification/1e9)s. Outer wall retains this work.
 [pcc:unverified]]
}
#pagebreak()
#set page(paper:"a4",margin:15mm)
#set text(font:"DejaVu Sans",size:9pt)
= Pair-graph mechanism and retained evidence

Every pair is certified by direct zero reduction, coprime leading terms, or
an LCM-dividing middle vertex with both edges already certified. Prior-edge
references prevent circular justification. All original input reductions and
native row provenance remain required, as do complete quotient/root/group
checks. Default exhaustive certification and the term-reduction kernel remain.
This reuses classical Buchberger criteria. [pcc:unverified]

First measured batch edge counters: product / chain / direct. Their sum is
all visited pair obligations. Counters are diagnostics, not wall-time shares;
primitive pricing is incomplete. Same-method A/A signatures must match.
[pcc:unverified]

#table(columns:(0.65fr,0.85fr,1.6fr,1fr),inset:2pt,
 [p],[Seed / batch],[Product / chain / direct],[Witness tests],
 ..s.strata.map(r=>{
 let g=r.first_measured_trace_stats.pool.pair_graph
 ( [#r.p],[#r.seed / #r.ordinal],
 [#g.product_edges / #g.chain_edges / #g.direct_edges],[#g.witness_tests] )
 }).flatten())

#rejected.len() rejected cases retained:
#for r in rejected {
 [p#r.p / seed#r.seed / batch#r.ordinal: #r.invalid_reasons.join("; "). ]
}

CPU0, RAYON/OMP1, 8GiB virtual, 900 seconds/process. Fixed-curve repeated-timing
intervals; no independent curve inference or IC/rho extrapolation. Public Fp^3
fixtures isolate this family absent from the standard panel. No admitted index,
signed independent verdict, strongest-external or canonical-schema validation.
Full summary/flow, source/binary/raw/pre/post hashes, copied checkpoint and host
provenance retained; original custody: #raw(e.raw.path). [pcc:unverified]
