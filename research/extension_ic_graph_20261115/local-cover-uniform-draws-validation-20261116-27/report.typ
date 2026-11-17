#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Sans",size:9.5pt)
#let s=json("summary.json")
#let e=json("local-evidence.json")
#let display_value(v)=if v==none {[not recorded]} else if type(v)==bool {repr(v)} else {str(v)}
= Rust IC: selected uniform draws and rejections
16 November 2026 · Public synthetic weak Fp6 family · p1009, seed281

Source #raw(s.source_revision.slice(0,9)). This once-only diagnostic checks
selected modular sampling counters with the pinned rand0.8.8 output and
RNG-state contract. Complete costs remain unknown. [pcc:unverified]

#table(columns:(2fr,1fr),inset:4pt,
 [Planned / started / completed],
 [#display_value(s.planned_executions) / #display_value(s.attempted_executions) / #display_value(s.completed_executions)],
 [Exit / required-counter and endpoint criterion],
 [#display_value(s.observed_process_exit) / #display_value(s.full_recovery_validation_passed)],
 [Comparative follow-up allowed],[#display_value(s.allows_new_frozen_comparison)],
)

#let a=s.outcomes.at(0)
#let b=s.outcomes.at(1)
#let sampler(o)=o.work_native.uniform_sampling.total
#table(columns:(1.7fr,1fr,1fr),inset:4pt,
 [Selected sampler event],[Native IC],[Pivot graph IC],
 ..("calls","raw_draws","rejections","other_modulus_calls").map(k=>
 ([#raw(k)],[#display_value(sampler(a).at(k))],[#display_value(sampler(b).at(k))])).flatten(),
 [Known integer subtotal (bitops)],
 [#display_value(sampler(a).known_native_priced_bitops)],
 [#display_value(sampler(b).known_native_priced_bitops)],
 ..("integer_sub64","integer_clz64","integer_shl64").map(k=>
 ([Unpriced #raw(k)],[#display_value(sampler(a).unpriced_native.at(k))],
 [#display_value(sampler(b).unpriced_native.at(k))])).flatten(),
 [Complete operation total],[Unknown],[Unknown],
 [Recovered scalar],[#display_value(a.recovered_scalar)],[#display_value(b.recovered_scalar)],
)

The qualifier checks all three required markers against actual counts:
native solver scans beyond RNG, positive descent residuals with at least two
scalar additions each, and positive selected sampler calls/rejections. It
checks draw conservation, phase sums, exact known native integer prices,
explicit unpriced events, projection/solver flow and scalar replay on the
outer curve. A marker alone is insufficient. [pcc:unverified]

The selected paths are Fq-coordinate draws, the u64 Wiedemann vector and
four initial cover labels. Known native integer prices are separate from
the primitive subtotal. Generator internals, other sampling paths and native
container work remain gaps. These are observed counts and partial prices,
not complete costs or physical gate measurements. [pcc:unverified]

Both workers use public inputs without the answer, identical incidence/shared
settings, one claimed Rayon thread, CPU0 and8GiB,900seconds each/1900whole,
zero warmups and no retries. No rho arm, A/A, ratios, confidence interval or
performance ranking is part of this design. [pcc:unverified]

Original raw, immutable copied Rust analyzer/binary/source checkpoint,
complete summary, progress, source and pre/post hashes: #raw(e.run_path).
These are producer checks; no independent signed verdict or admitted ranking
is asserted. [pcc:unverified]
