# Koblitz cover transfers:Round23 exceptional lifted geometry

2026-10-26–27,America/Los_Angeles. This report records exact computations and
a written geometric derivation;it does not make an official experiment
promotion,IC speed assessment or new relation-rank claim.

The two nonrational exceptional base supports now have explicit local source
certificates. SS5 has four degree20 closed points,all unramified. Z5 has two
degree10 closed points with ramification indices1 and7. At the second Z5 point
the two final registered polynomial denominator entries have orders4 and6.
The full proof,source/component hypotheses and exact equations are in CM.md.
These are local results on the original incidence function field;the complete
global divisor/RR construction and the F2 modular-factor transfer remain open.

## Exact observations and derivation

- **SS23 compact native certificate.** The actual polynomial h times Q2 lies in
  (F,Q0,Q1),with3 literal coefficients and exact zero residual. On SS5 h is U,
  with an explicit inverse modulo the original two quartics. Both Tate residual
  factorizations and the cofactorA=1 specialization are checked. The original
  relative Jacobian is a unit. The restored R22 finite-etale/Frobenius record
  then supplies the four residue fields of degree4 overF32. Proper degree16
  accounting exhausts the normalized fiber;the primitive lambda equation L0^4
  is not used as its ramification multiplicity.
- **Z23 complete components.** The unsaturated original fiber splits as2+18;
  the saved saturated closure splits as2+14. Exact intersection and comaximality
  checks pass. Both residues have degree2 overF32. All8 actual registered unit
  entries are visited,including the last entry previously missed by size(ideal).
- **Z23 source regularity.** Differentiate all51 closure generators before
  specialization. A minor of rows8,1,2 and columns(lambda,b,c) is a unit at each
  residue. This supplies regular local curve rings. The DVR length formula
  identifies e=1,7 and the weighted total16 excludes extra proper branches.
  Quotient dimensions8,12 give orders4,6 for S,N at the ramified point.
- **Generic nu denominator.** The source nu is a unit at both exceptional
  supports. Its generic denominator A²*b³*delta has lifted orders2(SS5),1 and7
  (Z5);the recovery numerator has the same orders. These are cancellations in
  generic elimination,not poles of the retained coordinate.

| Proper fiber | Closed points overF2 | Ramification | b,c,lambda,nu orders | A,delta,S,N orders |
|---|---|---|---|---|
| SS5 |4 of degree20|1,1,1,1|all0|1,0,0,0 at each|
| Z5 P1 |1 of degree10|1|all0|0,1,0,0|
| Z5 P7 |1 of degree10|7|all0|0,7,4,6|

This table is complete over the two indicated base supports. It is not a table
of complete global principal divisors. The five rational base supports still
require lifted branch equations and all coordinate/denominator valuations.

## Reproduction and verification

The retained native runs are ss23-compact-host2-08,z23-components-host2-07,
z23-smooth-host2-06 and ss23-unit-host2-09. SS23 compact-host2-01 is an earlier
successful recovery check with fewer Tate-factor assertions. Each invocation
uses Singular4.4.1,4GiB virtual memory and1200s maximum,with an S3 source
checkpoint before launch. REPLAY_NATIVE.sh is thin invocation/storage;all
arithmetic and certificates are native. No statistical sampling,seed,blinding
or timing comparison applies to these fixed exact identities.

REPLAY_LEAN.sh rebuilds all transitive project imports from this checkout with
Lean4.34.1 and pinned mathlib d13f23b. The final module has15 theorem declarations.
Its transitive axiom audit reports0 failing and0 trusted-native results. The
actual SS5 universal integer lift and characteristic2 inverse are kernel proofs.
The full scheme/normalization/DVR bridge remains written mathematics;conditional
arithmetic lemmas expose it rather than claiming it has been formalized. See
PROOF_MAP.md and the retained compile/audit output for the exact scope.

The source-bound rejection control ss23-corrupt-host2-10 adds1 to the actual
inverse. Singular emits the expected nonzero unit residual and error,then
continues to print its later success marker with native exit0. The wrapper
rejects it with exit1. Thus exit0 and a success marker alone cannot pass this
certificate workflow.

The native input model and every restored R22 raw file were verified by SHA256.
No completed G32/G33 integral recovery or earlier normalization failure was
repeated. Full root software validation has its own status in VALIDATION.md;
a research certificate is not substituted for passing software tests.

## Failures and migration reconciliation

Checkpoint10 recovered all9 source commits after another Spot host loss.
The old raw logs were gitignored and absent from that checkpoint. The selected
compact certificate survived in Git;missing verification receipts were replayed
on the new host. Full old quotient output and failed-attempt logs remain marked
lost,with the observed failures and exact source history retained in FAILURES.md.
The new wrapper archives raw output after each call so following checkpoints
include it. The working checkout and logs now live under /home/ec2-user.

GTZ's characteristic2 loop,c2 name shadowing,wrong-ring differentiation,a missed
last unit entry,and the stalled minor driver are recorded as failures or scope
defects. Their corrected successful outputs are separate. No unsupported first
attempt or exit0 alone is counted as a valid certificate.

## Next construction step

Use the same original-coordinate method at A0,B0,C0,D0 and infinity:construct
appropriate finite charts,certify their regular local rings and proper degree
accounting,and evaluate every registered denominator there. Combine those local
divisors with the now fixed SS5/Z5 orders to construct a complete prescribed
Riemann–Roch space. An actual factor map/correspondence must still satisfy the
function-field identities,F2 descent and nonzero order11 action. Shared residual
Hecke data alone do not supply that subgroup check.

Node187 still needs a source-bound polynomial separator/membership certificate
or a complete simultaneous arc. Full quotient saturation and a proper polarized
embedding remain separate. Direct relations still require arbitrary modular-
divisor reverse evaluation,including ordinary460/367 fibers/images. Existing
Klein n=3 and same-field n=5 regression controls are preserved;they are not
reported as the F2 modular-factor acceptance test. The user assesses faster IC.

## Sources and visuals

The immutable Round22 CM,PROOF_MAP,SOURCES and restored raw source circuit and
closure are the inputs. Primary references read:Stacks
[normal/proper curves](https://stacks.math.columbia.edu/tag/0BXX),
[etale Jacobian presentations](https://stacks.math.columbia.edu/tag/03PA),and
[degree/fiber accounting](https://stacks.math.columbia.edu/tag/0AYQ).
Installed Singular4.4.1 elim.lib/primdec.lib define the complete quotient,
saturation,radical and matrix APIs. No novelty claim or substitute independent
modular-model identification is made.

round23.svg is an editable vector diagram and exact local-dimension graph.
Its counts come from the retained native runs,not timing samples or estimates.
report.typ/report.pdf present the same scope. The export/manifest links the
frozen source frame to retained raw results and verification receipts.
