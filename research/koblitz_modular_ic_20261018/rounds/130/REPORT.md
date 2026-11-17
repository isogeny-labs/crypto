# R130 — complete degree-7 lambda fiber of the degree-112 source

The actual degree-16 G51 equation is squarefree over all three normalized R129 base residue fields. Its lambda-factor census is:

| Base factor | Base residue field | Lambda factors | F2 residue degrees and inherited ramification |
|---|---|---|---|
| `Y+123` | F128 | two irreducibles of degree 8 | two places `(56,1)` |
| `Y+90` | F128 | two irreducibles of degree 8 | two places `(56,2)` |
| `Q4` | F₂²⁸ | four linears and four cubics | four `(28,1)` and four `(84,1)` |

The derivatives with respect to lambda are the nonzero residue scalars 93, 70 and 28271853 respectively. Simple-factor lifting therefore gives an unramified lambda extension at these base completions; the only ramification index 2 is inherited from R129's base branch. There are **12 normalized F₂ places** over the degree-7 closed x-point. Their weighted x-fiber degree is `2·56 + 2·56·2 + 4·28 + 4·84 = 784 = 112·7`. These are complete place counts and degrees for this fiber, while N51 values on the twelve places remain to compute.

The committed 9,969-byte `f51/G51` excerpt is byte-exactly extracted from the R51 native donor (full donor SHA-256 in `protocol.json`). The producer pins that excerpt, the R105 parser, and R127/R129 results. It specializes all 29 actual G51 terms, then uses finite-field Frobenius and polynomial gcd to obtain distinct-degree factor products. The independent checker uses separate carryless F128 arithmetic and quartic-extension operations, recomputes all specialized G51 coefficients, verifies each factor product, and checks irreducible degrees using Frobenius tests and gcd at proper divisors. It also compares the excerpt against the available full R51 donor. Byte replay and a changed-excerpt control passed; Lean 4.34.1 checked derivative nonzero and all finite degree sums.

Source/protocol commit: `257de987d`. Result SHA-256: `27a0c684f1dd41bb02f33e6d2943626a11ee27832a424c58eee24d0f69383cf2`. Local exact computation, zero cloud cost. The interpretation uses [Stacks' Henselian factor lifting](https://stacks.math.columbia.edu/tag/04GE) and [étale base change](https://stacks.math.columbia.edu/tag/00U0); Lean does not formalize those general theorems.

Next evaluate the actual N51 rational coordinate at each of these twelve places, including the base-ramified pair. Then proceed to the degree-11/22 lambda-discriminant fibers and infinity to complete the divisor geometry.
