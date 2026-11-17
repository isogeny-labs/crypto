# Fourteen-target audit and formalization

The requested deliverables remain all items 1–14 in the task: exact source
hypotheses, exhaustive finite experiments in the stated ranges, appropriate
counterexamples or repaired theorems, and faithful checked Lean proofs.
This study does not redefine that request as finite arithmetic certificates.

Budget: no external paid compute. Proof compilation and unit tests run locally.
New experiments require the existing checkpoint gate. A gate failure blocks
that experiment, not static source inspection or proof construction.

Formal acceptance: pinned Lean and Mathlib revisions, actual compilation,
`#print axioms` for every reported declaration, no holes, native evaluation,
or research axioms. General algebraic lemmas expose every assumption.
Concrete witnesses are labeled finite witnesses, not general theorems.
Source assertions remain distinct from independent proof checks and signed PCC
standing. Every mathematical result is unverified under PCC until its gate
provides the required standing.

Initial discriminating obligations:
1. Extend existing S3 proof coverage to repeated x, and expose the zero-sum
   group-law interpretation. Do not re-prove the already present distinct-x
   root equivalence as a novelty.
2. Explain BK's excluded unipotent case with a general linear-algebra lemma
   and faithfully preserve the original r not dividing q-1 condition.
3. Separate odd prime orders from r=2 in the supersingular prime-field case.
4. Prove the alternating-pairing restriction to a cyclic subgroup is trivial;
   this qualifies a restricted pairing, not nondegeneracy on its full domain.
5–9. Read original conditions and heuristic quantifiers before counting. For
   Jacobian subgroup smoothness distinguish an actual Jacobian example from
   the general subgroup-confinement lemma.
10–11. Preserve asymptotic versus exact walk claims and the standards scope.
12–14. Audit the actual F4Run constructors; independent languages on the same
   host do not discharge the second-machine requirement; add a known-degree
   control to the existing closure criterion rather than merely asserting it.
