#set page(paper: "a4", margin: (x: 17mm, y: 16mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: .55em)
= Koblitz cover transfers: Round24
*Explicit infinity fiber, original valuations and constant field* \
2026-10-28 UTC. Exact computations and written geometric derivations;
no IC cost, relation-rank or official promotion claim.

== A complete proper fiber from original source charts
For the unchanged degree16 incidence field over
$E: v^2+v=u^3+u$, the fiber above infinity consists of four rational
points, with ramification indices1,1,7,7. In the actual base chart
$t=u/v$, $q=1/(t^3 v)$, the equation is
$q+1+(1+t)t^3 q^2=0$ and O has t=0,q=1.

The degree8 intermediate tower is
$H(Z)=Z^8+t^11 d Z^2+A^2 t^11 d Z+t^14 e$,
with d,e,A units of residue1, followed by
$lambda^2+A lambda=Z+b$.
The unramified chart has $Z=t^3 y$. The tame chart has
$t=w^7 x^2$, $Z=w^11 x^3$, with exact inverse
$w=Z^2/t^3$, $x=t^11/Z^7$.
Native checks certify the original Tate/base substitutions, both tower
identities and four unit Jacobian residues. Smooth local rings identify
the normalization at these points; the proper degree sum1+1+7+7=16
excludes extra branches. The scheme/DVR argument is written in CM.md.

== Actual coordinate and denominator orders
#table(columns: (1.2fr,.5fr,.7fr,.7fr,.7fr,.7fr,1.5fr), inset: 4pt, stroke: .4pt+gray,
  [*Point*],[*e*],[*lambda*],[*nu*],[*S*],[*N*],[*nu numerator / denominator*],
  [O1,0],[1],[5],[0],[0],[0],[11 / 11],
  [O1,1],[1],[0],[0],[0],[0],[11 / 11],
  [O7,0],[7],[11],[32],[44],[66],[109 / 77],
  [O7,1],[7],[0],[21],[44],[66],[98 / 77])

All residue degrees are1; b,c have orders3e,2e and A is a unit. The eight
actual registered polynomial-entry orders are (3,2,2,2,5,2,0,0) on either
unramified point, and (21,14,14,14,35,14,44,66) on either ramified point.
S,N are the unchanged nn65,nn75 entries, with original circuit denominators1.
The local zero-divisor contributions above O are16 for lambda,53 for nu,
88 for S and132 for N. Global zero/pole completeness remains open.

O24-series-02 uses the certified unique chart lifts modulo w192, checks
the original nu coefficient and all five source residuals modulo w100,
and displays nonzero leading coefficients below100. Denominator division
leaves at least115 correct coefficients on tame branches; printed high
nu tails beyond that effective precision are not claimed as true expansions.

== The constant field is F2
The unchanged primitive plane equation stays irreducible of lambda degree32
over F4, with exact product, separability and field controls. The constant
degree divides16 and10, hence is1 or2; nonsplitting excludes2. The rational
source points give a second written proof by evaluating global constants
into F2. Proper geometric normality and H0=F2 imply geometric integrality
(#link("https://stacks.math.columbia.edu/tag/0FD3")[Stacks33.10.7]).
This does not identify an independent modular model or evaluate a factor map.

#pagebreak()
= Evidence, proof scope and remaining construction
#image("round24.svg", width: 100%)

== Retained verification and failures
Six native calls:3 accepted positives,2 rejected implementation failures,
1 expected rejection. The corrupted actual factor product is rejected despite
native exit0 and a later success marker. The failed map syntax and incorrect
order helper remain in FAILURES.md with all raw output. Explicit valuation
controls distinguish a unit, order1 and order7 before accepting the series.

Lean02 rebuilds all transitive project imports and audits14 new theorem
declarations:0 failing,0 trusted-native. Lean01's elaboration failure is
retained separately. Exact chart identities, rational inverses and residue
algebra are kernel proofs. Proper normalization, Hensel lifting, divisor
interpretation and geometric constant-field identification remain written
bridges, exposed in PROOF_MAP.md.

The temporary archive ignore initially excluded raw bytes from checkpoints;
it is corrected, and a fresh checkpoint download confirms inclusion. No
migration occurred during that defect. Final raw archive/source pins and
export are separate immutable publication records. Root software and PR CI
statuses are reported separately; no full green root suite is inferred.

== Next discriminating construction
Resolve A0,B0,C0,D0 using source-bound finite/scaled charts; the successful
O substitution is a lead to test after exact coefficient translations,
not an assumed answer elsewhere. Enumerate additional zero supports by
norms/source fibers. Complete divisors must precede a complete prescribed
RR basis and candidate map. Acceptance still requires function-field
identities, F2 descent and nonzero order11 action.

Node187 source-bound membership/complete coupled arc, full quotient
saturation/proper polarized embedding and arbitrary reverse-divisor
evaluation with ordinary460/367 fibers/images remain open. The n3 and
same-field n5 controls and exact norm-zero recovery d=1 are preserved.
No new EKS job, duplicate Diem, fifth claim or PR merge is authorized here.

Reproduction: PROTOCOL.md, protocol.json, REPLAY_NATIVE.sh,
REPLAY_LEAN.sh, results.json, CM.md and source/raw manifests. The editable
SVG plots recorded exact orders at four points; there is no timing or
statistical comparison. Native cap:4GiB virtual memory and1200s per call.
