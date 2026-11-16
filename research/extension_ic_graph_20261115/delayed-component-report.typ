#let render(s, flow, p, plot, findings) = {
 set page(paper:"a4",margin:14mm)
 set text(font:"DejaVu Sans",size:8.7pt)
 set par(justify:true)
 heading(level:1,"Delayed quotient-graph kernel: p"+str(p))
 [16 November 2026 · Complete coordinate engine · Reference: #s.reference_kind]
 parbreak()
 [Source #raw(s.source_revision.slice(0,9)). #s.retained_execution_batches of
 #s.planned_execution_batches planned batches retained; #s.attempted_execution_batches
  starts, #s.in_flight_batches_without_completion unfinished started batches,
 #s.invalid_or_capped_batches invalid completed batches. No retries. [pcc:unverified]]
 plot
 findings
 parbreak()
 let fmt(v)=if v==none {"Missing"} else {str(calc.round(v,digits:3))}
 table(columns:(0.8fr,0.8fr,1.3fr,0.8fr,1.3fr),inset:3pt,
 [seed / r],[Wall ratio],[95% interval],[Local A/A],[Delayed / scalar steps],
 ..s.cases.filter(r=>r.p==p).map(r=>{
  let metric=r.at(s.comparison_metric)
  let native=r.first_measured_delayed
  ([#r.seed / #r.ordinal],
   [#if metric==none {[Missing]} else {fmt(metric.geometric_mean)}],
   [#if metric==none {[Missing]} else {[#fmt(metric.bootstrap_95.at(0))–#fmt(metric.bootstrap_95.at(1))]}],
   [#if r.eligible_local_controlled_comparison {[Pass]} else {[Rejected]}],
   [#if native==none {[Missing]} else {[#native.at(3).krylov.delayed_steps / #native.at(3).krylov.scalar_steps]}],)
 }).flatten())
 parbreak()
 [Each eight-call batch returns exactly the complete dense qualifier's canonical
 triples, including empty sets. Scalar-reference comparisons additionally require
 identical semantic product counts and quotient dimensions. Both A/A signatures,
 intra-batch consistency and reciprocal 1.10 timing limits apply. Intervals use
 10000 paired-round bootstrap samples on fixed systems. Every rejected/missing
 case remains diagnostic. [pcc:unverified]]
 parbreak()
 [All basis, border, matrix, extraction and failed attempts are inside the phase.
 Shared symbolic setup/input generation/qualification are outside and recorded
 separately. This is not full IC. Partial field counters cannot rank native cost;
 wall time prices the changed modular divisions. The guarded kernel requires
 at most 64 normalized terms and $p < 2^20$; otherwise scalar arithmetic is used.
 It preserves exact recurrence/Hankel/equation checks and charged fallback.
 [pcc:unverified]]
 parbreak()
 [Method: Faugere–Mou sparse FGLM section3.1; bounded modular accumulation as in
 FFLAS-FFPACK delayed fgemv and existing native F4 routines. Exact sources/hashes
 in Amendment 9. Local AMD EPYC 9V74, CPU0, 8 GiB virtual-memory cap, 900 second
 process limit, one attempt. Source/binary/plan/flow/raw custody hashes accompany
 this report; raw/copied binaries/checkpoint remain at the recorded local path.
 No signed independent verdict, strongest-external comparison, canonical encrypted
 schema validation or standard lab admission is asserted. [pcc:unverified]]
}
