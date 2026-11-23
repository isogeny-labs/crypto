# Matched regularity-halving goal, 2026-11-17

Owner request: achieve a 2x reduction in degree of regularity for the
P-256 factor-base relation systems. Tracking: draft PR4555. This protocol
does not replace the depth8, m=2–7 runtime experiment in ../PROTOCOL.md.

## Fixed acceptance and terminology

Primary algebraic metric: D(J)=reg_S(J), the Castelnuovo–Mumford
regularity of the **ideal module** of a nonzero proper homogeneous ideal
J in a standard graded polynomial ring. Report reg(S/J) separately;
D(J)=reg(S/J)+1. Homogenize the stated affine generators with one common
variable H, after the same explicitly named affine-linear preprocessing
on both arms. Retain generators' unsaturated homogenization, saturated
projective closure, and affine solving degree as distinct objects.
Use a minimal graded free resolution, or an exact complete-intersection
certificate, to certify D. A leading-ideal bound alone is not equality.
The first zero Hilbert degree after a linear section is usable only when
the section is proved regular and the quotient is Artinian.

Full-goal success requires 2*D_candidate <= D_baseline on the frozen
nontrivial complete relation workload, with matched factor base, target,
ordering/sign semantics and exact replay, and an explanation of added
variables and any extension-field components. Keep the actual retained
abscissa dense presentation as a challenger. Exclude a redundant sign
cover, a single-column instance, a solved relation truth table, or an
inflated full-fibre control from sole evidence for full-goal success.
Do not identify D with F4 maximum matrix degree or wall time.

## First proof-development obligation (no performance experiment)

Reuse the existing deterministic depth1,2,4 constructor and prior fixture
identities, including depth4 FB1h2704fe49495e. Prove/check the auxiliary
membership block separately from the complete relation ideal. A tower
step at infinity has form

    (X_next-X_i)*(X_i-r_i*H)-delta_i*H^2,

with X_next=c*H at its terminal step. Under lex X_0>...>X_(d-1)>H,
the monic equations should have pairwise coprime leading squares X_i^2.
Check every S-pair, homogeneous degrees, nonzero deltas, distinct full
fibre roots, and exact composition/product equality. Emit the Koszul
graded shifts, Hilbert numerator, quotient regularity and ideal regularity.
Also examine the block with the target cubic, explicitly checking the
height/linear-section hypotheses. Test corrupt degrees, a repeated
leading square and a zero delta; fail closed rather than print a bound.

Fairness control: compare the full fibre to the **retained** abscissae in
the existing FB1 dump. Record every excluded abscissa and its target
quadratic character. Full-fibre and retained ideals can agree on Fp
points while differing over the algebraic closure. A membership-block
certificate, even one exceeding 2x, does not satisfy the full goal.
No relation is reported from this component unless exact target group
replay is performed; no relation-search or performance claim is planned.

## Next obligations and accounting

After the component verifier, freeze the complete-system m=2–7 cells,
finite planted targets and deterministic unplanted targets, exact source
and input hashes, CAS budget, resources and stop conditions **before**
any new solving experiment. Keep original depth8 scope open if blocked.
Preserve all failed/timeout cells. Record total variables, equations,
coefficient/term volume, preprocessing, cover multiplicity, denominators,
replay, and complete-reference coverage. Auxiliary subgroup smoothness
does not transport P-256 logarithms.

Local builds, exact symbolic unit checks and analysis of retained fixtures
are development work. Performance/parameter-search experiments use the
existing EKS admission route. No new CAS API submission, cloud spend or
unplanted full-depth relation attempt is authorized by a passing component.

## Sources checked before choosing this obligation

KB review unavailable: no exposed kb_status/search_knowledge tools;
scripts/research-kb status previously exited127. Fallback: ../REPORT.md,
../aux_base.cpp, ../continuation_20261116/PROTOCOL.md and its 16-cell
CAS-unit matrix, and the prior literature review of Petit–Kosters–Messeng
(PKC2016), https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf .
Auxiliary-isogeny bases and quadratic chains are existing methods; no
novelty claim follows from this protocol.

Official Magma free-resolution manual (read2026-11-17):
https://docs.magma-maths.org/v2.30-1/CommutativeAlgebra/ModulesOverMultivariateRings/free-resolution.html
uses QuotientModule(I), FreeResolution and Regularity(M). Its quotient
module convention is kept distinct from ideal-module regularity.
Macaulay2's regularity documentation:
https://macaulay2.com/doc/Macaulay2/share/doc/Macaulay2/Macaulay2Doc/html/_regularity.html
distinguishes the ideal and quotient conventions. Native Koszul certificates
will supply the component equality; the manuals do not prove a reduction
for the complete P-256 relation systems.
