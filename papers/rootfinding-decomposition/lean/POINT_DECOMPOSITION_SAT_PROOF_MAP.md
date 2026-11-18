# Point-decomposition SAT proof map

Status: Lean 4.34.1 and Mathlib v4.34.1 checks passed locally,
`[pcc:unverified]`.

This map applies the repository's `lean-instance-gate` before the first Lean
edit. The formalization is the finite semantic one-hot CNF schema used by a
point-decomposition problem. It does not formalize or verify the production
XOR/Tseitin compiler, a CDCL implementation, or any real-curve attack path.

## Sources checked

- `S3Roots.lean`: `SemaevS3.S3_root_iff` supplies the Semaev `S₃`
  root equivalence, and `SemaevS3.S3_factor_mathlib` connects it to Mathlib's
  affine group law.
- `../rootfinding_decomposition.tex`, Proposition 1 and Algorithm 1: the
  direct-subtraction argument requires `xR ≠ xF`; the `xR = xF` case is a
  separate direct-lookup branch.
- `../../../research/sat_factor_base_review_20260908/MATHEMATICS.md`: finite
  domain constraints and rational-lift verification are separate obligations;
  SAT observations do not establish a complexity theorem.
- The shared research KB was unavailable: `scripts/research-kb status`
  exited 127 because no executable was configured. The review therefore used
  the repository sources above.

## Instance gate

1. **Structure above.** Mathlib supplies `Fintype`, `Fin`, `DecidableEq`,
   `AddCommMonoid`, finite sums and decidable equality. The new generic
   `Problem Choice` stores an arity and a Boolean validity checker. Its SAT
   semantics use one Boolean variable for each slot/candidate pair.
2. **Theorem used as stated.** No matching finite one-hot blocking-clause
   theorem exists in this project. The missing generic law is
   `PointDecompositionSAT.satisfiable_iff_valid`, stated over `Problem` rather
   than over a curve. The decomposition endpoint applies that theorem as
   stated.
3. **Object as an instance.** An additive decomposition problem maps each
   finite candidate to an element of an additive commutative monoid. Signed
   factor-base elements can be candidates themselves. The checker decides
   whether the finite sum equals the target.
4. **Assembly probe.** For arbitrary `base`, `target` and `arity`, the endpoint
   is
   ```lean
   Satisfiable (decompositionProblem arity base target) ↔
     ∃ xs, (∑ i, base (xs i)) = target
   ```
   and its proof is one application of `satisfiable_iff_valid` followed by
   simplification of the decidable equality checker.
5. **Missing components only.** The new components are the generic problem,
   assignment and clause-family semantics; the canonical one-hot assignment;
   generic soundness, completeness and satisfiability equivalence; and the
   additive endpoint. No field, curve or summation-polynomial law is
   re-derived.
6. **Inheritance table.** The table below records every inherited operation.

| Operation | Inherited from | Assembly |
|---|---|---|
| finite slots | `Fin problem.arity` | existing `Fintype` instance |
| finite candidates | `[Fintype Choice]` | existing instance |
| Boolean equality variable | `[DecidableEq Choice]` | `decide (p = xs i)` |
| positive clause per slot | `AtLeastOne` | existential semantic clause |
| pairwise negative clauses | `AtMostOne` | uniqueness implication |
| invalid-tuple blocking clause | `RejectsInvalid` | one false selected literal |
| point composition | `[AddCommMonoid G]` | `Finset.univ.sum` |
| target equality checker | `[DecidableEq G]` | `decide (sum = target)` |
| `S₃` roots/group-law bridge | `SemaevS3.S3_root_iff`, `S3_factor_mathlib` | separate existing algebra layer; not imported or composed here |

## Statement table

| Declaration | Kind | Quantified structure | Hypotheses |
|---|---|---|---|
| `sound` | generic law | `Problem Choice` | finite candidates, decidable equality, satisfying assignment |
| `complete` | generic law | `Problem Choice` | finite candidates, decidable equality, valid tuple |
| `satisfiable_iff_valid` | generic law | `Problem Choice` | finite candidates, decidable equality |
| `decomposition_satisfiable_iff` | endpoint | additive decomposition problem | finite candidates, `AddCommMonoid`, decidable equality |

## Proof chain

Soundness chooses one selected candidate in every slot from the positive
clauses. Pairwise clauses make each choice unique. If the decoded tuple were
invalid, its blocking clause would require one of those selected variables to
be false, contradicting the chosen true variable.

Completeness assigns true exactly to the variables named by a valid tuple.
The positive and pairwise clauses hold. Any invalid tuple differs from the
valid tuple at some slot, so that slot supplies the false literal required by
its blocking clause. The satisfiability equivalence combines these two
directions.

The generic clause theorem is an upstream candidate for a shared formal SAT
library if a second project needs it. This paper-local addition is its first
consumer and introduces no heartbeat override or composite unfolding.

## Verification

- `lake build PointDecompositionSAT` passed.
- `lake env lean AxiomAudit.lean` accepted all four new declarations with
  only `propext`, `Classical.choice` and `Quot.sound`.
- `lake env leanchecker --fresh --verbose PointDecompositionSAT` passed.
- `tools/lean/proof-gate.sh papers/rootfinding-decomposition/lean` passed,
  auditing 18 theorems with zero trusted-native exceptions.
