# H116 multi-prime omitted-Frobenius boundary protocol

Protocol `H116-FROBENIUS-BOUNDARY-PANEL-001` freezes two prospective cells
before either retained full map is expanded beyond its previously examined
eleven coefficients. The p=31 result is a calibration and is not counted as a
prospective cell. [pcc:unverified]

## Prediction

Write `a=t-1`. From `V-1=[a]-pi`, the normalized local parameter has leading
correction `-z^p/a`. The inverse-square pole therefore predicts that the
normalized x-map first changes at `z^(p-1)`, or coefficient `(p-1)/2` in
`w=1/x`, with full-map-minus-scalar delta `2/a`. [pcc:unverified]

| Cell | a modulo p | Precision | Frozen first index | Frozen delta |
| --- | ---: | ---: | ---: | ---: |
| `fp211-r197-d4` | 14 | 108 | 105 | 181 |
| `fp241-r257-d4` | 225 | 123 | 120 | 30 |

The complete machine protocol is `FROBENIUS_BOUNDARY_PANEL_PROTOCOL.json`.
Its retained-map input is `FORMAL_SCALAR_OOS_ORACLE.json`, SHA-256
`8fb0dad351eb1b7948e42f553b6e9ff7f33ae91527e4bd09d8566ed767fe4eaf`.
Those maps were constructed earlier from an extension-field kernel walk and
Velu formulas; only prefixes through degree ten were previously compared.
[pcc:unverified]

## Construction and controls

The producer builds `[a]` with division polynomials. For each cell it requires
both the retained full map and normalized scalar map to satisfy their cleared
curve identities. It also tests omitted target scaling, wrong target scaling,
and a one-coefficient mutation at the frozen boundary. [pcc:unverified]

The verifier does not construct division polynomials. It enumerates every
base-field affine x-coordinate, checks the scalar map against scaled
`x([a]P)`, and checks the retained map against the rational-point
specialization scaled `x([a-1]P)`. It independently recomputes the polynomial
identities, prefixes, first differences, deltas, and negative controls. A wrong
scalar action `[a+1]` must disagree on at least one point. [pcc:unverified]

Any failed prediction or control is retained. No curve, precision, prediction,
input map, source, or row may be replaced after execution. The requested route
is one ordinary Iso Lab EKS Spot correctness job, limited to one vCPU, 2 GiB,
15 minutes, one attempt, and a USD 0.10 ceiling. No timing claim is planned.
[pcc:unverified]

This panel tests the first omitted coefficient on two additional maps. It does
not establish the curve-level formal-group identity or an asymptotic runtime.
[pcc:unverified]

