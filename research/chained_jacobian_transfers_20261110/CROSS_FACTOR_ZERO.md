# Explicit second elliptic quotient and a vanishing cross-factor composite

Let C:y²=x⁶+1 over k of characteristic ≠2,3. There are two degree-two maps to E:v²=u³+1:

- f(x,y)=(x²,y).
- g(x,y)=(x^{-2},y/x³), interpreted as a morphism on smooth projective models, with x=0 mapping to infinity.

The deck involution of f is σ(x,y)=(-x,y). For x≠0, g(σ(x,y))=-g(x,y) on E. Thus on a generic f-fiber, g(P)+g(σP)=O. As homomorphisms of Jacobians, **g_* f^*=0**, since the composite vanishes on the generating classes [Q−O] after extending to an algebraic closure; equality then descends to k. This is a concrete negative control: the two elliptic quotients exist, but their cross-factor DLP transport vanishes.

The other composite is f_*f*=[2], so the same genus-2 curve simultaneously exhibits a successful round-trip and an unsuccessful cross-quotient map. The successful nonisomorphic endpoint in NONISOMORPHIC_TRANSFER.md is obtained by a separate 2-isogeny after f_*, not by g_*.

`python3 verify_fibers.py` checks rational split fibers of f over F_101, validates both maps land on E, and confirms f(P)+f(σP)=[2]Q and g(P)+g(σP)=O. This is a direct geometric fiber check, **not** an independent implementation of Jacobian divisor-class arithmetic. The general identities above are proved algebraically, including nonsplit fibers.

## Remaining implementation work
A complete independent genus-2 Jacobian implementation (Mumford/Cantor arithmetic, projective infinities, induced f* and f_* on arbitrary classes) is not included. Nor is an exhaustive search for a nonzero cross-quotient map beyond this bielliptic family, or any cost-improving DLP transfer.
