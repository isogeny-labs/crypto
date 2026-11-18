# H116 generating-set characteristic-certificate experiment

- Protocol: `H116-GENERATOR-CHARPOLY-001`
- Pre-data amendment: `H116-GENERATOR-CHARPOLY-002`
- Parent result SHA-256:
  `5c8f506a81280c650470dac6d2b32deca78c9e63a45282b1a579a6ace66368eb`
- Generator result SHA-256:
  `6a45fa51d3fc35c5c30c49101a8b158922aae01357a3202b4a1082d0583d9c4a`
- Independent verification SHA-256:
  `6c4fe82abd977c26168de5735c2f5dee7624664dae723d0adff5fdc0e9585bf3`

## Outcome

All eight frozen cells pass. Three extension point groups are cyclic and five
require two selected generators. The audited Lean theorem therefore reduces
the parent panel's 5,066 all-point characteristic evaluations to 13 generator
evaluations, saving 5,053 evaluations for a 389.69-fold check-count
compression on this finite panel. [pcc:unverified]

Every selected set generates the complete enumerated group. The
characteristic expression has zero failures on all 13 generators. Wrong trace
and wrong norm each reject on all 13 generators. Removing the sole generator
or the second generator leaves a proper subgroup in every cell. The independent
Python implementation reproduces every selected coordinate, order, closure
size and check count with zero mismatches. [pcc:unverified]

| Cell | Group order | Generators | Orders | Full closure | Dropped closure | Checks saved |
| --- | ---: | --- | --- | ---: | ---: | ---: |
| `fp5-a2-b1-d2` | 35 | `5:14` | 35 | 35 | 1 | 34 |
| `fp5-a2-b1-d3` | 112 | `6:69`, `19:41` | 28, 28 | 112 | 28 | 110 |
| `fp5-a2-b1-d4` | 595 | `6:266` | 595 | 595 | 1 | 594 |
| `fp7-a1-b1-d2` | 55 | `7:1` | 55 | 55 | 1 | 54 |
| `fp7-a1-b1-d3` | 380 | `10:185`, `8:24` | 190, 38 | 380 | 190 | 378 |
| `fp7-a1-b1-d4` | 2475 | `7:385`, `10:119` | 165, 165 | 2475 | 165 | 2473 |
| `fp11-a1-b1-d2` | 140 | `11:1`, `12:31` | 70, 10 | 140 | 70 | 138 |
| `fp11-a1-b1-d3` | 1274 | `13:557`, `0:1` | 182, 7 | 1274 | 182 | 1272 |

Point encodings are polynomial-basis integer indices `x:y` under the exact
moduli retained by the parent panel. Orders are computed by prime-divisor
reduction from the enumerated group order and independently confirmed by
repeated addition through the first return to infinity. [pcc:unverified]

## Controls

The wrong-trace and wrong-norm mutations each reject every selected generator,
so the compressed certificate remains sensitive to both coefficients. The
dropped-generator closure is proper in all cells; in the five two-generator
groups its size equals the first generator's cyclic subgroup order.
[pcc:unverified]

The parent missing-inverse mutation rejects 11 of 13 generators. It remains
invisible on the sole generator of the known `p=5`, degree-two collision and on
one generator of `fp11-a1-b1-d3`. It is diagnostic only under the amended
protocol and does not affect acceptance. [pcc:unverified]

## Cost and boundary

The Rust producer reports 0.987292486 seconds summed across cells; the separate
Python replay reports 32.485923913 seconds. These are single-run diagnostics,
not comparative benchmarks. The compression concerns only the number of
endomorphism evaluations after a generating-set closure certificate is
available; generator discovery still enumerates the small groups in this
experiment. [pcc:unverified]

This result makes the finite proof interface concrete and independently
reproducible. It does not prove the characteristic equation on algebraic
closure points, construct a separating rational/scheme morphism theorem,
reduce generic ECDLP complexity, or establish a subexponential algorithm.
[pcc:unverified]
