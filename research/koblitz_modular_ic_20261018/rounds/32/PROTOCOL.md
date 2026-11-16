# R32 prospective local incidence rank diagnostic

Conductor scope clear; existing local authorization/caps carry forward. Do not
restart external jobs or rewrite R31's sealed archive. Literature/dedup first:
read Stacks29.35, especially Lemma29.35.14, at
https://stacks.math.columbia.edu/tag/01V4 . Smoothness needs the local-dimension
hypothesis or a suitable standard-smooth presentation. A rank calculation
alone will NOT be called a completed normalization or smooth proper model.
Searched R26–R31 notes for earlier Jacobian-rank checks; no matching completed
check found. Original source is unchanged R27 LAMBDA14.sing, first254lines.

Inputs are all20 accepted R31 REVERSE31 quadratic point algebras: five
generators, two descent forms, two fields. Retain every factor via the full
quotient (L(Z),T²+T+1); the original physical coordinates include the AS shift.
Differentiate the five ORIGINAL equations in their four source variables,
then evaluate. Require all five4x4 minors to vanish modulo the quotient.
Require the forty3x3 minors jointly generate the unit ideal with the quotient;
retain a lift witness and verify its exact reconstructed identity. Record
the entire evaluated Jacobian and witness. No factor or denominator discard.

Reference controls: R31's original source/residual/unit checks already pass;
do not recompute their kernel search. Positive control is a verified unit
minor ideal in every algebra. Negative control is the zero minor family:
its ideal must stay proper. Any mismatch, error or cap is a retained failure,
not a mathematical impossibility. Exact exhaustive enumeration, no sampling,
timing comparison or statistical/IC claim. Native4GiB/1200seconds per field.

Acceptance is rank3 at every geometric point of these finite algebras, hence
one-dimensional presented tangent space. The proper normalization, local
dimension of the incidence scheme, nonprincipality and intended modular
factor image remain separate. Lean may certify generic unit-witness
specialization, but native matrices are not thereby imported into Lean.

## Pretrial refusal and infrastructure correction

jac83-01 stopped at checkpoint gate before any source copy/CAS invocation:
the condensed .resume/RESUME.md lacked seven required handoff sections.
Restore the complete handoff, not the gate/helper, and retry the unchanged
frozen input with fresh jac83-02. Preserve jac83-01/gate.log. No failed
scientific trial or altered mathematical acceptance criterion is inferred.
The sequential shell also reached jac284-01's checkpoint gate and was refused
for the same missing sections, again before CAS. Preserve both refusal logs;
the corrected field284 invocation must use fresh jac284-02. No source/math
retry was needed because neither original invocation reached the native CAS.

## Retained native recorder failure and exact correction

jac83-02 reached CAS and failed the explicit unit-identity reconstruction for
all ten algebras; the runner rejected it despite Singular exit0 and a later
terminal marker. No accepted rank result is inferred. Its original input/log
remain untouched. The scalar loop used size(ideal), which counts nonzero
entries and can truncate generator coordinate slots when minors vanish.
Replace only reconstruction by native matrix(generators)*witness, preserving
zero slots. Require exact equality1 as before and record both dimensions.
Fresh jac83-03 and jac284-02 use that correction; unchanged rank/source data,
controls and caps. The second field had not run after the failed first field.

## Prospective Lean witness specialization

KoblitzMinorWitness32 proves that an exact finite unit identity survives any
ring homomorphism into a nontrivial field and some minor remains nonzero on
every such branch. It does not import native matrix entries, prove Jacobian
rank or establish the incidence scheme's local dimension. Same pinned Lean,
mathlib,-j1-M4096/1200seconds and transitive standard-foundations-only audit.
