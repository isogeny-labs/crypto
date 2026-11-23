# H116 Frobenius fixed-loci exhaustion experiment

- Protocol: `H116-FROBENIUS-FIXED-LOCI-001`
- Protocol SHA-256:
  `0191f93cc986b025224fcad313e8be85842f1599a223f92aa2e94aefb2a2772d`
- Parent result SHA-256:
  `5c8f506a81280c650470dac6d2b32deca78c9e63a45282b1a579a6ace66368eb`
- Result SHA-256:
  `c2661066c46778b96d7d47a98b0ee903ac0de04b638d48589efd79c8b86a2c`
- Independent verification SHA-256:
  `21f7eec8b20de45d373f6f93a4f901749b62d1985a879b24308fd89ead021734`

## Outcome

All eight frozen cells pass, and the independent implementation has zero
mismatches. Every one of 5,066 points has a least positive Frobenius period no
larger than and dividing its frozen extension degree. Direct `pi^n` fixed
counts equal the sums of least-period strata dividing `n` in every cell. The
first fixed locus equals the independently retained base-field count, and the
degree-th fixed locus equals the full group. [pcc:unverified]

Across the panel, 64 points have least period one, 282 have period two, 1,740
have period three, and 2,980 have period four. Thus checking only the base
fixed locus covers 64 of 5,066 points and omits 5,002. Even the union of every
proper-period stratum covers only 142 points and omits 4,924 full-period
points. [pcc:unverified]

| Cell | Group | Least-period histogram | Fixed counts `n=1..d` | Proper-period union | Full period |
| --- | ---: | --- | --- | ---: | ---: |
| `fp5-a2-b1-d2` | 35 | `1:7, 2:28` | `7, 35` | 7 | 28 |
| `fp5-a2-b1-d3` | 112 | `1:7, 2:0, 3:105` | `7, 7, 112` | 7 | 105 |
| `fp5-a2-b1-d4` | 595 | `1:7, 2:28, 3:0, 4:560` | `7, 35, 7, 595` | 35 | 560 |
| `fp7-a1-b1-d2` | 55 | `1:5, 2:50` | `5, 55` | 5 | 50 |
| `fp7-a1-b1-d3` | 380 | `1:5, 2:0, 3:375` | `5, 5, 380` | 5 | 375 |
| `fp7-a1-b1-d4` | 2,475 | `1:5, 2:50, 3:0, 4:2420` | `5, 55, 5, 2475` | 55 | 2,420 |
| `fp11-a1-b1-d2` | 140 | `1:14, 2:126` | `14, 140` | 14 | 126 |
| `fp11-a1-b1-d3` | 1,274 | `1:14, 2:0, 3:1260` | `14, 14, 1274` | 14 | 1,260 |

## Characteristic and controls

The correct expression `pi^2-[t]pi+[p]` has zero failures in every
least-period stratum. Wrong trace and wrong norm each reject 5,058 of 5,066
points; the eight identities are the only points not rejected. Every cell has
a proper period-one locus, a proper union of all proper-period strata, and a
nonempty full-period stratum. These controls empirically match the Lean
single-locus counterexample: one finite fixed locus cannot certify the
algebraic point group. [pcc:unverified]

## Cost and boundary

The Rust producer reports 0.297264300 seconds summed across cells, 226,858
Frobenius maps, 138,822 point additions and 30,396 scalar multiplications. The
separate Python replay reports 8.021510951 seconds. These are one-run
diagnostics, not comparative benchmarks. [pcc:unverified]

This experiment instantiates the proved exhaustion interface on eight small
finite extensions. It does not prove the characteristic equation on arbitrary
finite fixed loci, identify pointwise `V-1` with a rational or scheme morphism,
transport it to a completed local series, solve ECDLP, or establish a runtime
improvement. [pcc:unverified]
