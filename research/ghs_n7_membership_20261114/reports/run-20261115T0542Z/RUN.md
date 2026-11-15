# Run 20261115T0542Z

Frozen q=2,4 direct/product and projective membership control on local
arm64 macOS, one deterministic run per field and both cubics. Protocol:
[`PROTOCOL.md`](../../PROTOCOL.md). Source revision: initial commit
`eacae612c8f8`. Commands: `cargo test --release` and `cargo run --release`
against this study's manifest. Both tests passed. Exact counts and field
moduli are in [`result.txt`](result.txt); no statistical interval applies.

This run found the first implementation's full-field kernel setup. It
therefore supports finite equality of direct and projective sets, but not
the total q² membership-cost interpretation. See [`AMENDMENT_1.md`](../../AMENDMENT_1.md)
and the [study report](../../REPORT.pdf) for the full evidence boundary.
The study PDF and quantitative SVG are copied into this run directory.
