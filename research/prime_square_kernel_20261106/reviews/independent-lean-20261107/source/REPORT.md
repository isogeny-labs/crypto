# Independent source-fidelity review: square-kernel normalizer

Review task T-1829; draft producer PR #3642. Reviewed immutable crypto `main` commit `a2be660914e8e485eefd45f2958eb6d18344dadf`, against the frozen `REVIEW_PLAN.md` dated 2026-11-07 18:02 UTC. This is a **source-fidelity** review, not a fresh Lean build or an independent native execution. I did not read the blind reviewer's report or private directory.

## Verdict by claim

The types below are the committed source statements. The coordinator owns fresh elaborated-type, build, and axiom verification; the producer's saved audit reports 26 audited declarations but is not fresh independent machine evidence. Thus machine status for each row here is **not_verified by this reviewer**, even where the source correspondence is good.

| Declaration(s) | Exact scope and source-fidelity judgment |
| --- | --- |
| `choose_all_ones`, `choose_unit` | For every `a k : ℕ` with `k < 2^a`, the row `choose (2^a-1) k` casts to one in `ZMod 2` and its rational 2-adic valuation is zero. **faithful_scoped** to the all-ones row argument. No statement for arbitrary binomial rows. |
| `weight_ne_zero`, `weight_valuation` | `weight a k = k! (2^a-1-k)!` is nonzero for all natural `a,k`; valuation equality to `(2^a-1)!` requires `k < 2^a`. **faithful_scoped**. For out-of-range `k`, natural subtraction is truncated, but no valuation claim is made. |
| `signedWeight_ne_zero`, `signedWeight_valuation`, `signedWeight_ratio_unit` | The signed factorial product is nonzero for all `a,k`; valuation equality needs a valid index; quotient unit requires both indices valid. **faithful_scoped**. |
| `puncture_zero`, `puncture_shift`, `puncture_formula`, `puncture_eq_signedWeight`, `puncture_normalizer_unit` | `puncture n k` is the rational product over `j=0..n`, with the `j=k` factor replaced by one. The factorial formula requires `k≤n`; the power-of-two valuation result requires both indices below `2^a`. **faithful_scoped** to the omitted-root product. The all-natural shift identity has no hidden valid-index claim. |
| `shiftedPuncture_eq`, `center_lt`, `cauchyRho_ne_zero`, `cauchyRho_unit` | Rational translation cancels factorwise; `cauchyRho a k` divides the translated product on `2^(a+1)` nodes by the center product. Its nonzero and unit claims require `k < 2^(a+1)`. **faithful_scoped** to native `rho(l,d)` for `l=2^a`, `d=k-l`. |
| `normalized_square_valuation` | For any valid `k` and any **nonzero rational** `w`, `v₂(w²/rho)=2v₂(w)`. **faithful_scoped** as an algebraic identity; it does not identify `w` with a native computed value. |
| `baseKernel_node` | For every `a,k,m : ℕ` with `m` valid, `baseKernel(a,k,m-L)` equals `rho(a,k)` if `m=k`, else zero. The theorem does not require `k` valid, but the interesting diagonal case then forces validity through `m=k`. **faithful_scoped** to the native omitted-root kernel's source-node values. |
| `squareKernel_node_preserved` | For every valid `k,m`, the normalized square equals the original kernel at source node `m-L`. **faithful_scoped** to the node claim, not polynomial equality. |
| `squareKernel_valuation` | For every valid `k` and rational `t` with `baseKernel(a,k,t)≠0`, its square's valuation doubles the original's. **faithful_scoped** pointwise; no coefficientwise or chart-content statement. |
| `cauchyRho_center`, `baseKernel_origin`, `squareKernel_origin` | Center ratio is one; original origin is Kronecker for **all** natural `k`; square origin is Kronecker for valid `k`. **faithful_scoped**. |
| `matrixEntry_origin` | For all natural `x,y < 2^a`, `squareKernel(a,y+2^a-x,0)` is one iff `x=y`. **faithful_scoped** to entrywise identity at `T=0`; it does not describe the evaluated native matrix at other `T`. |

Overall source-fidelity verdict for these **pointwise algebraic declarations**: **holds, conditional on the coordinator's fresh machine check**. The broader experiment/algorithmic claim is **partial** and cannot receive a validated-scoped outcome from these theorems alone. No proof-map row appears to misstate a theorem's index or nonzero precondition. The `PROOF_MAP.md` explicitly lists the remaining gaps.

## Native correspondence and totalized division

