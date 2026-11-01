# N6 author-source correction and affine coefficient bridge

The prior source audit attributed an `N6` factor linear in `W1` to the
authors' Magma implementation. That attribution was wrong. In the pinned
[`jac-on-jac.m` at `ec98e115`](https://github.com/ozbayelif/jac-on-jac/blob/ec98e115ed359c8b7facf559eb0d9f240703f6f5/jac-on-jac.m#L2506-L2512),
the `DBL2` function assigns `WW1:=W1^2` before its `N6` assignment multiplies
by `WW1`. The full pinned file hashes to
`7416a7fffaafff371738e7498a6795057372e1c477e0079297eebf2347c2e72a`.
The production Rust `double22_weighted` calls its variant with the squared
`w` flag, so these two source expressions agree on the `W²` exponent.

The [2024 article, §4 and Eq. (7)](https://link.springer.com/article/10.1007/s13389-024-00351-7)
defines the weighted representation and prints the affine doubling
coefficients. Under
`Q=qz², R=rz⁴, S=sz³w, T=tz⁵w, Z=z, W=w`, the pinned
author-style intermediates reduce to:

| Coefficient | Weighted expression / affine Eq. (7) | Lean result |
| --- | --- | --- |
| `A` | `z¹¹w³` | Checked |
| `B` | `z¹⁰w²` | Checked |
| `C` | `z⁹w³` | Checked |

The five universal integer-polynomial equalities in
[N6AffineBridge.lean](N6AffineBridge.lean) prove those three reductions and
the exact `A` and `C` defects if `W¹` is substituted. The [CSV](scaling-weights.csv)
and [graph](scaling-weights.svg) expose the weight pattern; the
[PDF](REPORT.pdf) records the same limited result. The validator checks the
full pinned Magma source hash, alias ordering, Rust source hash and selected
source expression, the Lean axiom log, and the rendered tables.

The frozen `p=739, W=3` record remains a valid **synthetic mutation control**:
its `transcribed_*` keys reflect the earlier mistaken transcription, rather
than the author code. It cannot be used as evidence of an author-code error.
The original `N6Homogeneity.lean` source begins with the same mistaken
attribution. Its bytes and the native identity certificates are preserved
because those certificates bind the exact source hash; this correction
supersedes that historical comment while retaining its polynomial theorems.

This packet does not prove complete Magma/Rust equivalence, compiled Rust
semantics, the smooth-projective/Pic⁰ class equation, or a revised cost ratio.
The full 1,260-target collector results remain unchanged.

To replay with the pinned author file:

```sh
curl --fail --location --silent --show-error \
  https://raw.githubusercontent.com/ozbayelif/jac-on-jac/ec98e115ed359c8b7facf559eb0d9f240703f6f5/jac-on-jac.m \
  --output /tmp/jac-on-jac-ec98.m
(cd research/projective_law_followup_20261022/formal && \
  lake env lean ../n6-homogeneity/source-correction/N6AffineBridge.lean \
  > ../n6-homogeneity/source-correction/lean.log 2>&1)
python3 research/projective_law_followup_20261022/n6-homogeneity/source-correction/verify.py \
  --author-source /tmp/jac-on-jac-ec98.m --check
```
