#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Sans",size:10pt)
#let s=json("summary.json")
#let e=json("local-evidence.json")
#let display_value(v)=if v==none {[unknown]} else if type(v)==bool {repr(v)} else {str(v)}
= Graph IC: stop collection at certified rank
16 November 2026 · Public weak-family validation, p = 1009

Source #raw(s.source_revision.slice(0,9)); seed #display_value(s.seed).
All three public-input workers recover scalar #display_value(s.actual_scalar),
with native and additional elliptic replay plus held-out agreement.
The generator's answer remains outside the workers. [pcc:unverified]

#table(columns:(2fr,1fr,1fr),inset:5pt,
 [Observation],[Fixed 2LP],[Rank stop],
 [Unique B values],[#display_value(s.fixed_b_count)],[#display_value(s.rank_stopped_b_count)],
 [Verified original sources],[#display_value(s.fixed_proof.verified_original_inputs)],[#display_value(s.rank_stopped_proof.verified_original_inputs)],
 [Complete affine certificate rows],[#display_value(s.fixed_proof.certificate_replays)],[#display_value(s.rank_stopped_proof.certificate_replays)],
 [Independent final matrix rank],[#display_value(s.fixed_proof.matrix_rank)],[#display_value(s.rank_stopped_proof.matrix_rank)],
 [Accepted descents],[#display_value(s.fixed_proof.accepted_descents)],[#display_value(s.rank_stopped_proof.accepted_descents)],
 [Attempted descent residuals],[#display_value(s.fixed_proof.descent_residuals)],[#display_value(s.rank_stopped_proof.descent_residuals)],
 [Graph pivots],[#display_value(s.fixed_proof.graph.pivots)],[#display_value(s.rank_stopped_proof.graph.pivots)],
 [Observed primitive subtotal, bitops],[#display_value(s.outcomes.at(1).accounting.observed_primitive_subtotal_bitops)],[#display_value(s.outcomes.at(2).accounting.observed_primitive_subtotal_bitops)],
 [Complete operation total],[Unknown],[Unknown],
)

Original IC's observed primitive subtotal is
#display_value(s.outcomes.at(0).accounting.observed_primitive_subtotal_bitops) bitops.
These are incomplete counts, not complete costs or a ranked speed comparison.
Native containers/scans/fixed-factor setup, PRNG internals/other sampling,
hashing/serialization and strongest-rho pricing remain open. [pcc:unverified]

Rank checks: #display_value(s.rank_check_proof.checks.len()). At complete batches of 1024 B values,
boundaries, the opt-in collector filters and replays every current homogeneous
certificate, then measures rank with counted modular RREF. After each check,
at least 128 new source relations are needed before another check. A rank of
the small base size minus one triggers collection stop; actual affine target recovery remains
the final correctness condition. All checkpoint verification/rank arithmetic
is included in the recorded LA phase. [pcc:unverified]

The original source list is an exact prefix of the fixed collector:
#display_value(s.original_source_prefix_equal). Common descent prefixes agree:
#display_value(s.common_descent_prefix_equal). The copied Rust qualifier
replays all checkpoint and final affine certificates, independently checks
rank/target, binds phase sums/prices and requires unchanged public answers.
These are producer checks without an independent signed verdict.
[pcc:unverified]

One instance and validation round, zero warmups; original IC, then fixed 2LP,
then rank-stop 2LP, CPU0/8GiB/one claimed thread; 60 seconds for the fixture,
900 per worker and 2800 for the whole run, no retries. Failed or censored outcomes remain retained.
Walls are practicality observations without A/A intervals. No cross-run
composition, complete cost ranking or rho claim. [pcc:unverified]

Exact raw certificates/summary/flow/status, immutable copied binaries/source,
preworker/prepost hashes and host facts: #raw(e.run_path). [pcc:unverified]
