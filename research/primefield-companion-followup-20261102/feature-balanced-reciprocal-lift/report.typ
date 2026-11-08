#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Balanced reciprocal substitution
T-1923 · Prime fields · 2026-11-08

The optional recovery adapter replaces the reciprocal lift's growing coefficient loop with balanced homogeneous recursion. Character-domain provenance, phase-free initialization, endpoint multiplicity correction and final point checks are retained.

#image("flow.svg",width:100%)

*Recurrence.* For a declared degree d and split a=low+Wᵏ high, define R_a=Vᵈa(V+V⁻¹). Then R_a=V^(d−k+1)R_low+(V²+1)ᵏR_high. Split coefficient lengths in half, reuse powers within one transform, and retain declared lengths even when high coefficients vanish.

*Arithmetic scope.* With regular multiplication cost M(n), the standard balanced bound is O(M(n) log n). The former dense coefficient loop was quadratic. No timing crossover is selected. Cached powers are copied; shifts, map operations, polynomial scratch storage, allocations and output remain charged. Initial character domains still grow with L.

*Qualification.* 1,728 coefficient equalities and 6,912 point identities pass over four fields, with input lengths through 1,025 and zero/sparse/dense patterns. 641 phase-certified lift comparisons, 1,282 endpoint controls and 641 cap controls pass. All 3,600 public cases recover correctly, including 228 odd-L cases; 3,372 predecessor cases retain outcomes, routes and source statistics. PCC-20261108-1228b4 is unsigned producer evidence.

*Custody.* The fresh build and focused/inherited checks pass under 90/60/60-second bounds. No failed attempt occurred. 140 copied sources and 157 compiler dependencies are hash-bound; inherited output is byte-identical. The large-prime polynomial fixture is not a large-prime recovery run. Independent signed review pending.

*Next construction.* An observation-derived low-degree polynomial could allow phase intersection using D_L modulo that polynomial, avoiding dense D_L construction. Producing such a useful polynomial from public observations without degree-L work remains the missing construction; fixture-derived polynomials do not establish it.

*Sources.* Exact predecessor certificate/lift modules and retained NIST DLMF18.5.1–2 reciprocal identities. No novelty or timing claim.
