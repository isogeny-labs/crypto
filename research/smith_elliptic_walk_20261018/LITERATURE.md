# Q3 literature note: hyperelliptic-to-hyperelliptic genus-3 isogenies

Date: 2026-10-18. Scope: text and citations only; nothing here is implemented.
Status of statements: **[read]** = stated in an abstract or page fetched by
`curl` on 2026-10-18; **[derived]** = our own dimension count or argument, not
a citation; **[not read]** = cited from memory of the area, text not fetched.

## Question

Does a genuine `(2,2,2)` or `(l,l,l)` isogeny `J(H) -> J(H')` between Jacobians
of two *hyperelliptic* genus-3 curves, compatible with the principal
polarizations, exist on a positive-density set of curves over `F_q`, and is any
of them implementable here?

## What the sources say

1. **B. Smith, "Isogenies and the Discrete Logarithm Problem in Jacobians of
   Genus 3 Hyperelliptic Curves", arXiv:0806.2995 (J. Cryptology 2009).**
   [read] "explicit formulae for isogenies with kernel isomorphic to
   `(Z/2Z)^3` ... for any hyperelliptic genus 3 curve over a field of
   characteristic not 2 or 3. These isogenies are rational for a positive
   fraction of all hyperelliptic genus 3 curves defined over a finite field of
   characteristic `p > 3`." The target is a **non-hyperelliptic** Jacobian
   (that is the point: the DLP moves to a plane quartic, where index calculus
   is faster). So the positive-density family exists for
   hyperelliptic-to-**quartic**, not hyperelliptic-to-hyperelliptic. This is
   the construction `smith_prop24` implements (`full_success` density 0.1857).
2. **B. Smith, "Families of Explicit Isogenies of Hyperelliptic Jacobians",
   arXiv:0909.5406.** [read] Hyperelliptic-to-hyperelliptic isogenies exist in
   explicit families built from decomposable pairs `(f,g)` with `f(x1) - g(x2)`
   reducible (Cassou-Nogues, Couveignes). For genus 3 the abstract lists
   **two-dimensional** families. The hyperelliptic locus in genus 3 has
   dimension 5, so those families are of codimension 3 (a share of order
   `q^-3` of the curves over `F_q`) [derived from the stated dimensions].
3. **E. Howe, F. Leprevost, B. Poonen, "Large torsion subgroups of split
   Jacobians of curves of genus two or three", arXiv:math/9809210 (Forum Math.
   2000).** [read] Constructs families of genus 2 and 3 curves with split
   Jacobians and large torsion. It is about Jacobians isogenous to products of
   elliptic curves, not about isogenies between hyperelliptic Jacobians; it is
   relevant here only because the descended curves `H` of the walk are of this
   kind (see "Why the elliptic walk is special" below). E. Howe's
   arXiv:math/0304471 (pairs of curves, one hyperelliptic and one plane
   quartic, with isomorphic Jacobians) [read] is the same flavour: isomorphic
   *unpolarized* Jacobians, a hyperelliptic and a quartic.
4. **J.-M. Couveignes, T. Ezome, "Computing functions on Jacobians and their
   quotients", arXiv:1409.0481.** [read] Quasi-optimal `(l,l)` isogenies
   between **genus-2** Jacobians (odd `l`, maximal isotropic kernel). Genus 2
   only; every ppas isogenous to a Jacobian is a Jacobian or a product, so the
   question "is the target hyperelliptic" does not arise there.
5. **E. Milio, "Computing isogenies between Jacobian of curves of genus 2 and
   3", arXiv:1709.06063.** [read] Velu-style `l`-isogenies (odd `l`) from a
   maximal isotropic subgroup of `J[l]` for genus 2 and genus-3 hyperelliptic
   *domains*, with a theta-function route for non-hyperelliptic domains. The
   target is whatever principally polarized threefold the kernel gives; the
   paper does not claim it is hyperelliptic, and generically it is not
   (item "Dimension count" below).
6. **Song Tian, "Translating the discrete logarithm problem on Jacobians of
   genus 3 hyperelliptic curves with `(l,l,l)`-isogenies", arXiv:2007.03172.**
   [read] `(l,l,l)` isogenies from hyperelliptic genus-3 Jacobians to
   **non-hyperelliptic** Jacobians, for the same DLP translation as Smith.
7. **T. Katsura, "Decomposed Richelot isogenies of Jacobian varieties of
   curves of genus 3", arXiv:2103.01800; T. Moriya, M. Kudo, "Some explicit
   arithmetic on curves of genus three and their applications",
   arXiv:2209.02926; R. Ohashi, H. Onuki, M. Kudo, R. Yoshizumi, K. Nuida,
   "Listing superspecial curves of genus three using Richelot isogeny graphs",
   arXiv:2401.10500.** [read] Richelot (`(2,2,2)`) isogenies in genus 3 whose
   target is a *product* (decomposed), and, in characteristic `p`, Richelot
   isogeny graphs on **superspecial** threefolds where some vertices are
   hyperelliptic Jacobians. These are special loci (decomposed, or
   superspecial), not a positive-density set of ordinary hyperelliptic
   Jacobians over `F_q`.
