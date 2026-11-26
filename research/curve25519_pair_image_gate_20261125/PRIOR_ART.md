# Prior-art and degree audit

The repository research-KB wrapper is unavailable in this environment
(`scripts/research-kb status` exits 127), so this is a bounded repository and
primary-reference audit; no novelty claim is made.

## Repository predecessors

- `research/prime31_ic_20261115/native/PAIR_S3_ELIMINATION_POLYNOMIAL_20261115.md`
  defines the same finite pair-abscissa product. For a 31-point factor base it
  has 465 off-diagonal quadratics, 31 diagonal linear factors, and exact degree
  `31^2=961`.
- `research/prime31_ic_20261115/native/S3_ROOT_INDEX_CONTROL_AND_NATURAL_16384_20261115.md`
  implements the coordinate root index and explicitly classifies it as
  mathematically generic pair-sum/MITM-equivalent.
- `research/prime31_ic_20261115/native/M8_RATIONAL_QUOTIENT_BRIDGE_20261116.md`
  retains the degree-961 pair coordinate polynomial as an exact selected-root
  interface, not a non-generic extractor.
- `research/prime31_ic_20261115/native/M4_FACTORIZED_S4_RESULTANT_IMPLEMENTATION_20261115.md`
  reduces one quotient component from `O(B^6)` to `O(B^4)` by multiplying
  short resultants, while explicitly retaining `O(B^2)` pair-support
  construction and declining an asymptotic ECDLP claim.
- `docs/prime-signed-mitm.md` gives the direct group-law version: `2B^2`
  stored representations and `Theta(B^2)` work/storage, with every witness
  replayed.

The exact broad split
`S3(X1,X2,Y), S3(X3,X4,Y)` is also the Amadori--Pintore--Sala prime-field
construction already cited by that module. Pair-image intersection is
therefore established machinery, not a new attack.

## Root-of-unity trace blocks

For odd `d`, write an unordered pair as

```text
w1=z*q, w2=z/q,
z in mu_d, c=q+q^-1.
```

The trace set has `(d+1)/2` values. At `c=2`, the repeated pairs produce `d`
linear S3 factors. Every other trace represents `d` off-diagonal pairs and
therefore `d` quadratic factors. Unless a leading coefficient vanishes or
factors collide, the block degrees are

```text
d + ((d-1)/2)*(2d) = d^2.
```

Thus the pair trace removes the swap label but does not reduce the generic
pair-image degree. The frozen certificate tests the two instance-specific
escape hatches—degree loss and repeated algebraic roots—before considering a
structured implicit-resultant algorithm.

## Sources

- Petit, Kosters, and Messeng, *Algebraic Approaches for the Elliptic Curve
  Discrete Logarithm Problem over Prime Fields*, PKC 2016.
- Amadori, Pintore, and Sala, *On the discrete logarithm problem for
  prime-field elliptic curves*, Finite Fields and Their Applications 2018,
  especially the split-S3 construction.

These sources motivate the algebraic image and split; they do not supply a
subquadratic witness-producing common-root algorithm for this root-of-unity
factor base.
