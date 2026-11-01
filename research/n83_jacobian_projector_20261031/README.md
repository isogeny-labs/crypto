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

The native replay command is:

```sh
cargo run --release --bin n83_jacobian_projector_audit -- \
  --output research/n83_jacobian_projector_20261031/certificate.json
```

No result in this directory establishes a discrete-log speedup.
