#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Full orbit-word projector and all-mode phase
T-1940 · Prime fields · 2026-11-08 · Unsigned producer evidence

*Construction.* Retain all raw orbit signs. For candidate character t and observed sign s, use (t²+st)/2, which rejects t=0. Multiply filters in X^m=Y. Average over rotations: the zero section times m divided by the observed rotational stabilizer is the exact image indicator. Generic zero-testing powers are removed; character powers and explicit node representatives remain.

#image("flow.svg",width:100%)

*Phase.* Unnormalized Fourier mode e gives m k^e Aₑ(Y). Nonzero modes provide powers k^e. Extended gcd combines them with k^m=Y. The final gcd equals the joint rotational stabilizer. Gcd1 yields a scalar, checked against the public point; larger gcd stays unresolved. Synthetic period2/period3 words demonstrate an all-mode anchor with vanishing oriented modes, without asserting a positive curve transcript of that form.

*Validation02.* 220 systems;1,140 exact image memberships including L1;164 phase checks;3 symmetric images;6 strict support reductions versus three-mode equations. Another96 Bezout identities,6 zero-safe selector checks and4 capacity controls pass. No compile/assertion failures; validation01 source and logs retained before adding L1 coverage.

*Public cases.* E/F17, a2 b2 P(5,1) r19 m3, all19 scalars; E/F257, a1 b7 P(1,3) r281 m5, ten fixed scalars. Radii[2,1,3,2]. All29 return checked scalars:18 extraction and11 identity/endpoint. Nominal332 source calls,27 guard calls,2,600,842 monomial pairs. Prior selected-terminal path recorded685,238 pairs on these fixtures. No timing or total-work ranking; source setup/calibration, shifts, DFT, gcd/inversions, copies, allocation and verification add costs.

*Large control.* r998244353, m7, L71,303,168:11,379 nodes; fixture-image evaluation passes. Unknown-image extraction refuses64 terms after14,594 pairs and5,696 stored slots. The indicator singleton support bound still requires L coefficients.

*Custody and next.* Ten copied sources/14 compiler dependencies hash-bound; bounded90-second compile/60-second checks. Original4,563-case cohort unchanged. Next push selected trace operations through structured word-product circuits with intermediate-state accounting. Derivation and exact logs accompany this report. KB unavailable. Fourier convention checked against FFTW DFT manual; finite-field orthogonality/orbit argument proved in DERIVATION.md. Independent signed review pending.
