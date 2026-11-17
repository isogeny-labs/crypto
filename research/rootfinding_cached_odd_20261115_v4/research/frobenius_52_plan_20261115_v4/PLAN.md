# Fresh unexecuted reduction: the cubic constant coefficient is pure Frobenius

This is a symbolic derivation/next hypothesis, not measured performance, novelty, or a new complete root solver. All current frozen workers/evidence remain unchanged. Keep regular guards a!=0,z!=0,a!=z,B!=0 and the fixed rational source ordinate/target. Since z is outside additive V containing0, z!=0 is automatic.

Let D=(X+z)(X+a)² and Q=N/D. At X=0, D(0)=z a² and N(0)=t(k)²+B k². Because t=t0+t1*k is affine over K, the cubic constant coefficient obeys

    U(k)=[t0²+(t1²+B)k²]/(z a²).

Thus betaU is IDENTICALLY ZERO for every regular anchor, not just the a=1 torsion case. Independently, put c=a+z, s1=ya/a+a, r1=(ya+s1*c+yT)/c² and R=z*r1+s1. Expanding the existing formulas gives r0=z+hR and betaU=r0+h*s1+z*(1+h*r1)=0. This checks the identity by a second route. Hence Delta=alphaU*betaT. A future source variant must verify all these identities against literal norms and retain alphaU=0 exceptions.

When alphaU!=0, Frobenius is a field bijection. Define eta=(sqrt(alphaU))^-1, k0=eta*sqrt(gammaU). Then

    k=k0+eta*sqrt(U).

Parameterize U directly by an independent basis of W3, dimension52 for the actual low18 domain. This parameterization is injective and exactly enforces U in W3. Impose ONLY T(k) in W2, giving at most48 binary equations on52 support coordinates, rather than at most79 equations on83 k bits. The complete affine family is equivalent after the explicit coordinate map; a rank/nullity observation remains measured rather than assumed. Candidate filtering still needs exact cubic support and all output branches. The forbidden k=ha must be removed only from the declared regular chart.

Do not compute52 square roots afresh for every cell. Cache sqrt(W3 basis) once per field/domain and pay its cost in every compared workload; alphaU depends on a/ya/target through r1 but not on h, so eta can be shared across fixed-anchor h cells after paying its setup. All transformations/conversion/full-family recovery, literal verification and output costs count. Compare against the strongest cached explicit current implementation, not only the old uncached one.

If alphaU=0, U is the constant gammaU. If gammaU is outside W3 the entire conditional coefficient family is empty; otherwise the U constraint imposes no restriction on k. Retain the original83-k formulation/complete affine family or UNKNOWN under the cap. Never divide by zero or assert a four-dimensional bound in this exception. Generic-anchor controls and both alphaU branches are required; an unobserved exception remains a gap.

The prospective improvement is smaller exact linear algebra and cheaper shared operator setup. It does not remove approximately2^18 admissible h times roughly2^18 source-a outer choices, original auxiliary/ghost branches, odd-characteristic work or the genuine F4/F5/F6 release requirement. New source, independent code review, frozen hashes/inputs and a bounded declared experiment are required before execution.

Future independent small oracle: over GF4/GF8/GF16 exhaust every alpha,gamma and every k for U=alpha*k²+gamma. Nonzero alpha must yield a bijection and exactly the same mapped complete family; alpha=0 must produce the full k field or an empty coefficient family according to constant membership. Include U=0 and nonzero gamma, repeated/zero-root cubics and both ordinate/torsion guards. Compare mapped complete affine families and every Q/R/R4/source ordering to the unchanged83-k implementation before timing. A single mismatched family or missing exception refutes the implementation. New generic-anchor input must be source-only selected before observations. The strategic priority remains the full affine-rank4/whole-cell route; this is a component optimization.
