# Vitse / Prime31: Lean inheritance and feasibility review

Date: 2026-11-23. Parent: crypto PR #5016. This is a source/inheritance audit for the correctness guard; it contains no new Lean declaration, build receipt, or formal verification claim. [pcc:unverified]

## Gate and pinned dependencies

Read the complete [`lean-instance-gate`](https://github.com/isogeny-labs/workspace/blob/main/.agents/skills/lean-instance-gate/SKILL.md) before any Lean edit. No Lean file was edited.

Shared library inspected: [`aburan28/crypto-lean`](https://github.com/aburan28/crypto-lean/tree/ff61caed5cd414cf133e110bdf5740356de1dd54), revision `ff61caed5cd414cf133e110bdf5740356de1dd54`.

- `lean-toolchain`: `leanprover/lean4:v4.35.0`.
- `lake-manifest.json`: Mathlib revision `cfc55fc58714a59dc06e6e79edc6d1d95140d3c2`.
- Tree query was complete (`truncated:false`). Targeted code searches: `semaev symmetry`, `Equiv.sum_comp`, and `Semaev`.
- Complete primary modules read: `CryptoLean/Semaev/S3.lean`, `CryptoLean/Group/Torsion.lean`, and `CryptoLean/Group/ShiftRepresentations.lean`.
- `command -v lean` and `command -v lake` produced no path in the available runtime. Consequently no `lake env lean` assembly probe, `lake build`, `#print axioms`, or proof-gate run was performed.

The earlier LFD thread's Lean 4.19 constraint is not silently substituted for this library's currently inspected 4.35 pin; this review reports the actual source dependency only.

## Six instance-gate fields

| Field | Fixed-target sum contract | S3 coordinate-sign distinction |
|---|---|---|
| Structure above | A finite index type `I`, `AddCommGroup G`, a permutation of `I`, and an additive homomorphism `A : G ->+ G`. | The existing `CryptoLean.Semaev.S3` over `[Field K]`; do not define another S3. |
| Existing laws used as stated | `Equiv.sum_comp` reindexes a finite sum; `Finset.sum_add_distrib` separates translations; additive-hom `map_sum` moves `A` through the sum. | `CryptoLean.Semaev.S3_factor` and `S3_root_iff` establish the existing off-diagonal curve-root interpretation. They do not state the new note's sign-difference identity. |
| Object as instance | Intended actual object is the elliptic point group; point negation is a common additive homomorphism, and fixed translations are added afterward. | Instantiate `K` with the existing finite field and `a,b` with the pinned curve coefficients. The current Rust coefficient normalization agrees textually with the inherited definition. |
| Assembly probe | Not run: no Lean/Lake executable or materialized pinned dependency build. | Not run for the same reason. |
| Missing components only | Source-bound link from the current `PrimePoint` representation and group operations to the actual Mathlib point group; selector and finite-chart compatibility for the particular action. | If needed as a named reusable theorem, a generic two-coordinate-negation difference law belongs in `CryptoLean/Semaev/S3.lean`; then a checked bridge between Rust evaluations and that field expression. |
| Inheritance | Generic commutative-group finite-sum laws; no new curve-specific group law. | Existing S3 definition and root theorems; no copied Semaev definition or new curve law. |

## Faithful proof chain for the target condition

The more general existing-structure formulation avoids a new sign type. For an additive homomorphism `A`, permutation `pi`, points `P_i`, and translations `T_i`:

\[
\sum_i(A(P_{\pi(i)})+T_i)
=\sum_i A(P_{\pi(i)})+\sum_iT_i
=A\left(\sum_iP_{\pi(i)}\right)+\sum_iT_i
=A\left(\sum_iP_i\right)+\sum_iT_i.
\]

The successive lines use, respectively, finite-sum additivity, the additive-hom sum law, and permutation reindexing. If `sum P_i=R` and `sum T_i=R-A(R)`, the expression equals `R`. Identity and point negation are instances of `A`; per-leaf independent sign choices are not a single `A` and are not covered.

The shared library's receipt `docs/receipts/auxiliary-shift-20261115/moments-api001.log` supplies the exact source-visible signature of `Equiv.sum_comp` over `Fintype` and `AddCommMonoid`. That receipt also contains an unrelated failed API lookup; it is used here to locate the declaration, not represented as a clean build receipt for this task.

## Inheritance and fidelity table

| Claim in the review | Existing declaration / intended owner | Scope or missing bridge | Present verification status |
|---|---|---|---|
| Reindexing all leaves preserves their sum. | Mathlib `Equiv.sum_comp`. | Finite sum only; says nothing about unequal selectors or finite-prefix exclusions. | Existing source/API located; no task-local kernel replay. |
| Balanced translations preserve a fixed target. | Composition of `Finset.sum_add_distrib`, `map_sum`, `Equiv.sum_comp`; any reusable wrapper belongs in crypto-lean. | Actual point-map, membership, and chart hypotheses must be supplied. | Written derivation only. |
| Roots of S3 are x-coordinates of plus/minus sums for distinct input x-coordinates on the curve. | `CryptoLean.Semaev.S3_factor`, `CryptoLean.Semaev.S3_root_iff`. | Requires the two curve equations and `xR != xF`; does not cover diagonal or infinity cases. | Full declaration and proof source inspected; no new build or axiom receipt. |
| `S3(-X,-Y,t)-S3(X,Y,t)=4(X+Y)(t(XY+a)+2b)`. | Generic law, if formalized, in `CryptoLean/Semaev/S3.lean`. | Not found in the complete inspected S3 module. Must inherit the existing definition, not restate it in the study. | Written algebraic derivation; no Lean declaration produced. |
| Finite Mobius containment without poles implies a bijection. | Existing finite-set/injective-map facts, with projective-map injectivity. | Current Rust constructor and evaluator semantics need a bridge; not proved by naming PGL2. | Code/mathematical source review only. |
| The current guard allows safe quotienting of the complete signed solver. | None claimed. | Deliberately stronger than the finite-coordinate API; target, curve action, and chart obligations remain separate. | Not an asserted result. |

## Bounded decision

Do not add uncompiled Lean for appearance, duplicate S3, introduce an elliptic-group abstraction, or claim inherited source inspection as a new proof. The current deliverable is an exact finite-domain correctness guard plus a source and applicability audit. A future formal refinement should first import the pinned library, type-check the finite-sum assembly probe, and bind the guard's concrete coordinate/map semantics. If the S3 sign-difference law is needed, add it generically to the shared owner and retain its proof-gate and axiom receipts before making a formal claim. The missing runtime is a concrete build limitation; the mathematical dependencies and remaining implementation bridges are recorded above. [pcc:unverified]
