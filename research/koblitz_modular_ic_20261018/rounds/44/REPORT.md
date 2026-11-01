# R44: source orientation is an additional obstruction to ordinary transfers

This is an observations report with explicit algebraic/geometric deductions,
not an independent evidence verdict or IC efficiency assessment. The task
continues on the n=5 carrier. Large ordinary residue fields here are not
larger-n Koblitz curves or evaluated cryptographic-scale transfers.

## What changed

R30 correctly distinguished an individual F32 color component from the sum
of all five components in the scalar C5 kernel model. That distinction did
not settle the orientation involution on the full C61 source. R44 constructs
the missing source algebras for all two degree12 middles and the one degree436
middle, retaining the entire degree5 cyclic11 kernel and both orientations.
Their actual quadratic orientation trace is1. The source Frobenius/sign
identity then kills the mixed contribution of each F32 closed source divisor,
including individually colored components. Scalar color separation alone
therefore does not rescue these particular ordinary supports.

The exact finite computations are indexed by results.json. Complete tuple
coordinates, quotient equations, controls, timeouts and proof logs are kept.
The proper source/divisor interpretation still uses the retained moduli,
function-field and normalization bridge. R44 does not construct a new target
map or perform a global quotient saturation computation.

## Full finite-algebra source checks

Inputs are the literal accepted R30 compatible middles and second cyclic11
kernels; R30/R31 artifacts were restored and every member hash checked.
No accepted point producer was rerun merely to resume work. Kernel monicity,
separability and Rabin irreducibility are checked. The Tate normalization
transforms the actual cyclic7 kernel. The relative dimensions are5 for its
kernel field,10 for the source algebra,20 with the physical auxiliary T.

For every source block, all five ORIGINAL incidence equations and eight
registered unit inverses are checked on the entire quotient at all ten
actual diamond iterates. The fifth iterate is sign and the tenth identity.
Independent Tate group arithmetic verifies exact order11. Corrupt nu fails.
Neither a preferred kernel root nor an orientation branch is dropped.

The four degree12 blocks have full source tuple period120 and
Frobenius^12 diamond steps9,9,3,3 (two middles, two forms). The degree436
source field-generation certificate distinguishes the actual coordinate
field from its ambient algebra dimension. Exact b,c generation gives degree2180;
AS irreducibility then gives source tuple degree4360. Source-block rows and
both forms' exact Frobenius steps are in results.json; no model identity is
inferred from a matching degree alone.

Over F32, a degree120 source orbit has five components of degree24; a
degree4360 orbit has five components of degree872. These are SOURCE degrees.
The labels367/460 describe base supports. They cannot be substituted as
source divisor degrees to justify padding into R43's degree512 setup.

## Why the mixed ordinary contribution vanishes

Write F for absolute Frobenius, T for the actual diamond and sigma=T^5.
The checked source equality is

    F^(5d)(P)=sigma(P),   d=12 or436.

Since F32 Frobenius is F^5, sigma preserves each F32 closed point containing
P. Diamond commutes with Frobenius, so all its translates are also sign-fixed
as CLOSED DIVISORS. Therefore

    E[P]=(1-sigma)(5-N5)[P]=0.

This is a formal-divisor cancellation on the bound source, stronger than
base E49 coincidence. It needs no Picard solve or claim that a nonzero formal
vector is a nonprincipal class. Any F32-rational divisor supported on those
actual source components has zero mixed contribution, independently of its
integer multiplicities. This does not account for every other ordinary or
exceptional term of a collector's complete principal divisor, or supply an
IC relation rank. The collector/moduli source binding remains the written
bridge named above.

