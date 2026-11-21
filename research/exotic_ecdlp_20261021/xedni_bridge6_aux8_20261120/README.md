# An order-eight auxiliary policy constructs a coefficient-six local bridge

The frozen coefficient search constructs a rational curve with **both auxiliary coefficient-six relations**: `#E(F₇)=4` and `P+6Q=O` at 7, plus `#E(F₁₁)=8` and `P+6Q=O` at 11. For the fixed source pair `P=(2,2)`, `Q=(-16,17)`, the first accepted local multipliers are `(-3,3,1)` at 7 and `(-3,0,1)` at 11. Their centered CRT combination is `(-3,-11,1)`, giving the integral Weierstrass coefficients `[-141,-517,47,-9683,20960]`. On this rational curve the exact group check records `P+6Q≠O`; the construction has passed the local lift gates and is a concrete input to a rational-lift search.

| Stage | Exact count or value |
| --- | ---: |
| Source pair | Index 31 of the frozen 1,722-pair law |
| Local triples examined at 7 / 11 | 343 / 1,331 |
| Local gate passes at 7 / 11 | 2 / 4 |
| Selected centered CRT triple | `(-3,-11,1)` |
| Constructed coefficients | `[-141,-517,47,-9683,20960]` |
| Constructed curve's group orders at 7 / 11 | 4 / 8 |
| Constructed curve's local coefficient-six relations | true / true |
| Constructed curve's rational coefficient-six relation | false |

The [protocol](protocol.json) was committed before execution as `1bf0a85092f`. It fixes the source pair, local coefficient boxes, lexicographic first-pass rule, per-coordinate centered CRT, and rational check. The [runner](run.py) retains all 1,674 local statuses in [compressed rows](local-rows.jsonl.gz), including 44 bad-reduction rows at 7 and 112 at 11. The [result](result.json) records source and arithmetic hashes, row hashes, every status count, both selected local triples, the CRT coefficients, group orders, and relation checks. A [checked-Sage audit](sage_audit.py) independently reconstructs and checks all 1,674 local rows and the rational CRT curve; its [result](sage-audit-result.json) is `PASS`. The [runtime receipt](sage-runtime-info.json) precedes Sage work and stderr is empty. This is exact-arithmetic construction evidence, without a wall-time comparison.

## Mathematical and formal proof map

The local multipliers agree with the centered multipliers modulo their respective primes. Because the two point equations determine `a₄,a₆` for this pair and its x-coordinate difference is 18, coprime to 7 and 11, the five constructed Weierstrass coefficients reduce to the selected local coefficient vectors at both primes. The [Lean module](formal/CRT.lean) checks the centered multiplier congruence and all five coefficient congruences at 7 and 11. Its [build output](formal/build-output.txt) lists only standard Lean foundations under `#print axioms`; no reported theorem uses `sorry`, `admit`, a custom claim axiom, or native evaluation. The pinned toolchain is `v4.35.0-rc4`, with Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9`.

| Result | Lean declaration or exact receipt | Scope | Status |
| --- | --- | --- | --- |
| Centered CRT multiplier | `XedniBridge6Aux8.centered_multiplier_congruences` | `-11≡3 mod 7`, `-11≡0 mod 11` | Lean checked |
| Five coefficient reductions | `curve_coefficients_mod7`, `curve_coefficients_mod11` | Exact three selected coefficient vectors | Lean checked |
| Group orders and local relations | [Sage audit](sage-audit-result.json) | Exact curves over `F₇` and `F₁₁`, fixed point pair | Independently replayed exact arithmetic; full finite-field curve binding in Lean remains open |
| Rational relation flag | [Sage audit](sage-audit-result.json) | Exact constructed curve and pair | Independently replayed exact arithmetic |

The source pair was selected using its reduced coefficient-six relation as a **correctness control**, and the two local searches cost 1,674 coefficient checks. This is not an input-independent selector for an unknown point. The next experiment can vary representatives congruent to `(-3,-11,1)` modulo 77 and test rational relation yield with a frozen height/resource cap, then extend the auxiliary policy to the full 1,722-pair law. Any rational relation should be evaluated for novel row rank and target coverage before one-target timing against same-point rho.

## Replay

```sh
python3 research/exotic_ecdlp_20261021/xedni_bridge6_aux8_20261120/run.py > research/exotic_ecdlp_20261021/xedni_bridge6_aux8_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_bridge6_aux8_20261120/run.stderr
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_bridge6_aux8_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python /Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119/research/exotic_ecdlp_20261021/xedni_bridge6_aux8_20261120/sage_audit.py > research/exotic_ecdlp_20261021/xedni_bridge6_aux8_20261120/sage-audit-result.json 2> research/exotic_ecdlp_20261021/xedni_bridge6_aux8_20261120/sage-audit.stderr
sh research/exotic_ecdlp_20261021/xedni_bridge6_aux8_20261120/formal/build.sh > research/exotic_ecdlp_20261021/xedni_bridge6_aux8_20261120/formal/build-output.txt 2> research/exotic_ecdlp_20261021/xedni_bridge6_aux8_20261120/formal/build-stderr.txt
```
