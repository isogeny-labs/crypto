# Exact target coverage under a fixed-curve xedni height cap

For `E/ℚ: y²=x³−47x²+751x−3997`, generator `Q=(17,10)`, and the candidate law `x=a/b²` in lowest terms with `H=max(|a|,b)`, the rational points satisfying **`H≤2^130`** are exactly the signed nonzero multiples `±Q,...,±13Q`. Reduction modulo 47 maps these to **26 of the 42 nonidentity points** of `E(F₄₇)`, a covered fraction of `13/21`. The remaining 16 nonidentity points require greater coordinate height on this curve.

The finite computation gives `H(13Q)` 119 bits and `H(14Q)` 136 bits. It checks every `n=1,...,20` with exact rational arithmetic; `x(−nQ)=x(nQ)` gives the negative cases. The checked-Sage replay independently reconstructs every rational multiple, its square denominator, its reduction, and the cap decision. The reduced `Q̄` has order 43, so the 26 signed multipliers are distinct modulo 43. The order and target-class group law are also established in the earlier [Lean finite-order packet](../xedni_finite_order_lean_20261121/README.md).

For the infinite tail, the earlier exact `x(65Q)` numerator has 2,894 bits and the model-specific Silverman bound is `<14`. The pinned Lean theorem proves that, under canonical-height quadraticity and those explicit height inputs, `|n|≥21` contradicts `H(nQ)≤2^130`:

`h(x(nQ)) > 441(2893·(2/3)−14)/4225−14 = 785218/4225 > 182`,

while the cap gives `h(x(nQ))≤260 log 2<182`. The group identification `E(ℚ)=ℤQ`, good-reduction homomorphism, Silverman comparison, and canonical-height law are the mathematical inputs that connect the finite census and tail to **all** rational points of this fixed curve. The generator identification has a proof-enabled Sage/mwrank certificate and independent replay; its complete Lean curve bridge remains open.

## Evidence and proof map

| Step | Evidence or declaration | Scope and status |
| --- | --- | --- |
| `E(ℚ)=ℤQ` | [rank-one generator certificate](../xedni_rank1_generator_certificate_20261121/README.md) | Proof-enabled Sage/mwrank; full Lean generator proof open |
| `Q̄` has order 43 | `XedniFiniteOrder.Q_order` | Checked on Mathlib's actual `E(F₄₇).Point` |
| Exact heights, square denominators, reductions and cap decisions for `1≤n≤20` | `run.py`, `result.json`, `sage_replay.py`, `sage-replay.json` | Independent Fraction and checked-Sage computations agree |
| `|n|≥21 ⇒ n²≥441` | `XedniHeightCoverage.tail_square` | Lean checked |
| The tail violates the cap given the displayed height inputs | `XedniHeightCoverage.tail_excluded` | Lean checked, conditional on the actual canonical-height and model comparison inputs |
| The residues of `±1,...,±13` modulo 43 are 26 distinct elements | `XedniHeightCoverage.coveredResidues_card` | Lean checked by kernel-reduced `decide` on a 26-element finite set |
| Full fixed-curve coverage count `26/42` | Finite census, tail implication and rank-one generator certificate | Written mathematical proof; direct Lean encoding of the generator, reduction map and comparison remains open |

The [protocol](protocol.json) fixed the curve, point, cap, full target law, source digests, predicted threshold and resource limit before execution. [Result JSON](result.json) retains all 20 finite height and reduction records. The checked Sage [runtime](sage-runtime-info.json), [replay](sage-replay.json), and [process receipt](sage-replay.stderr) are retained. The [Lean module](formal/HeightCoverage.lean) uses the pinned toolchain and Mathlib revision; its [build output](formal/build-output.txt) lists only `propext`, `Classical.choice`, and `Quot.sound` under `#print axioms`. No claim axiom or proof placeholder is used.

## Reproduction

From the repository root:

```sh
python3 research/exotic_ecdlp_20261021/xedni_height_coverage_profile_20261121/run.py
/Volumes/SSD990/cryptanalysis/sage --runtime-info
/Volumes/SSD990/cryptanalysis/sage -python "$(pwd)/research/exotic_ecdlp_20261021/xedni_height_coverage_profile_20261121/sage_replay.py"
sh research/exotic_ecdlp_20261021/xedni_height_coverage_profile_20261121/formal/build.sh
```

The next construction test is target-conditioned rational-point search on the 16 excluded public points, charging the full search and verification work for each target. This profile specifies a height threshold that such a method must cross on this curve.
