# H116 x-only parity obstruction

The experimental identity is explained by a generic pairing argument, now
formalized in `lean/H116FiberTrace.lean`.

Let `P` be killed by an odd natural `n`. For any `k<=n`, additive scalar
arithmetic gives

```text
(n-k)P + kP = nP = 0,
```

hence `(n-k)P=-kP`. Oddness gives opposite low bits:
`(n-k) mod 2 != k mod 2`. If `O(-Q)=O(Q)`, then any deterministic decoder
`D(O(Q))` makes the same prediction at the two points and cannot match both
labels. The Lean theorem `negInvariant_binaryDecoder_pair_failure` states this
without assuming elliptic coordinates, a field, or a particular decoder.

For a short-Weierstrass curve, x is unchanged by negation. Products of
`x-x([j]P)`, their Legendre symbols, and the split/inert types of the quadratic
covers in the experiment therefore satisfy the observable premise. The result
does not apply to y-sensitive functions because y changes sign.

The mutation `n=4,k=1` has `(4-1)%2=1%2`, proving that odd order is not a
decorative premise. The formal source hash is
`42b403444a6de1465f0436489d76de54def3b6c6ba74bf0234b729a1ebd7c79a`;
the pinned 3,164-job build and 219-theorem audit pass with zero failures and
zero trusted/native decisions.
