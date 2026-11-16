# R32: exact local incidence rank at the R31 divisor candidates

Observations only, without an IC, Picard or proper-model verdict. The persistent
research goal continues. R31's completed raw archive is not rewritten.

Parameter distinction: the ordinary support degrees83/284 belong to the
existing level77, n=5 transfer lane. They are NOT new Koblitz extension
degrees83/284. R31's larger CM character targets n83 through571 form a
separate lane with explicit geometric maps still absent.

## Native results

All20 accepted R31 quadratic point algebras pass the corrected local rank
calculation: five cyclic11 generators, two descent forms, degrees83 and284.
The Jacobian is the5x4 matrix of derivatives of the ORIGINAL five source
equations, differentiated before evaluation in the physical source coordinates.
At every algebra all five4x4 minors vanish. The forty3x3 minors, together with
the defining residue-algebra equations, have an explicit exact unit identity.
The entire evaluated matrix, generator vector and lift witness are retained.
The zero-minor-family negative control stays proper. No branch is removed.

Thus the matrix has rank3 at every geometric point of each finite algebra;
the presented tangent space has dimension1. This checks a local prerequisite
at the candidate divisor support. It does not prove local scheme dimension1,
the normalization, global proper carrier, nonprincipality or modular-factor
image. In particular, the R31 degree83 twist formal divisor is still only a
candidate for proper Picard arithmetic.

## Failures, correction and controls

Two pretrial invocations, jac83-01 and jac284-01, were refused by the durable
gate because the condensed resume note lacked required headings. Neither
reached CAS. The complete handoff was restored; the gate/helper was unchanged.

jac83-02 reached CAS, but the runner rejected its unit-witness reconstruction
for all ten input algebras. Singular returned0 and printed a later terminal
marker; the runner's error-line gate correctly rejected the run. This was
not accepted geometric evidence. The reconstruction loop used size(ideal),
which counts nonzero generators and truncated coefficient positions. The
corrected logs show26 nonzero generators occupying42 slots. Full native
matrix(generators)*witness preserves all slots and is required to equal1
EXACTLY. The failed source and log remain intact. No acceptance condition,
minor list, mathematical input or resource cap was weakened.

jac83-03 and jac284-02 then pass: two accepted native runs, one rejected
native recorder attempt, two preserved pretrial gate refusals. Each accepted
run contains ten complete point-algebra checks. No failed attempt is erased
or counted as an accepted scientific trial.

## Formal and geometric scope

KoblitzMinorWitness32 proves generic specialization of an exact finite unit
identity under any homomorphism to a nontrivial commutative ring, and that
some minor remains nonzero on every such branch. The ring statement includes
field-valued branches. It assumes the unit identity; it does not import the
native matrices, establish Jacobian rank, or formalize local scheme dimension.
See the separately retained compile and transitive axiom audit.

Literature/dedup preceded implementation: R26–R31 notes searched for prior
local Jacobian rank checks, original R27 LAMBDA14.sing source and R31 accepted
point records read, and Stacks29.35/Lemma29.35.14 inspected. That lemma requires
local-dimension/presentation hypotheses, so tangent rank alone is deliberately
not promoted to full smoothness or a proper normalization certificate.

https://stacks.math.columbia.edu/tag/01V4

Native4GiB/1200seconds each; pinned Lean/mathlib,-j1-M4096/1200seconds.
No Python mathematical computation, EKS/Diem/freeze restart, new coordination
claim, cap increase, merge or auto-merge. Shared session-log scope remains
reserved, so SESSION.md carries this work's handoff.
