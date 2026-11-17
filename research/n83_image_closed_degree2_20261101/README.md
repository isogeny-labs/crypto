# n=83 image-closed degree-two fiber audit

This study tests the first higher-degree image-compatible relation design for
the binary cubic cover.  The result is a scoped rejection: on every requested
odd extension, a generic rational fiber has closed-point degrees `1+2`, but
their elliptic labels are exactly `P` and `[2]P`.  The resulting projected
identity is only `[3]P=P+[2]P`, so it contributes no independent logarithmic
information.

Because the frozen information gate fails, no relation collector or EKS
timing run is admitted.  Ordinary ambient genus-three collection and fixed
bounded-degree branching also fail the required comparison with source rho.
A growing-degree or adaptive target-local router remains open; this audit does
not assert that all higher-degree image-compatible calculi are impossible.

- [Results and evidence](RESULTS.md)
- [Frozen protocol](PROTOCOL.md)
- [Requirement ledger](REQUIREMENTS.md)
- [Machine-readable assessment](ASSESSMENT.json)
- [Deterministic native receipt](certificate.json)
- [Editable decision diagram](figures/degree-two-gate.dot)
- [Rendered decision diagram](figures/degree-two-gate.svg)
- [Formal group identity](ImageClosedDegreeTwo.lean)
- [Illustrated report](REPORT.pdf)

Native replay:

```sh
cargo run --release --bin n83_image_closed_degree2_audit -- \
  --output research/n83_image_closed_degree2_20261101/certificate.json
```

No result in this directory establishes a discrete-log speedup.
