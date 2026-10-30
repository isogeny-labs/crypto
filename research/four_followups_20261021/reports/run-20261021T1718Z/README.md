# Rational two-torsion model gate, 2026-10-21 17:18 UTC

The Rust `kummer-model-gate` binary evaluated the exact genus-2 quintic at
every element of each of 21 base fields, checked each root with the
derivative and reconstructed the polynomial from its five linear factors.
All 21 models had five distinct finite rational roots and the rational point
at infinity. A second run matched the 21 raw JSONL rows byte-for-byte;
SHA-256 is `280b06474489bd78cba9a1fedd548aac9ac529a843f60156060d692851432680`.

The command was `cargo run --release --locked --bin kummer-model-gate` from
the study directory. [Raw rows](../../results/kummer-model-gate.jsonl)
record coefficients, exact roots and checks. The
[figure source](plot_kummer_model.py) plots the recorded roots normalized by
their field prime; the [SVG](kummer-model-roots.svg) and
[PDF](report.pdf) show all 21 models. The host was an Intel Xeon 6975P-C
with 32 visible CPUs. This exact gate has no wall-speed claim or rho ratio.
It establishes the rational two-torsion field-of-definition condition, not a
differential walk or signed relation labels.
