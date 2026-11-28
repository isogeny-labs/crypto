# Actual cubic-map formal proof map

Full written statement: ACTUAL_CUBIC_MAP_PROOF_E.md. Native inputs: source91db340e and unsigned PCC-20261127-758cbf. No new Lean theorem has been checked in this continuation. The following gate is a design/provenance record, not a replacement proof. [pcc:unverified]

## Six instance-gate fields

1. Structure above: Mathlib Polynomial (ZMod 7), its existing commutative-ring instance, and Polynomial.eval₂RingHom into a field carrying the coefficient homomorphism. Use the library's field/ring/polynomial operations; no syntax, finite cyclic group or new algebra wrapper is the primary object.
2. Existing laws used as stated: Polynomial.eval₂RingHom, map_add, map_mul, map_pow, map_natCast, the polynomial derivative laws and the existing field division identities. Exact pinned source also supplies Polynomial.evalRingHom, Polynomial.eval_mul, Polynomial.eval_pow. Kernel normalization would discharge only the concrete coefficient equalities, followed by the existing evaluation homomorphism.
3. Actual instance: H,F,G,N,R are exactly the five F7 polynomials in the native/written certificate. The intended affine endpoint has hypotheses Y^2=eval(H) and eval(G) nonzero and conclusion that (eval(N)/eval(G),Y eval(R)/eval(G)^2) satisfies the original elliptic equation. It quantifies over coefficient-field extensions and excludes precisely the pole chart; proper extension remains a separate geometric obligation.
4. Assembly probe: planned application of the evaluation ring homomorphism to H=FG and N^3+2NG^2+6G^3=FR^2, followed by the existing field laws. Type-checking is blocked before loading a theorem by the bounded runtime/cache failure below. No study Lean file has been authored before this gate completes.
5. Missing components: cached pinned Mathlib environment and a working Lean startup under the retained bound; then the concrete polynomial equalities. Full proper genus-two curves, degree/separability, Picard push/pull and the actual Prym/chain interpretation have no instantiated formal objects in this study. They must be supplied generically in their owning library, without assuming the desired identity or substituting an unrelated group.
6. Inheritance: coefficient arithmetic from ZMod's instances; polynomial ring from Mathlib's instances; multiplication/power transport from eval₂RingHom and named hom laws; derivative from Mathlib's existing derivative. All are intended applications, not new laws. The algebraic geometry rows remain open.

## Dependencies and fidelity

Shared crypto-lean pin: aburan28/crypto-lean@1b0d7e3830e54f9a26ff3895f44143431cb05f57; Lean4.35.0; Mathlib cfc55fc58714a59dc06e6e79edc6d1d95140d3c2, exactly from that library's manifest. The Mathlib checkout is pristine and uses the official master cache only. No pin, backend, trust manifest or proof limit was changed.

| Report obligation | Formal object / inherited law | Current receipt / gap |
| --- | --- | --- |
| Full H=FG and elliptic polynomial identity | Polynomial(ZMod7), ring equality | Proposed instance obligations; no accepted theorem yet. |
| Affine rational-map equation over coefficient extensions | eval₂RingHom plus existing field laws | Assembly probe blocked by startup; no conclusion assumed. |
| Squarefree model / coprimality | Actual Bezout equalities plus existing polynomial algebra | Native exact witnesses; formal component pending. |
| Proper degree-three cover and ramification | Actual smooth proper curve/function field | Definitions and generic geometric bridge open. |
| H3^*=Gamma, retractions and norm3 | Actual Jacobians/Picard functor, duality and torsion-free algebraic Hom | Written proof retained; full faithful formalization open. |

The intended dependency order is the existing polynomial/evaluation structures, their actual coefficient obligations, then the affine endpoint. The missing geometric infrastructure is upstream work, not a collection of study-local group axioms. There are no new generic laws proposed for the polynomial layer.

## Actual bounded infrastructure attempts

The selected environment has Lean4.35.0 but no current crypto-lean/Mathlib package in the checked paths. Read the exact shared manifest, official Mathlib cache instructions and Lean runtime source, then fetched the pinned Mathlib source in owned RAM. A targeted official lake exe cache get failed with exit134 before any theorem. Its unchanged2GiB process bound and120second cap were retained. A second attempt set documented8MiB thread stacks and disabled the extra main thread; it failed identically. A focused taskset/strace control shows a1073745920-byte MAP_STACK reservation failing ENOMEM. Running the exact official LakeMain source through the pinned Lean interpreter with explicit one-thread/8MiB flags also failed before theorem checking. All whole diagnostics remain retained. [pcc:unverified]

This is a runtime/dependency blocker, not a failed mathematical assertion. No cache fallback, unsafe cache scope, native_decide, placeholder group, custom axiom, source suppression or increased cap was used. The original PR38 import failures and zero accepted generic theorems remain separate historical records. Once startup works within the retained authority, finish the assembly probe, faithful coefficient/affine theorems, #print axioms, canonical proof gate and lean4checker --fresh where available. Keep the full geometric theorem open until its actual objects and bridge are built.
