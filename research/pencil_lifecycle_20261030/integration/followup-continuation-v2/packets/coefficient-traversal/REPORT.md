# Cached-inverse coefficient traversal

T-1167, 2026-10-31. Producer source-model proof; independent source-fidelity review pending.

The complete coefficient traversal now composes the preceding single-slot word update. A size-indexed array retains its length throughout. A single inverse word is computed before traversal and passed unchanged through every step, matching the source placement of Field::inv.

For any valid event sequence, each intermediate array stays canonical and each decoded final slot is its original value plus the sum of contributions addressed to that slot. The transform theorem instantiates the generated inverse, initializes a zero array and executes the complete nested schedule. Its domain is 0<=n<=6, prime 1<p<2^31, canonical coefficients and parameters, and nonzero scale.

A finite theorem for every length 0 through 6 proves that the schedule is exactly range(n).flatMap(k => range(k+1).map(j => (k,j))), in that order. Additional theorems give n(n+1)/2 updates and no repeated index pair. Thus the n=6 traversal has 21 coefficient updates; this is a derived schedule count, not a timing measurement.

Seven main declarations and eleven controls passed, followed by 18 named standard-axiom readbacks. All four native module checks passed on their first attempts. Controls cover empty, constant, quadratic, scaled quadratic, zero, identity and shifted quintic inputs. Modulo 17 the coefficients [1,2,3,4,5,6] under x->x+1 become [4,2,3,16,1,6]. Omitting the highest input row changes this output. All controls are fixed formal examples, not a campaign cohort.

Source bindings: projective_law_gate.rs transform, prior Storage.lean, WordCoefficient.lean, Inversion.lean and Loop.lean. Primary documentation: https://lean-lang.org/doc/api/Init/Data/List/Control.html and https://lean-lang.org/doc/api/Init/Data/Array/Set.html. Standard traversal and field identities are reused without novelty claims.

The final expression is a sum over the exact nested schedule. Its identification with the previously proved polynomial composition, trailing-zero trimming and fixed-size divisor copying remain the next bindings. This source logical model is not a Rust compiler proof. Native receipts bind sources and existing compiled dependencies; no independent binary rechecker or independent semantic verdict is claimed.

Includes inspected PDF, editable SVG, structured result domains, custody and receipts. No scientific campaign, timing or cost ratio; full cohort and authorization limits unchanged.
