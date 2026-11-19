# Prime PKM Semaev / T₂-Torus Acceleration & P-256 Aux Chase
Branch `cursor/prime-pkm-aux-tower-3858` · PR #4820 · 2026-11-19
Standing: all measurements `[pcc:unverified]`.
Smoke note: `PKM_FAST_SMOKE=1` skipped bits≥16 algebraic rungs, M31 Dickson e2e, exact P-256 depth-8 F4 presentations, and Panel A empirical scaling (library unit tests still cover ParamS5 / tower soundness / P-256 FB id).

## Lead P0-1c — ParamS5, algebraic rungs, Dickson
### ParamS5 specialize vs grid
| Field | m | setup mults | spec/target | grid baseline | speedup | mismatches |
|---|---:|---:|---:|---:|---:|---:|
| alg-k20 (icv1-fp20-t727-cd198a38) | 2 | 659 | 7 | 1850 | 264.3× | 0 |
| alg-k20 (icv1-fp20-t727-cd198a38) | 3 | 21785 | 97 | 64800 | 668.0× | 0 |
| alg-k20 (icv1-fp20-t727-cd198a38) | 4 | 27207299 | 3547 | 2951786 | 832.2× | 0 |
| M31 (2^31 - 1) | 2 | 920 | 7 | 1850 | 264.3× | 0 |
| M31 (2^31 - 1) | 3 | 22945 | 97 | 64800 | 668.0× | 0 |
| M31 (2^31 - 1) | 4 | 27351110 | 3547 | 2951786 | 832.2× | 0 |

### Algebraic isogeny rungs (verified `[d]G==Q`)
| bits | oracle | ops speedup | S(0.1) speedup | wall speedup | vs ρ S(0.1) |
|---:|---|---:|---:|---:|---:|
| 12 | `pkm-semaev:m=2,guess=2,trace=1,inherit=1,descent=1` | 1.16× | 1.16× | 3.23× | ρ=4.72 |
| 12 | `pkm-semaev:m=2,guess=2,trace=1,inherit=1,descent=1` | 1.17× | 1.17× | 2.93× | ρ=2.69 |
| 14 | `pkm-semaev:m=2,guess=2,trace=1,inherit=1,descent=1` | 1.64× | 1.63× | 36.16× | ρ=2.21 |
| 14 | `pkm-semaev:m=2,guess=2,trace=1,inherit=1,descent=1` | 1.64× | 1.63× | 34.29× | ρ=1.71 |
| 16 | `pkm-semaev:m=2,guess=2,trace=1,inherit=1,descent=1` | 2.05× | 2.05× | 30.49× | ρ=2.55 |
| 16 | `pkm-semaev:m=2,guess=2,trace=1,inherit=1,descent=1` | 2.05× | 2.05× | 22.50× | ρ=1.33 |
| 20 | `pkm-semaev:m=2,guess=3,trace=1,inherit=1,descent=1` | 2.17× | 2.17× | 21.30× | ρ=0.84 |
| 12 | `pkm-semaev:m=3,guess=1,mode=sym,trace=1,inherit=1,descent=1` | 0.93× | 0.93× | 0.93× | ρ=4.72 |

### M13 Dickson blind pipeline
| m | t | mode | ops/call speedup | S(0.1) speedup | wall speedup | verified |
|---:|---:|---|---:|---:|---:|---|
| 3 | 3 | chain | 0.99× | 0.99× | 0.97× | True |
| 4 | 2 | sym | 2.46× | 2.46× | 3.36× | True |
| 3 | 3 | chain | 1.96× | 3.23× | 1.46× | True |
| 4 | 2 | chain | 1.21× | 1.43× | 1.59× | True |
| 4 | 2 | sym | 1.87× | 2.66× | 1.82× | True |
| 5 | 2 | chain | 1.02× | 1.28× | 0.85× | True |

