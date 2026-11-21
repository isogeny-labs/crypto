# Coefficient-six bridge meets an order-five auxiliary gate

**The proposed order-five auxiliary gate at 11 is impossible for every nonsingular elliptic curve over `F₁₁`.** Hasse's bound requires `|#E(F₁₁)−12|² ≤ 44`, while order five gives `49`. This structural obstruction resolves the gate before coefficient search. The relation word `P+6Q=O` would connect all three sign-folded components left by the `P+2Q` word on the fixed order-43 curve, so a different Hasse-admissible auxiliary policy is needed.

The frozen diagnostic remains useful for tracing the attempted policy: it narrows 1,722 ordered source pairs to two rational point lifts and checks all 250 coefficient triples in `[-2,2]^3`. Four constructed curves pass the order-four relation gate at 7; each has nonintegral coefficients at 11. All 250 fail the simultaneous gates, as the Hasse argument requires, and none satisfies the rational coefficient-six relation. The [versioned amendment](amendment.json) records the missed pre-run admissibility check without changing the frozen protocol or rows.

| Stage | Exact count |
| --- | ---: |
| Frozen ordered source pairs | 1,722 |
| `P+6Q=O` at the fixed `F₄₇` reduction | 42 |
| Also equal x-coordinates modulo 11 | 2, source indices 1328 and 1370 |
| Coefficient triples per surviving pair | 125 |
| Constructed nonsingular rational curves | 250 |
| Order-four relation gate passes at 7 | 4 |
| Those four with integral coefficients at 11 | 0 |
| Simultaneous auxiliary passes / rational relations | 0 / 0 |

The [protocol](protocol.json) was committed before the checked-Sage run as `567b91092ae`. It fixes the full source law, the post-run reduced-compatibility filter, the necessary x-coordinate filter, the coefficient box, the two local gates, and the rational check. The [source-selection rows](source-selection.jsonl.gz) retain all 1,722 statuses: 1,680 reduced-excluded, 40 x-excluded, and two candidates. The [candidate rows](candidate-rows.jsonl.gz) retain every constructed coefficient tuple, local status, order, relation flag, and rational relation flag. At 7, 224 choices have the wrong order, 22 have bad reduction, and four pass. At 11, 230 are nonintegral, 18 have the wrong order, and two have bad reduction. The [result](result.json) stores both raw and compressed stream hashes; the [runtime receipt](sage-runtime-info.json) records the accepted local Sage build. These are exact-arithmetic correctness runs, without a timing comparison.

The [independent audit](audit.py) recomputes all 1,722 fixed-field group sums, all 250 rational coefficients and discriminants, the 250 rational group sums, and both finite-field curve orders and relation flags using Python fractions and an independent general-Weierstrass group law. Its [result](audit-result.json) is `PASS`, and both final stderr files are empty.

## Proof map and scope

The [pinned Lean module](formal/OrderFive.lean) proves `no_order_five_under_F11_hasse_square`: any natural card satisfying the squared Hasse inequality at 11 differs from five. It also proves the exact arithmetic contradiction `49>44`. The full elliptic-curve Hasse theorem is used in the written proof; the bridge from a concrete curve's cardinality to that inequality remains a separate Lean formalization obligation. The module additionally proves `coefficient_six_forces_negation`: if `5Q=O` and `P+6Q=O` in any abelian group, then `P=-Q`. On a nonsingular Weierstrass curve, inverse points have the same x-coordinate, giving the recorded source-pair prerequisite. Its [build output](formal/build-output.txt) reports only standard Lean foundations under `#print axioms`; these declarations use no `sorry`, `admit`, custom claim axiom, or native evaluation. The toolchain is `v4.35.0-rc4` with Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9`.

| Result | Lean declaration or receipt | Exact hypothesis | Status |
| --- | --- | --- | --- |
| Order five contradicts the `F₁₁` Hasse square | `order_five_violates_F11_hasse_square` | Integer inequality `((5:ℤ)-12)^2 ≤ 44` | Lean checked contradiction |
| Hasse admissibility excludes order five | `no_order_five_under_F11_hasse_square` | Natural `card` and squared Hasse inequality `((card:ℤ)-12)^2 ≤ 44` | Lean checked conditional implication |
| Coefficient-six word under order-five annihilation | `coefficient_six_forces_negation` | Additive commutative group; `5 • Q=0`, `P+6 • Q=0` | Lean checked |
| Curve-cardinality bridge | Hasse theorem for elliptic curves over `F₁₁` | Nonsingular elliptic curve, `card=#E(F₁₁)` | Written proof; full Lean formalization open |
| Bounded coefficient-box statuses | [Sage result](result.json), [independent audit](audit-result.json) | Frozen 1,722-pair law, two-pair post-run filter, `[-2,2]^3` | Independent exact arithmetic replay |

The two-pair filter is a **post-run feasibility diagnostic**: it uses the reduced coefficient-six relation and cannot serve as an uncharged selection rule for an unknown target. The earlier order-six gate at 11 also excludes coefficient six structurally because `6Q=O` there would force `P=O`. This packet excludes the proposed order-five policy universally; the 250-curve result has the narrower declared coefficient box. The next screen must choose a Hasse-admissible order and derive a full-law, input-independent auxiliary policy before testing wider coefficient representatives. If a rational bridge appears, its contribution must be measured by novel relation rank and target coverage before a complete candidate is compared with same-point rho.

## Replay

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_bridge6_aux5_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python /Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119/research/exotic_ecdlp_20261021/xedni_bridge6_aux5_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_bridge6_aux5_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_bridge6_aux5_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_bridge6_aux5_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_bridge6_aux5_20261120/audit-result.json 2> research/exotic_ecdlp_20261021/xedni_bridge6_aux5_20261120/audit.stderr
sh research/exotic_ecdlp_20261021/xedni_bridge6_aux5_20261120/formal/build.sh > research/exotic_ecdlp_20261021/xedni_bridge6_aux5_20261120/formal/build-output.txt 2> research/exotic_ecdlp_20261021/xedni_bridge6_aux5_20261120/formal/build-stderr.txt
```
