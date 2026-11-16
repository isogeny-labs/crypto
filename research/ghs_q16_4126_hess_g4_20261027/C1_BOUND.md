# Exact genus bound for the standard cubic Hess descent

27 October 2026. This is a complete **finite-field classification within Hess's published `c=1` construction**, not a classification of all GHS-style covers and not a vulnerability result. The [executable census](c1_census.py), [certificate](c1_certificate.json), [alternate audit](c1_audit.py), [Magma controls](c1_controls_receipt.json), and [Lean count proof](C1Counts.lean) retain the claim and its limits.

## Criterion and rigorous scope

Let `k=GF(16)`, `K=GF(4096)`, `E_(a,b): y²+xy=x³+a x²+b`, `a∈{0,32}`, `b∈K*`, and `δ=√b`. Choose Hess's factor `c=1∈k*`. Since `[K:k]=3` in characteristic two, `Tr_(K/k)(1)=1`, so the Frobenius descent condition is satisfied for either trace class of `a`. The cubic Frobenius module of `δ` has three disjoint cases:

1. `δ∈k*` (equivalently `b∈k*`): Frobenius span dimension one; the `c=1` descent has genus one.
2. `δ∈U*=ker Tr_(K/k)\{0}` (equivalently `Tr_(K/k)(b)=0`): Frobenius span dimension two; the `c=1` descent has genus three.
3. `δ∉k∪U`: Frobenius span dimension three; the `c=1` descent has genus four.

The dimension assertion follows from the semisimple decomposition of the cyclic order-three Frobenius operator in characteristic two: its minimal polynomial divides `(T+1)(T²+T+1)`. The fixed line `k` and trace-zero plane `U` intersect only at zero. Substitution in [Hess's genus formula and descent condition](https://doi.org/10.1112/S146115700000108X), as already worked out in the [prior complete genus-three criterion](../ghs_cubic_hess_closure_q16_20261027/REPORT.md), gives the genera above. This argument concerns the exact `c=1` model; it does not assert minimal genus across arbitrary covers.

| Normalized ordinary models | `c=1` genus 1 | genus 3 | genus 4 | total |
| :--- | ---: | ---: | ---: | ---: |
| All `a∈{0,32}`, `b∈K*` | 30 | 510 | 7650 | 8190 |
| Outside the complete genus-three Hess isogeny closure | 10 | 0 | 2865 | 2875 |

The first row follows from `|k*|=15`, `|U*|=16²−1=255`, and `|K*\(k∪U)|=3825`, each multiplied by the two normalized `a` choices. The second row is an exact cross-tabulation against all 128 source orders and 8,190 equations in the prior frozen closure certificate. The ten residual subfield equations have orders 3982, 4090, or 4214 and their exact `a,b` codes are in the certificate. They are a genuine exception to the statement that every residual model has a genus-four `c=1` image.

Three independent Magma `WeilDescentGenus(E,k,1)` controls agree with the three cases: `b=372` gives genus one, `b=14` genus three, and `b=63` genus four, all with `a=32` over the same field basis. The alternate Sage audit recomputes the complete partition via `Tr(b)=0` rather than taking square roots and checks every per-order count. [C1Counts.lean](C1Counts.lean) checks the finite cardinality arithmetic, while the field/module argument and Hess theorem remain mathematical inputs external to Lean.

## What this bound does and does not give

The exact census shows that **2,865** normalized models outside the genus-three Hess isogeny closure have a direct, published genus-four `c=1` descent curve. For the four high-prime sources with newly or previously witnessed quartics, the separate [full attack benchmark](REPORT.md) evaluates the source-subgroup map and solves the transferred target. Those four complete transfer runs are slower than rho by measured factors 512–698 in the Magma calculator backend. The genus count alone does not certify nonzero action on every other source subgroup, an evaluable map to any particular quartic, a full solver cost for the other 2,861 models, or a new vulnerable family.

The earlier genus-three closure statement is therefore a **lower boundary for that exact construction**, not a security boundary: most residual sources still have a genus-four Hess image. The ten subfield exceptions require separate subgroup-map analysis; a genus-one image can kill the chosen large subgroup.

## Reproduction

```sh
sage -python research/ghs_q16_4126_hess_g4_20261027/c1_census.py
python3 research/ghs_q16_4126_hess_g4_20261027/run_magma.py c1_controls
sage -python research/ghs_q16_4126_hess_g4_20261027/c1_audit.py
lean research/ghs_q16_4126_hess_g4_20261027/C1Counts.lean
```
