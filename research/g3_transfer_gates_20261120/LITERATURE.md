# Lead #3 literature gate: affine-type (2,2,2)-isogenies of genus-3 hyperelliptic Jacobians

Date 2026-11-20. Standing: `[pcc:unverified]` throughout.

## How the search was done, and its limits

- WebSearch only, with plain mathematical terms (no project details). Six queries.
- Full texts could **not** be read: in this session the hosts `arxiv.org`,
  `ar5iv.labs.arxiv.org` and `eprint.iacr.org` do not resolve (outside the session's
  allowed network domains). Fetching was stopped after the third such failure, as the
  session rules require. Every claim below therefore comes from a **search-engine
  summary** of the source, not from reading it. Treat every attribution as
  "reported by a search summary", not as a verified quotation.
- research-kb was not available in this session (as in the breakthrough search).
- To read the sources, `arxiv.org` and `eprint.iacr.org` have to be allowed in the
  session's network settings.

## Sources and what each was reported to say

| # | Source | What the search summary attributed to it | Bears on |
|---|---|---|---|
| S1 | B. Smith, "Isogenies and the Discrete Logarithm Problem in Jacobians of Genus 3 Hyperelliptic Curves", EUROCRYPT 2008, LNCS 4965, 163-180; J. Cryptology 22 (2009), doi 10.1007/s00145-009-9038-1; arXiv 0806.2995; IACR ePrint 2007/428 | Explicit isogenies with kernel `(Z/2Z)^3` from hyperelliptic genus-3 Jacobians to non-hyperelliptic ones, built from Recillas' trigonal construction; the kernel comes from pairing the Weierstrass points into disjoint pairs (disjointness makes it 2-Weil isotropic); rational for about 18.57% of curves over a given finite field; the quotient by a maximal isotropic subgroup is a 3-dimensional ppav, a Jacobian over `F_{q^2}` (Oort-Ueno). One summary also said that a maximal isotropic subgroup containing no nontrivial difference of two Weierstrass points cannot be "tractable" (the affine-type subgroups are exactly those: all seven nonzero elements have weight 4). | Pair type only; affine type not treated by the construction |
| S2 | R. Donagi, R. Livné, "The arithmetic-geometric mean and isogenies for curves of higher genus", Ann. Sc. Norm. Super. Pisa Cl. Sci. (4) 28 (1999), 323-339 | For a trigonal map `g` on the hyperelliptic curve `H`, Recillas' construction gives a genus-3 curve `X` (with a degree-4 map to P^1) and `J_X = J_H / S`, `S` determined by `g`. Secondhand (summary of a citing paper): no analogous process exists for `g > 3`. | Pair type (trigonal kernels) |
| S3 | E. Milio, "Computing isogenies between Jacobians of curves of genus 2 and 3", Math. Comp. 89 (2020); arXiv 1709.06063 | Starting from a hyperelliptic genus-3 curve and a maximal isotropic subgroup of the `l`-torsion, the isogenous curve is "generically non-hyperelliptic", justified by a dimension count (hyperelliptic locus has dimension 5 in the 6-dimensional moduli space). Computes isogenies via theta functions; equations for hyperelliptic quotients in genus 3 using the (64,29)-configuration of the Kummer threefold; for the non-hyperelliptic quotient, no explicit map. | Generic heuristic only; does not single out the affine orbit |
| S4 | Tian, "Translating the discrete logarithm problem on Jacobians of genus 3 hyperelliptic curves with (l,l,l)-isogenies", arXiv 2007.03172 (2020) | Extends Smith's `l = 2` reduction to `(l,l,l)`-isogenies from hyperelliptic genus-3 Jacobians to Jacobians of non-hyperelliptic curves; cites Frey-Kani for the moduli-of-covers derivation of Smith's construction. | Lead #5 (odd `l`) prior art |
| S5 | G. Frey, E. Kani, "Normal forms of hyperelliptic curves of genus 3", Des. Codes Cryptogr. (2015), doi 10.1007/s10623-015-0122-2 | Which hyperelliptic genus-3 curves are images of the Donagi-Livné-Smith construction; its "inverse". | Pair type |
| S6 | "Listing superspecial curves of genus three using Richelot isogeny graphs", arXiv 2401.10500; Research in Number Theory (2025), doi 10.1007/s40993-025-00657-3 | Algorithms computing the squared theta null-points of all abelian threefolds `(2,2,2)`-isogenous to a given one. | Method (the same theta test as the numerical arm here) |
| S7 | T. Katsura, "Decomposed Richelot isogenies of Jacobian varieties of curves of genus 3"; arXiv 2108.06936 (generalized Howe curves); Howe-Leprévost-Poonen (Forum Math. 2000) | Named by summaries for decomposed (split) Richelot isogenies in genus 3. Not checked further. | Decomposable codomains (control curve) |

