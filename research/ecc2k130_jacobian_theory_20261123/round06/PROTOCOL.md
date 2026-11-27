# Quadratic-cover Prym reduction

2026-11-23. Same active goal/draft PR5079. Scope: a smooth
geometrically connected separable degree2 cover X->D over F2, for which
the entire connected norm kernel P is F2-isogenous to the specified A.
No assumption that the natural representative is preserved. This does
not cover arbitrary embeddings of A into a larger Prym.

Derive the characteristic2 zeta congruence from split/inert/ramified
local Euler factors. Use L_A mod2=Phi131 and ord131(2)=130 to classify
the possible branch products and reduced base numerator. Combine with
dimension(P)=130 and wild Riemann–Hurwitz. Check all resulting integer
genus and ramification cases natively; use the existing round05
hyperelliptic exclusion to reject the genus130 option.

Also derive degree2 fiber inequalities for the two possible larger
genera. Test the tempting base classes J(D)~A and J(D)~A*E0 by exact
counts; no curve is constructed by such a formal product.

Keep polarization separate: read the characteristic2 primary source
Achter–Casalaina-Martin, Putting the P back in Prym, rather than
assuming the characteristic-not2 ramified-cover formula. No curve
search, Jacobian point arithmetic, DLP solve or performance claim is
planned. Native C++/GMP and PARI static certificate checks only.

Additional written check: in the etale genus261 option, an ordinary
hyperelliptic base has132 simple branch points. Partitioning them gives
all geometric etale double covers, by the Artin–Schreier description and
the2-rank count. Check the two-factor Prym decomposition and its
Frobenius descent. Do not assume the characteristic-not2 bigonal
construction applies to the ramified option.
