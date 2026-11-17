# Fixed GF(8)/GF(512) extension screen

Frozen 2026-10-26 before the quartic sample. An exploratory exhaustive
elliptic-order lookup identified the low-cofactor source orders
`482,502,514,526,538,542,554`; the lookup is not a quartic witness or an
attack. The purpose of this extension is to test whether the q=4 structural
coincidence persists at a larger binary cubic field.

- `k=GF(2^3, modulus="conway")`, `K=GF(2^9, modulus="conway")` in Sage.
- Enumerate all `b != 0` and the two normalized `a` choices: zero and the
  first integer-encoded element with absolute trace one. Check every order
  and relative-trace class exactly.
- Draw 1024 homogeneous quartics with `x^4` coefficient one and the other
  14 coefficients uniform from `0..7`, using `random.Random(2026102608)`.
  Use the same 15-monomial order as `PROTOCOL.md`.
- Retain smooth quartics with `N1=9`, `N2=65`. Determine `N3` by enumerating
  roots of the quartic on projective affine charts over `GF(512)`. Accept a
  Weil-restriction class witness if `t=(513-N3)/3` is integral and
  `#Jac(k)=513-t` equals a source `#E(K)`.
- Independently replay retained `N1,N2,N3` through Sage projective rational
  points, checking smoothness and exact full Weil polynomial. Keep the first
  witness per order and every source model at that order.
- Mark a possible new transfer route only if no class-B source at that
  order and the selected prime subgroup has cofactor at most four. A
  matching quartic is still only an isogeny-class witness. The geometric
  source map, its prime-subgroup action, complete DLP and rho costs remain
  separate gates.

Prior work includes Joux--Vitse's characteristic-two cover/decomposition
construction and the LMFDB q=8 isogeny classes. The LMFDB records queried
before this protocol contained hyperelliptic models for the listed
low-cofactor classes, but no plane quartic models in the `curves` arrays.
That catalog status is not proof that no quartic was previously known.