## Answer to the lead #3 question

**(a) Codomain type of the affine-type (2,2,2)-isogeny.** No source reached here states
it. The published explicit route (S1, S2, S5) covers the 105 pair-type (trigonal)
kernels. S3's "generically non-hyperelliptic" is a dimension-count heuristic for a
general pair `(C, L)`. It does not, on its own, rule out that one whole Sp-orbit of
kernels (the 30 affine ones form one orbit under the S_8 monodromy) maps onto the
hyperelliptic locus. So the literature gate is **open as far as these summaries go**.
Smith's own discussion of the non-tractable subgroups could not be read.

The numerical arm of this study (`RESULTS.md`, arm T) settles the generic case over C.
On three random genus-3 hyperelliptic curves (real branch points), none of the 48
affine-type codomains computed (16 per curve) has a vanishing even theta-null. The
smallest relative value is 1.9e-3, against about 1e-17 for the one vanishing
theta-null of `J` itself. All 135 Lagrangians were also covered on one curve. So the
affine-type codomain is **generically the Jacobian of a smooth plane quartic**, not
hyperelliptic and not decomposable. A theta-null that vanished identically on the
family would vanish on the real-branch-point locus, which is Zariski-dense in the
5-dimensional hyperelliptic moduli. So "generic" here means "off a proper closed
subset". That subset is not computed. On it the codomain can be hyperelliptic for
special curves. This is numerical evidence, not a proof or a citation.

**(b) Galois stability for odd `#J`.** This is a derivation, not a literature claim.
In the odd-degree model, odd `#J` holds iff `f` is irreducible, so Frobenius acts as a
7-cycle on the finite Weierstrass points and fixes infinity. No pair-type Lagrangian
is stable: infinity's partner would have to be fixed. Exactly 2 affine-type
Lagrangians are stable, and their kernel points are defined over `F_{p^7}` (`k = 7`).
A 7-cycle lies in exactly 30·384/5760 = 2 conjugates of AGL_3(F_2). On `J[2]`,
`P mod 2 = Phi_7 = (T^3+T+1)(T^3+T^2+1)` is squarefree with one dual pair, which
gives 2^1 Lagrangians.

- The lab already derived and checked this: L06 DERIVATION section 4, verified by the
  L06 assessor and by `wf/g3transfer/lagr.py`.
- It is confirmed here on 600/600 arm-A and 600/600 arm-B prime-order curves (census
  control P3).

Even-degree models:

- A prime-order Jacobian has cycle type `(1,7)` or `(3,5)` (`smith_even_census_20261115`).
- `(1,7)` also gives 2 stable affine-type Lagrangians.
- `(3,5)` gives none of any kind: `P mod 2 = (T^2+T+1)(T^4+T^3+T^2+T+1)`, both factors
  are self-dual of multiplicity 1, and AGL_3(F_2) has no element of order 5.
- This even-model case is derived, not measured.

## Caveats for the transfer program (not tested here)

- Over `F_q` the quotient `A = J/L` is a ppav defined over `F_q`. Oort-Ueno (via S1)
  makes it a Jacobian only over `F_{q^2}`. If `A` is the quadratic twist of `J_X`
  (Serre's obstruction), the `F_q`-points of `A` embed in `J_X(F_{q^2})`, which would
  ruin the exponent gain. The obstruction's frequency on these quotients is not known
  here.
- No efficient construction of the affine-type isogeny is known from these sources.
  S3 notes that no explicit map is given for non-hyperelliptic quotients. The kernel
  lives over `F_{q^7}`.
