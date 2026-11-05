# Exact proof map

All names below have namespace `TernaryPacket`. They are checked in
`TernaryPacket.lean`. Unless explicitly specialized, `A : Type*` has the
Mathlib structure `AddCommGroup A`, pairs/tags are arbitrary, and all stated
variables are universally quantified. No field, nonzero-pair, graph, or
coefficient-ring assumption is implicit.

| Mathematical statement | Lean declaration | Exact additional hypotheses | Status |
| --- | --- | --- | --- |
| `S^3 = id` on pairs | `rot_thrice` | None | Checked |
| `S ∘ S^-1 = id` | `rot_invRot` | None | Checked |
| `S^-1 ∘ S = id` | `invRot_rot` | None | Checked |
| Zero digit acts identically on pairs | `pairAction_zero` | None | Checked |
| Zero digit acts identically on tags | `tagAction_zero` | None | Checked |
| Pair digit action composes by addition | `pairAction_add` | `x,y : ZMod 3` | Checked |
| Tag digit action composes by addition | `tagAction_add` | `x,y : ZMod 3` | Checked |
| Selected row at `x` equals selected column at `x+c` | `digit_locality` | `c : ZMod 3`, `c≠0` | Checked |
| Pointwise slot action composes by addition | `slotAction_add` | `a : Nat`, `x,y : Fin a → ZMod 3`, `g : Fin a → Tag A` | Checked |
| Equal reached classes stay equal after a slot translation | `slotAction_translate` | Same slot types; `slotAction x g = slotAction y g` | Checked |
| Translation bijects fibers over any two reached classes | `orbitFiberEquiv` (definition with proved inverse laws) | Same slot types | Checked definition; its laws are kernel checked |
| Reached class fibers have equal cardinality | `orbit_fiber_uniform` | Same slot types; both classes are explicitly `slotAction x g` and `slotAction y g` | Checked |
| Slot domain has cardinality `3^a` | `slot_count` | `a : Nat`, including zero | Checked |
| Every reached class fiber has positive cardinality | `orbit_fiber_pos` | Same slot types; class is `slotAction x g` | Checked |
| Pointwise selected row/column groups agree at the selected coordinate | `slot_locality` | Same slot types; `i : Fin a`, `c≠0`, **`y i = x i+c`** | Checked; integer-to-digit bridge remains open |
| The forward rotation fixes exactly the zero pair | `rot_fixed_iff` | `NoThreeTorsion A` | Checked |
| The inverse rotation fixes exactly the zero pair | `invRot_fixed_iff` | `NoThreeTorsion A` | Checked |
| One-step tag action fixes exactly the all-zero tag | `tagAction_one_fixed_iff` | `NoThreeTorsion A` | Checked |
| The label field `F67` has no additive three-torsion | `noThreeTorsion67` | `A = ZMod 67` | Checked finite `decide` |
| Forward rotation fixes exactly zero over `F67` | `rot_fixed_iff67` | `A = ZMod 67` | Checked |
| One-step tag action fixes exactly zero over `F67` | `tagAction_one_fixed_iff67` | `A = ZMod 67` | Checked |

The 20 reported theorems are exactly the theorem rows (excluding the
`orbitFiberEquiv` definition). Each has a `#print axioms` command. The pinned
auditor traverses their dependencies and found only the standard Lean
foundations `propext`, `Classical.choice`, and `Quot.sound`; zero theorems
require a trusted native compiler evaluation. Small private digit arithmetic
lemmas and the F67 torsion statement use ordinary kernel-checked `decide`.
Both the module-only official kernel replay and the import-inclusive
`leanchecker --fresh` replay passed, as did the unmodified shared proof gate.

## Open formal obligations

These are not assumed as axioms, encoded as hypotheses claiming the full
result, or represented by unrelated arithmetic certificates:

1. Define integer slots, their base-three digit extraction, and 3-adic
   valuation; prove that `y-x=t≠0`, `i=v3(t)`, and the nonzero leading digit
   `c` imply the exact digit hypothesis used by `slot_locality`.
2. Connect the Cauchy kernel's nonzero support at node `t` to those slot
   differences and then to the formal label-group equality. The present
   module proves the label implication itself, not the kernel theorem.
3. Classify the full product-action orbits as powers of three according to
   their active coordinates, prove the precise fiber size `L/|orbit|`, and
   formalize equal-load packet padding. The checked positive uniform fibers
   and fixed-point classification are central ingredients, not a proof of
   the full graph construction.
4. Define graph ports, random label assignment, variance bounds, common class
   load `H`, packet counts, selected-symbol marginals and child-size bounds.
5. Formalize the mandatory target-specific mode split and its unique-source,
   unique-target factorization, including the corrected `18M/D` mode bound.
6. Define the characteristic-three coefficient extension, creation-precision
   lifts, integrality, Cauchy expansions, Hermite interpolation and valuation
   losses. No statement about these is proved by the label-field proofs.
7. Prove all parameter-admission inequalities, recurrence, space bound and
   complete contraction complexity. The same-class exponent claimed by the
   written adaptation is not a Lean-certified result here.
8. Any reduction from prime-field ECDLP to that contraction theorem, including
   its size, degree, success probability, and complete cost. There is no
   formal ECDLP algorithm or original-breakthrough claim in this project.
