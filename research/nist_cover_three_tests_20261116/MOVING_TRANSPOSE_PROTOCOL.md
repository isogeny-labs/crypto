# Reverse map at boundary points via line complements

Frozen before data,2026-11-17. Same source/quartic/global/reference inputs and
native local qualifier standing as CLOSED_TRANSPOSE_PROTOCOL.md. Sources:
the exact hyperplane-divisor relation on a plane quartic, finite-field norm
and the existing transpose correspondence. No novelty claim or paid campaign.
Each invocation <=600seconds/1GiB, new snapshots/manifests and all failures kept.

For a finite closed point Q over Fp^e, try lines y=a*x+(yQ-a*xQ).
Divide its exact quartic restriction by x-xQ; require degree3 remainder.
For an infinity point Q=(1:m:0), try y=m*x+a; require restriction degree3.
Here Q plus the residual R_Q is a hyperplane section in both cases.
The missing point (0:1:0) is explicitly excluded by the actual curve equation.

Use the qualified finite-algebra trace-cycle constructor to norm R_Q to Fp,
including all multiplicities. Run the closed transpose on that entire norm.
Require every resulting raw transpose fiber to have degree9; do not accept a
degree drop by silently losing boundary components. On a rejection restore
both field contexts and retain the line parameter/error before trying again.

Let A_Q be this source class, and choose Q0=retained point0. Since
Q+R_Q is linearly equivalent to a hyperplane section, define
psi(Q-eQ0)=e*A_Q0-A_Q. Sum over irreducible factors and multiplicities for
closed divisors. This accounts for a bad original point using line equivalence
instead of evaluating saturated equations at that point.

Search integer line parameters in the base field up to
(2*deg(Dden)+8)^4+32, with the600second process limit overriding execution.
The report must distinguish this conservative algebraic avoidance proposal
from an independently checked universal bound; observed termination on the
fixed fixtures alone does not establish all-input completion. Factor splitting
retains the earlier1024shift cap. All exhausted caps/timeouts are failures,
not permission to drop support or change the target.

Fixed qualifiers: compare normalized moving outputs with the executed direct
transpose for indices0,1,2,3,223,224,225 and the first split-fiber point.
Check all six pinned source-side compositions again through this new path.
Factor and process every geometric infinity point as a closed point; compare
the image of the full infinity hyperplane, with multiplicities, against the
image of the entire first finite split vertical fiber by exact source equality.
Retain all residual cycles, factor fields, raw source fibers, line decisions,
source images and composition results. Reject altered source-side composition.

This constructs explicit signed-divisor interfaces. It does not automatically
convert every opaque KM evaluation space to a geometric divisor, supply
independent verification, or certify the proposed universal avoidance bound.
