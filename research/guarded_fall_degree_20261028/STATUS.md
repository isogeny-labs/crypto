# Requirement and proof status

| Requested item | Status | Evidence or remaining gap |
|:--|:--|:--|
| Put the established proof into `isogeny-labs/crypto` | Committed and pushed; ready PR opened with auto-merge enabled | [PR #2795](https://github.com/isogeny-labs/crypto/pull/2795); live CI/review/merge state is tracked there |
| Preserve executable verification | Implemented natively and tested | `native/`, compact input/DAG files, native validation receipts |
| Extend to larger prime curves | Implemented and checked on four public standard-curve fixtures | 256-, 384-, and 521-bit exact arithmetic; [large-curve receipts](extensions/large_curves/README.md) |
| Larger binary curves | Theorems apply for arbitrary `n`; numerical replay not extended here | SEC2 odd binary sizes inherit the conditional `(4,5)` statement; the separate Boolean upper bound is independent of `n` |
| Extend to growing factor bases | Derived for the explicit degree-`B` membership model | `extensions/growing_b/proof.txt`, `binary_bridge.txt` |
| Extend to characteristic two | Derived for ordinary binary coefficient-field systems | `extensions/characteristic_two/extension_proof.txt` and independent audits |
| Formalization attempt | Partial, checked | `formal/PROOF_MAP.json`; twenty algebraic dependency lemmas, no complete degree theorem |
| Fully Boolean binary upper bound | Derived for the specified one-auxiliary two-S3 chain | `d_F<=3*ell+1`, independent of `n`, with explicit bit equations; `extensions/characteristic_two/boolean_upper_bound.txt` |
| Prime-field bit-restricted upper bound | Derived for the stated `F_p`-coefficient lookup presentation | Normalized inputs: `d_F<=3*ell+2`; raw inputs: `d_F<=max(3*ell+2,d_raw)`; rational-lift and auxiliary-encoding hypotheses are essential |
| Growing Boolean lower bounds and exact degrees | Unresolved | Cardinality lower bounds do not transfer; applying the direct upper theorem to E12 still requires an encoding audit |
| New scientific experiment | Not run | None needed for the algebraic extension; future sweeps use the repository EKS policy |
| Performance or security improvement | Not claimed | No complete-DLP, speed, or asymptotic security measurement |

The characteristic-two classification uses true `GF(2^n)` coefficients and
ordinary polynomial degree. The full closure theorem, triple-selector lifting,
finite-grid inversion, Boolean assignment-localization bound, and curve/sign
interpretation are not Lean theorems in this packet. Passing the identity checks must not be described as formal
verification of those missing bridges.
