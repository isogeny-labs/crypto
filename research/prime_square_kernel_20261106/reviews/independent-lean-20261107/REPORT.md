# Independent technical review: square-kernel Lean normalizer

**Outcome: validated_scoped for the 24 named pointwise algebraic statements.** This review binds crypto main `a2be660914e8e485eefd45f2958eb6d18344dadf`, the two source hashes in [the frozen plan](REVIEW_PLAN.md), and the pinned Lean 4.35.0 / CryptoLean `148f5dc` / Mathlib `cfc55fc` environment. It is a technical proof review, not an experiment decision, a PCC admission, or a claim about a native attack.

## Independent review result

| Claim group | Mathematical / source-fidelity evidence | Fresh machine evidence | Judgment |
| --- | --- | --- | --- |
| All-ones binomial row, factorial weights and omitted-root products | Blind algebra derivation; source reviewer checked valid-index hypotheses and native puncture formula. `choose(2,1)=2` defeats an unguarded row generalization. | Both modules build; 26 declarations pass axiom audit; fresh kernel replay succeeds. | Validated at the exact valid-index scope. |
| Shifted-grid ratio `rho` has 2-adic valuation zero | Independent product derivation and corrected `a=2` controls; native `d=k-2^a` matches Lean's shifted index. `a=0,k=2` is an out-of-range counterexample. | Same pinned build, audit, replay. | Validated for `k<2^(a+1)`. |
| Normalized square preserves source-node values and origin identity | Both reviewers checked center/noncenter nodes and natural-index bounds. At `a=0,k=0,t=1`, original is 1 and square is -1, so the result is nodewise rather than polynomial equality. | Same pinned build, audit, replay. | Validated at source nodes and at `t=0` for the matrix identity. |
| 2-adic valuation doubles after normalized squaring | Independent algebra uses `v₂(rho)=0`; source reviewer checked nonzero kernel and valid-index hypotheses. Zero and invalid-index controls are excluded as specified. | Same pinned build, audit, replay. | Validated pointwise where `baseKernel≠0`. |

The blind reviewer read only the supplied definitions and targets; the source-fidelity reviewer read producer source and native `check.cpp` but not the blind report. Their separate attestations and original reports remain in [blind/](blind/) and [source/](source/). The blind report's `a=2` table initially had three sign errors; [corrigendum](blind/corrigendum.md) preserves that history and gives the independently recomputed sequence `[35,-5,5/3,-1,1,-5/3,5,-35]`. The signs do not alter the unit-valuation or node verdict. The coordinator compared both reports after freezing and checked the receipt fields; no sibling-read claim was contradicted by the recorded inputs.

## Machine verification and custody

The isolated tree was built from the frozen source and manifest; its two Lean source hashes remained `b30198a97a6564b81b68cb373999e9d49922e1e3a087455a09ae787dc150019a` and `f14e3da98cb3270fb7e4b82de8d5ebe9b73584a092ccf936444047cf6e6557f9` before and after verification. CryptoLean HEAD is the pinned `148f5dc389b341401e06d76abe7783c4ccf42f2e`, clean. `lake build CauchyNormalizer KernelNodes` completed 3142 jobs with exit 0. The existing [axiom audit](machine/axiom-audit.log) examined 26 declarations, with zero failures and zero native-trusted declarations; each used only `propext`, `Classical.choice`, and `Quot.sound`. A source scan found no `sorry`, custom `axiom`, `unsafe`, or `native_decide` in the two reviewed files.

The first fresh `leanchecker --fresh --verbose KernelNodes` attempt hit its 180-second limit without a verdict; that empty log and [attempt receipt](machine/attempt.json) remain intact. The one bounded 600-second retry finished at 18:26:07 UTC with exit 0 and the [fresh-recheck receipt](machine/fresh-recheck.json). The timeout is an infrastructure outcome for the first attempt, not a theorem failure. No EKS job or new experiment was submitted for this review.

## Fidelity boundary and next proof work

Formula-level comparison maps native `l=2^a`, `d=k-l`, and `s=j-l` to Lean's shifted puncture and base kernel; matrix indices `k=y+l-x` stay valid for `x,y<l`. This is a correspondence of definitions and indices. A compiled-code refinement theorem, including GMP operations, integer conversion, sentinel valuation, and serialization, remains open. Coefficientwise chart integrality/content, Newton weights, mode counts, degree and recovery cost, recursion, and any end-to-end ECDLP claim also remain open. The next formal step is a Polynomial-typed coefficient and degree bridge with explicit native input/output correspondence, before using these lemmas in a performance claim.

[The diagram](scope.svg) maps the proved dependencies to those open bridges. No canonical quantitative graph was changed: this review introduces no new measured values or official experiment decision. The companion [PDF](report.pdf) contains the same scope map and review verdict. Sources checked were the committed `PROOF_MAP.md`, the exact Lean modules, native `check.cpp`, and the frozen prior review plan; no novelty claim is made.
