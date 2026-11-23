# Finite-difference point sieve candidate

Prepared an uncompiled source candidate that replaces only the two Horner evaluations at the head of point_sieve_candidates. The original function is preserved alongside it. This is an arithmetic optimization hypothesis without a measured bottleneck or timing result.

Initialize U=u0, D=u1+u2, E=u2+u2, V=v0 using canonical modular adds. At each x, capture U and V, then advance U by D, D by E and V by v1 before entering the unchanged branch body. The invariant is U=u0+u1*x+u2*x*x and D=u1+u2*(2*x+1). Zero padding uses active slices, preserving degree-zero and degree-one behavior despite nonzero inactive tails.

The native add/sub operations require a no-overflow premise: canonical operands and 0<p<=2^63 suffice. This source's multiplication uses u128, unlike the earlier narrow native inverse. Every recurrence state remains canonical. Updating only at loop bottom would skip advances on continue; the candidate avoids that error.

The proposed native wrapper is limited to 180 seconds, 2 GiB and one job. It compares 6,386 full sieve pairs plus 12 inactive-tail fixtures, 20,207 exhaustive arithmetic sequences, 32 boundary transitions and three incorrect-recurrence controls. Candidate sets, counts and ordered buckets must match. Real arithmetic/load counters remain distinct and their exact expected changes are checked.

A separate proposed Lean wrapper has 300 seconds, 8 GiB and one worker for invariant, canonical-add and ordered-sequence proofs, named axiom readbacks and controls. Neither wrapper has run. The actual native crate/dependency closure must be selected and frozen first; the checked-out producer path dependency is absent.

For degree d, evaluation changes from (2d+1)*p additions and multiplications to 3*p+2 additions and zero evaluation multiplications. Low-degree addition increases and the final unused advance are included. The unchanged exceptional-root and batch-inversion branches remain part of the paired functional tests.