8. **D. Lehavi, C. Ritzenthaler, "Formulas for the arithmetic geometric mean of
   curves of genus 3", arXiv:math/0403182; and the Bost-Mestre-type AGM
   construction, arXiv:alg-geom/9712027.** [read] The genus-3 analogue of the
   Richelot/AGM step is stated for plane quartics (Lehavi-Ritzenthaler) and
   its genus-2 form by Humbert, Bost-Mestre. The `(2,2,2)` step on a
   *quartic* returns a quartic, which is consistent with items 1 and 6.
9. **Cosset-Robert, "Computing `(l,l)`-isogenies in polynomial time on
   Jacobians of genus 2 curves", eprint 2011/143 (Math. Comp. 2015); Dartois,
   Maino, Pope, Robert, "An algorithmic approach to `(2,2)`-isogenies in the
   theta model", eprint 2023/1747.** Titles read from the eprint pages; the
   abstracts were not returned by the page scrape. Both are dimension 2;
   nothing about genus 3. [read titles only]
10. **Bruin-Doerksen, "The arithmetic of genus two curves with `(4,4)`-split
    Jacobians", arXiv:0902.3480.** [read] Genus 2, split into elliptic curves;
    no genus-3 hyperelliptic-to-hyperelliptic content.
11. **Smith's thesis (Sydney 2005, explicit endomorphisms and correspondences)
    and Ritzenthaler's other genus-3 papers:** [not read]; no extra claim is
    taken from them.

## Dimension count (our argument, [derived])

`J(H)[2] = F_2^6` with the Weil pairing; a `(2,2,2)` isogeny corresponds to a
maximal isotropic (Lagrangian) 3-dimensional subspace. The quotient `A'` is a
principally polarized abelian threefold, hence by Torelli either a Jacobian of
a genus-3 curve or a product. The hyperelliptic Jacobians form a divisor
(dimension 5) in `A_3` (dimension 6). For a fixed combinatorial type of
kernel, the preimage of that divisor under the (finite) correspondence
`H -> A'` has dimension 4 in the 5-dimensional hyperelliptic locus: so
generically `A'` is the Jacobian of a **plane quartic**, matching items 1, 6,
8, and hyperelliptic targets occur on a codimension-at-least-1 subvariety,
which over `F_q` contains a fraction of order `1/q` of the curves, not a
positive fraction. Smith's two-dimensional families (item 2) are consistent
with codimension 3 for the families he constructs; we do not claim they are
the whole hyperelliptic-target locus.

## Conclusions

- **Positive density, hyperelliptic to hyperelliptic: no.** The only positive
  fraction in the literature read is hyperelliptic to **plane quartic** (item
  1; this is the Smith step the program already uses). Hyperelliptic targets
  appear on proper closed subvarieties.
- **Implementable here: not the hyperelliptic-to-hyperelliptic version.**
  Smith's explicit families (item 2) would need a decomposable pair `(f,g)` per
  curve (codimension 3 and no walk structure); Milio's odd-`l` formulas
  (item 5) give a target that is a quartic generically; decomposed Richelot
  isogenies (item 7) end in products. Nothing found gives a graph of
  hyperelliptic Jacobians with enough edges to walk on.
- **The implementable walk is the elliptic one** (this study): rational
  2-isogenies of `E/F_{p^3}`.

## Why the elliptic walk is special (our observation, [derived])

The descended `H` has `L_H(T) = 1 - tT^3 + p^3T^6`, so over `F_{p^3}` its
Jacobian is isogenous to `E x E^sigma x E^(sigma^2)`: `J(H)` is a geometrically
*split* (completely decomposable up to isogeny) threefold, the kind studied in
item 3. Consequently:

1. The isogeny `J(H) -> J(H')` between two members of an elliptic 2-isogeny
   class is a composite through the product `E^3 -> E'^3`; it is defined over
   `F_p` as an isogeny of abelian varieties but it is **not** a
   polarization-preserving `(2,2,2)` isogeny of principally polarized
   Jacobians, and the target's Smith data are not the images of the source's.
   This is why passing is a property to be measured on each `H'` and not
   something transported along the edge (the protocol's Q1 measures it).
2. These `H` are a codimension-large locus of the hyperelliptic moduli, so
   Smith's 0.1857 for random `H` need not describe them; Q1 measures the pass
   rate on exactly this locus.
