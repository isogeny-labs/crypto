# Second jets uniquely decode difference pairs on Q1

The frozen release gate passed all3825 admissible unordered rational-point pairs
on the reconstructed Q1/F127. Every difference Hessian selected exactly the
original pair among the six pairs on its chord. Every sum-surface control admitted
two pairs (original and residual), and every zero-Hessian control admitted all6.
These cases are correlated: several input pairs share a chord. This is an
exhaustive check of the registered admissible finite-field pairs, not3825
independent observations or a statistical confidence claim.

| Category | Number of unordered input pairs |
|---|---:|
| All pairs among129 rational points |8256|
| Excluded chart: infinity or equal x |306|
| Excluded F_y=0 after chart restriction |0|
| Excluded chords without four rational intersections |4125|
| Admitted and uniquely decoded |3825|

Sourcef476d0dedabd, pre-run checkpoint:
s3://setec-astronomy-artifacts/checkpoints/crypto/research__hec-principal-relations-20261114/20261115T161127Z-f476d0dedabd/.
Single release run exit0, empty stderr. First8 complete cases in RUN.txt, all
aggregate exclusions retained. No rerun or adaptive replacement. Native decoder
is new; point enumeration is pinned reused code. Hessians are constructed from
the proved target derivative formula; this gate does not independently compute
them from the source quotient or a second Jacobian implementation.

PROOF.md supplies the coordinate calculation: gradient gives the chord; mixed
Hessian vanishes on the actual pair. For the complementary pair the value is
-2*a*product((r1-r3),(r1-r4),(r2-r3),(r2-r4))/(r2-r1), nonzero for transverse
chords in odd characteristic. The four pairs sharing an endpoint also give
nonzero values. Multiplying the local equation by a unit only rescales the
restricted Hessian. A post-run written specialization argument establishes the
Gamma00 divisor as the reduced translate psi(T)+(C-C); it does not alter the executed test.

This makes the next source gate concrete: translate the inversion-symmetric
Mumford chart to a source point on the pulled-back Gamma00 divisor, evaluate
actual section gradients and Hessians, and decode the pair on Q1. Verify
constant terms against scalar evaluation, differential chart conventions,
regularity, rationality and group identities. Then construct general-public-input
decompositions and consistent signs. No runtime comparison or10x claim is made.
This study extends the genus3 transport construction; the genus2/4 portions of
the broader objective remain open. Existing closed records/defaults/leaderboard
and their canonical graphs are unchanged. PAIRS.svg is this run's new graph.
