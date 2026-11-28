# R134: ordinary lambda fibers through F2^88

The four R133 derivative-unit base places admit complete, squarefree degree-16 lambda fibers. Their exact distinct-degree decompositions are:

| x-point | base residue degree | lambda degrees and counts | resulting F2 residue degrees | closed places |
| --- | ---: | --- | --- | ---: |
| P11 | 11 | 4 of degree 4 | 4 of degree 44 | 4 |
| P11 | 55 | 4 of degree 1; 4 of degree 3 | 4 of degree 55; 4 of degree 165 | 8 |
| P22 | 44 | 4 of degree 4 | 4 of degree 176 | 4 |
| P22 | 88 | 2 of degree 2; 2 of degree 6 | 2 of degree 176; 2 of degree 528 | 4 |

All twenty lambda places are unramified over their respective base places by the nonzero derivative and simple-factor lifting criterion. Their F2 degree-weighted contributions are 1056 over P11 and 2112 over P22, totaling 3168. This accounts for the ordinary y-degree-six part of each x-fiber. The missing degree-one base place at each x-point contributes another 528 to the full expected weighted degree 3696 and requires separate local normalization.

The producer constructs the four residue fields as explicit towers `F2[x,y]/(P_d,Q_m)`, specializes all 29 actual R51 G51 source terms, and computes the complete factor products using finite-field Frobenius and gcd. A separate carryless tower implementation checks all coefficients, factor products, Frobenius degree identities, and weighted degrees; it reports `LAMBDA134_INDEPENDENT_FACTOR_DEGREES_PASS`. The accepted calculation used one local core and finished in about 4.3 seconds under the 600-second/4-GiB bound. Its result SHA-256 is `ea8b73bb49965f801dec664c16e004861c686081ca3c41387fe5f2e12b089fce`; a fresh replay was byte-identical. A changed R133 input was rejected before output with `AssertionError: r133_result`. Lean checks the stated finite degree arithmetic; it does not formalize the factorization. The vector chart was visually inspected. No cloud resources were used.

Source/protocol commit `0d9197e88` was pushed before the accepted computation. Next normalize G51 at the two exceptional linear base places using local Hasse/Newton expansions, then evaluate N51 on all completed P11/P22 lambda places. The present factorization alone does not determine N51 valuations. The local lifting inference uses the [Stacks simple-root criterion](https://stacks.math.columbia.edu/tag/04GE).
