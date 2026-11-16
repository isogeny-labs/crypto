# Trace +22 carrier: bounded degree-seven search

16 November 2026 UTC. The two frozen source-orbit screens in [PROTOCOL.md](PROTOCOL.md) completed with **zero exact degree-seven covers** in the `B=H²` and `B=Q_iH²`, quadratic-`H` shapes on carrier mask 32280. This is a scoped null result. Finite claims remain [pcc:unverified] until PCC admission.

## Exact curves and evidence

Let `K=F3[a]/(a^5+2a+1)` and

```text
Y/F3:    y²=x^11+x^9+x^8+2x^7+2x^6+2x^4+x³+x²+2x
E_68/K:  v²=u³+u²+2+a+a²+2a³
E_84/K:  v²=u³+u²+a+a⁴.
```

The carrier has Weil polynomial `X^10−22X^5+243`; both sources have trace +22 and order 222 over K. Masks 68 and 84 represent the two five-element source Frobenius orbits. The search source SHA-256 is `2efd9340a653158fb4f2cfd43b1501469cc03c80d67d2d254b93e5056b9e012d`, matching both complete [mask-68](screen_68.json) and [mask-84](screen_84.json) receipts. The associated stderr files retain `/usr/bin/time -l` measurements.

| Source mask | `H²` systems | `Q_iH²` systems | Final exact hits | User CPU seconds |
|---:|---:|---:|---:|---:|
| 68 | 243 | 405 | 0 | 167.83 |
| 84 | 243 | 405 | 0 | 167.29 |

Each elimination represents 37,946,232 logical assignments. The program verifies squarefreeness, the five quadratic branch factors, one source-cubic K-root, exact source order, degree seven, numerator-denominator coprimality and the identity `fC²=B(A³+A²B+bB³)` for every final candidate. It found no `C` square in the specified shapes. Both receipts come from one algebra-system implementation; an independent mathematical elimination is not claimed. The null result also transports to the corresponding simultaneous quadratic twists of these exact curves and shapes, because untwisting preserves the cover equation and degree. It does not exclude other genus-five carriers in the trace ±22 classes or other denominator shapes and degrees.

## Cost screen and construction gate

The [exact extension-order receipt](order_screen.json) computes the Lucas trace recurrence `T_0=2`, `T_1=22`, `T_s=22T_(s−1)−243T_(s−2)` and `#E(F_(3^(5s)))=243^s+1−T_s`, with all reported prime factors checked by Sage.

| `s` | Largest source prime | Bits |
|---:|---:|---:|
| 7 | 1,541,616,119 | 31 |
| 11 | 785,807,256,797,924,665,975,331 | 80 |
| 13 | 68,559,575,550,818,977 | 56 |

At `s=11`, the bare genus-five Diem exponent gives `log₂((3^11)^(8/5))≈27.90`, while Frobenius-aware rho gives `log₂(sqrt(r/11))≈37.96` for the 80-bit prime. The units and constants differ, and no evaluated cover or subgroup map is available for this carrier. These powers are a lead-selection screen, not a measured attack or a vulnerability claim. A different trace +22 carrier, correspondence, or denominator shape must first produce an explicit nonzero map; only then can a full same-target comparison be run.

The shared KB and targeted exact-equation web searches found no matching publication for this fixed `f`, but absence of a search result does not establish priority. The carrier census is already recorded in `../ghs_odd5_carrier_20261115/THEOREM.md`. [Iijima–Momose–Chao 2015](https://eprint.iacr.org/2015/805.pdf), [Joux–Vitse 2012](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf), and [Diem 2006](https://arxiv.org/abs/math/0606607) remain the primary nearby construction and attack baselines. No new vulnerable trace +22 family has been established.
