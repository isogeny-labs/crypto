# Original genus-four targets solved through the rational quotient

The frozen release binary exited zero. The theta arm recovered both original scalars from rational plane triples, the calibrated factor-base logarithms and two distinct anchor offsets. Each scalar passed multiplication on the original hyperelliptic Jacobian. Both source slope estimates are mutually consistent: 17 × 26616 mod 239753 = 212719.

| Source multiple | Source witnesses m | Target offsets a | Scalar | Prepared original (ms) | Theta descent (ms) |
|---:|---|---|---:|---:|---:|
| 1 | 2, 10 | 37, 38 | 122246 | 0.735417 | 207.671875 |
| 17 | 38, 48 | 31, 152 | 226885 | 1.038417 | 912.224958 |

The theta arm checked one final scalar candidate per fixture. For the second fixture it visited 48 source multiples and 153 target anchors. Each degree-four visit currently constructs a dense extension-field spinor by Clifford multiplication and elimination. A chart hit recomputes the spinor and differentiates four sets of equations. This implementation performs the mathematical transport through section evaluations and gradients. It returns a scalar on the original target without constructing an explicit divisor map for arbitrary input classes.

The online theta times were about 282 and 879 times the prepared-original times. These are single paired observations, with prior curve reconstruction and factor-base precomputation omitted from the theta times. Neither case met the performance goal. With these reference times the total budgets for a tenfold improvement are about 74 and 104 microseconds respectively. Even before charging setup, an optimized path would need a materially cheaper per-anchor formula, fewer anchors, or both.
