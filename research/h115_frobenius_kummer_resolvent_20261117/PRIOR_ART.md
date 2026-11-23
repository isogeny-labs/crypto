# H115 prior-art and overlap screen

Screen date: 2026-11-17. Scope: Kummer connecting maps, Frobenius/Verschiebung,
division torsors, theta/biextension attacks, elliptic nets and explicit
division-polynomial representations. This is a scoped search, not proof of
external novelty.

## Primary sources checked

1. Lauter and Stange, *The Elliptic Curve Discrete Logarithm Problem and
   Equivalent Hard Problems for Elliptic Divisibility Sequences* (SAC 2008),
   [arXiv:0803.0728](https://arxiv.org/abs/0803.0728). The paper makes several
   EDS problems subexponentially equivalent to ECDLP and connects EDS
   association to Tate-pairing/MOV/Frey--Rück/Shipsey attacks. H115 must not
   call an elliptic-net or division-sequence restatement a new algorithm.
2. Robert, *Some notes on algorithms for abelian varieties* (2024),
   [IACR ePrint 2024/406](https://eprint.iacr.org/2024/406). Section 7.4.1
   explicitly explores DLP information through isogeny torsors,
   biextensions/theta groups, and notes the overlap with elliptic nets. H115's
   torsor starting point and broad attack motivation are therefore prior art.
3. Dartois, De Feo, Maino, Pope and Robert, *On the efficient representation
   of isogenies* (2024),
   [IACR ePrint 2024/1071](https://eprint.iacr.org/2024/1071). Appendix A
   records Frobenius eigenvalue/field-of-definition cases and the generic
   division-polynomial route: degree `O(N^2)` with stated soft-`O(N^3)` cost.
   This is the representation/cost baseline, not an H115 contribution.
4. Robert, *The geometric interpretation of the Tate pairing and its
   applications* (2023),
   [IACR ePrint 2023/177](https://eprint.iacr.org/2023/177). This supplies the
   torsor/Kummer/pairing context and prevents treating a cohomology class as a
   newly discovered object.
5. Lombardo and Tronto, *Explicit Kummer Theory for Elliptic Curves*,
   [arXiv:1909.05376](https://arxiv.org/abs/1909.05376). Its setting is number
   fields and division towers, not H115's finite-field experiment, but the
   explicit Kummer representation and cohomology are established theory.
6. Stange, *The Tate Pairing via Elliptic Nets*,
   [IACR ePrint 2006/392](https://eprint.iacr.org/2006/392). Elliptic nets
   already compute pairings and reflect the curve group law; H115 excludes
   pairing transfer and elliptic-net recurrence claims from its novelty scope.

## Exact overlap and remaining experimental question

The following are known mathematics, not novelty claims:

- the `[r]`-division fiber is an `E[r]`-torsor;
- `R -> Frobenius(R)-R` represents the Kummer cocycle;
- Frobenius and Verschiebung satisfy `pi*V=[p]` and `pi+V=[t]`;
- torsor, theta, biextension, pairing and elliptic-net routes have already been
  proposed for DLP information;
- division polynomials and explicit isogeny representations already provide
  general algorithms with degree-dependent costs.

The scoped H115 question not located in this search is narrower: on a
prime-order curve with `r=p+1-t`, the identity

```text
(V-1)(pi-1) = [r]
```

compresses the `r^2` Kummer-cocycle multiset to the degree-`r` fiber of
`V-1`. H115 freezes low-order `x/y` power-sum resolvent moments of that fiber
and asks whether one constant, coordinate-public channel predicts a radix-six
scalar digit across several curves and generator relabelings. It also measures
the actual degree/extension/enumeration cost and requires a separate uniform
shortcut before any subexponential claim.

No primary source found in the scoped search states this exact frozen moment
family or its cross-curve test. That absence does not establish publication
novelty. A positive toy result would still overlap conceptually with the
torsor/biextension literature and would require a broader search and expert
review.

## Exclusions

H115 is not index calculus, Pollard rho, pairing transfer, EDS association,
generic table lookup, a curve-sized fitted classifier, or a claim that a
degree-`r` isogeny fiber is cheap. No candidate may use the hidden scalar,
post-run coefficients, an independently chosen formula per curve/row, or an
unpriced precomputed table.
