# Reduction map of multiples on one fixed rational witness curve

**The known rational lift `Q=(-4,19)` on `E: y²+94xy+188y=x³+1738x+3805` reaches every nonidentity point of the fixed 43-point `E(F₄₇)` under reduction.** Its 42 exact rational multiples `kQ`, for `k=1,…,42`, reduce to the 42 distinct nonidentity points in the frozen full-law source set, once each. The same fixed curve was a Lean-checked rational-relation witness in the [wide-period packet](../xedni_wide_period_relation_20261120/README.md). The rational coordinates grow with the multiple: at `k=42`, the `x` numerator and denominator use 1,937 and 1,927 bits, and the `y` numerator and denominator use 2,904 and 2,890 bits.

| Complete fixed-curve enumeration | Exact value |
| --- | ---: |
| Rational multiples checked | 42 |
| Distinct nonidentity reductions at 47 | 42 |
| Reduced curve order / reduced `Q` order | 43 / 43 |
| Largest `x` numerator / denominator bit length | 1,937 / 1,927 |
| Largest `y` numerator / denominator bit length | 2,904 / 2,890 |

The [protocol](protocol.json) and [Sage runner](run_sage.py) were pushed before measurement in `435a8f44a65`. Every [row](multiple-rows.jsonl) retains the scalar index, reduced point, exact rational-coordinate hashes, and numerator/denominator bit lengths. The [result](result.json) fixes the row digest. The runner checks each incremental rational addition against independent Sage scalar multiplication, reduces the rational coordinates at 47, checks the point against `kG` on the 43-point reduced curve, and compares the complete set with the frozen source law. An [independent Fraction group-law audit](audit.py) reproduces every rational-coordinate hash, bit length, and reduction; its [result](audit-result.json) passes. The accepted [Sage runtime](sage-runtime-info.json) and empty stderr are retained. These bit lengths are exact integer sizes, not a timed performance estimate.

## Proof map and selector boundary

The [Lean theorem](formal/LiftIndex.lean) `reduction_of_multiple` proves `ρ(kQ)=kρ(Q)` for every natural `k` and an additive reduction map supplied as a hypothesis. `unique_scalar_index` proves that, in a nonzero generator coordinate of `ZMod 43`, each target has exactly one scalar index. Its pinned Lean `v4.35.0-rc4` / Mathlib `021c46bdd042f67b538b9f9871d556a93713a9a9` [build](formal/build-output.txt) reports only standard `propext`, `Classical.choice`, and `Quot.sound` axioms. The concrete good-reduction map, identification of the 43-point elliptic-curve group with `ZMod 43`, and 42 rational coordinate calculations are checked by the Sage and independent arithmetic packets rather than constructed in Lean. That is the remaining formalization bridge; `lean4checker --fresh` is unavailable on this host.

This enumeration supplies rational lifts when the index `k` is already known. For a public target specified only as a reduced point, choosing which multiple `kQ` reduces to it is exactly the scalar-index question in this fixed generator coordinate. Other rational-lift constructions could choose points by a different method; this experiment does not bound them. A useful next test must specify such a method, its target-dependent work, and whether it yields independent relations on one curve rather than merely reproducing the known multiple index.

Replay:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_fixed_lift_map_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_fixed_lift_map_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_fixed_lift_map_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_fixed_lift_map_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_fixed_lift_map_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_fixed_lift_map_20261120/audit-result.json
sh research/exotic_ecdlp_20261021/xedni_fixed_lift_map_20261120/formal/build.sh > research/exotic_ecdlp_20261021/xedni_fixed_lift_map_20261120/formal/build-output.txt
```
