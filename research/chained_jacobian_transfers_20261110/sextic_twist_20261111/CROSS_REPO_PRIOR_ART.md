# Cross-repo prior art and actionable completeness gap (2026-11-11)

This note updates the CT09 follow-up in draft PR #3909 and avoids reimplementing already known quotient/isogeny ideas. **No new algorithmic novelty or DLP gain is asserted.** [pcc:unverified]

## Existing work found on default branches

- `isogeny-labs/crypto-autoresearcher/docs/reports/ecdlp-model-comparison-20261110.md` explicitly records **sextic genus-2 cover** `y²=x⁶+b` as previously explored transfer L3, including the second involution `(x,y)↦(1/x²,y/x³)` (suitably scaled to the second twist). The family itself must not be reported as a new idea.
- `isogeny-labs/crypto/src/cryptanalysis/j0_twists.rs` already enumerates sextic twist classes, finite-field orders, and CM trace values; `secp256k1_cm_audit/howe_sextic_twists_check.py` already uses unequal traces to exclude base-field isogenies between sextic twists. Reuse those checks rather than adding a second twist-census framework.
- `isogeny-labs/cryptanalysis/suite/src/isogeny/velu.rs` already implements pointwise rational Vélu kernels, plus `suite/examples/exotic_charts.rs` explores a 3-isogeny quotient chart. **However, the generic odd-isogeny search omits Frobenius-stable kernels without Fp-rational nonzero kernel points.**

## Exact counterexample (independent local smoke check)

`E: y²=x³+3` over F103 has **124** Fp points, so no rational point of order 3. But the geometric subgroup `{O,(0,√3),(0,-√3)}` is Frobenius-stable: its nonidentity points are swapped by Frobenius, because 3 is nonsquare. The standard degree-3 rational map

`h(x,y)=((x³+12)/x², y*(x³-24)/x³)`

lands on `E':Y²=X³-81 ≡ X³+22 (mod103)`. Both have 124 Fp points. A standalone Python arithmetic checker exhaustively verified the equation and additivity on **124×124** ordered elliptic point pairs, including infinity. This is a concrete Fp-defined 3-isogeny even when `3 ∤ #E(F103)`; **do not** use the missing rational 3-torsion as a no-isogeny certificate.

Tracking issue: https://github.com/isogeny-labs/cryptanalysis/issues/1470 . The existing `velu_isogeny_odd` should be described as enumerating pointwise rational kernels only until it implements Frobenius-stable kernel polynomials. Since Conductor is not available and native Rust toolchain is not installed in this invocation, this round files the mathematically checked counterexample rather than changing active `suite/src/` code without ownership or compilation.

## Next discriminating work

1. Add typed explicit `η` and kernel-polynomial support to the native Rust CT09 backend, without confusing `Fp`-defined isogenies with those admitting rational kernel generators. Integrate the positive and negative vectors from this PR.
2. Reuse cryptanalysis's existing Vélu and crypto's j0 twist infrastructure, adjusting only the incomplete enumeration boundary with a test for `(103,0,3,3)`.
3. In CT05 compare native Jacobian path against the **collapsed direct degree-3 isogeny**, not just against a direct rho walk. The whole chain equals `η∘[4]` on the relevant subgroup, so a gain requires an intermediate algorithm unavailable through direct η.
4. Obtain independent signed PCC evidence and root CI on exact heads after parent PR coordination. No EKS/S3 experimental spend until the repository's admission gate and budget visibility pass.
