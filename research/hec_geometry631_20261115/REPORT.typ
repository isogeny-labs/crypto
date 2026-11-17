#set page(paper:"a4",margin:18mm)
#set text(size:11pt)
= Fresh quotient geometry over F631
Genus 3 · 15 November 2026
#image("flow.svg",width:100%)

Both fixed kernel cases reconstruct smooth quartic models from fresh F631 functions and normalization. Kernel0 matches the source point-trace sign; kernel1 has the opposite sign and is excluded from the current source-transfer path. Both outcomes are retained.

#table(columns:5,inset:5pt,[Kernel],[Gamma00 rank],[Quadric rank / rows],[Points],[Sign],
 [0],[7],[36 / 64],[597],[Source],
 [1],[7],[36 / 64],[667],[Opposite])

== Exact local construction
Fresh principal functions, primitive cube root and character trace basis produce the eight descended sections. Degree-four jets pass exact curve residual, base-field descent and odd-coefficient vanishing. The unique rank7Gamma00relation yields the quartic; both smoothness checks pass. All64source-multiple section samples support rank36globalquadric exclusion. Jet-ring controls pass.

The source has a complete rational2-primary group of order4 and one degree-three2-torsion divisor, u=[436,451,7,1],v=0. Projection of the first source place has order4, so one doubling is needed before the localorder2chart. Both cases record projection depth1. The original order2projection assumption would fail here.

== Newly constructed coefficients
Kernel0: HC=[156,615,212,277,71,173,170,1]; QF=[1,627,254,511,40,66,318,33,29,463,86,201,389,519,310].

Kernel1: HC=[35,630,606,77,268,402,363,1]; QF=[1,550,515,146,323,263,170,121,519,1,564,451,2,341,432].

No expected target coefficients were supplied. Full coefficient files and per-case stdout/stderr/status are retained under run/.

== Custody and next step
Sourcea70cc5d9dd88/checkpoint014404Z preceded the once-only two-process cohort. All statuses0 and stderr empty; no kernel was discarded or silently retried. Point counts597and667 distinguish the source/opposite traces here. Point trace alone does not independently establish the full target L-polynomial, group order or actual transfer map.

Kernel0 now supplies a concrete target model for correspondence and subgroup verification. Actual public-input images, calibration, complete destination IC and source/destination scalar verification remain. No timing, exponent, novelty or tenfold gain is claimed. All construction/control costs remain chargeable. Genus two/four and the full objective remain open; the earlier failed charged comparison is unchanged.
