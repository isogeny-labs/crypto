#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Sans",size:9.5pt)
#let s=json("summary.json")
#let e=json("local-evidence.json")
#let audit=json("coverage-audit.json")
#let display_value(v)=if v==none {[not recorded]} else if type(v)==bool {repr(v)} else {str(v)}
= Rust adapted two-LP graph: public-target recovery
16 November2026 · Restricted synthetic weak Fp6 · p1009, seed307

Source #raw(s.source_revision.slice(0,9)). Both workers receive only public
curve/order/G/Q, rebuild transfer with a zero answer placeholder and return
an actual scalar checked against the held-out parent answer. This is a
full-recovery validation; complete costs remain unknown. [pcc:unverified]

#let a=s.outcomes.at(0)
#let b=s.outcomes.at(1)
#table(columns:(1.7fr,1fr,1fr),inset:4pt,
 [Observed endpoint],[Original IC],[Adapted2LP graph IC],
 [Actual recovered scalar],[#display_value(a.actual_scalar)],[#display_value(b.actual_scalar)],
 [Elliptic replay / held-out agreement],[#display_value(a.verified)],[#display_value(b.verified)],
 [Observed primitive subtotal, bitops],[#display_value(a.accounting.observed_primitive_subtotal_bitops)],[#display_value(b.accounting.observed_primitive_subtotal_bitops)],
 [Complete operation total],[Unknown],[Unknown],
)

#let c=s.candidate_proof
#table(columns:(2fr,1fr),inset:4pt,
 [Full base / small base],[#display_value(c.full_base) / #display_value(c.small_base)],
 [Original verified source rows],[#display_value(c.verified_original_inputs)],
 [Accepted descents / attempted residuals],[#display_value(c.accepted_descents) / #display_value(c.descent_residuals)],
 [Affine certificate replays],[#display_value(c.certificate_replays)],
 [Independent matrix rank],[#display_value(c.matrix_rank)],
 [Independently determined target],[#display_value(c.target_scalar)],
 [Output nonzeros],[#display_value(c.output_nonzeros)],
 [Full-recovery criterion],[#display_value(s.full_recovery_validation_passed)],
)

The candidate sieves degree9 with75percentprefixsmallbase and one complete
1018081-value B enumeration. Verified residual rows retain target coefficient
minus b and RHSa. The existing LP hypergraph eliminates only omitted points;
every affine certificate is replayed before incremental modular RREF. The
standalone copied qualifier independently checks coefficients/RHS and solves
the recorded target with batch Gaussian elimination, rejecting inconsistent
systems. Actual source Jacobians and elliptic replays are checked natively.
[pcc:unverified]

Both arms include cold transfer/model/base/relations/descent/linear work and
elliptic replay; public fixture generation/domain checks and the qualifier
are excluded. Original IC keeps its existing adaptive collection/Wiedemann;
both use incidence/shared. Legacy report calibration counters are emitted
separately and excluded from the IC primitive subtotal, while process wall
includes that spent work. Candidate caps are8192residuals/32accepted descents,
with no oldsolvefallback. This is a full-method endpoint check, not a
matched-line speed comparison. [pcc:unverified]

Native containers, PRNG internals, RREF scans/fixed-factor internals and
hashing/serialization remain named gaps; selected sampler integer prices
and unpriced events stay separate. Complete costs/comparative eligibility
remain null/false. No rho arm, A/A, ratio, interval, speed or ranking claim.
These are producer checks without an independent signed verdict.
[pcc:unverified]

*Post-call coverage audit:* the reused RREF implementation bypasses the
modular product/subtraction hooks. Its raw counter records
#display_value(audit.recorded_rref_products)modular products outside the
priced subtotal; subtraction counts were not separately emitted. Its
inversions are already hooked. The original raw and summary stay unchanged;
the arithmetic hooks must be corrected before a primary cost comparison.
[pcc:unverified]

Flow planned/attempted/completed #display_value(s.flow.planned)/#display_value(s.flow.attempted)/#display_value(s.flow.completed),
onevalidation/zero warmup,CPU0/8GiB/oneclaimedthread,900seconds perworker/
1900whole,60seconds fixturecap,noretries. Exactraw/proofs, public and parent
fixtures, immutable driver/qualifier/source, fullsummary/flow/environment
and pre/post/pre-worker hashes: #raw(e.run_path). [pcc:unverified]
