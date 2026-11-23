#let s=json("summary.json")
#let e=json("local-evidence.json")
#let fmt(v)=str(calc.round(v,digits:2))
#for (i,p) in (67,127,1039).enumerate() {
 if i>0 {pagebreak()}
 set page(paper:"a4",margin:15mm)
 set text(font:"DejaVu Sans",size:9pt)
 heading(level:1,[Certified S4 trace reuse: p=#p])
 [16 November 2026 · Public cubic-extension fixtures · Coordinate phase only]
 parbreak()
 [All 864 cold batches complete in 131.978 seconds: 144 retained warmups,
 720 measured batches and 6912 coordinate calls on 36 ordered fixtures.
 Every output matches dense qualification; no unsupported result is treated
 as empty roots. Source #raw(s.source_revision.slice(0,9)), exit zero,
 one attempt and zero retries. [pcc:unverified]]
 image("ratios-p"+str(p)+".svg",width:100%)
 [Candidate/reference cold batch wall, geometric means and 95% paired-round
 bootstrap intervals. Both methods use the same delayed sparse extraction.
 Candidate constructs a new certified trace pool for each batch of eight
 distinct residuals. Gray cases fail timing controls. [pcc:unverified]]
 parbreak()
 table(columns:(0.9fr,0.6fr,1.4fr,0.8fr,0.7fr,1fr),inset:3pt,
 [Seed / batch],[Ratio],[95% interval],[Local A/A],[Learn / accept],[Cert. products],
 ..s.strata.filter(r=>r.p==p).map(r=>(
 [#r.seed / #r.ordinal], [#fmt(r.candidate_over_reference.geometric_mean)],
 [#fmt(r.candidate_over_reference.bootstrap_95.at(0))–#fmt(r.candidate_over_reference.bootstrap_95.at(1))],
 [#if r.eligible_local_controlled_comparison {[Pass]}else{[Rejected]}],
 [#r.first_measured_trace_stats.pool.learned / #r.first_measured_trace_stats.pool.accepted],
 [#r.first_measured_trace_stats.pool.certification_multiplications],
 )).flatten())
 parbreak()
 [All 33 controlled cases lose: every lower interval exceeds 1, with cost
 ratios 2.981–16.086. Three A/A rejections remain: p67/seed179/batch3,
 p67/seed181/batch0 and p127/seed179/batch0. All 12 largest-field cases pass
 controls but lose at 11.823–14.956 times the reference. Their first measured
 pools accept six or seven replays, so reuse is exercised. [pcc:unverified]]
 parbreak()
 [The frozen condition requires six controlled largest-field gains with
 accepted replay. Observed count: zero, so the full-IC follow-up is omitted.
 These timings concern this implementation and cold
 eight-residual batches; the paper's gains on different systems are not
 transferred. No pipeline or rho comparison follows from this phase.
 [pcc:unverified]]
 parbreak()
 [Charged inside each timer: pool construction and destruction, first
 learning, family checks, certification, rejected replay, fallback and
 complete coordinate extraction. Shared outside phase: curve/symbolic
 preparation #fmt(s.shared_cost_ns.curve_and_symbolic_once_per_instance/1e9)s,
 residual construction #fmt(s.shared_cost_ns.residual_fixture_construction/1e9)s,
 dense qualification #fmt(s.shared_cost_ns.dense_qualification/1e9)s.
 Outer process wall retains all shared work and logging. [pcc:unverified]]
}
#pagebreak()
#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Sans",size:10pt)
= Scope, counters and retained evidence

Learn/accept and certification products are the first measured trace batch's
counters. Same-method semantic counts match across A/A repeats; primitive
accounting is incomplete and these counts are not wall-time shares. Intervals
cover repeated timing on fixed curves/batches, not independent curve samples.
[pcc:unverified]

The candidate uses the existing certified trace pool. Each accepted replay
checks reductions of the current inputs and every Buchberger pair. Learning,
rejected replay, storage refusals, certification and fallback remain charged.
The pool is new for every measured batch, bound to one original curve/field/
target, with a total retained cap of 1000000 words. The batch contains eight
distinct abscissae; it is not a repeated identical-input replay. [pcc:unverified]

All 864 batches complete and match the dense qualifier, with no protocol
integrity problems. The three rejected cases fail the timing A/A threshold;
they are retained with their point estimates and intervals as diagnostics.
There are no controlled gains. The paper's F4Remake results concern different
overdetermined systems and do not predict the measured performance of this
complete checked cubic-extension implementation. [pcc:unverified]

CPU0, RAYON/OMP1, 8GiB virtual ceiling, 900-second process limit. No
strongest-external, signed independent, canonical encrypted-schema or
standard lab admission. Public synthetic Fp^3 isolates this family absent
from the standard panel. No admitted benchmark-index or leaderboard row.
[pcc:unverified]

Evidence: #raw("local-certified-s4-trace-20261116-18"), full 39910-byte native
summary, frozen runtime plan, exact flow/custody in local-evidence.json and
pre/post-call hashes. Raw, original driver/analyzer binaries, source checkpoint
and host provenance remain under
#raw("/workspace/ic-local-runs/local-certified-s4-trace-20261116-18").
One measurement attempt, zero retries. Every failure and control rejection
is retained. [pcc:unverified]
