# Ordinary-binary n23 m3..15 enumeration: 26-cell run report

All 26 frozen natural cells have terminal restricted S3 custody and individual
Git checkpoints. The same field, curve, generator and target were used across
all arities within each twist. The opposite twists have distinct public inputs.
The requested factor base of three retained four points on both twists.

| Arity | Untwisted | Quadratic twist | Exact interpretation |
|---:|---|---|---|
| 3 | verified | verified | 64 complete candidates and 84 group-add transitions per cell; exhaustive fixed-base negative |
| 4 | verified | verified | 256 complete candidates and 340 transitions per cell; exhaustive fixed-base negative |
| 5 | verified | verified | 1,024 complete candidates and 1,364 transitions per cell; exhaustive fixed-base negative |
| 6..15 | budget_exhausted at each m | budget_exhausted at each m | 4,096 retained transitions per cell; censored depth-first prefix, no decomposition verdict |

The [official export](../experiment-export.json) passed the pinned checker
with zero findings: 26 planned, attempted and completed rows, with zero
exclusions, interruptions or resumes. Its SHA-256 is
`5ca6728ef97e3c013f6851ef5c8a95da8557dfaecb38ab7fd8c5595e0a7ee8bf`.
The [independent audit](../AUDIT.json) rehashed all 26 restricted archives
(7,001,623 bytes) and 832 native artifacts. It checked fixed axes, native
method and status binding, ordinary-binary field/twist certificate types,
coefficient-field degree 23, and the repeated public core across arities.
Public-core SHA-256 is
`7924562a4db84c8b855b770191bd0debf7424f767614b359c8fc3c7d147b33b4`
untwisted and
`f27b9070bc39da0feb1c7a1da70c056afe4fa698acdce32deb642f32feaab6eb`
twisted. The [status chart](outcomes.svg) displays the full denominator.
No cell promoted a PDP witness or independently recovered ECDLP scalar.

[Per-cell Git checkpoints](../checkpoints/) bind every archive and custody
pointer to producer S3 PUT and fresh GET/hash comparison. An
[independent first/last S3 sample](../S3-READBACK-SAMPLE.json) repeated fresh
GET and byte-hash checks. The pre-data
protocol and plan were frozen and committed before any trial at
`bc661164a5a9`; the pre-data tar checkpoint was read back from S3 with
SHA-256 `6410094a4c13f3dfa4ecb4e0e38df223f459247df800bb20bc034f64029e854d`.
The protocol body SHA-256 is
`74d789dbd62cb665eda7aaa95800970dd39c56807bf2f77da6ad3b9fbb6d7d68`,
protocol file SHA-256 is
`6abd4361f7d96583a3ec81e54fd93d85e1aef48056fda76926888051eac81638`,
and plan file SHA-256 is
`4906b6c2a1372f6a10f161e05187a9d7d3b3ffa1317c9cd186840f58e60e1c55`.
The clean native source is `c41bec150e16e4c4f9a00634002b633b821de262`
and binary SHA-256 is
`b8e0881ec5e46f60c23e6bbb49ad7c3677b9168199f5787b68d89169bfaaa77e`.

[Cost observations](../cost-observations.jsonl) retain 104 per-phase telemetry
records and exact native group-add/candidate counters. No phase verifies
whole-worker retired-instruction coverage, so there are zero eligible
instruction-to-solve training labels. Wrapper clocks are diagnostic; this
fixed population makes no timing-effect comparison. The 26 outer generate
calls had a 3,900-second hard aggregate bound at 150 seconds each, within an
eight-hour campaign ceiling that also allowed finalization and S3 custody.
Actual spend needs a separate priced resource receipt; no dollar estimate is
inferred from the wall cap alone.

Ordinary-binary coefficients have degree 23 over F2 and recorded Frobenius
power 23, so this run does not credit nontrivial model Frobenius orbit
reduction. The native target was sampled independently of factor-base
construction. A planted-positive PDP control requires a separately versioned
target-construction mode, restricted planting labels, a frozen protocol, and
its own denominator. This natural block remains 26 of the requested 78
Koblitz/binary/prime arity cells. For algebraic context,
[Shantz and Teske](https://eprint.iacr.org/2013/596.pdf) distinguish
experimental Weil-descent behavior from proved complexity; this run measures
the bounded native enumeration adapter.