Native `check.cpp` computes `derivative(l,d)=∏_{s=-l}^{l-1,s≠d}(d-s)`, `rho(l,d)=derivative(l,d)/derivative(l,0)`, and `original(l,d,t)=∏_{s≠d}(t-s)/derivative(l,0)`. Substitute `l=2^a`, `s=j-l`, `d=k-l`, and `j=0..2l-1`. Lean's `shiftedPuncture(2l-1,k,l)` and `baseKernel(a,k,t)` have exactly these factors for valid `k`. `matrixEntry` uses `k=y+l-x`; with `x,y<l`, natural subtraction is exact, giving `d=y-x` and `0≤k<2l`. The native panel uses that same `d` in `original(l,d,T)` and in the ratio. The panel itself uses only `l=4,8,16,32`, whereas the Lean statements cover `a=0` and `a=1` too.

Lean rational division is totalized, so a nonzero denominator matters. The center shifted puncture is nonzero by `puncture_eq_signedWeight`, `signedWeight_ne_zero`, and `center_lt`; `cauchyRho_ne_zero` proves the further divisor is nonzero for valid `k`. `squareKernel_node_preserved` uses that nonzero condition on its diagonal. The valuation theorems exclude zero `w`/`baseKernel` and invalid `k`; they cannot be applied to the native `val(0)=100000` sentinel as a true 2-adic valuation. The native panel checks integrality before modular conversion, but no Lean theorem here proves that chart property.

## Exact small and adverse controls

I recomputed the products and node evaluations using Python `fractions.Fraction` and integer loops directly from the definitions; this was a local mathematical control, not a build or experiment run.

| `a`, `L`, center puncture | `rho(k)` for `k=0..2L-1` | Node and origin check |
| --- | --- | --- |
| `0`, `1`, `1` | `[-1, 1]` | At `k=0`, `B(t)=t`, `S(t)=-t²`; at source nodes `-1,0`, values agree and origin is `0`. Center `k=1` has origin `1`. |
| `1`, `2`, `-2` | `[3, -1, 1, -3]` | For `k=0,2,3` and all sampled source nodes `m=0,2,3`, `S(m-L)=B(m-L)`; center origin is `1`, other origins `0`. |
| `2`, `4`, `-144` | `[35, -5, 5/3, -1, 1, -5/3, 5, -35]` | For `k=0,4,7` and source nodes `m=0,4,7`, node equality holds; center origin is `1`, other origins `0`. |

The displayed ratios all have numerator and denominator odd in lowest terms, as required by the unit claim. The control `choose(2,1)=2` is even, confirming that the all-ones-row condition is substantive. For `a=0,k=2` outside `k<2`, the product ratio is `2`, hence `v₂(rho)=1`: this is a **mathematical counterexample to an unguarded extension**, and the Lean theorem correctly excludes it. At the same invalid index, `w=1` gives `v₂(w²/rho)=-1`, not `2v₂(w)=0`. Setting a normalizer artificially to zero invalidates division-based node preservation; `w=2` gives a totalized rational quotient zero while the intended doubled nonzero valuation is two. This is a control of the denominator hypothesis, not a counterexample to the proved theorem. At valid `a=0,k=0,t=1`, `B(t)=1` and `S(t)=-1`: node preservation is **not** polynomial equality. Zero kernel values are outside the valuation theorem's hypothesis.

## Unproved bridges and next checks

1. No theorem equates the Lean definitions with compiled GMP calls or checks serialization, integer conversion, loop bounds, and native sentinel behavior. The substitution above establishes formula-level correspondence, not code-to-Lean execution equivalence.
2. The Lean work is pointwise over `ℚ`. It does not prove coefficientwise chart integrality/content, Newton difference weights, uniform Smith or moment mode counts, or extension-ring jet behavior. The finite panel's mixed counts and retrospective protocol/export validation failure cannot be repaired by these lemmas.
3. No Polynomial-typed degree statement or Hermite recovery budget is proved. In particular, the candidate degree `4L-2` and original degree `2L-1` need an explicit cost bridge.
4. No proof connects the pointwise kernel to a recursive tensor algorithm, child-size recurrence, actual implementation correctness, ECDLP reduction, success probability, end-to-end bit cost, or novelty claim.

The coordinator should fresh-build and audit the exact two modules and named declarations, compare elaborated types with this table, and retain the actual input hashes. The producer's archived gate is useful for locating those checks but cannot replace fresh review execution.

## Source binding

The committed source hashes match the frozen plan: `CauchyNormalizer.lean` `b30198a97a6564b81b68cb373999e9d49922e1e3a087455a09ae787dc150019a`; `KernelNodes.lean` `f14e3da98cb3270fb7e4b82de8d5ebe9b73584a092ccf936444047cf6e6557f9`; `PROOF_MAP.md` `09bc97d2c045fc6c84acbd3aea1638d4f5dafc56626e0406f1a95acdcf2ca70a`. `lean-toolchain` pins Lean 4.35.0; `lakefile.toml` and manifest pin CryptoLean `148f5dc389b341401e06d76abe7783c4ccf42f2e` and Mathlib `cfc55fc58714a59dc06e6e79edc6d1d95140d3c2`. I did not validate installed dependency bytes or run Lean.
