#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Sans",size:9.5pt)
#let s=json("summary.json")
#let e=json("local-evidence.json")
#let show_value(v)=if v==none {[not recorded]} else if type(v)==bool {repr(v)} else {str(v)}
#let value(o,k)={
  let d=o.at("component_accounting",default:none)
  if d==none {[not recorded]} else {show_value(d.component_totals.at(k).diagnostic_priced_bitops)}
}
#let remainder(o)={
  let d=o.at("component_accounting",default:none)
  if d==none {[not recorded]} else {show_value(d.unallocated_linear_algebra.diagnostic_priced_bitops)}
}
= Rust graph IC: locate observed linear-algebra work
16 November 2026 · Public synthetic weak Fp6 family · p1009, seed271

Source #raw(s.source_revision.slice(0,9)). This once-only diagnostic separates
projection and witness replay from actual matrix-solver calls. All values are
incomplete observed bitop/v1 costs; a complete total remains unknown.
[pcc:unverified]

#table(columns:(2fr,1fr),inset:4pt,
 [Planned / started / completed],
 [#show_value(s.planned_executions) / #show_value(s.attempted_executions) / #show_value(s.completed_executions)],
 [Exit / endpoint and accounting criterion],
 [#show_value(s.observed_process_exit) / #show_value(s.full_recovery_validation_passed)],
 [Allows comparative follow-up],[#show_value(s.allows_new_frozen_comparison)],
)

#let a=s.outcomes.at(0)
#let b=s.outcomes.at(1)
#table(columns:(1.8fr,1fr,1fr),inset:4pt,
 [Observed component],[Native IC],[Pivot graph IC],
 ..("mapping","filter","replay","output","original_solve","projected_solve").map(k=>
 ([#raw(k)],[#value(a,k)],[#value(b,k)])).flatten(),
 [Other observed LA work],[#remainder(a)],[#remainder(b)],
 [Complete operation total],[Unknown],[Unknown],
)

The Rust analyzer recomputes integer prices, checks call order and matrix
shapes, and subtracts component primitive counts from the enclosing LA phase.
A negative remainder, changed marker, reordered stage or inconsistent
projection/solve shape is rejected. This is producer checking, not independent
signed evidence. [pcc:unverified]

#for o in s.outcomes {
  let d=o.at("component_accounting",default:none)
  if d!=none {
    block(below:4pt)[*#o.arm matrix observations*]
    table(columns:(1fr,1.2fr,1fr,1fr),inset:4pt,
      [Attempt],[Solve kind],[Rows],[Nonzeros],
      ..d.solve_calls.map(c=>([#show_value(c.attempt)],[#c.kind],[#show_value(c.rows)],[#show_value(c.nonzeros)])).flatten(),
    )
    for c in d.projection_calls {
      [Graph input #show_value(c.input_rows) rows / #show_value(c.input_nonzeros) nonzeros;
      candidate #show_value(c.candidate_rows) / #show_value(c.candidate_nonzeros);
      accepted #show_value(c.accepted).]
      parbreak()
    }
  }
}

Both workers receive public curve/points with no answer and recover an actual
scalar checked natively and on the outer curve. Both use incidence/shared
finding, one claimed Rayon thread, CPU0 and8GiB;900seconds each/1900whole,
zero warmups and no retries. Refused and fallback work stays in the record.
No rho arm, A/A, ratios, confidence interval or performance ranking belongs
to this design. [pcc:unverified]

Native bookkeeping, graph containers, scalar/RNG coverage and strongest-rho
pricing remain open. These observations locate measured work; they do not
resolve those gaps or infer an overall gain. Raw records, copied analyzer,
exact summary, progress, source and pre/post hashes: #raw(e.run_path).
[pcc:unverified]
