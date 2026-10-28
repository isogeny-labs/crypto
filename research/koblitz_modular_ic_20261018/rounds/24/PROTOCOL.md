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
