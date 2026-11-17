#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Sans",size:10pt)
#let s=json("summary.json")
#let e=json("local-evidence.json")
#let display_value(v)=if v==none {[unknown]} else if type(v)==bool {repr(v)} else {str(v)}
= Skip zero products in pivot normalization
Recorded-matrix component replay · Runpod CPU · 16 November 2026

Source #raw(s.source_revision.slice(0,9)) replays the exact saved35matrix,
original source #raw(s.original_source_revision.slice(0,9)), scalar modulus
#display_value(s.l). Both modes produce the same target
#display_value(s.actual_scalar), rank #display_value(s.rank), using
#display_value(s.rows_used) of #display_value(s.rows_available) rows.
This is a matrix replay, without a new DLP sample. [pcc:unverified]

#table(columns:(210pt,120pt,120pt),align:left,inset:5pt,
 [Measured arithmetic],[Baseline],[Skip zeros],
 [All modular products],[#display_value(s.outcomes.at(0).counts.products)],[#display_value(s.outcomes.at(1).counts.products)],
 [Normalization products],[#display_value(s.outcomes.at(0).counts.normalization_products)],[#display_value(s.outcomes.at(1).counts.normalization_products)],
 [Modular subtractions],[#display_value(s.outcomes.at(0).counts.subtractions)],[#display_value(s.outcomes.at(1).counts.subtractions)],
 [Inversions],[#display_value(s.outcomes.at(0).counts.inversions)],[#display_value(s.outcomes.at(1).counts.inversions)],
 [Added normalization zero tests],[0],[#display_value(s.new_zero_tests)],
 [Observed primitive bitops],[#display_value(s.outcomes.at(0).observed_primitive_subtotal_bitops)],[#display_value(s.outcomes.at(1).observed_primitive_subtotal_bitops)],
 [Complete operation cost],[Unknown],[Unknown],
)

#image("normalization.svg",width:100%)

The existing solver already avoids zero coefficients during forward
updates and uses compact support for backward updates. The candidate only
skips zero coefficients and RHS entries in pivot normalization, preserving
the modular reduction routes, inverse, column choice and update order.
The actual per-insertion pivot states agree with a separate column-keyed
u128-arithmetic oracle. A batch solve independently checks the full affine
system. Avoided products must equal the oracle's zero count; subtraction,
inverse, factor-construction and update-scan profiles stay unchanged.
[pcc:unverified]

The candidate's added zero predicates are charged as word events in the
existing bitop/v1 convention. Other native loop/container/quotient setup
and Euclidean work remains incomplete. Baseline zero observation and both
per-insert state hashes are diagnostic instrumentation, outside arithmetic
subtotals and inside process wall. No wall speed, complete cost, pricing
band, cost ratio, pipeline improvement or rho-parity claim. Earlier raw
records and35's price are unchanged. [pcc:unverified]

One paired replay, zero warmups or retries;60s worker/180s whole/900s runner.
Runpod CPU4vCPU/8GiB, actual pinned CPU #display_value(e.pinned_cpu), pinned
official Rust image. Exact input/source/binaries/argv/flow/status/raw/copied
qualifier/hash custody: #raw(e.run_path). Artifacts are collected before owned
pod termination. The native export claims no unavailable canonical-schema
conformance. Producer checks carry no independent signed verdict.
[pcc:unverified]
