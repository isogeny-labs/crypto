# H111: first-layer p² halving carry

## Result

The typed carry exists and all 9,096 lift/kernel/formal/torsion checks passed,
but none of the 24 frozen public predicates was exact on any of 28 rows.
Best canonical accuracy was 0.526427–0.657658 (median 0.561422), overlapping
random-section and symmetry-matched-label controls.  No decoder ran.

Stable scientific digest:

```
383eb1faf53a4fe17088780cb71528df3f20913abdc022c4348f411c26ce4fd5
```

## Formal build

The actual Lean source is `H111P2HalvingCarry.lean` in this directory; the
`formal/` path in the frozen result report is a path error. The adjacent Lake
project pins Lean v4.34.1 and Mathlib at
`d13f23b723b8a846827a245b89c10fc7d3f11612`. From the repository root,
run `tools/lean/proof-gate.sh research/p2_halving_carry_20261101` to build the
source and audit its transitive theorem axioms. The targeted pull request
workflow runs that same command and retains its receipt.

| Lean theorem | Exact condition used | What it establishes |
| --- | --- | --- |
| `carry_mem_reduction_kernel` | The reduction is additive, the section reduces to its input, and twice the chosen half equals the target. | Reduction of the carry is zero. |
| `homomorphic_section_carry_zero` | The section is additive and twice the half equals the target. | Its carry is zero. |
| `exact_predicate_supplies_parity` | The predicate is already exact for every point. | The queried bit equals scalar parity. |
| `reconstruct_eq` | The oracle returns integer parity for every nonnegative input. | Recursive reconstruction returns that integer. |
| `verified_scalar_unique` | Both scalar images equal the target and the scalar-action map is injective on the admitted domain. | The two scalars are equal. |
| `totalCost_bound` | The full seven-term cost sum is at most the stated budget. | The defined total cost is at most that budget. |

The Lean file does not prove that a frozen H111 predicate is exact, that an EC
scalar-action map is injective without restricting representatives, or a
numerical budget bound. Those remain separate empirical or application
premises.

## Reproduce

```bash
python3 p2_halving_carry_lab.py --output p2-halving-carry.json
python3 h111_reference_verify.py p2-halving-carry.json
```

The canonical runner uses the bundled H110 first-layer arithmetic module.  The
reference verifier does not import the H111 runner and independently recomputes
every carry, control word, predicate score, and digest.  The Lean artifact
proves the carry/kernel, torsion-zero, conditional reconstruction, EC
verification, and cost interfaces; it does not assume the empirically rejected
exact-predicate premise.

This is a scoped null for the preregistered 24-predicate family.  External
novelty and a generic subexponential claim are not established.
