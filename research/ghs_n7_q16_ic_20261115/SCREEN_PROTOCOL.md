# Frozen q16 search inside the published n7 genus-eight selector

15 November 2026. Freeze this candidate screen before computing any elliptic-curve orders at q16. It follows the already checked q4 selector, not a proposed new GHS family. The aim is to choose a larger, full-absolute-field exact equation for a separate subgroup-map and complete attack-cost test. [pcc:unverified]

## Deterministic field and coefficient selection

- Set `K=F2[z]/(z^28+z^3+1)` and `q=16`. Sage has checked this polynomial irreducible before this protocol; that check did not compute an elliptic-curve order.
- Enumerate the 16 elements of the unique `F16` subfield as powers of `z^((2^28-1)/15)`, plus zero. Choose `a` with the smallest polynomial mask among those of absolute trace one.
- Form the two published cubic Frobenius kernels `V1=ker(Frob_16^3+Frob_16+1)` and `V2=ker(Frob_16^3+Frob_16^2+1)` as linear maps on the 28-dimensional F2 vector space K. Verify each has F2 dimension 12 and their intersection is zero. For each kernel, enumerate nonzero vectors by increasing polynomial mask and set `b=v+c` for the smallest nonzero subfield element `c` by mask. Keep only coefficients whose minimal absolute F2 field is K.
- Combine both arms, remove duplicates, and sort by `(arm, mask(b))`. Compute `#E(K)` for the first 128 retained candidates where `E:y^2+xy=x^3+a x^2+b`. Factor each order and record the largest prime factor `r`, cofactor, Frobenius trace and exact masks. Stop at 128 even if no candidate is suitable. A screen candidate has cofactor at most eight and `r` at least `2^24`; choose the first such candidate by the frozen order. If none qualifies, report a null screen. [pcc:unverified]

## Checks and claim boundary

Recompute the kernel equations, trace of `a`, nonzero relative trace of `b`, absolute field degree of `b`, curve nonsingularity, Hasse bound, and factorization of every retained order. Save the exact Sage version, script digest and results. Do not infer a subgroup map, genus, new-family status or rho advantage from this screen. The separate full attack protocol must fix the exact chosen equation, point, target, map, factor base, stopping rules and matched rho before its first relation measurement. [pcc:unverified]

Sources checked (KB): revision `9141e386254340a9bdba3036f2bb706a`; `source:8618328474ca57d3a61b1a8832d41daf` (known binary composite GHS lead) and `source:23ec376126e8b485c89c2064b8a8f518` (conditional cover-cost rows). Primary family source: [Teske's n7 selector](https://www.lix.polytechnique.fr/Labo/Francois.Morain/Crypto/IsogenyParty/teske.pdf). The exact q4 predecessor is [the prior stratum report](../ghs_n7_traceone_genus8_20261115/STRATA_REPORT.md).
