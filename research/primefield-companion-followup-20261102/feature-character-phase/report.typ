#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Character-phase canonical recovery
T-1918 · Prime fields · 2026-11-08

The new adapter uses the initial public character directly as its phase label. It admits m dividing h=(r−1)/2, including odd L=h/m, without constructing a fourth root of −1. The source contract and final point verification remain explicit.

#image("flow.svg",width:100%)

*Invariant.* With x=k/b₀ and V=xᵐ, the phase is Vᴸ=ε=χ(k)χ(b₀). In W=V+V⁻¹ the domain is the squarefree part of D_L−2ε. Inversion preserves this character. Canonical fibers retain both orientations together, separate ramification at W=±2, and filter any orientation subsequently distinguished by observations.

*Transport.* At another radius, the variable is cV and its character is εcᴸ. The streaming signed equation retains (−1)ʰ, which matters for odd h. The powered path directly evaluates the signed mode product. Both paths impose mode0 at initialization.

*Qualification.* 1,936 factor controls and 23,552 direct-sign support equalities pass. All 3,600 public cases recover correctly, including 228 cases on the newly admitted order-19, m=3 fixture. The 3,372 predecessor public cases retain outcomes, routes and recorded source statistics. 298 single-call budget cases pass. PCC-20261108-cfa48d is unsigned producer evidence; independent review pending.

*Custody and failures.* Validation01 failed to compile at a test tuple equality: PullStats lacks PartialEq. No tests ran. The snapshot and compiler output are retained. A fresh validation02 binary passes focused and inherited checks; 131 copied sources and 146 compiler dependencies are hash-bound. Inherited output is byte-identical. Limits were 90/60/60 seconds. No timing ranking or cloud run.

*Costs and next step.* Domains still grow with L. All canonical checks, phase powers, inversions, allocations, source extraction, replay and point checks remain charged. The section-slot cap is per branch, not peak memory. Next reduce initial phase-domain work while preserving the exact character and transport invariant.

*Sources.* Predecessor reciprocal/canonical/signed/transport modules and NIST DLMF 18.5.1–2. The derivation and receipt list exact local sources. KB unavailable; no novelty claim.
