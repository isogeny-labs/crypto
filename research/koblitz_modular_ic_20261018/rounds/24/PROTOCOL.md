# Round 24: constant-field gate before further pole geometry

2026-10-28 UTC. Continue the existing user-authorized construction task in the
persistent checkout, parent 12ba46f9ab301a136152ff17daef2e5cae42c2a8. This is a
fixed exact computation, not an IC cost experiment or an official promotion.

## Reconciliation and sources checked before choosing this calculation

The approved migration receipt and managed receipts were read. The boot remains
8ffd9345-7a79-45a3-9db0-6d0e8631d66c. Diem is deadline-expired, 50/52,
pending=false; the legacy status is complete. EKS overview has no active jobs
(three succeeded and one failed historical n83 job remain visible). No existing
Koblitz computation is running. Round 23 and its final source checkpoint are
complete. Do not repeat G32/G33 or capped global normalizations.

Read R22 CM/PROOF_MAP and the retained plane32 function-field data; R23 CM,
NEXT, DIVISOR_SCOPE_ADDENDUM and DESCENT_CHECKS; the previous session log and
RESUME. Primary references rechecked: Stacks
https://stacks.math.columbia.edu/tag/0BXX (proper curves/function fields),
https://stacks.math.columbia.edu/tag/0AYQ (constant field and degrees),
https://stacks.math.columbia.edu/tag/03PA (etale local presentations).
Use established factorization and constant-extension criteria; no novelty claim.

## Frozen CF24 calculation

Take the exact polynomial f in R22 cm22-plane-places.sing, obtained from its
source-bound monic degree-32 plane equation by clearing u^64. Factor it in
F4[u,lambda], with F4=F2[a]/(a^2+a+1). Retain every factor and exponent.
Check their product, their lambda degrees, separability, and the primitive
content. A factorization failure or cap is inconclusive, not irreducibility.

If it splits into two conjugate positive-degree factors, extract q=q0+a*q1
in a polynomial ring over F2. Check the conjugation/product identity and seek
an exact Bezout/unit certificate for q1 in F2(u)[lambda]/(plane). Verify that
alpha=q0/q1 satisfies alpha^2+alpha+1=0 in that actual field. This is an
explicit constant witness, not an inferred L-polynomial relation. Record
whether either factor is irreducible according to the native factorization.

Use the already certified degree-16 extension of the geometrically integral
elliptic base and an actual degree-10 place only with an explicit written
constant-field divisibility argument. Formalize feasible field identities and
the resulting integer divisibility constraints; expose the geometric bridge.
Do not assume geometric connectedness or identify another modular model.

## Controls and exact acceptance

Positive control: x^2+x+1 must split over this F4 presentation and its factor
product must agree. Negative control: x^2+x+a must remain irreducible over F4,
with all four candidate roots explicitly evaluated. Every source identity has
zero residual; final pass requires native exit 0, the exact final marker, and
no Singular error diagnostics. A corrupted witness will be rejected by the
same residual detector if a constant witness is obtained.

No stochastic samples, seeds, statistical power, A/A, timing comparison or
solver performance metric applies to these fixed algebraic identities.
Report exact counted degrees, factors and zero/nonzero residuals. Preserve
native outputs and unsuccessful attempts separately. Freeze any adaptation
before taking the data it changes.

## Backend, bounds, publication and open requirements

Use the inherited local native Singular 4.4.1 post-processing lane, at most
4 GiB virtual memory and 1200 seconds per invocation. Shell only invokes and
stores native work. No new EKS job, Diem generator, waiting-freeze action,
fifth Conductor claim, merge or auto-merge. Checkpoint to the authorized
setec-astronomy-artifacts bucket before every native call; preserve raw output
in an archive included in later checkpoints. No budget increase.

Conductor check clears the study path. T-409 is failed at attempt 4 of its
four-attempt budget: do not claim or restart it. This continues user-authorized
work without another claim. The shared 2026-10-28 daily log is held by T-647;
record this session here and defer its shared-log append until scope clears.
The migrated checkout has no .conductor/WORKFLOW.md; do not invent one.

