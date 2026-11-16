# Trace +10 carrier: bounded degree-seven search

16 November 2026 UTC. The frozen search in [PROTOCOL.md](PROTOCOL.md) finished on both trace +10 Frobenius orbit representatives. It found **zero exact degree-seven covers** with `B=H²` or `B=Q_i H²` and monic quadratic `H` for this fixed genus-five carrier. This is a null result for those shapes, not a classification of all trace +10 covers. Finite claims remain [pcc:unverified] until PCC admission.

## Exact objects and receipts

Let `K=F3[a]/(a^5+2a+1)` and `Y/F3: y²=x^11+x^7+2x^5+2x^4+x^3+x²+1`. The target carrier has Weil polynomial `X^10−10X^5+243` and is therefore a distinct isogeny class from the completed trace ±26 cover orbit. The two source equations are

```text
E_12/K:  v²=u³+u²+a+a²
E_151/K: v²=u³+u²+1+2a+a²+2a³+a⁴.
```

Both have 234 K-points. Their coefficients represent the two five-element Frobenius orbits in the trace +10 class, as fixed in the protocol. `f` factors over K into one linear and five quadratic factors; the source cubics each have exactly one K-root. The adapted search program checks these premises and the polynomial identity for every final candidate. Its SHA-256 is `60937b599b11235811729058f4252f262cc68fb5477e5ad8c3f5280dab5a3969`, matching both saved [mask-12](screen_12.json) and [mask-151](screen_151.json) receipts.

| Source mask | `H²` systems | `Q_iH²` systems | Final exact hits | User CPU seconds |
|---:|---:|---:|---:|---:|
| 12 | 243 | 405 | 0 | 172.52 |
| 151 | 243 | 405 | 0 | 173.01 |

The two bounded eliminations each represent 37,946,232 logical denominator-and-branch assignments; resultants and specialized roots avoid enumerating that full grid directly. The complete counters and `/usr/bin/time -l` records are saved. The two runs used the same script, so this is one algebra-system implementation rather than an independent mathematical replay. A zero hit excludes only the frozen denominator shapes on this fixed `Y` and the two source orbits; another genus-five carrier, a different pole shape, or a higher-degree map may still exist.

## Subgroup-size screen

[The exact recurrence and factor receipt](order_screen.json) computes `T_0=2`, `T_1=10`, `T_s=10T_(s−1)−243T_(s−2)` and `#E(F_(3^(5s)))=243^s+1−T_s`. Sage checked every listed factor prime and multiplied factors back to each group order.

| `s` | Largest source prime | Bits | Interpretation |
|---:|---:|---:|---|
| 7 | 82,307 | 17 | Too small to support a meaningful crossover test here |
| 11 | 7,657,219,609,931 | 43 | Large enough for a transfer test, but no map is known |
| 13 | 808,759,483,376,157,344,971,031 | 80 | Leading exponents would merit testing if a map is constructed |

At `s=13`, the bare Diem genus-five exponent is `log₂(q^(8/5))≈32.97` for `q=3^13`, and the Frobenius-aware rho scale is `log₂(sqrt(r/13))≈37.86` for the 80-bit factor. These are different operation units with omitted constants and no transfer map; they are a research screen, not an attack-cost comparison or vulnerability claim.

## Prior work and next construction gate

The local shared KB search and targeted exact-equation web search found no published match for this carrier equation, but absence from those searches does not establish priority. The carrier identification and 100-class census are already project records in `../ghs_odd5_carrier_20261115/THEOREM.md`. [Iijima–Momose–Chao 2015](https://eprint.iacr.org/2015/805.pdf) describes a nearby direct `(2,...,2)` degree-five genus-five case; [Joux–Vitse 2012](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf) and [Diem 2006](https://arxiv.org/abs/math/0606607) supply established cover/decomposition and asymptotic attack mechanisms. The direct case does not classify every degree-seven map.

The next decisive construction gate is an explicit map on a **different trace +10 carrier model** or a different denominator/pole configuration, followed by an independently replayed nonzero prime-subgroup image. Without that map there is no end-to-end GHS attack to price. The completed trace ±26 result remains the only measured GHS-faster exact instance in this branch.
