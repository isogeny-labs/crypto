# Exhaustive rational Smith chart results [pcc:unverified]

Freeze 44693a66ca8; input hashes pass; sole bounded invocation exits 0. Native Rust, overflow checks enabled. Git custody; no S3 polling.

| Map | Eligible t | Excluded zero delta | Excluded repeated-root projection | Rational chart points | Split-fibre points | Nonsplit-fibre points | Rational source incidences |
|---|---:|---:|---:|---:|---:|---:|---:|
| 0 | 58 | 2 | 1 | 55 | 4 | 51 | 42 |
| 1 | 57 | 2 | 2 | 54 | 8 | 46 | 46 |

All 6,900 nonzero-rho candidates and 420,900 (rho,sigma) pairs on the eligible parameters are exhausted. Exactly one reconstructed b occurs for each scalar quartic root, and none for nonroots. All 109 full polynomial congruences, inverse Veronese reconstructions and constant-coefficient mutation rejections pass. All 88 rational source incidences satisfy both the original source equation and trigonal-map equation.

There are 97 nonsplit-fibre points. This extends coverage beyond the split-only check, but the restricted opens differ: map 0 retains only 4 of the earlier 8 split points, while map 1 retains 8. Do not describe this as a superset or compare the counts as complete curve point counts. Every point is retained in CONTROL.txt with t, rho, b and split classification.

These are exhaustive base-field checks on a restricted affine chart. They do not prove a function-field birational inverse, certify the smooth projective model, determine genus or nonhyperellipticity, or implement a Jacobian map on complete divisors. In particular, the 88 incidences are not complete transported source or target divisor classes; images supported at degree-two or exceptional places remain open. No runtime speed comparison was performed. The requested full 10x genus2+ objective remains unachieved.

Next construction gate: derive rational inverse expressions over F61(t,rho), certify their denominators and exceptional places, and compute canonical adjoints/normalization. The nonsquare maps over F211/F401 remain subject to SMITH-TWIST-TRANSPORT.md and are not substituted here.