The cross-repository experiment.protocol/v1 checker/schema is absent at this
revision. Retain this inherited exact-computation contract and export without
claiming that checker passed. Publish source, raw manifests, report, editable
diagram/PDF and faithful Lean in the existing PR 2683. Keep all R01-23 evidence
immutable. The five rational charts, additional zero supports, complete RR,
actual F2 factor map/descent/nonzero order-11 action, node187 polynomial
membership, quotient geometry and arbitrary reverse evaluator remain open.

## Frozen O24 chart construction, after CF24 and before chart data

CF24-factor-01 completes with a single degree-32 irreducible factor over F4,
exact product, derivative gcd 1, and both field controls. It supplies no
constant witness. The full constant-field conclusion uses the written bounds
w|16 and w|10 together with nonsplitting over F4.

At the base point O use t=u/v and q=1/(t^3*v). Check the exact base chart
E0=q+1+(1+t)*t^3*q^2 and the original b,c substitutions below. Put
Delta=t^11*d and C=t^14*e in the existing degree-8 tower H(Z); require the
exact polynomial divisions and d(0,1)=e(0,1)=A(0,1)=1.

Unramified chart: Z=t^3*y. Check H(Z)/t^14, the quadratic
lambda^2+A*lambda=Z+b, and the original Tate equation. Verify the Jacobian
in (q,y,lambda) is a unit at t=0,q=y=1,lambda=0 and 1.

Tame chart: t=w^7*x^2, Z=w^11*x^3, with q=x=1 at w=0. Check
H(Z)=w^88*x^24*(1+A^2*d*x+w^11*d*x^4+w^10*e*x^4),
the same lambda equation and unit Jacobian in (q,x,lambda). Retain rational
inverses w=Z^2/t^3 and x=t^11/Z^7. Source-bound generic identification,
regular local rings and proper degree exhaustion must precede any claim of
four normalized points with weighted sum 1+1+7+7=16.

Only after these chart checks pass, freeze the finite-precision coefficient
evaluation of nu and all eight original registered entries. No mere Newton
slope or finite projected factor multiplicity supplies a geometric branch.
All resource limits, checkpoint and no-new-job/claim rules above persist.

## Frozen O24 original-coordinate valuation evaluation

O24-charts-02 passes all base/tower/toric identities and all four unit
Jacobian checks. Compute the unique coefficient lifts in F2[[w]]/(w^192)
of those four etale charts, with all unknown residues fixed before the run.
For e=1 put t=w,Z=t^3*y; for e=7 use t=w^7*x^2,Z=w^11*x^3. Iterate the
displayed char-2 implicit equations with unit inverses and require residuals
zero modulo w^192. Lift both lambda residues through its quadratic equation.

Evaluate the exact original nu equation from the retained R22 generic record;
check its n coefficient is A^2*Delta. Do not recompute the generic Groebner
basis. Divide the numerator and denominator by their actual displayed w
orders. If nu has a pole or vanishes beyond the precision, stop this case
without inventing a valuation; a Laurent extension would need a new frozen
adaptation. If nu is integral, evaluate all five original source residuals
modulo w^100 and all eight actual registered polynomial entries. Every claimed
valuation must have a displayed nonzero leading coefficient below order 100.
The true series precision for nu is at least 192-77=115 on the tame chart.

Finite truncation identifies a valuation only with the already certified
unique smooth-chart lift and exact rational function identity. It is not a
standalone simultaneous arc for node187 or a complete global divisor. Retain
the truncations, all residuals and orders; no timing claim is made.

Before o24-series-02, correct the rejected order helper: in the fixed
univariate global ordering compute the degree of the last polynomial term.
Require helper(1+w^191)=0, helper(w+w^3)=1, helper(w^7+w^179)=7 before any
geometry is evaluated. This corrects implementation, not a precision increase,
pole assumption or chart change. The first attempt's highest-degree outputs
and resulting source failures are retained and are not interpreted as orders.
