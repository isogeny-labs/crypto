#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Adaptive sparse-fiber character decoder
2026-11-05 · T-1648

The native decoder reads 15 source signs and requests the sixteenth only on two complete exceptional fibers. Its 471-term cubic polynomial returns the field mean within each pair and the input elsewhere; a conditional character correction restores the input. [pcc:unverified]

#image("pipeline.svg", width: 100%)
The branch counts cover the complete F479 input set, not a random sample. They count requested sign values, not time or field operations. Evidence: test.log and checker-positive.log. [pcc:unverified]

== Exact construction
With delta_t(X)=1-(X-t)^478, form F(X)=X+74 delta_25-74 delta_173+74 delta_292-74 delta_440. F takes the means 99 and 366 on the two supplied pairs. If M tests their first-15-sign signatures, the identity is X=F(X)+405 M chi(X+15). The old signatures are equal inside each pair at every feature degree; increasing degree alone cannot fix that window. [pcc:unverified]

The constructor validates disjoint complete fibers and a separating extra sign, builds a centered odd target, and checks cubic-span membership. Runtime retains coefficients and sign predicates, not witness scalars or an answer map. The public-point adapter uses E/F509, a=2, b=10, generator (3,68), order479, and accepts Q without its logarithm. It extends the last shifted point and computes one extra source sign only when needed. [pcc:unverified]

== Recorded checks
49 optimized overflow-checked tests pass. The fixed decoder recovers all479 field inputs and makes extra requests exactly at25,173,292,440. Eight public-point integrations pass, including all four exceptional inputs. Invalid pairs, signatures, parity, extra signs and source errors are tested. Twenty source/data locks bind the run; seven compiler warnings are retained. [pcc:unverified]

A separate checker imports no producer code. Ordinary polynomial expansion checks F, square-set characters check the adaptive identity at all inputs, and a coefficient-sign corruption is rejected. This separate implementation is from the same producing session, not signed independent review. [pcc:unverified]

== Cost and next work
The full-field count is 15 times475 plus16 times4 =7189 sign values. Setup, dense construction, predicate work, bitplane evaluation, point addition and direct fallback phase remain additional costs. The adapter currently prepares window state per recovery. Next retain reusable preparation and use the last two seed pairs for the exceptional extension. No timing panel or cloud job ran; the original4563-case cohort is preserved. [pcc:unverified]

== Evidence standing and sources
Claims PCC-20261105-12d0e7 and PCC-20261105-db06e4 have local verdict not_covered and no signed standing. See DERIVATION.md, claims.json, raw logs, receipts and source-lock.json. Prior work: saved fiber-repair and signature-selection records; #link("https://dlmf.nist.gov/27.9")[NIST DLMF27.9] for character conventions. No novelty-priority claim.
