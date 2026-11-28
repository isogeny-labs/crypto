# Closed-divisor reconstruction and public-adjacency orientation

Positive and target solvercells both completed once with exactdimension0/
degree12parametrizations and all12original polynomial identities verified.
No solver rerun. This separate frozen recovery consumes original targetRUR,
rechecks its identities, and constructs actual Jacobian images.

Factor the squarefree degree12elimination polynomial using distinct-degree
factorization d1..3 and equal-degree quadratic-character splitting overF127.
Fixed SplitMixseed20261115, cap256splits per call; retain failure. This is
bounded finite-algebra factoring, not enumeration of field/group candidates.
Verify exactfactor product and Frobenius irreducibility identities. Degrees
above3are unsupported here: rational unordered triples have Frob permutations
of orders1,2or3under the existing rational-Jacobian premises.

For EVERY irreducible parameter factor, reduce all8coordinate functions in
its field. Form monic u(X)=product(X-x_i) and Lagrange v(X) using y_i. Require
distinct x_i and allu/v coefficients in F127; retain unsupported projections/
descent. Reconstruct all cubic annihilator conditions and create targetQClass.
At least1supported class required, no factor or input substituted to force it.

Recompute targetcentering beta and knownG from prior geometric D1certificate.
For each class imageX require nonzero/ell-subgroup and Kummercoords equal the
PUBLIC-KUMMER input. Select orientation ONLY by comparing kappa(X+G) and
kappa(-X+G) to PUBLIC-NEXT-KUMMER, the independently computed public coordinate
of B+2D1. Exactlyone must match. Odd-order proof: both would force2X=0or2G=0,
excluded by the verified subgroup premises. Unsupported charts retained.

SubtractG after orientation to recover the Bimage. ONLY THEN compare against
prior geometric Bcertificate, reconstructed from doubled/scaled pointdifference.
It is a post-recovery validation oracle, not equation construction, factoring,
or sign choice. A corrupted adjacentKummer input must be rejected, guarding
against oracle-driven orientation. Require all accepted candidates give exactly
one nonzero ell-subgroup Bimage. Shared arithmetic means no independent replay.

The verifier reuses pinned exact coefficient/target arithmetic; newcode only
factorization, extension-to-base descent and class/orientation checks. Compile/
commit/fingerprint/checkpointgate before one execution to RECOVER.txt/stderr/
EXIT-RECOVER.txt. Keep failedpositive producer receipts and allsolveroutputs.
No input discrete logarithm, group enumeration, timingcomparison or10xclaim.
Full coldconstruction/calibration/relations/LA/descent and genus2/4 remain open.
