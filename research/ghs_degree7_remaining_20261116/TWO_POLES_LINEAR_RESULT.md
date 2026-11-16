# Two branch poles plus linear square: fixed trace +22 carrier

The committed source at `7804e49e1678` completed the exact denominator
class `B=Q_iQ_j(x-c)²` for the two trace-`+22` source-orbit representatives.

| target mask | pole pairs | branch assignments | denominators | logical candidates | point-filter survivors | coprime and divisible | exact squares | CPU s |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 68 | 10 | 270 | 65,340 | 7,906,140 | 11,076 | 11,028 | 0 | 27.232 |
| 84 | 10 | 270 | 65,340 | 7,906,140 | 11,195 | 11,146 | 0 | 27.648 |

The [b=68](two_poles_linear_68.json) and
[b=84](two_poles_linear_84.json) receipts bind to source SHA256
`07eae12724fb69d9663a645d817d61a9f61005d916fce9a51da248149df42b49`.
The square-value point filters are necessary conditions and every survivor
was checked against exact division and squarehood. The result excludes this
denominator class for the fixed carrier and both source models; Frobenius
conjugation covers the other eight models in their two orbits. The search
does not cover the `Q_iQ_jQ_k` denominator, other carriers, or non-curve-map
correspondences. No independent algebra-system replay is recorded here.
[pcc:unverified]
