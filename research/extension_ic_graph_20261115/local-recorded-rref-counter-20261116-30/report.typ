#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Sans",size:10pt)
#let s=json("summary.json")
#let e=json("local-evidence.json")
#let display_value(v)=if v==none {[not recorded]} else if type(v)==bool {repr(v)} else {str(v)}
= Recorded IC matrix: exact RREF arithmetic counts
16 November2026 · Immutable public-target proof from diagnostic29

New source #raw(s.source_revision.slice(0,9)); original source
#raw(s.original_source_revision.slice(0,9)). This replays the recorded affine
matrix only. No relation search, descent or new original-DLP sample occurs.
The old raw and pipeline summary remain unchanged. [pcc:unverified]

#table(columns:(2fr,1fr),inset:5pt,
 [Unchanged actual target],[#display_value(s.actual_scalar)],
 [Unchanged rank / columns],[#display_value(s.rank) / #display_value(s.columns)],
 [Rows available / used],[#display_value(s.rows_available) / #display_value(s.rows_used)],
 [Legacy modular products],[#display_value(s.legacy_products)],
 [Normalization products],[#display_value(s.normalization_products)],
 [Counted modular products],[#display_value(s.observed_primitives.modr_mul)],
 [Counted modular subtractions],[#display_value(s.modular_subtractions)],
 [Already-hooked inversions, counted once],[#display_value(s.observed_primitives.modr_inv)],
 [Observed arithmetic subtotal, bitops],[#display_value(s.observed_arithmetic_subtotal_bitops)],
 [Counter/scalar conservation criterion],[#display_value(s.recorded_counter_validation_passed)],
 [Complete operation total],[Unknown],
)

Each forward/backward elimination product has one modular subtraction,
including RHS work. An independent pivot normalization has columns+1products
and no modular subtraction. Thus subtraction counts are legacy products
minus rank times(columns+1). The existing inverse hook remains; it is not
charged again. Small hand and wide-prime source controls accompany the
matrix replay. [pcc:unverified]

The counted adapter leaves values, legacy product counts and generic solver
implementation unchanged. It classifies products/additions by modulus and
uses the existing bitop/v1 prices. Native coefficient scans, fixed-factor
quotient/setup, allocations/copies and solver control remain uncounted.
This is an arithmetic-coverage correction without a performance gain claim.
[pcc:unverified]

Never compose this replay's price with29's old subtotal as a new end-to-end
measurement. Complete costs/comparative/newfullDLP/rho flags remain
null/false; no ratio, interval or ranking belongs to this protocol.
These are producer checks without an independent signed verdict.
[pcc:unverified]

One run/zero warmups,CPU0/1GiB/one claimed thread,60seconds,noretries.
Exact immutable input, new raw/result/flow, copied driver/qualifier/source,
environment and pre/post hashes: #raw(e.run_path). [pcc:unverified]
