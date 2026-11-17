#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Endpoint-corrected orbit-word traces
T-1943 · 2026-11-08 · PCC-20261108-87a4e4 · Unsigned producer evidence

*Construction.* Replace (t²+st)/2 by the linear factor (1+st)/2. Away from the queried offsets the product is already exact. At each distinct public offset, compute its erroneous contribution and subtract it from the appropriate image value and the two trace moments. Repeated factors stay in the product; only the correction summation deduplicates offsets. Initial-sign phases are respected.

#image("flow.svg",width:100%)

*Exact qualification.* Validation01 passes220 paired systems,1,140 image memberships and164 phase checks. Thirty-five image values need nonzero correction. Four explicit endpoint controls test single/repeated factors and both phases. Correction-cap refusal passes. All29 public outcomes match:18 extracted scalars/11 identity-endpoints. Neither runtime receives a hidden scalar or image.

*Accounting.* Transcript polynomial pairs:4,439,003 corrected versus4,841,072 reference; correction adds156,070 field multiplications including16,031 exponentiations. Aggregate nodes203,402 versus222,420. Public correction adds19,844 multiplications including1,730 exponentiations; do not charge those exponentiations twice. Each arm makes332 nominal plus27 guard source queries,718 combined excluding setup. These are fixed-operation diagnostics, not timing or total-work rankings. Source setup, shifts, additions, DFT, maps/copies/allocation, construction, phase inversion/gcd and point verification remain additional.

*Large boundary.* At r998244353,m7,L71,303,168, corrected nodes11,097 versus11,379. Both methods still refuse64 terms after14,594 polynomial pairs. Correction costs15,388 multiplications including415 exponentiations. Fixture-image membership passes; unknown-image extraction remains capped.

*Fixtures and custody.* E/F17,a2,b2,P(5,1),r19,m3, all19 scalars; E/F257,a1,b7,P(1,3),r281,m5, ten fixed scalars; radii[2,1,3,2]. Fresh overflow-checked build/check pass within90/60 seconds, no native failure. Eleven copied files/15 dependencies hash-bound. Original4,563-case cohort unchanged. Source/proof/receipt/logs retained; independent signed review pending.

*Next and sources.* Contract selected traces before explicit phase-quotient expansion, charging intermediate/dual states. Prior T1940 derivation/NEXT and FFTW unnormalized DFT convention checked; finite-field correction proof supplied in DERIVATION.md. KB unavailable. No cloud work or novelty claim.
