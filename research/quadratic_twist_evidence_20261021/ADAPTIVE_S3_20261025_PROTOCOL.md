# Adaptive certificate-seeking S3 solver: frozen protocol

2026-10-25, starting head44510cf8e96dfe0ffd944895a3a9a89b428d8710;
run `adaptive-s3-20261025`, backlog XP-20261025-reduced-s3-mutant-reinsertion.

## Question and scope

Does reinsertion of newly derived low-degree rows change the operational
matrix degree needed by a precisely named, certificate-seeking solver on the
actual S3 inputs? Test original and reduced generators on all98 size2 cases,
every target and both matched-base controls from the previous frozen round.
Keep the188 comparisons /1,384 comparison-targets; preserve all losses/nulls.
The original and reduced affine ideals are equal by the preceding Lean proof.

Operational matrix degree, maximum supplied-input degree, expanded witness
degree, homogeneous leading cutoff and F4/F5/runtime are separate quantities.
No general or intrinsic twist advantage is presumed. This is a bounded
certified finite solver experiment, not an optimal-solver or speed claim.

## Exact algorithm

Original inputs are (S3,gx,gy) or (h,gx,gy) in that order. Each pool element
carries three polynomial multipliers certifying it as an original-input
combination. Initial multipliers are the three unit vectors. Arithmetic is
over the declared prime field. Zero inputs are omitted from matrix rows.

For D=0,...,8 build the full row space of every admissible monomial multiple
of every pool element, with product degree<=D. Use graded descending columns
(total degree then x exponent), and fully reduced row form. At each layer:

1. If1 is in the row space, return UNSAT with its expanded original-input
   combination witness.
2. Otherwise enumerate the constrained ground-field roots (the two membership
   roots per variable, with the actual S3 constraint), and evaluate all
   degree<=D monomials. Accept a positive root-set certificate only when
   matrix_rank+evaluation_rank=number_of_columns and evaluation has full
   root-interpolation rank. Include every actual signed point lift. This
   contract deliberately requires a polynomial row-space certificate even
   though direct factor-base enumeration already supplies candidate roots.
3. Sort nonzero reduced rows of degree<D by (degree,canonical polynomial).
   Promote a row only when it is not already in the degree<=its_degree row
   space of the current pool, including earlier promotions. Carry its exact
   original-input witness. Rebuild at the same D until no promotion remains.
   Then increase D. Cap64 rounds/layer and expanded witness degree32; report
   a cap honestly rather than returning a fabricated degree.

Track the highest matrix degree actually visited. Also track the maximum
degree of any expanded original-input multiplier product ever constructed,
including generated rows before reduction, and report the final certificate's
expanded degree separately. Reinsertion may lower the operational degree
without lowering the expanded witness degree. Do not hide that distinction.
The solver must not use recorded scalar labels to discover solutions.

## Independent verification and controls

C++ may reuse immutable prior field/group helpers. Rust uses its independent
field/point arithmetic and batch RREF. It recomputes spaces/ranks/promotion
eligibility and symbolically expands every published row witness in the
original inputs. Compare actual solution sets and useful-rank traces with the
frozen study. Verify bounds, fixed-point decisions and completeness, not just
hashes. Reject rehashed false row combination, false operational/expanded
degree, missing target, false source binding and false recovered scalar.

For F43 additionally solve all11 separate synthetic main targets through the
actual phi(x,y)=(41x,11y) map. Derive twist base logs from actual relations at
the public generator, query the certified reduced solver at target shifts,
recover the scalar from signed relation rows, and verify it on the original
main point. The recovery function takes the point, not its known-answer label;
no logarithm table defines the map. Keep every attempted shift and no timing
or shared-target speed comparison.

## Lean bridge, evidence and decision

Attempt faithful Lean proofs of row-adjunction ideal invariance, the F43
degree2 obstruction/degree3 unit certificate, and the positive degree2
vanishing-space certificate. Reuse the unchanged actual point/group and
scalar-preserving map theorems. Expose any unformalized algorithm component;
do not equate a checked certificate with a full general solver formalization.
Any new polynomial identity gets a source-bound IDC1 certificate and meaningful
false-identity/source controls. Preserve all preceding frozen sources/manifests.

Each native command has a600-second cap. The declared single-iteration native
envelope is3600 seconds; use existing local resources, no paid provisioning,
external upload or credential reuse. Keep compact canonical outputs, failed
attempts, exact corruptions, source pins, replay receipts, report/graph/PDF and
kernel axiom audit. Run the unmodified scoped proof gate, full mandatory
library suite, native replay/controls and workflow/backlog metadata checks.
Extend PR2381 with ordinary commits, no force/rebase, and preserve all gates.

A smaller operational degree with verified witnesses advances this named
solver bridge. Equal/worse degrees retain a bounded result and select the next
lead. A finite yield/degree example does not establish stationary yield,
intrinsic regularity, general ECDLP hardness, GHS or a measured speedup.

The actual shared Conductor check still exits127; no reservation or substitute
coordinator is claimed. The lead atlas was consulted before this protocol;
this advances the existing closure-degree/MutantXL lead, not an invention claim.
