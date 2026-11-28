# Toy DLP reduction: two-arm end-to-end replay (2026-11-11)

**PCC standing:** [pcc:unverified]. This is an independent tiny-field functional check, **not** the CT05 full-cost, single-target optimized rho benchmark and **not** an end-to-end genus-2 Jacobian implementation.

Inputs are the exact committed JSON vectors `sextic_twist_3isogeny_vectors.json` from this branch, F101, source `E_2:y²=x³+2`, target `E_4:y²=x³+4`, P=(28,21) of order 17, and direct elliptic reduction `T=η∘[4]`, with T(P)=(78,82).

A fresh JavaScript V8 implementation with Euclidean modular inverse, elliptic additions, scalar multiplication and baby-step/giant-step (m=ceil(sqrt(17))=5) independently recovered every unknown log **k=0..16** from the source Q=[k]P and from the transported point T(Q)=[k]T(P). Each recovered scalar was checked by scalar re-multiplication.

| Arm (17 individual targets, no cross-target state shared) | Recovered targets | Counted group-add function calls |
|---|---:|---:|
| Direct source BSGS | 17/17 | 278 |
| Transported target BSGS | 17/17 | 278 |

The same BSGS procedure is instantiated fresh per target in both arms. Counted calls include identity and scalar-verification operations; they **do not** price field multiplications or inversions, curve conversion, full Jacobian f*/f_*/g* composition, construction, or any relation/linear-algebra phases. The equal arithmetic call count is unsurprising because the prime-order subgroup is preserved by isogeny. No timing, resource-envelope, statistical uncertainty, rho implementation, or cryptanalytic gain is inferred. This is an explicit valid reduction but not a cheaper one.

**Next admitted test:** single-target exact-head Rust native Jacobian path vs direct `η∘[4]` shortcut vs matched optimized rho, including five online IC phases when applicable, all receipts and checkpoint/budget gates. The current repo's CT05 checkpoint and live-budget blockers still apply.
