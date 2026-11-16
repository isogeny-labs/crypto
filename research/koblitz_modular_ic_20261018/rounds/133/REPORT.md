# R133: where the lambda derivative vanishes

The degree-16 lambda polynomial has zero specialized derivative at exactly **one of the three base places** above each of R100's degree-11 and degree-22 x-points. At P11 it is the linear y-factor `y+1242` over F2^11; at P22 it is the linear y-factor `y+1612991` over F2^22. The two remaining base places above each x-point have nonzero derivative. Thus normalization of the specialized lambda polynomial is concentrated at two base places; the other four admit squarefree lambda factorization and simple-factor lifting.

| x-point | exact `gcd_y(f51, dG51/dlambda)` | zero derivative | nonzero derivative |
| --- | --- | ---: | ---: |
| P11 | `y+1242` | one degree-11 base place | degree-11 and degree-55 base places |
| P22 | `y+1612991` | one degree-22 base place | degree-44 and degree-88 base places |

The producer specializes all seven y-coefficients of the actual R51 lambda derivative `C`, using the R130 source excerpt, and computes the gcd against each R132 factor. `result.json` gives the complete coefficient arrays. The independent checker recomputes the coefficients with separate carryless multiplication, recomputes Euclidean gcds, and verifies every zero/nonzero factor product. It reports `DERIVATIVE133_INDEPENDENT_GCD_PASS`. A fresh process replay was byte-identical to SHA-256 `2e5f7e397d11f867a5aacc858d714f7a79a1521fa6a142ae0a2efe235152f2a2`. Changed-source rejection passed before output. The first negative-control fixture had a broken relative symlink, so its failure was discarded; the corrected fixture changed the pinned R130 source and was rejected with `AssertionError: r130_excerpt`. Lean checks only the finite degree partition. Local exact work used no cloud resources.

This gcd locates derivative-zero specializations of the monogenic order. It does not yet determine lambda branch counts, ramification, the order of the discriminant, or N51 valuations. Next: factor G51 at the four derivative-unit base places, then compute Hasse/Newton expansions at the two exceptional linear places. R100's P11 and P22 multiplicities suggest different local orders, but the gcd alone cannot determine them.

Source and protocol were pushed before data at `b4f1d7f26`. The diagram `publication/support133.svg` and matching PDF display the y-degree partition.
