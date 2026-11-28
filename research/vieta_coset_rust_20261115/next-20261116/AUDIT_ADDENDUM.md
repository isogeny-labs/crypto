# Scope clarification and nonempty controls

The pinned generic tower argument is read with depth d>=1. For an empty
composition, membership means x=c: retain x_1-c,x_2-c as linear generators
or substitute those constants. Dropping membership constraints while retaining
free abscissa variables changes the generating system and is not covered.
The runtime supports the empty composition as a certified singleton.

The first normalized-Vélu fixture (terminal6 over F17) has an empty point
factor base on E:y^2=x^3+2x+2. Its all-target equality check is a valid empty
control, but cannot provide a nonempty positive control. The separate
native/tests/velu_positive.rs uses terminal12, certified abscissae5,7 and four
points. It checks exactly two ordered relations for target(7,11), including
(5,1)+(7,6), through scalar addition/lookup and the unchanged tower solver.

Post-run tests additionally use membership x^2=4 over F17 and target(0,0).
For A13,B0 the two distinct-abscissa ordered pairs are nonempty; for A4,B0
the four diagonal pairs are nonempty. Both Newton/Euclid and half-GCD agree
with those explicit sets; a tiny query budget is rejected. This closes the
vacuous-positive-test gap without changing any measured solver, analyzer,
protocol or binary bytes. The tests supply producer correctness evidence,
not independent signed standing [pcc:unverified].
