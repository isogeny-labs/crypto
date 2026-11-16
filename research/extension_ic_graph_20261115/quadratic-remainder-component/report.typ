#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
#set heading(numbering: none)
= Quadratic remainder relations over Fp²
Library follow-up PR4452 after PR4288 · 16 November 2026

The opt-in degree-nine extractor keeps seven signed small-point classes and
a quadratic remainder class over Fp². The quadratic must be irreducible over
Fp. The existing split-support collectors and their defaults are unchanged.
Producer observations and source arguments have unverified PCC standing.

#image("flow.svg", width: 100%)

== Algebra and checks
The actual source convention is A=A0+t·s·A1 and C=sqrt(s)·B. The extractor
requires F=A²−C²H exactly, a smooth monic degree-seven H, deg(A)≤4,
deg(C)=1, a squarefree degree-nine norm, seven distinct small roots, and a
monic irreducible quadratic U. Extended polynomial GCD computes C inverse
modulo U, then V=−A/C modulo U. Both V²=H modulo U and the original full
Jacobian identity are checked. These operations retain the existing hooks;
there is no claim of complete native cost coverage.

The pole order of A+C·y is nine: C·y has order nine, while A has order at
most eight. The squarefree norm and nonzero denominators select one zero
over each support abscissa. D=(U,V) has degree two, at most genus three, so
it is already reduced. This explains the small-point sum plus D. Explicit
Cantor addition must also give the identity.

The bounded graph adapter keys D versus −D, retaining orientation as a
coefficient. It rechecks cover/source binding and every original group
identity before filtering, then replays each emitted input combination.
The scalar modulus and cofactor projection retain the existing caller
contract. These homogeneous rows supply no target descent by themselves.

== Evidence and limits
Final-source03 release checks report19 passed, including4 new controls.
The actual p251/seed2 cover gives U=x²+146x+78 at B-index30, s147; the old
Fp-split extractor refuses it. Source hashes match after the run.
PCC-20261116-2329b3 remains unverified. The first development run retained
a failed zero-row assertion; the corrected test checks zero added rank and
replays the emitted zero equation. The superseded02 build was canceled.
See the README and unchanged validation logs. Constructed
controls are not samples of natural collision yield. Opposite copies of one
relation provide no new rank. A future frozen matched collection must retain
fully smooth rows, count unique classes and useful rank, and charge all
extraction, validation, filtering, descent and final replay. No benchmark,
cloud job, throughput estimate or end-to-end gain is reported.

Release library Clippy passes with warnings denied. The broad parent suite
hits its120-second bound; six device controls pass after their tracked source
is restored. Historical manifest and PR artifact-access gates remain unresolved.
PCC-20261116-c9e4c3 is unverified; no whole-parent-suite pass is claimed.
Concurrent prototype stage40 has its own rank record; these library checks
do not reprice it. Canonical integration claim PCC-20261116-dfda61 preserves
the original development records and remains unverified.

== Sources
Existing quadratic remainder lead and exact native sieve/Cantor/filter code.
GTT February2005 draft, sections2/3:
#link("https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf")[corr2004-29.pdf].
JV source convention follows the committed sections3.2/3.3 review; a fresh
2011/020 PDF request returned403. KB MCP and wrapper unavailable.
