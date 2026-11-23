# Signed graph same-root cycles, anchors and scalar constraints

2026-11-01. Ten main declarations, eight controls and eighteen separate named axiom readbacks passed. Source: three_leads.rs SignedGraph.constraint/anchor/edge. No scientific execution.

## Derived semantics

constraint_solution derives (-a)*b^-1=k from a+b*k=0 and b!=0 over a field; it does not assume the returned answer. zero_coefficient derives a=0 when b=0, exactly the consistency assertion required by the native None branch. An inconsistent zero-coefficient relation is excluded by the input relation, not accepted as a valid solve.

For a same-root reduced edge cu*R+cv*R=A+B*k, opposite_cycle uses cu+cv=0 to derive the source constraint equation -A+(-B)*k=0. opposite_cycle_solution proves the source double-negated numerator/negated denominator result equals k. This checks the native sign convention directly.

For equal signs c, same_cycle_anchor derives R=A/(2c)+(B/(2c))*k, requiring 2!=0 and c!=0. same_sign_branch derives equality of native signs from their +/-1 domain and cu+cv!=0. native_denominator relates source if cu==1 then2 else-2 to2c. In ZMod of a prime modulus r>2,2 is nonzero; source's representative r-2 must additionally be connected through canonical word arithmetic. A characteristic-two control records why nonzero2 is necessary. No inversion of2 is assumed valid in characteristic2.

A first fixed anchor stores the derived root equation. If the root already equals x+y*k, existing_anchor_equation derives (x-a)+(y-b)*k=0 by subtracting two valid root descriptions. existing_anchor_solution returns k for nonzero y-b; existing_anchor_zero proves x-a=0 when y-b=0. These are the semantic obligations behind anchor's Some/None branches, not proofs of mutable-state updates.

## Controls and retained attempts

Controls cover direct constraint solve, native positive-cycle sign convention, both +/- same-sign anchors, existing fixed anchor solve, equal-anchor zero coefficient, inconsistent zero coefficient rejection and characteristic2. All pass with separate readbacks. First proof attempt used a misoriented negation equation and had a redundant tactic; second extended attempt lacked DecidableEq for the source denominator conditional. Both failed sources/logs/receipts retained. Corrected exact module and controls pass. Nonfatal unused tactic warning retained.

## Composition boundary

Premise reduced edge must come from valid relations and the prior signed-potential reduction theorem. Premise fixed root equations must be maintained by forest/anchor updates. Field model does not itself establish native prime constructor, canonical words, modular inv/modpow, +/-1 i8 closure, r-2 representation, recursion/index safety, metadata storage, counter headroom or final independent answer verification. Earlier Ring word bridge covers add/sub/neg/mul under its bounds, but is not silently a proof of inv. Independent semantic review of this new packet remains pending.

Source-first plan was written before construction; native source and prior GraphAlgebra were inspected. Fresh checks use copied local helper binaries and cached Mathlib with explicit pre/post bindings and unchanged inputs. Standard field cancellation and affine equations supply the mathematics; no novelty claim. No scientific jobs, targets, cost ratios or repository edits.
