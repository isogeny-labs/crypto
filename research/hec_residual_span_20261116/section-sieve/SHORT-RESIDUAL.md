# Short target-bearing residuals: what can actually vary

This is a classical geometric admission constraint, not a new impossibility theorem for index calculus or a performance measurement. It applies to smooth odd-degree hyperelliptic models y²=f(x), deg f=2g+1, odd characteristic, with rational infinity, as used by this work. It explains why lowering the residual degree in a fixed-anchor sieve is not automatically a new yield mechanism. [pcc:unverified]

## Genus2: Riemann–Roch gives the exact dichotomy

For a degree2line bundle L on a smooth genus2curve, Riemann–Roch gives

    h0(L)=1+h0(K tensor L^-1).

The bundle on the right has degree0. A nonzero section of a degree0bundle has no zeros, so it trivializes the bundle. Therefore h0(L)=2 if L=K, and h0(L)=1 otherwise. There is no degree2moving family in a noncanonical class, even over the algebraic closure. For the odd quintic model, K is equivalent to2infinity; its canonical pencil consists of hyperelliptic fibers P+iota(P). Their class relative to2infinity is0.

Consequently, for a nonzero anchor R, decompositions E of degree2with [E-2infinity]=-R have a unique effective representative if one exists. A degree2pencil cannot manufacture many target-bearing decompositions of that fixed class. This does not say that its unique representative is necessarily split over the base field.

By contrast every degree3bundle has h0=2, since K-L has negative degree. Degree3is the first degree allowing a moving system in every target class. This recovers the existing genus2degree3residual pencil without proving its computational optimality.

## All retained genera: pair removal and reduced uniqueness

Let E be a fully rational effective affine divisor of degree e<=g, with multiplicities. Remove all possible conjugate pairs P+iota(P); if P is ramified, a pair is2P. Each pair has class0 relative to2infinity because div(x-x(P))=P+iota(P)-2infinity, with its ramification multiplicity. The remaining divisor E0 is semi-reduced of degree<=g and hence reduced. In particular

    [E-e*infinity]=[E0-deg(E0)*infinity].

For a fixed Jacobian class, the reduced Mumford representative is unique. Therefore every such E has the SAME E0. Any variation is only the addition/removal of conjugate pairs whose signed factor-base rows cancel exactly. In an odd-prime-order quotient, a rational ramification point has zero projected logarithm, consistent with its order2; its pair likewise adds no information. Identity/zero-degree and multiplicity cases are included, not discarded.

For the full signed factor base already used by the controls, after identifying logs(iota(P))=-logs(P), every fully split E of degree<=g in a fixed class gives the same reduced decomposition row. Certified automorphism contraction can only identify more rows, not revive the cancelled ones. Hence this specific short-residual variation supplies at most one distinct folded row per fixed anchor. It cannot supply an arbitrary number of independent rows merely by varying a low-degree plane pencil.

This is not a statement that all relations in the index calculus have degree>g. Ordinary Gaudry collection deliberately samples NEW anchors of degree<=g. It is the attempt to vary many decompositions of ONE fixed class that is constrained. A different factor-base membership rule excluding E0 cannot get around the conclusion by adding cancelling pairs: E0's support is unchanged.

## Singular plane quartic proposal

Suppose a genus2curve is presented as the normalization of a plane quartic, and a pencil of lines through a double point has a fixed pullback divisor B of degree2. Let the pullback hyperplane bundle be A, degree4. Its moving residual bundle is A(-B), degree2, with at least two sections. The genus2dichotomy forces A(-B)=K. Thus this particular projection pencil gives canonical residual pairs. The fixed divisor and plane-model hyperplane class must be retained; neither turns these pairs into arbitrary target-bearing decompositions. More complicated singularities/base loci need their own pullback calculation, not blind use of this example.

## Scope and next admission criterion

A promising next mechanism must do at least one of the following explicitly: generate many different tracked anchors more cheaply; use residuals above degreeg with a justified yield advantage; exploit nontrivial endomorphism/isogeny geometry with actual DLP transport; or share evaluations without pretending identical reduced rows are independent. This note does not exclude any of these, nor prove a general runtime lower bound or preclude a constant-factor10xgain. Genus2coverage cannot be silently replaced by a nonhyperelliptic genus3plane-quartic result.

No scientific run is required for this exact algebraic argument. No source/target/benchmark record is changed. General mathematical10xgenus2+goal remains open.

## Sources and review boundary

The Stacks Project, Lemma53.5.2, Riemann–Roch and duality over an arbitrary field, https://stacks.math.columbia.edu/tag/0BS6 , read as the finite-characteristic foundation. The degree2dichotomy follows by the displayed specialization and the degree0section argument above; it is not inferred by transferring a complex-only theorem.

Improved sieving on algebraic curves, sections2.1–2.3, https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/SievingB.pdf , defines semi-reduced/reduced Mumford divisors and the unique reduced representative; the short-residual conclusion is the explicit pair-removal argument supplied here. It is a classical-method consequence, not priority.

KB lexical query `genus 2 degree two linear system canonical hyperelliptic plane quartic index calculus`, revision9b2119e8fbac482b98b50521c0d604bb: source:37965b1557e457ad28a6ada00ccc8c27 is a Diem plane-curve literature note; source:a687b9162f8a1ad741edd2cf35e177d8 is a project paper mentioning genus3plane-quartic routes. Neither supplies a genus2speedup or independent proof of this note. Their performance prose is not adopted here. No absence/novelty claim follows. Proof is for mathematical review, not formally checked certification.
