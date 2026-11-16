#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Direct squarefree phase domains
T-1913 · Optional prime-field recovery initializer · 2026-11-08

Positive phases now use a shared pair of second-kind polynomials to construct their squarefree domains directly. This avoids the larger Dₙ polynomial and outer squarefree extraction. Negative phases share the already-squarefree Dₙ. Canonical squarefree validation remains in place.

#image("flow.svg",width:100%)

*Construction.* Let t=floor(n/2), a=Sₜ and b=Sₜ₋₁, where Sⱼ=Uⱼ(W/2). For even n, the positive phase domains are (W²−4)b and 2a−Wb. For odd n, they are (W−2)(a+b) and (W+2)(a−b). One division-free doubling recursion produces a,b. Explicit W±2 factors preserve ramified endpoints, including n=1.

*Representation and costs.* Positive factory intermediates have degree at most floor(n/2)+1. Negative support still has degree n, and its inherited Dickson helper may construct the adjacent degree-n+1 polynomial. Canonical validation gcds, phase powers, quotient arithmetic, source calls, copies, allocations, inversions, replay and final verification remain charged. No timing or total-work ranking is asserted; slot limits are not peak memory limits.

*Sources and proof scope.* Standard reciprocal Chebyshev identities, NIST DLMF18.5.1–2, and the predecessor phase/canonical modules. Written derivation plus deterministic checks; no new Lean runtime refinement. KB unavailable; exact-source fallback documented.


*Qualification.* 325 second-kind recurrence equalities, 992 direct factor equalities and 1,984 ramified endpoint checks pass. Integrated checks pass 15,264 states and all 2,529 public target/capacity cases with equal outcomes, routes and source statistics. Existing phase-degree, capacity and one-call-limit controls remain. 123 copied-source files and 138 dependencies are hash-bound; inherited stdout is byte-identical. PCC-20261108-26a7c5 is unsigned producer evidence; independent review pending.

*Failure retained.* The first build collided with inherited adapter/check filenames and stopped before execution. Failed source snapshots and stderr are preserved. The corrected revision uses unique filenames and a fresh binary; build and checks pass. No cloud or timing panel ran.

*Next construction.* For even n, $D_n = D_(n/2)^2 - 2$ suggests splitting negative domains into two factors after constructing and verifying a public square root of 2. This halves each quotient degree but doubles branches; root setup, duplicated orientation labels and total work need explicit treatment. It is documented, not implemented here.
