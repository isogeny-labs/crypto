# Lean proof of the leaf-sum solution correspondence

The Boolean-vector substitution `(u,v) ↦ (u,u xor v)` is an involution at every dimension. It gives an exact equivalence between roots of any original equation predicate `P(u,v)` and roots of the transformed predicate `P(u,u xor s)`, with equal finite root counts. This formalizes the inverse-map and root-set check used in the [five-residual](../g2_leaf_sum_fresh_panel_20261120/README.md) and [witness-backed](../g2_leaf_sum_witness_panel_20261120/README.md) F4 panels.

| Mathematical result | Lean declaration | Exact scope | Status |
| --- | --- | --- | --- |
| Substitution and inverse coincide | `G2LeafSum.change_involutive` | All `n` and all pairs of `Fin n → Bool` vectors | Checked |
| Root bijection | `G2LeafSum.solutionEquiv` | Any predicate `P` on such pairs; transformed predicate is `P ∘ change` | Checked |
| Equal root counts | `G2LeafSum.solution_card_eq` | Decidable original and transformed predicates on the finite Boolean-pair space | Checked |
| Polynomial-ideal equality under the exact serialized S3 row transformation | — | Explicit multivariate polynomial ring and the invertible equation-row operation in the experiment builder | Open bridge; source-level symbolic involution and exhaustive root replay are recorded in the panels |

The readable proof is direct: XOR is self-inverse in the second coordinate, so applying the substitution twice returns both original vectors. Restrict that equivalence to points satisfying `P`. A bijection of finite subtypes gives equal cardinality. This establishes solution preservation independently of the F4 solver's particular basis representation; it does not assert equal elimination work, which is measured in the panels.

[LeafSum.lean](LeafSum.lean) contains the full theorem statements and proofs. [build.sh](build.sh) checks the source SHA-256, pinned toolchain and Lake manifest against the local Mathlib project, and the exact Mathlib commit `021c46bdd042f67b538b9f9871d556a93713a9a9` before compiling. The checked Lean toolchain is `leanprover/lean4:v4.35.0-rc4`. Run from the worktree root:

```sh
sh research/exotic_ecdlp_20261021/g2_leaf_sum_formal_20261120/build.sh
```

The command completed with exit code zero. [build-output.txt](build-output.txt) retains `#print axioms` results: `change_involutive` and `solutionEquiv` use `Quot.sound`; `solution_card_eq` uses `propext`, `Classical.choice`, and `Quot.sound`. The file contains no placeholders or custom axioms. `lean4checker --fresh` is unavailable on this host.
