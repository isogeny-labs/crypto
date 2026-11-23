# n=83 adaptive image-router audit

This study tests the remaining projector-only interpretation of a compact
adaptive image-compatible relation router.

The result is scoped: the normalized norm/pullback projector collapses every
pushforward-kernel coset and therefore supplies no class-specific Prym label.
An additional label/descent or Prym-cancellation oracle remains an open design
obligation.

Artifacts:

- [frozen protocol](PROTOCOL.md)
- [exact results](RESULTS.md)
- [requirements trace](REQUIREMENTS.md)
- [native certificate](certificate.json)
- [Lean proof](AdaptiveImageRouter.lean)
- [transfer assessment](ASSESSMENT.json)
- [editable decision diagram](figures/router-gate.dot)
- [illustrated report](REPORT.pdf)

Native replay:

```sh
cargo run --release --bin n83_adaptive_image_router_audit -- \
  --output research/n83_adaptive_image_router_20261118/certificate.json
```

Formal replay:

```sh
tools/lean/proof-gate.sh research/n83_adaptive_image_router_20261118
```

No artifact in this directory reports a wall-clock performance gain or an
ECDLP advantage.