The odd-constant descent lemma also explains the retained83/284 controls:
if F^d is sign and F^(2d) fixes a point, then F^(5d)=F^d on its orbit.
A quadratic orientation that remains nonsplit cannot be opened by a constant
extension of degree5. Every one of the20 retained source blocks agrees with
the new trace screen: original83 has trace1, its diamond twist trace0;
both284 forms have trace1. Trace0 is eligibility, not proof of nonzero class
or independent column. The surviving83twist orbit was already bound to the
R33/R37/R39 class data; rediscovering it creates no new relation.

## A cheaper orientation screen and its exact certificate

Let A=1+c, r the selected11-generator abscissa, and c2 the original7-kernel
quadratic coefficient. The normalized orientation constant satisfies

    C=(b+g2)/A²=c2+delta,
    delta=b/A²+r=b/A²+b/A³.

Every identity is checked on the actual kernel field. Native traces computed
separately give orientation1=Tate-twist1+seven-kernel0 in all three fixed
middle cases. The trace audit is independent of the full source period scan.
This suggests screening the remaining500/580-support families before building
large divisor spaces. It is not a point-counting novelty, smoothness estimate,
map construction or performance claim.

## Algorithms, controls and Lean scope

Quotient-first evaluation reconstructs all13 original/registered polynomials
in the original binary ring, then reduces powers/products before substitution
expands. Its values agree with direct evaluation on all four degree12algebras
(52 comparisons). Multiplication-matrix inverses match all32 old Groebner
inverses; every original matrix row and actual quotient product are checked.
Exact b,c period certificates agree with the old exhaustive degree12scan.

For y²+y=C, rho0=0,rho(k+1)=rho(k)²+C gives

    y^(2^k)=y+rho(k).

This supplies the actual source Frobenius map from the kernel Frobenius image,
with beta fixed after d steps. All16 coordinate comparisons and four fifth-map
sign comparisons agree with direct powering on the accepted degree12algebras.
The final436completion checks the actual quotient map and all ten diamond
comparisons, plus five map compositions against sign. Its source-equation and
unit checks and base-generation proof come from the complete stages of the
third capped run; the capped run itself remains partial, never labelled pass. Field generation and
AS irreducibility certify the period; ambient dimension is not assumed to be
coordinate degree. These are exact alternative certificates, not relaxed tests.

Lean covers seven named mathematical theorems plus four generated equations:
closed-orbit sign fixation/mixed cancellation; cleared orientation identity;
quadratic Frobenius recurrence; odd constant descent. All accepted modules
use Lean4.34.1/mathlibd13f23b,-j1-M4096/1200s and audit zero failing/trusted.
Native finite arrays, matrix ranks, finite-field generation and proper geometry
are not formalized by these generic lemmas. Independent semantic review and
independent scientific replay remain pending.

## Retained failures and next work

The initial degree12 procedure used a reserved Singular name and printed a
late marker after errors; strict checks rejected it. Three full436implementations
hit the unchanged1200s cap. The third completed both source-geometry blocks
and the first full Frobenius/period block; a separate receipt completes only
the missing twisted Frobenius check, reusing and hashing those finished stages. They are computationally censored, not mathematical
refutations. The first Frobenius control accidentally nested a list and was
strictly rejected; its repair passed fresh controls. One Lean import was
unavailable and was replaced by the pinned ring tactic. All attempts remain,
with source fixes committed before fresh runs. No accepted producer reran,
no cap increased and no duplicate live436producer was launched.

Next: apply the orientation screen to the remaining ordinary supports while
retaining every middle, kernel color and descent conjugate; bind survivors to
actual closed divisors and full class evaluation. The proper level77chosen
Koblitz-model map, full quotient saturation/polarization, theta and node187
coupled valuations remain separate obligations. R43's complete larger spaces
remain useful setup, not a replacement for actual fiber degrees/branches.

Sources and primary links are in LITERATURE.md; exact formats in FORMAT.md.
Native caps remain4GiBvirtual/1200s. No cloud job, managed-generator/freeze
restart, new claim/agent or PR merge was performed. No speedup, unknown-scalar
recovery or evaluated larger-n transfer is asserted.
