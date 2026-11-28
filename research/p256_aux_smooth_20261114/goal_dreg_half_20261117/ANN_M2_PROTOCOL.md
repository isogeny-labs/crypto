# Next qualification: exact planted m2 annihilator certificate (unexecuted)

Goal remains active. Exact unplanted m2 is locally certified198to14. The
next concrete target is the already-fixed PLANTED m2 case, which has two
ordered tuples representing one relation row. No new base or target.

This is a separately bounded algebraic certificate, not a solver retry.
The complete equations, point embeddings, f and the two reference tuples
are frozen before it starts. Do not use the relation table to construct or
preprocess equations. Use it only afterward to certify the degree of the
unchanged complete ideals. The old feature-lift and CAS cells stay closed.

1. Read/hash-check the same288 dense and chain points and exact m2
   reference tuples (193,262),(262,193). Preserve existing exact chain GB.
   Produce a degree-compatible dense point basis if needed for interpolation;
   its already-known quotient regularity97 must agree with the strict CI.
2. Interpolate the two point-indicator functions needed in each one-point
   algebra. Check their coefficient vectors against every point and the
   certified graded footprint. No target/group search, no scalar/log input.
3. Tensor the indicator vectors for the two ordered accepted tuples.
   For each degree cutoff, compute the dimension of their span with zero
   coefficients above the cutoff. This gives the exact graded filtration
   of Ann_A(f), since complete m2 replay proves its support is precisely
   those two tuples. At most two columns and288^2 tensor entries per arm.
4. Use the exact multiplication sequence
   `Hilb(A/(f))=(1-t^4)Hilb(A)+t^4 Hilb(Ann_A(f))`.
   The accepted two-point algebra has numerator1+t (certify a separating
   linear coordinate). Subtract it to obtain finite H-torsion; its last
   degree and the accepted algebra's regularity determine exact full D.
   This avoids assuming the general Gorenstein duality lemma, and avoids
   incorrectly applying the unplanted regular-element argument to planted f.
5. Reject altered indicator, missing accepted tuple, incorrect filtration
   rank and incorrect ideal/quotient shift. Compare the dense annihilator
   with the independent strict-CI duality prediction, without using that
   prediction as the numerical input. Native GMP arithmetic only.

Freeze implementation and input hashes before one native correctness
invocation<=180s/2GiB, only after success one fresh-directory byte replay
with the same bound. <=10000BMmonomials if dense interpolation needs BM.
No retry on timeout/failure, no CoCoA/publicCAS/EKS/paid run. Record setup
API calls separately, no F4 or runtime gain. Primary m2--7/depth8 scope and
independent signed/Lean admission remain; no planted exact result is yet
asserted by this preregistration.

Prior art: classical point interpolation/BM (NATIVE_BUDGET.md); filtered
duality reviewed in FULL_BOUND_PROOF.md. This finite annihilator calculation
is a certificate route, not a novelty or complexity claim. KB unavailable;
repository/native artifacts and linked primary papers are the fallback.
