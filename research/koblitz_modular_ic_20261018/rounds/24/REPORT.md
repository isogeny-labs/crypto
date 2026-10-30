# Koblitz cover transfers: Round24 infinity fiber and constant field

2026-10-28 UTC. This records exact computations and written geometric
derivations, with their formalization scope. It makes no official experiment
promotion, IC cost assessment or new relation-rank claim.

The proper fiber above infinity on the retained incidence field now has
explicit source charts and complete original-coordinate orders. It consists
of four rational points over F2 with ramification1,1,7,7. The full constant
field is F2 and the proper smooth source is geometrically integral under
the written normalization bridge. This does not identify an independent
level77 modular model or construct its factor map.

## Observed certificates and written consequences

CF24-factor-01: the unchanged R22 plane polynomial has one irreducible
lambda-degree32 factor over F4. Its exact product, separability and field
controls pass. O24-charts-02: actual base substitutions, transformed degree8
tower, toric rational inverses and four unit Jacobian residues pass.

The unramified chart has Z=t^3*y. The tame chart is t=w^7*x^2,
Z=w^11*x^3, with exact inverse w=Z^2/t^3,x=t^11/Z^7. Every chart has
lambda residues0 and1. The original generic field and these smooth local
charts identify the normalization locally. The degree sum1+1+7+7=16
exhausts the proper fiber. This is a written curve/DVR argument, not an
interpretation of a repeated projected factor. See [CM.md](CM.md).

O24-series-02: all four unique smooth-chart lifts are checked modulo w^192.
The original nu coefficient is A^2*Delta, and all five original source
residuals vanish modulo w^100 at every branch. Every reported valuation
has a nonzero leading coefficient1 below100.

| Point above O | e | ord(lambda) | ord(nu) | ord(S) | ord(N) | nu numerator/denominator |
|---|---:|---:|---:|---:|---:|---|
| O1,0 |1|5|0|0|0|11/11|
| O1,1 |1|0|0|0|0|11/11|
| O7,0 |7|11|32|44|66|109/77|
| O7,1 |7|0|21|44|66|98/77|

All points have degree1. The complete eight registered polynomial-entry
orders are(3,2,2,2,5,2,0,0) on either e1 branch and
(21,14,14,14,35,14,44,66) on either e7 branch. The original coordinates b,c
have orders3e,2e. S,N are the unchanged nn[65],nn[75] entries, with circuit
denominators1. The recovered nu coefficients on the tame branch are valid
through at least order114 after denominator division; high printed tails
are not interpreted as true coefficients.

Derived local zero-divisor contributions above O are16 for lambda,53 for nu,
88 for S and132 for N, summing the displayed degree1 orders. These are
local contributions, not complete global zero divisors. A rational source
point injects the global constant field into F2, giving a second written
proof of the constant-field conclusion. Geometric integrality then follows
from proper geometric normality and H0=F2; primary reference
[Stacks 33.10.7](https://stacks.math.columbia.edu/tag/0FD3).

## Validation, failures and flow

Six native invocations: three accepted positive runs (cf-factor01,
o-charts02,o-series02); two rejected implementation failures (o-charts01,
o-series01); one expected rejection (cf-corrupt01). The corruption changes
the actual factor product by1 and is rejected despite native exit0 and a
later success marker. No unaccepted run is used as a valuation certificate.

Two Lean compilation attempts: lean01 fails elaboration and is retained;
lean02 rebuilds every transitive project import, compiles14 new theorem
declarations and passes the transitive audit with0 failing and0 trusted
native results. The actual chart identities and both rational inverses are
kernel proofs. The full geometric/constant-field/Hensel bridge remains
written; [PROOF_MAP.md](PROOF_MAP.md) lists every formal hypothesis.

All native invocations preserve the inherited local4GiB/1200s cap. There is
no new EKS job, Diem launch, waiting-freeze action or Conductor claim. The
source checkpoint and raw retention defect was corrected before further
runs; a fresh downloaded checkpoint is verified to contain the raw archive.
No migration occurred during that defect. The full [failure ledger](FAILURES.md)
and raw output remain available, including the failed source snapshot.

Root software validation and remote CI have separate statuses in
VALIDATION.md; native math/Lean success does not mean the full root suite or
all PR checks pass. R01-23 evidence remains immutable. Round22's exact
integral equivariant recovery d=1 and the n3/same-field n5 controls are not
rerun or presented as the F2 modular acceptance test.

## Remaining construction and new lead

Four finite rational polar supports A0,B0,C0,D0 remain. Use the compact
degree8 tower with actual coefficient orders and coordinate translations
to seek similar explicit regular charts. This is a proposed extension;
the O transformation is not assumed valid at those points. Additional zero
supports still require exact norm/source-fiber enumeration. Only complete
global divisors permit the prescribed complete RR construction.

Any resulting factor map/correspondence still must satisfy function-field
identities, F2 descent and nonzero order11 action. Node187 polynomial
membership/complete coupled arc, full quotient saturation/proper polarized
embedding and arbitrary modular-divisor reverse evaluation with ordinary
460/367 fibers/images remain open. The user assesses faster IC.

The editable [diagram/valuation graph](round24.svg), [PDF](report.pdf),
protocol, native scripts, results.json, source pins and export provide the
reproduction record. The graph plots exact recorded orders at four points;
there is no sampling or timing uncertainty to estimate.
