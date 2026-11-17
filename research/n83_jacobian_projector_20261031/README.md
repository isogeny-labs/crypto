# n=83 Jacobian norm and image-projector audit

This study closes the arbitrary-class pushforward gap for the binary cubic
cover and applies the resulting projector to the pinned n=83 construction.

The headline result is structural, not an attack: the Prym has Frobenius
polynomial `X^4+q^2` over `GF(2^83)), splits up to isogeny into elliptic
factors of traces `+2^42` and `-2^42`, and has order coprime to the target
prime `r`.  It therefore shares neither the target prime nor an elliptic
isogeny factor with the source curve.

- [Results and evidence](RESULTS.md)
- [Frozen protocol](PROTOCOL.md)
- [Requirement ledger](REQUIREMENTS.md)
- [Machine-readable assessment](ASSESSMENT.json)
- [Editable transfer diagram](figures/projector-flow.dot)
- [Formal identities](Projector.lean)
- [Polynomial identity certificate](prym-factorization.identity.json)

## Formal proof map

The declaration namespace is `N83JacobianProjector`; the `Projector.` prefix
in the frozen results report is a source-reference error.

| Lean theorem | Exact premise | Scope of conclusion |
| --- | --- | --- |
| `imageOperator_quadratic` | Additive pull and push maps with `push (pull x) = degree • x` for every source element. | The composite image operator satisfies `e² = degree • e`. |
| `complementary_push_zero` | The same pull/push composition law. | Pushforward kills `degree • y - pull (push y)`. |
| `prym_factor_expansion` | Integer polynomial variables only. | Expands the product of the two quadratic factors. |
| `prym_factorization` | The supplied relation `a² = 2q`. | The product equals `X⁴ + q²`. |

These Lean identities do not construct the geometric maps, establish the
composition law for the concrete cover, or derive the stated Frobenius traces
from the curve. Those inputs have separate native and mathematical evidence.

The native replay command is:

```sh
cargo run --release --bin n83_jacobian_projector_audit -- \
  --output research/n83_jacobian_projector_20261031/certificate.json
```

No result in this directory establishes a discrete-log speedup.
