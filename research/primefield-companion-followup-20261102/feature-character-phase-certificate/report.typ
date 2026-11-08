#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Character-domain certificate
T-1921 · Prime fields · 2026-11-08

The internally constructed character domain already implies its phase equation at every retained geometric point. The specialized initializer and lift therefore omit repeated phase powers while preserving canonical observations, selected orientations and final point checks.

#image("flow.svg",width:100%)

*Proof.* If g divides D_L−2ε and w=v+v⁻¹ is a root, then (vᴸ−ε)²=vᴸ(D_L(w)−2ε)=0. In a field, vᴸ=ε. Canonical endpoint splitting matters: the raw quadratic quotient at w=±2 contains nilpotents, so the proof is about reduced fibers.

*Lift.* Vᵈg(V+V⁻¹) has simple roots except that included endpoints produce (V−1)² or (V+1)². Divide each included endpoint factor once. All remaining roots satisfy the phase already. Retain the original 2d cap and selected-orientation filter. An arbitrary-domain negative control shows why provenance is required.

*Qualification.* 641 lift equalities, 1,282 endpoint controls, 641 phase no-op signature comparisons and 641 cap checks pass. There are 23,552 direct-sign support comparisons and 3,600 public recovery cases, including 228 odd-L cases. The 3,372 order-281 public cases retain predecessor outcomes, routes and source statistics. PCC-20261108-33fb0d is unsigned producer evidence; independent review pending.

*Custody.* Validation01 failed to compile because generated counter names collided; no tests ran. Snapshots and stderr are preserved. The fresh validation02 binary passes focused and inherited checks. 135 copied files and 151 dependencies are hash-bound; inherited stdout is byte-identical. Limits: 90/60/60 seconds.

*Remaining work.* Initial domains still grow with L; reciprocal substitution still has a quadratic dense loop. All validation, inversions, copies, source extraction, replay and point checks remain charged. Next implement balanced homogeneous substitution using the recurrence in the derivation. No timing ranking or cloud run.

*Sources.* Predecessor character/canonical/reciprocal modules and NIST DLMF18.5.1–2. Research KB unavailable; no novelty claim.
