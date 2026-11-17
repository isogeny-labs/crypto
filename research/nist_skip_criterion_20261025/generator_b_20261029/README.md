# Generator and b-value investigation

This addition prepares the user's expanded comparison. The finite family and
all-row/unknown handling are in [PROTOCOL.md](PROTOCOL.md). No full-cohort
measurement has run yet; the checkpoint/upload identity is unavailable in the
restored runtime. Do not cite prepared code as measured output.

Read the [current report](REPORT.md), [PDF](REPORT.pdf),
[development checks](reports/development/REPORT.md) and
[local review-bundle custody](evidence/local-review-bundle.json).
The new actual-cohort comparison has zero attempts.

The supplied canonical G is a function of the curve. In the defining equation
y²=x³−3x+b, x=0 gives y²=b and x=1 gives y²=b−2 modulo p. These consequences
alone do not distinguish a selected curve from an omitted one. The new test
keeps canonical points, standardized published points, changed generators and
P-224 sign conventions explicit.

Source review: [SEC 1 v2, section 3.1.3.2](https://www.secg.org/sec1-v2.pdf)
describes a seed/hash point-generation procedure; it is not evidence that the
1997 program used that later document. [RFC 5639 A.2](https://www.rfc-editor.org/rfc/rfc5639.html#appendix-A.2)
uses a scalar multiple of a smallest-x point, providing a concrete public-scalar
recipe family to test. Neither standard supplies the historical NIST skip rule.
The pinned [std-curves](https://github.com/J08nY/std-curves/tree/77fe6e3585ca2c2225b59d7df24b7c775437276f)
records supply published generator coordinates, not hidden historical skip
generators. The original [curve-origin account](https://www.anthropic.com/research/ecdsa-curve-origins)
and prior FreeLIP prefix reconstruction remain relevant prior work.

The existing complete-factor and structural comparison remains frozen in
[factor_followup_20261026](../factor_followup_20261026/REPORT.md). Numerical
coordinate traits are representation-dependent; compare them under matched
models before interpreting an apparent difference. Neither hypothesis about
weak selected or skipped curves is established.
