#set page(paper: "a4", margin: (x: 18mm, y: 17mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 10pt)
#set par(justify: false, leading: 0.6em)
#set heading(numbering: none)
= Koblitz cover transfers: Round23
Exceptional lifted geometry of the original Tate/QR incidence curve

2026-10-26–27, America/Los_Angeles. Exact computations and written geometric
derivations; no IC cost, new relation rank or official experiment promotion.

== Two exceptional proper fibers now have local source certificates
The supersingular support SS5 has four degree-20 closed points over $bb(F)_2$,
all unramified. The discriminant support Z5 has two degree-10 closed points,
with ramification indices 1 and 7. At the ramified point the two last registered
polynomial denominator entries have orders 4 and 6. These local results belong
to the specified incidence function field; the actual modular-factor map and
complete global pole geometry remain open.

#table(columns: (1fr, 1fr, .8fr, 1.7fr), inset: 5pt, stroke: .4pt + gray,
  [*Support*], [*Closed points / degree*], [*Ramification*], [*Orders of A, delta, S, N*],
  [SS5], [4 / 20], [all 1], [1, 0, 0, 0 at each],
  [Z5 P1], [1 / 10], [1], [0, 1, 0, 0],
  [Z5 P7], [1 / 10], [7], [0, 7, 4, 6])

The original coordinates b,c,lambda,nu are units at all these points. The generic
nu denominator $D=A^2 b^3 delta$ has orders 2,1,7 respectively. Its recovery
numerator has exactly the same orders because the retained nu is a unit. A
vanishing generic-elimination denominator therefore does not imply a nu pole.

== What supplies the source geometry
SS23 retains an exact original-ring membership certificate for the missing
equation, together with a unit h at the full SS5 fiber. Both Tate factorizations
are checked, with cofactorA=1 on the fiber. Its actual inverse and original
Jacobian show that this is an etale source neighborhood. The restored R22
quartics and Frobenius certificate give the complete residue-degree count.

At Z5, complete saturation and a complementary ideal quotient split the saved
closure into local algebras of dimensions 2 and 14 over F32. The original
unsaturated equations instead give 2 and 18. Both closure residues have degree2.
Exact intersections and comaximality are checked, and every registered unit is
visited. A unit minor of the actual 51-generator closure Jacobian is found at
both supports. On this regular curve the fiber lengths determine e=1 and e=7.
The quotient dimensions 8 and 12 determine the orders 4 and 6 via DVR lengths.

These are written proper-curve/DVR consequences of source certificates and the
R22 generic-component identification. CM.md states that bridge and its exact
scope. The complete scheme normalization is not formalized in Lean.

#pagebreak()
= Exact data and the construction gap
#image("round23.svg", width: 100%)

The bars show exact vector-space dimensions of one fixed fiber, not times,
speedups or statistical estimates. Sources: z23-components-host2-07 and
z23-smooth-host2-06. SS5 source: ss23-compact-host2-08; integer unit lift:
ss23-unit-host2-09. Every native invocation keeps 4GiB virtual memory and a
1200-second maximum. No resource increase or new research EKS job is used.

#pagebreak()
= Verification, failures and next construction
== Formal scope
Lean4.34.1 and mathlib d13f23b are pinned. REPLAY_LEAN.sh rebuilds every
transitive project import from the current source. The new module contains
15 theorems; the transitive axiom audit reports zero failing and zero trusted
native results. The literal universal integer lift and characteristic-two SS5
inverse are kernel proofs. Ideal membership implications, degree accounting
and valuation arithmetic expose their hypotheses. The full normalization,
proper-curve identification and divisor semantics remain written mathematics.
See PROOF_MAP.md. A separate fresh-checker pass is not claimed.

== Migration and failures
A second Spot host loss removed the ignored raw outputs in /var/tmp, while
checkpoint10 retained all nine source commits and the selected compact
certificate. Missing verification receipts were regenerated on a persistent
/home checkout. Original raw records that were lost are explicitly marked so;
no fabricated logs or terminal statuses replace them. The native wrapper now
archives raw results after each call so the following checkpoint includes them.

The characteristic-two GTZ loop, monomial-name shadowing, differentiation in the
wrong ring, omission of the last unit after a zero entry and the stalled minor
driver are preserved in FAILURES.md with source history and corrected replay
references. Native exit0 alone is never accepted as a certificate. Root software
validation and any unrelated infrastructure failures are recorded separately
in VALIDATION.md; a native algebra certificate is not a software-test substitute.

== Remaining acceptance criteria
Complete the normalized lifts above the five rational base supports, including
infinity, and combine all coordinate and registered-denominator orders into
complete global divisors. Construct a complete prescribed Riemann–Roch space,
then an actual nonconstant factor map or integral correspondence. Verify its
function-field identities, F2 descent and nonzero action on the order-11 subgroup.
The shared residual Hecke system does not replace that last test.

Node187 still needs explicit source-bound polynomial membership or a complete
simultaneous arc. Full quotient saturation and proper polarized embedding remain
separate. Direct relations still require arbitrary modular-divisor reverse
evaluation and the ordinary degree-460/367 fibers and images. Existing n=3 and
same-field n=5 controls remain regression fixtures. The user assesses faster IC.

== Sources
R22 CM/SOURCES/PROOF_MAP and original circuit/closure; installed Singular4.4.1
elim.lib and primdec.lib; Stacks normal/proper curves (tag0BXX), etale Jacobian
presentations (tag03PA), and degree accounting (tag0AYQ). Full linked references
and derivation: REPORT.md and CM.md. Source-bound raw manifests and verification
receipts accompany this report; no independent modular model is substituted.
