# Point counts of both `t=−1` elliptic quotients

For the exceptional opposite-x ratio, the genus-two curve
`D_{−1}: w²=1600−(x³−3x)²` has an elliptic quotient under `x↦−x`.
With `X=x²` and `U=2−X`, that quotient has short Weierstrass model

`E₁: W²=U³−3U+1598` over `p=2^61−1`.

The second quotient has quartic model
`V²=X[1600−X(X−3)²]`. On `X≠0`, put `T=1/X`, `v=V/X²`,
`Z=1600T−3`, and `Y=1600v`. It becomes the short Weierstrass curve

`E₂: Y²=Z³+9573Z−2531254`.

The [dependency-free Rust Hasse-interval search](native/examples/quotient_order.rs)
finds exactly one integer in `[p+1−⌊2√p⌋,p+1+⌊2√p⌋]` that kills a
chosen point on each curve:

| Curve | Exact point count |
|---|---:|
| selected `b=40` | `2305843009662956113` |
| `E₁` | `2305843011224697631` |
| `E₂` | `2305843008686133067` |

Each search uses 77,936 baby steps, explicitly rejects a baby collision,
and exhausts giant steps across the entire Hasse interval. Every possible
curve order must kill the chosen point; a unique such integer is
therefore the exact curve order. The executable also runs the same method
on the selected `b=40` curve and recovers its independently certified
`2305843009662956113` points. As a separate arithmetic cross-check,
`(2,40)∈E₁(F_p)` is killed by the reported order and is not killed after
division by either factor `121067` or `19046007675293`. Likewise,
`(1597,1263391228187558620)∈E₂(F_p)` is killed by its reported order
and is not killed after division by any of its factors
`23,31,2181169,1482691811`. These factors were independently checked
prime with deterministic 64-bit Miller–Rabin; the BSGS uniqueness proof
does not depend on their factorization.

The `E₁` and `E₂` orders differ from the selected curve's by
`+1,561,741,518` and `−976,823,046`, respectively. Elliptic curves over
the same finite field have the same point count if they are isogenous over
that field. Hence **neither quotient is `F_p`-isogenous to the selected
`b=40` curve**. Their geometry simplifies the opposite-x pair-sum
x-query, but these elliptic factors do not transfer the selected
prime-subgroup ECDLP.

Reproduce:

```sh
cargo run --release --offline --manifest-path research/m61_root_relation_20261126/native/Cargo.toml --example quotient_order
```
