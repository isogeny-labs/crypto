# Tame odd automorphisms do not repair absolute-simplicity coverage in genus four

This is a structural route screen, not a new priority claim, an attack lower bound, or a measured DLP speedup. Work over an algebraic closure k of characteristic different from2. Let C be a smooth hyperelliptic curve of genus4 and let sigma be an automorphism of nontrivial ODD order n prime to the characteristic. Then J(C) is geometrically nonsimple.

## Branch-orbit proof

The hyperelliptic involution iota is central and has order2. The induced automorphism on the hyperelliptic base P1 therefore still has order n. A tame cyclic P1 action is conjugate to x→zeta_n*x, with two fixed points0/infinity and all other orbits of length n. There are ten hyperelliptic branch points. Write10=mn+r, with0≤r≤2 fixed branch points. For oddn>1 the only possibilities are n=3,5,9.

For n3, r1: move the fixed branch point to infinity. The curve has form y²=x⁹+a*x⁶+b*x³+c with c nonzero and smooth cubic in z=x³. The degree3 map (x,y)→(x³,y) has elliptic quotient y²=z³+a*z²+b*z+c. Its pullback produces a proper positive-dimensional abelian subvariety of J(C), so J(C) is not geometrically simple.

For n9, the same argument gives y²=x⁹+c. Quotienting by its order3 subgroup gives the elliptic map already derived in holdout/COVERAGE-THEORY.md. Again there is a proper elliptic factor.

For n5, r0: two branch orbits give a model y²=(x⁵−a)(x⁵−b), with a,b nonzero and distinct. Choose r,s in k with r⁵=ab and s²=ab, compatibly so s²=r⁵. Then

 tau(x,y)=(r/x, s*y/x⁵)

preserves the curve: f(r/x)=ab*f(x)/x¹⁰, and tau²=identity because s²/r⁵=1. It is not the hyperelliptic involution. At x²=r, exactly one of the two x values has x⁵=s; its two nonbranch points are fixed by tau. The other has x⁵=−s and no fixed point. Neither0norinfinity is fixed. Smoothness/a!=b excludes branch points there. Thus tau has exactly two fixed points, and the tame degree2 Riemann–Hurwitz formula gives

 2*4−2=2*(2*g_quotient−2)+2, hence g_quotient=2.

Its genus2 quotient supplies a proper abelian surface factor of J(C). This proves the assertion in all cases.

The construction/branch action is classical. Gutierrez–Shaska, Hyperelliptic curves with extra involutions, https://arxiv.org/pdf/math/0601456, sections2 and Lemmas1–2, supplies the primary normal-form/extra-involution setting. The explicit n3/n5/n9 argument above is the scope deduction used here, not a claim that the source states this exact genus4 corollary. Guralnick–Howe https://arxiv.org/abs/0804.0578 gives complementary automorphism characteristic-polynomial context.

## What this does not rule out

This is geometric nonsimplicity over k, not a statement that every factor descends individually to the original Fq. In the n5 case r and s may require an extension. An Fq-simple but geometrically split variety is not excluded. The theorem does not cover wild automorphisms, even-order automorphisms, non-hyperelliptic curves, non-automorphism endomorphisms/correspondences, or methods without this symmetry. It does not prove generic tenfold index calculus impossible. It does rule out obtaining an absolutely simple genus4 control merely by switching the present odd-order orbit construction to another tame odd order.

## A distinct even-order candidate, not a certified replacement

The smooth odd-polynomial family

 C: y²=x⁹+a7*x⁷+a5*x⁵+a3*x³+a1*x, a1!=0

admits sigma(x,y)=(-x,i*y) when i²=−1. Here sigma²=iota and sigma has order4, so the only nontrivial proper subgroup in this cyclic group is the hyperelliptic involution. Its quotient is P1; this cyclic action alone supplies no positive-genus intermediate quotient. The polynomial identity f(-x)=−f(x) verifies the map. Over Fq it is rational if q1mod4 and i is chosen in Fq.

This absence of a forced intermediate quotient is NOT a proof of absolute simplicity or absence of other automorphisms. The rational degree1 factor-base signed orbits have size2 after identifying negation, rather than5/7/9 in the old family. On a qualified cyclic prime subgroup an action eigenvalue must satisfy lambda²=−1; the old character certificate pi=P(sigma) is NOT automatically valid, since its dimension assumptions have changed. A replacement needs exact curve/group order, scalar-compatible action/projection, actual subgroup strength, independent simplicity/Frobenius evidence and complete charged comparison. No log/eigenvalue/free factor-base identification can be imported from the old proof.

Decision: investigate this class only under a new prospective proof/qualification protocol. Do not promote it as a valid generic replacement or a speedup until those missing obligations are met. The broad10xgenus2+ objective remains open.
