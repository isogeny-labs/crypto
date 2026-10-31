# Requirement and proof status

| Requested item | Status | Evidence or remaining gap |
|:--|:--|:--|
| Put the established proof into `isogeny-labs/crypto` | Merged | [PR #2795](https://github.com/isogeny-labs/crypto/pull/2795), merge `cca7d4fe08d49f6e63b8e8f37883eb1d181eb194` |
| Preserve executable verification | Implemented natively and tested | `native/`, compact input/DAG files, native validation receipts |
| Extend to larger prime curves | Implemented and checked on four public standard-curve fixtures | 256-, 384-, and 521-bit exact arithmetic; [large-curve receipts](extensions/large_curves/README.md) |
| Larger binary curves | Theorems apply for arbitrary `n`; numerical replay not extended here | SEC2 odd binary sizes inherit the conditional `(4,5)` statement; the separate Boolean upper bound is independent of `n` |
| Extend to growing factor bases | Derived for the explicit degree-`B` membership model | `extensions/growing_b/proof.txt`, `binary_bridge.txt` |
| Extend to characteristic two | Derived for ordinary binary coefficient-field systems | `extensions/characteristic_two/extension_proof.txt` and independent audits |
| Prove exact pair `(4,5)` in Lean | Proved for the original prime-field model | [Complete proof](formal/complete-exact-pair.md): `GuardedExactPair.exact_pair_prime`; actual generator degrees/tops, graded Koszul first fall, constructed degree-five refutation and rationality/unit-ideal bridge |
| Fully Boolean binary upper bound | Derived for the specified one-auxiliary two-S3 chain | `d_F<=3*ell+1`, independent of `n`, with explicit bit equations; `extensions/characteristic_two/boolean_upper_bound.txt` |
| Prime-field bit-restricted upper bound | Derived for the stated `F_p`-coefficient lookup presentation | Normalized inputs: `d_F<=3*ell+2`; raw inputs: `d_F<=max(3*ell+2,d_raw)`; rational-lift and auxiliary-encoding hypotheses are essential |
| Growing Boolean lower bounds and exact degrees | Unresolved | Cardinality lower bounds do not transfer; applying the direct upper theorem to E12 still requires an encoding audit |
| New scientific experiment | Not run | None needed for the algebraic extension; future sweeps use the repository EKS policy |
| Performance or security improvement | Not claimed | No complete-DLP, speed, or asymptotic security measurement |

The characteristic-two classification uses true `GF(2^n)` coefficients and
ordinary polynomial degree. The original odd-characteristic prime-field
three-abscissa exact pair is now proved in Lean, including its degree-five
construction and scalar-grid inverse. Triple-selector lifting, the binary
classification, Boolean assignment-localization bounds, numerical large-curve
witnesses and group-law/sign interpretation remain outside this formal claim.
The central theorem assumes polynomial UNSAT, exactly as its source states.
