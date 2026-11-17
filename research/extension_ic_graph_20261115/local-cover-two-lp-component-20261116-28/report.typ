#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Sans",size:9.5pt)
#let s=json("summary.json")
#let e=json("local-evidence.json")
#let display_value(v)=if v==none {[not recorded]} else if type(v)==bool {repr(v)} else {str(v)}
= Rust adapted two-large-prime sieve and graph
16 November2026 · Restricted public synthetic weak Fp6 · p1009, seed293

Source #raw(s.source_revision.slice(0,9)). Fixed300000B values, degree9,
75percentprefixsmallbase, identical incidence/shared line cohort.
This is a relation/rank diagnostic; complete costs and fullDLP recovery
remain unknown/unexecuted. [pcc:unverified]

#let a=s.outcomes.at(0)
#let b=s.outcomes.at(1)
#table(columns:(1.7fr,1fr,1fr),inset:4pt,
 [Observed component],[Full-smooth small base],[Adapted2LP + graph],
 [Recorded Jacobian-verified inputs],[#display_value(a.recorded_jacobian_verified_inputs)],[#display_value(b.recorded_jacobian_verified_inputs)],
 [Complete rows / replays],[#display_value(a.complete_rows) / #display_value(a.certificate_replays)],[#display_value(b.complete_rows) / #display_value(b.certificate_replays)],
 [Independent modular rank],[#display_value(a.rank)],[#display_value(b.rank)],
 [Output nonzeros],[#display_value(a.output_nonzeros)],[#display_value(b.output_nonzeros)],
 [Observed primitive subtotal, bitops],[#display_value(a.observed_primitive_subtotal_bitops)],[#display_value(b.observed_primitive_subtotal_bitops)],
 [Complete total],[Unknown],[Unknown],
)

Full base #display_value(s.full_base), small base #display_value(s.small_base),
ordered lines #display_value(s.line_count). Exact cohort digest
#raw(s.cohort_digest). Nontrivial2LP graph criterion
#display_value(s.nontrivial_two_lp_graph); binding/replay/repricing checks
#display_value(s.binding_replay_price_checks_passed). [pcc:unverified]

For each source, the collector peels small roots, requires its remaining
degree-at-most-two cofactor to split overFp, recovers signed points and checks
the original divisor relation in the Jacobian. The copied native qualifier
independently replays modular certificates and computes rank over the
checked prime subgroup order. More rows alone do not establish independence.
These are producer checks, not an independent signed verdict. [pcc:unverified]

Both arms include cold cover/full-base/reduced-table setup, the fixed line
cohort, original source verification, duplicate removal, graph filtering and
all certificate replays. Instance generation is excluded; qualifier work is
outside the measured component. The reference uses the SAME small base.
Full-base construction and repeated source verification remain charged.
[pcc:unverified]

Native/container work, generator internals, some sieve native work and
cohort hashing/serialization remain unpriced. Selected sampler integer prices
and explicit unpriced events are retained separately in the native summary.
No A/A, speed/cost ratio, interval, ranking, original-DLP recovery or rho
comparison is part of this protocol. Descent/target recovery integration and
complete strongest-reference pricing remain open. [pcc:unverified]

Both arms run once with one claimed thread, CPU0/8GiB,900seconds each/
1900whole, zero warmups/no retries. Flow planned/attempted/completed
#display_value(s.flow.planned)/#display_value(s.flow.attempted)/#display_value(s.flow.completed).
Original source relations/certificates, immutable driver/qualifier/source,
full summary, progress/environment and pre/post hashes: #raw(e.run_path).
[pcc:unverified]
