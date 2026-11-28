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
| invalid-tuple blocking clause | `RejectsInvalid` | one false tuple variable, hence one true negated literal |
| point composition | `[AddCommMonoid G]` | `Finset.univ.sum` |
| target equality checker | `[DecidableEq G]` | `decide (sum = target)` |
| `S₃` roots/group-law bridge | `SemaevS3.S3_root_iff`, `SemaevS3.S3_factor_mathlib` | separate existing algebra layer; not imported or composed here |

## Direct-subtraction refinement extension (2026-11-26)

This extension is the semantic contract used by the production
`rootfind-subtract` / `rootfind-floor` search: choose one factor-base point,
subtract it from the target, and decompose the remainder. It does not verify
the Rust loop, Bloom filter, IFMA kernel, witness tables, counters, compiler or
solver.

### Instance gate

1. **Structure above.** Mathlib's `AddCommGroup` supplies addition,
   subtraction, negation, associativity and commutativity; `Fin` and
   `Fin.sum_univ_succ` split a tuple indexed by `Fin (remaining + 1)` into its
   head and tail.
2. **Theorem used as stated.** `decomposition_satisfiable_iff` supplies the
   SAT-to-finite-sum equivalence. The refinement endpoint rewrites by that
   theorem as stated and only rearranges the resulting group equation with
   `Fin.sum_univ_succ` and `eq_sub_iff_add_eq`.
3. **Object as an instance.** A direct-subtraction witness is a head candidate
   and a `remaining`-candidate tail whose sum is `target - base head`. The
   corresponding full tuple is assembled with `Fin.cases head tail`.
4. **Assembly probe.** For arbitrary `remaining`, `base` and `target`, the
   endpoint is
   ```lean
   Satisfiable (decompositionProblem (remaining + 1) base target) ↔
     ∃ head tail, (∑ i, base (tail i)) = target - base head
   ```
   after `decomposition_satisfiable_iff`; both directions close by the two
   named Mathlib group/sum laws. The `remaining = 1` corollary simplifies the
   tail sum to the single lookup equation.
5. **Missing components only.** The paper-local predicate
   `DirectSubtractionSpec`, its generic refinement endpoint, and the `m = 2`
   corollary. No field, curve, coordinate, SAT or group law is re-proved.
6. **Inheritance table.** The operations used by the extension are inherited
   below; there is no upstream candidate.

| Operation | Inherited from | Assembly |
|---|---|---|
| split full tuple | `Fin.cases`, `Fin.sum_univ_succ` | head at `0`, tail at `Fin.succ` |
| assemble full tuple | `Fin.cases` | `Fin.cases head tail` |
| move head across equality | `eq_sub_iff_add_eq` | commutative rewrite of head plus tail |
| SAT semantics | `decomposition_satisfiable_iff` | theorem applied as stated |

### Declaration plan and proof chain

Dependencies are `DirectSubtractionSpec` →
`decomposition_satisfiable_iff_direct_subtraction` →
`decomposition_two_satisfiable_iff_direct_lookup`. All declarations remain in
the paper module because they are endpoints for its existing generic SAT law.
The proof chain is: rewrite SAT by `decomposition_satisfiable_iff`; split or
assemble the `Fin (remaining + 1)` tuple; rewrite the finite sum once with
`Fin.sum_univ_succ`; replace the full-sum equality with the subtraction
equality using `eq_sub_iff_add_eq`. The two-summand endpoint then specializes
`remaining` to one and simplifies the unique tail slot.

## Statement table

| Declaration | Kind | Quantified structure | Hypotheses |
|---|---|---|---|
| `sound` | generic law | `Problem Choice` | finite candidates, decidable equality, satisfying assignment |
| `complete` | generic law | `Problem Choice` | finite candidates, decidable equality, valid tuple |
| `satisfiable_iff_valid` | generic law | `Problem Choice` | finite candidates, decidable equality |
| `decomposition_satisfiable_iff` | endpoint | additive decomposition problem | finite candidates, `AddCommMonoid`, decidable equality |
| `DirectSubtractionSpec` | endpoint predicate | `AddCommGroup` decomposition problem | none |
| `decomposition_satisfiable_iff_direct_subtraction` | endpoint | `AddCommGroup` decomposition problem | finite candidates, decidable equality |
| `decomposition_two_satisfiable_iff_direct_lookup` | endpoint | two-summand `AddCommGroup` decomposition problem | finite candidates, decidable equality |

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

- Pinned environment: `leanprover/lean4:v4.34.1`, Mathlib v4.34.1 at
  `d13f23b723b8a846827a245b89c10fc7d3f11612`.
- `lake build S3Roots PointDecompositionSAT` passed locally.
- The workflow's forbidden-construct scan found no `sorry`, `admit`,
  `native_decide` or custom `axiom` in the three audited files.
- `lake env leanchecker --fresh --verbose PointDecompositionSAT` passed locally.
- `lake env lean AxiomAudit.lean` accepted all 13 audited declarations. Both
  new theorems depend only on `propext`, `Classical.choice` and `Quot.sound`.
  The workflow normalizes pretty-printer line wrapping before checking the 13
  retained `#print axioms` records.

## Fidelity table

| Claim as written | Lean theorem | Weaker or different in Lean | Receipt |
|---|---|---|---|
| Splitting off one factor-base candidate preserves the finite decomposition/SAT semantics. | `decomposition_satisfiable_iff_direct_subtraction` | Semantic finite-domain theorem only; it does not verify the Rust search, Bloom/IFMA kernels, counters, compiler or solver. | pinned local build, fresh-kernel replay and 13-declaration axiom audit above |
| A two-summand direct-subtraction query is one exact lookup equation. | `decomposition_two_satisfiable_iff_direct_lookup` | Abstract `AddCommGroup`; no curve coordinates, data structure or runtime behavior. | pinned local build, fresh-kernel replay and 13-declaration axiom audit above |
