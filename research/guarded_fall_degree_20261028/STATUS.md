# Requirement and proof status

| Requested item | Status | Evidence or remaining gap |
|:--|:--|:--|
| Put the established proof into `isogeny-labs/crypto` | Prepared on the feature branch; publication tracked by its PR | This directory, commit, and PR |
| Preserve executable verification | Implemented natively and tested | `native/`, compact input/DAG files, native validation receipts |
| Extend to growing factor bases | Derived for the explicit degree-`B` membership model | `extensions/growing_b/proof.txt`, `binary_bridge.txt` |
| Extend to characteristic two | Derived for ordinary binary coefficient-field systems | `extensions/characteristic_two/extension_proof.txt` and independent audits |
| Formalization attempt | Partial, checked | `formal/PROOF_MAP.json`; algebraic identities only |
| Transfer to growing Boolean Weil descent | Unresolved | Need a degree-preserving encoding/derivation transfer; cardinality bounds do not supply it |
| New scientific experiment | Not run | None needed for the algebraic extension; future sweeps use the repository EKS policy |
| Performance or security improvement | Not claimed | No complete-DLP, speed, or asymptotic security measurement |

The characteristic-two classification uses true `GF(2^n)` coefficients and
ordinary polynomial degree. The full closure theorem, triple-selector lifting,
finite-grid inversion, and curve/sign interpretation are not Lean theorems in
this packet. Passing the identity checks must not be described as formal
verification of those missing bridges.
