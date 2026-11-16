#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Sans",size:9.5pt)
#let s=json("summary.json")
#let e=json("local-evidence.json")
#let show_value(v)=if v==none {[not recorded]} else if type(v)==bool {repr(v)} else {str(v)}
#let observed(o,k)={
  let d=o.at("work_native",default:none)
  if d==none {[not recorded]} else {show_value(d.observed_primitives.at(k,default:none))}
}
= Rust graph IC: operation-accounting diagnostic
16 November 2026 · Public synthetic weak Fp6 family · p1009, seed269

Source #raw(s.source_revision.slice(0,9)). This once-only cohort checks the
counting adapter and actual scalar recovery. It records incomplete observed
work; a complete IC total and comparative ranking remain unset.
[pcc:unverified]

#table(columns:(2fr,1fr),inset:4pt,
 [Planned / started / completed],
 [#show_value(s.planned_executions) / #show_value(s.attempted_executions) / #show_value(s.completed_executions)],
 [Process exit],[#show_value(s.observed_process_exit)],
 [Endpoint and accounting checks],[#show_value(s.full_recovery_validation_passed)],
 [Allows comparative follow-up],[#show_value(s.allows_new_frozen_comparison)],
)

#let a=s.outcomes.at(0)
#let b=s.outcomes.at(1)
#table(columns:(1.7fr,1fr,1fr),inset:4pt,
 [Observed primitive],[Native IC],[Pivot graph IC],
 ..("field_add","field_mul","field_sqr","field_inv","field_sqrt","legendre",
 "modr_add","modr_mul","modr_inv","word_op","hash_probe","rng_words").map(k=>
 ([#raw(k)],[#observed(a,k)],[#observed(b,k)])).flatten(),
 [Diagnostic priced subtotal],
 [#show_value(a.at("work_native",default:(:)).at("diagnostic_priced_bitops",default:none))],
 [#show_value(b.at("work_native",default:(:)).at("diagnostic_priced_bitops",default:none))],
 [Complete operation total],[Unknown],[Unknown],
 [Recovered scalar],[#show_value(a.recovered_scalar)],[#show_value(b.recovered_scalar)],
)

Each diagnostic is repriced with the repository's fixed bitop/v1 formulas,
including the subgroup width for scalar arithmetic. Phase sums are checked.
RNG draws are an auxiliary count already included in word operations.
These are producer checks, not independent signed evidence. [pcc:unverified]

The counted interval includes transfer, factor base, relation finding,
descent, graph arithmetic and sparse AXPY, solving and answer replay. Public
domain checks are outside it. Graph maps/sets, normalization/scheduling,
certificate bookkeeping, scalar/RNG coverage and native bookkeeping remain
named gaps. The measured subtotal is not a complete cost. [pcc:unverified]

Both workers use the same incidence/shared relation finder, one claimed
Rayon thread, one pinned CPU and8GiB, with900seconds each and1900seconds for
the cohort, zero warmups and no retries. No rho arm, performance ratio,
confidence interval or parity verdict belongs to this design. Prior22censor
and23MST refusal remain unchanged. [pcc:unverified]

Evidence: original raw records, copied Rust analyzer, phase counts, exact
summary, progress and pre/post hashes under #raw(e.run_path).
The next requirement is complete coverage and strongest-reference pricing
before a separately frozen comparison. [pcc:unverified]
