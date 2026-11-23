# Multiplier order and rational polarized identifications for the659 family

[pcc:unverified] Proposed lattice deduction under the established ordinary, rank-one cyclotomic Tate, canonical pairing and End_F421(J)=O=Z[zeta7] premises. Independent mathematical review remains required. Arithmetic checks below support the deduction; they do not construct any curve, theta null or geometric isogeny.

## The quotient multiplier order

Identify the local Tate module with a rank-one O_659 module. For a kernel H, the quotient Tate lattice is L_H=T+659^-1*tilde(H). An element a in O preserves L_H iff its reduction preserves H. Any rational endomorphism of the quotient lies in K=Q(zeta7) and is integral, hence in O; away from659 the source/quotient Tate lattices agree. Thus its endomorphism order is exactly the preimage of this stabilizer.

For the mixed kernel rows(1,t,0,0,0,0),(0,0,-t,1,0,0), e_gamma with t!=0, a diagonal cyclotomic element preserves H iff a410=a389 and a307=a144. The other two coordinates are unrestricted. Therefore all1316 kernels have the same order S, of index659² in O. The conductor in O has zero residues at410,389,307,144 and arbitrary residues at55,12, so its norm is659^4. The discriminant is disc(S)=-7^5*659^4.

The integer order basis, ring/conjugation closure, conductor ideal, discriminant, and every kernel's independent pairing-derived stabilizer are checked in the gate. This is an actual coefficient-lattice calculation, unlike a geometric coordinate kernel. An order does not determine a polarized variety.

## Within each family: exactly94 rational polarized classes, conditionally

Suppose the quotients for slopes t,t' in the SAME gamma family are rationally polarized-isomorphic. Compose that isomorphism with the source quotient isogenies to obtain a rational quasi-endomorphism u in K with u*bar(u)=1. Its denominators can be supported only at659. At each of the six local split coordinates, the projection of L_H is659^-1 Z_659 for the four mixed coordinates and the selected gamma coordinate, and Z_659 for the unselected coordinate. These projection ideals are identical for t,t'. Thus multiplication by u preserving the two lattices forces valuation0 at every coordinate. Away from659 u is also a unit. Hence u is an O-unit.

A norm-one unit has absolute value1 at every complex embedding, so is a root of unity (Kronecker); in K these are +/-zeta7^i. Minusone acts trivially on subspaces. Sigma acts on slope by rho=389/410 mod659, a primitive seventh root. Its action is free on F659*: there are658/7=94 orbits, and source automorphisms realize every identification in an orbit. The argument also excludes further rational polarized identifications WITHIN a family. This is not merely treating kernel count as quotient count.

## The two families are identified by a norm-one quasi-endomorphism

Let alpha generate the prime above659 selected by sigma=55, with norm659. Classnumberone ensures existence; the prospective gate searches a fixed bounded coefficient box and may fail to find a representative. If found, u=alpha/bar(alpha) has normone and local valuations+1 at55, -1 at12, and0 at the other four coordinates. Multiplication by u exchanges the selected gamma projection while preserving the other projection levels. On the mixed coordinates it maps slope t to c*t, where c=u389/u410 mod659; its dual action maps the annihilator line correctly because u*bar(u)=1.

Therefore u maps the first family's lattices isometrically onto the second family's lattices, with the induced slope change. These are prime-to-characteristic lattices; integrality of the induced map on these local lattices and units away from659 gives a rational isomorphism of the quotients, not a source automorphism. This may also be phrased in Deligne's ordinary polarized-module category.

The map t->c*t commutes with rho and is bijective. Consequently the conditional rational polarized quotient count of the entire1316 mixed-kernel family is94, not1316 or188. This does NOT classify the eight O-stable kernels, geometric isomorphisms over extensions, every principally polarized variety in the isogeny class, or curves with a different polarization.

## What a norm witness checks

The gate checks exact norm(alpha)=659, a unique zero at55 modulo659, and computes w=659*alpha/bar(alpha) by exact Cramer inversion. The identity w*bar(w)=659² proves normone for u=w/659. It checks the induced bijection on the94 slope orbits. Local valuation facts follow from the prime norm and unique residue zero, not a finite-precision division by zero. No value of u at the zero embedding is obtained by dividing residues.

## Sources and next construction obligation

Primary: S.Marseglia, Computing square-free polarized abelian varieties over finite fields, https://arxiv.org/pdf/1805.10223 , sections2–5 on fractional ideals, multiplier rings and polarized ordinary varieties. It explicitly separates ideal-theoretic data and period-matrix computation from curve equations; its fractional-ideal framework does not make every ideal invertible. Prior Milne sources and classnumber/unit proofs are linked in cyclotomic-self-quotients/THEORY.md. KB source:4d8b2562e8262e324820aca6600a116e is an unreviewed Waterhouse pointer; source:42b1dc462f0364482a276b87e8b59558 concerns powers of elliptic curves and is not applied to this simple degree6 field.

This specialization is our independently unreviewed deduction, not a theorem attributed verbatim to the references. A constructive next step must identify the source polarized CM lattice/CM type consistently with reduction, construct a representative over-lattice and principal Riemann form, and provide a bounded theta/CM/p-adic route back to F421. None is supplied by an endomorphism order alone. Non-hyperellipticity, explicit target transport and complete IC cost remain separate obligations. No speedup follows from reducing the candidate-class count.
