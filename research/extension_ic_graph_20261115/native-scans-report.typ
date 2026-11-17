#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Sans",size:9.5pt)
#let s=json("summary.json")
#let e=json("local-evidence.json")
#let show_value(v)=if v==none {[not recorded]} else if type(v)==bool {repr(v)} else {str(v)}
#let primitive(o,k)={let w=o.at("work_native",default:none);if w==none {[not recorded]}else{show_value(w.observed_primitives.at(k))}}
= Rust IC: native solver scans and scalar labels
16 November 2026 · Public synthetic weak Fp6 family · p1009, seed277

Source #raw(s.source_revision.slice(0,9)). This once-only diagnostic checks
newly counted modular-solver data scans/comparisons and the cover stream's
scalar-label additions. Complete costs remain unknown. [pcc:unverified]

#table(columns:(2fr,1fr),inset:4pt,
 [Planned / started / completed],
 [#show_value(s.planned_executions) / #show_value(s.attempted_executions) / #show_value(s.completed_executions)],
 [Exit / endpoint and required-counter criterion],
 [#show_value(s.observed_process_exit) / #show_value(s.full_recovery_validation_passed)],
 [Allows comparative follow-up],[#show_value(s.allows_new_frozen_comparison)],
)

#let a=s.outcomes.at(0)
#let b=s.outcomes.at(1)
#table(columns:(1.7fr,1fr,1fr),inset:4pt,
 [Observed primitive],[Native IC],[Pivot graph IC],
 ..("field_mul","modr_add","modr_mul","word_op","rng_words").map(k=>
 ([#raw(k)],[#primitive(a,k)],[#primitive(b,k)])).flatten(),
 [Complete operation total],[Unknown],[Unknown],
 [Recovered scalar],[#show_value(a.recovered_scalar)],[#show_value(b.recovered_scalar)],
)

#for o in s.outcomes {
  let d=o.at("component_accounting",default:none)
  if d!=none {
    block(below:4pt)[*#o.arm solver counters*]
    table(columns:(1.2fr,1fr,1fr,1fr),inset:4pt,
      [Kind],[Rows],[Word events],[RNG words],
      ..d.solve_calls.map(c=>([#c.kind],[#show_value(c.rows)],
        [#show_value(c.primitives.word_op)],[#show_value(c.primitives.rng_words)])).flatten(),
    )
    [Descent residuals #show_value(o.work.descent_residuals);
    descent scalar additions #show_value(o.work_native.phases.at(6).primitives.modr_add).]
    parbreak()
  }
}

The qualifier requires both counted-item markers, solver words beyond RNG,
positive descent residuals with at least two scalar additions each, exact
integer repricing, consistent projection/solver flow and matrix shapes, and
component primitive sums within LA. A marker alone is insufficient. Returned
scalars are checked natively and on the outer curve. [pcc:unverified]

The source keeps modular arithmetic and legacy operation counters intact.
Pure allocation, copying and loop overhead are outside this scan patch.
RNG rejection/internal and other native/container work remain named gaps;
these primitive counts and priced subtotals are not complete costs or physical
gate measurements. [pcc:unverified]

Both workers receive public inputs without the answer, use the same
incidence/shared finder, one claimed Rayon thread, CPU0 and8GiB,900seconds
each/1900whole, zero warmups and no retries. No rho arm, A/A, ratios,
confidence interval or performance ranking is part of this design.
[pcc:unverified]

Original raw, immutable copied Rust analyzer/binary/source checkpoint, exact
summary, progress, source and pre/post hashes: #raw(e.run_path).
These are producer checks; no independent signed verdict or admitted ranking
is asserted. [pcc:unverified]
