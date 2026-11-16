#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Selected terminal trace moments
T-1934 · Prime fields · 2026-11-08

The optional path computes count and first moment of the terminal product directly, replacing its coefficient vector and Newton reconstruction. Earlier node, predicate and prefix calculations remain; a prefix can already equal the final support indicator.

#image("flow.svg",width:100%)

*Derived pairing.* In Y^L=epsilon, coefficient j of AB sums aᵢ times b at index (j−i) mod L, with factor epsilon when i>j. Iterate the smaller support and look up partners. Count/first moment use positions0 andL−1, coinciding for L1. At most twice the smaller support's size in probes is needed.

*Exact checks.* Validation01 passes1,408 selected/full-product equalities,22 full-rank controls and144 transcript comparisons. Terminal probes1,390 and matched products1,367. Total entered pairs1,145,278 versus1,153,637 for the predecessor; not total-work or timing measurements.

*Public recovery.* All29 outcomes/degrees/call counts match:18 extracted singletons and11 verified identity/endpoints. Selected nominal queries332 and pairs685,238; reference queries332 and pairs689,538. Another27 guard queries bring both arms plus guards to691, excluding source setup/calibration. No supplied image is passed to recovery.

*Large controls.* At L71,303,168 a supplied sparse pair uses2 probes and1 matched product. The actual transcript refuses64 terms at14,621 pairs before any terminal probe. The sparse-operand result qualifies the kernel only.

*Rank boundary.* The constant-coefficient form is an invertible weighted permutation matrix of rankL. A universal exact sum of K products of separate scalar linear forms has rank at mostK, hence K≥L. This bounds a bilinear scheme on arbitrary dense operands, not methods exploiting the particular circuit structure.

*Next and accounting.* Push selected functionals into predicate powers and section circuits, accounting for constructed operands or dual states. Source operations, validation, lookups, wrapping/trace scalings, copies, allocation and verification remain charged. Probe/pair limits are explicit. The output-degree cap is unused by this two-value path.

*Custody and sources.* Fresh overflow-checked build/check pass within90/60 seconds. Twelve copied files and17 dependencies are hash-bound. Bostan-Lecerf-Schost, ISSAC2003, sections3-4: linear transposition leaves nonlinear precomputation. KB unavailable. Unsigned producer evidence; independent review pending. No timing claim or cloud work.