## Lead P1-3a — m=8 KM3-SPEC-LADDER / rational_prime_growth
| family | source | w | α | γ | E_all | kill_all | kill_one |
|---|---|---:|---:|---:|---:|---|---|
| Kummer (2^t | p - 1) | Lean rational_prime_growth + KM3-SPEC-LADDER (c = 1/14) | 1 | 0.00893 | 0.589 | 0.2378 | False | False |
| Kummer (2^t | p - 1) | Lean rational_prime_growth + KM3-SPEC-LADDER (c = 1/14) | 2 | 0.00893 | 1.178 | 0.2557 | False | False |
| Kummer (2^t | p - 1) | KM3-SPEC-LADDER from sharp m=2 LFD >= t (c = 1.0) | 1 | 0.12500 | 4.349 | 0.6212 | True | True |
| Kummer (2^t | p - 1) | KM3-SPEC-LADDER from sharp m=2 LFD >= t (c = 1.0) | 2 | 0.12500 | 8.697 | 1.2424 | True | True |
| T_2 Torus / Dickson (p = 3 mod 4, 2^t | p + 1) | Cayley F_{p^2} half-degree transfer of Lean floor (c = 1/28) | 1 | 0.00446 | 0.330 | 0.2307 | False | False |
| T_2 Torus / Dickson (p = 3 mod 4, 2^t | p + 1) | Cayley F_{p^2} half-degree transfer of Lean floor (c = 1/28) | 2 | 0.00446 | 0.660 | 0.2398 | False | False |
| T_2 Torus / Dickson (p = 3 mod 4, 2^t | p + 1) | Cayley F_{p^2} half-degree transfer of m=2 LFD >= t (c = 0.5) | 1 | 0.06250 | 2.698 | 0.3855 | False | False |
| T_2 Torus / Dickson (p = 3 mod 4, 2^t | p + 1) | Cayley F_{p^2} half-degree transfer of m=2 LFD >= t (c = 0.5) | 2 | 0.06250 | 5.397 | 0.7709 | True | True |
| T_2 Torus / Dickson (p = 3 mod 4, 2^t | p + 1) | Direct leading-ideal equivalence y_j^2 -> y_{j+1}+2 (c = 1.0) | 1 | 0.12500 | 4.349 | 0.6212 | True | True |
| T_2 Torus / Dickson (p = 3 mod 4, 2^t | p + 1) | Direct leading-ideal equivalence y_j^2 -> y_{j+1}+2 (c = 1.0) | 2 | 0.12500 | 8.697 | 1.2424 | True | True |

## Lead P2-5 — Scaled j1728 / P-256-aligned towers
| curve | d | m | D | cols | ops speedup | wall speedup |
|---|---:|---:|---:|---:|---:|---:|
| M19 (p = 2^19 - 1, w^2 = x^3 - 3x) | 3 | 2 | 5 | 63 | 1.91× | 7.87× |
| M19 (p = 2^19 - 1, w^2 = x^3 - 3x) | 4 | 2 | 5 | 206 | 2.11× | 6.25× |
| M19 (p = 2^19 - 1, w^2 = x^3 - 3x) | 3 | 3 | 6 | 1541 | 3.38× | 2.07× |
| M31 (p = 2^31 - 1, w^2 = x^3 - 3x) | 3 | 2 | 5 | 63 | 1.91× | 8.41× |
| M31 (p = 2^31 - 1, w^2 = x^3 - 3x) | 4 | 2 | 5 | 201 | 2.11× | 5.76× |
| M31 (p = 2^31 - 1, w^2 = x^3 - 3x) | 3 | 3 | 6 | 1528 | 3.38× | 1.94× |

Exact P-256 depth-8 presentation F4: `FB1h16882188194e` (smoke-skipped).

## Verdicts `[pcc:unverified]`
- ParamS5 specialization is `264×–832×` cheaper than grid interpolation (0 mismatches).
- Trace+inherit+descent wins `1.16×–1.64×` ops / up to `~35×` wall on m=2 isogeny rungs at 12–14 bits; m=3 sym at 12 bits is a slight ops regression (`0.93×`).
- M13 Dickson m=4/sym gets `2.46×` ops / `3.2×` wall; m=3/chain does not beat baseline grid.
- Sharp m=2 LFD≥t (c=1) kills Kummer m=8 under both all- and one-output floors; Lean c=1/14 does not. T₂ with direct c=1 also kills; Cayley half-degree Lean floor does not.
- Scaled P-256-aligned towers: trace replay `1.9×–3.4×` ops, `2.3×–9.0×` wall on M19/M31.
