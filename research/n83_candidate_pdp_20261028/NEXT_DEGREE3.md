# Next measurement: exact V3 closure of preserved m2 systems

This is a plan, not a result or frozen execution protocol. Preserve the full
all-family m2..9 objective. The next bounded diagnostic uses the four saved
spike40 m2 Koblitz/binary inputs and their four paired baseline controls.
Do not regenerate targets, discard spike40, or substitute new encodings.

The completed V2 data has candidate linear rank17 with exactFFD2 and unknown
exactLFD; baseline has rank53/54 and globalLFD2. This motivates a degree3 closure
that can supply a stronger certificate, rather than assuming a plateau is LFD.

Implement an ordinary-degree-bounded closure modulo the saved Boolean field
equations, extending the existing exact V2 semantics. Use graded monomial
columns and immutable rows. For54 variables there are
1+54+binom(54,2)+binom(54,3)=26,290 squarefree columns through degree3.
This column count is a combinatorial calculation, not a measured matrix rank.
Only multiply a current polynomial when its ordinary polynomial degree plus the
multiplier degree is at most3; Boolean reduction is justified using the retained
field equations. Retain every row operation and fresh-process replay.

A new polynomial of degree at most2 in V3, with explicit membership trace and
verified nonmembership in the already complete V2 row space, certifies a degree3
fall and hence LFD>=3. A rank increase in degree3 rows alone does not do so.
To certify exactLFD3, additionally require the unit or the existing exhaustive
affine-completion global upper certificate, and check the strict lower witness.
No such certificate has been computed yet.

Controls must include a genuine degree3-only fall, a no-fall nonlinear system,
wrong-degree multiplication rejection, modified trace rejection, and the four
baseline controls. Each run retains source hashes, column scheme, ranks,
explicit lower witness, upper certificate or null, resource limits and errors.
Freeze a new protocol and native analysis before data. Preserve every attempted
case and replay. If V3 does not close, keep LFD unknown or its proved bound;
do not turn a resource limit or plateau into an exact answer.

Prime LFD, larger-m closure, highest-form regularity and intermediate solver
degree remain separate unfinished measurements. An FFD/LFD result alone does
not establish an elite base or regularity improvement.
