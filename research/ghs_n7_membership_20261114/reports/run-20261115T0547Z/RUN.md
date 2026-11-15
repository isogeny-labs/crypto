# Run 20261115T0547Z

Amended q=2,4 Gaussian-kernel/projective membership arm on local arm64
macOS, one deterministic run per field and both cubics. Protocol:
[`PROTOCOL.md`](../../PROTOCOL.md) and [`AMENDMENT_1.md`](../../AMENDMENT_1.md).
Commands: `cargo test --release` and `cargo run --release` against this
study's manifest. Both tests passed. Exact counts and field moduli are in
[`result.txt`](result.txt); no statistical interval applies. The source
was uncommitted at run time and subsequently extended for q=8, so a unique
source hash was not captured for this run. This is a provenance limit.

The Gaussian basis construction avoids full-field enumeration in the
candidate arm. Direct enumeration remains the finite reference. This run
does not measure end-to-end attack cost. See [study report](../../REPORT.pdf).
The study PDF and quantitative SVG are copied into this run directory.
