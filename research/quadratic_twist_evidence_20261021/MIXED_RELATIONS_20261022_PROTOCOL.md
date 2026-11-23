# Mixed Frobenius relation falsification protocol (2026-10-22)

User request: keep finding routes and experimenting to disprove a quadratic
twist advantage for elliptic-curve index calculus. The universal absence of
connections or faster algorithms is not an acceptance claim of this finite
experiment. Invalid-curve attacks are outside scope; all point inputs must
satisfy their declared nonsingular model.

## Frozen questions and population

Use every nonsingular short model over p = 5, 7, 11, 13, 17, 19, with
0 <= a,b < p, exactly the 942-model population already registered by the
native study. For each p use its least positive nonsquare d and K=F_p[u]/(u^2-d).
The rational twist d*y^2=x^3+a*x+b maps to E(K) by (x,y) -> (x,u*y).
Record the existing ICV1 identifiers of E and of the normalized monic twist
Y^2=X^3+d^2*a*X+d^3*b; no new curve population or global catalog is introduced.

For every odd prime r dividing both rational group orders, choose the first
lexicographic nonidentity point of exact order r in each group. Let P be the
rational generator and T the mapped twist generator. Exhaust all ordered words
of lengths 3, 4, 5, 6 from the signed representatives
{+/-j*P, +/-j*T : 1 <= j <= (r-1)/2}. This is the entire nonzero cyclic
subgroup in each component, not a sampled factor base. A word is accepted only
when actual extension-curve addition gives O. The producer may use the pair of
known-answer component residues to enumerate efficiently, but must check the
point sum for every accepted word and independently verify that the r^2 pairs
of residues give distinct actual points. The finite table thus also certifies
rejection of every other word. No unknown-scalar recovery or timing is measured.

## Falsification targets and controls

1. Try to falsify the claim that an odd-order mixed zero relation has zero
   sum in each Frobenius component. A counterexample is a valid accepted word
   whose rational or twist component is nonzero. Retain it and stop promotion.
2. At each maximum word length L, compare the rank over F_r of all accepted
   signed relation rows of lengths <= L with the sum of the ranks of the pure
   rational and pure twist rows under the same length bound. Record counts by
   length and number of rational letters, zero rows, distinct rows, both ranks,
   and all failures. Rank gain would falsify the proposed finite obstruction;
   rank equality within this population is not a general hardness proof.
3. Check every rational/twist pair on all 942 models for a rational sum; retain
   every nonzero two-torsion exception. A non-two-torsion twist contribution is
   a counterexample to the mixed rational-relation obstruction.
4. Positive failure controls: remove the odd-order condition and exhibit a
   valid nonzero common two-torsion point; replace a nonsquare twist parameter
   by a square and show a valid image is Frobenius-fixed rather than negated.
5. Rejection controls: an artificial one-dimensional log equation P-T=O
   must fail actual point verification; perturb a saved relation coefficient
   and require replay rejection. Controls use valid points throughout.

## Proof work and verification

Attempt a Lean proof of the actual twist embedding's Frobenius-negation
property. Start with the finite-field nonsquare-root lemma, then the explicit
map of actual Mathlib points, including O. Derive the existing rational-relation
obstruction with no assumed hB for such images. Preserve exact characteristic,
short-model, nonsingularity, extension, and odd-annihilator hypotheses; expose
any remaining group-map or geometric bridge. Build and audit every new theorem
using the pinned Lean/Mathlib versions, with no research axioms or placeholders.

The native producer reuses the existing C++ field/curve backend. A separate
native replay must reconstruct the finite subgroup tables, verify saved rows,
and recompute the ranks. State any shared implementation rather than claiming
independence. Run controls in separate outputs, preserving unsuccessful builds.

## Execution, accounting, and stop rules

Freeze this protocol in Git before running. Use deterministic enumeration, no
random seeds, and a fresh dated output directory. Save commands, source/input
hashes, tool versions, exits, complete compact per-model/per-subgroup records,
and failed attempts. Limit each native full run to 300 seconds; a timeout is
incomplete coverage, not a negative result. The proof attempt is bounded to
this work session; unfinished declarations are not included as checked results.
No wall-time or solver-speed comparison is admitted, so all end-to-end costs,
rho ratios, fastest-external-tool claims, and speedups remain unmeasured.

Publish a source-linked report, quantitative coverage/rank graph, explanatory
diagram and inspected PDF in the existing PR. Preserve every historical archive,
run, proof receipt, catalog provenance dependency and measured leaderboard row.
Keep the broader formal, shared-Conductor, CI and review completion gates open.

At preregistration the Conductor check returned 127 (CLI unavailable), with no
shared reservation or replacement coordinator. This access failure does not
establish any mathematical obstruction.
