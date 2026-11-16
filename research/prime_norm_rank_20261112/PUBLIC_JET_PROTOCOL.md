# Public norm jets: prospective algebra and finite correctness contract

2026-11-13. Acceptance criterion remains a novel non-generic sub-exponential
ECDLP algorithm for general prime fields, with an honest full bit-cost proof.
This iteration tests one construction; it does not weaken that requirement.

For a public support polynomial G and curve f, consider explicit corrections
A'=A+GJ, B'=B+GK. Derive the exact residual map, and determine whether arbitrary
first quotient jets are reachable without violating the original degree window.
The polynomial norm is N(A,B)=A²-fB². Do not identify this correction with the
older W_base+C(T)H(T) matrix family without an actual map.

Predictions to check, not prior results [pcc:unverified]:

1. N(A',B')=N(A,B)+G(2AJ-2fBK)+G²(J²-fK²).
2. If AU+fBV+GW=1 and 2 is invertible, J=U Delta/2,
   K=-V Delta/2 reaches any first quotient change Delta modulo G².
3. If both the old and new A/B polynomials have degree strictly below degG,
   corrections divisible by G cannot change either polynomial. Thus local
   reachability need not give reachability in the pole-degree window.
4. Starting with public square-root residue s²-f=GR and B=1, a public Bezout
   inverse fV+GW=1 supplies corrections targeting N=cG modulo G². The remaining
   higher-order residual is retained, never accepted as an exact certificate.

Formalization: new NormPublicJets module, exact hypotheses in every theorem;
actual pinned Lean4.35.0/CryptoLean/Mathlib, no holes/custom axioms, fresh checker
and tracked-source proof gate. Keep every failed attempt and exit receipt.

Native finite correctness contract (not a finder, timing or scaling panel):
use the existing public F17 curve f=X³+2X+2, support G=X at node(0,6),
A=6, B=1, inverse V=9 and W=-(X²+2)/2, c=1. Check the exact full residual
identity and arbitrary Delta in F17[X] of degree<=1 (all289 polynomials).
A positive zero-Delta correction is unchanged; at least one nonzero correction
must reach its prescribed residue modulo G² while failing the full norm/degree
conditions. Independently test the first-jet formula by coefficient arithmetic
and by point/derivative evaluation. Retain characteristic2 as an excluded case,
not silently divide by2. Test low-degree rigidity via direct modular residues.
Source freeze precedes any artifact invocation. Tests are local development
checks; no attack solver, relation yield, host timing, performance ratio, EKS
launch or paid admission is inferred. Any later scaling run needs live admission
and a separate frozen protocol. Existing positive/changed-support rank artifacts
are read-only and are not rerun as new samples.

Sources checked: cached Galbraith textbook2.13 (Hensel's first-order expansion)
and8.4 (Riemann-Roch degree/pole restrictions), existing DERIVATION.md,
NormCertificateRecovery.lean and NEXT_FINDER.md. KB unavailable: no MCP tools;
canonical wrapper exits127 (no executable). No novelty or exhaustive review
claim for these classical local-algebra ingredients.

Pre-native refinement (after symbolic derivation, before native implementation):
include coupled corrections (A+sH+GJ,B+H), which preserve A-sB modulo G without
fixing individual residues. Formalize that G|(s²-f) and G|(A-sB), together with
actual pole bounds and nonzero norm, imply N=cG with c!=0. This alternative is
not ruled out by componentwise rigidity. It still requires a root-sign choice.

Additional fixed development control, no sign search: take the six public
branches (0,6),(3,1),(5,1),(6,3),(10,11),(13,7) on the same F17 curve. Construct
the Lagrange root residue from these points and reconstruct a bounded A/B by
a six-by-six homogeneous linear solve (degree A<=3, degree B<=1). Supply no
A,B to that solve; compare its checked output to the existing fixture only
as an independent known-answer control. This is reconstruction for a supplied
branch assignment, not a method of selecting signs. Flip the branch at x=13
and report the actual kernel dimension without assuming global infeasibility
of the underlying unsigned support. Verify every returned vector by original
point equations, actual degree bounds, and full norm identity, not rank alone.
