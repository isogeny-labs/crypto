# Root-free Weil evaluation: frozen correctness protocol

Previous turn: progress, explicit two genus3 kernels published at da5c219bafc3,
CI passed. No scientific process running. Preserve full genus2/3/4 10x objective.

Sources checked: prior torsion13 protocol/results/source and dated session log.
KB revision9141e386254340a9bdba3036f2bb706a ready lexical, last refresh
2026-11-14T21:44:48Z predates current studies. Two queries about Tian Weil
functions and Frobenius orbit descent returned internal adjacent notes, not
validation of this construction. Primary Tian https://arxiv.org/html/2007.03172
Section2.1 Eq2.2 and Section5 read. These give determinant Weil functions and
regularity requirements; normalize separately, retain sign eta_P=f_{-P}.
Established Riemann-Roch/Hensel/determinant arithmetic, no novelty claim.

For a stored nonzero 3-torsion P=(u,v), require deg u=3, u squarefree and
coprime to v. Lift y=v modulo u to w modulo u^3 with Newton/Hensel.
Solve the 9x7 homogeneous system a+b*w=0 mod u^3 with deg a<=4, deg b<=1.
Require nullity1, normalize first nonzero coefficient to1. Set h=a+b*y.
Verify exact a^2-f*b^2=c*u^3 with c nonzero, and both vanishing congruence
and pole order9. Hence div(h)=3E-9infinity. Reject unsupported degree or
ramification rather than silently changing input.

With theta=2infinity, D_P=E+2infinity has basis
1,x,(y+v)/u, while D_0=5infinity has basis1,x,x^2.
For evaluation divisor X=(U,V) degree3 and squarefree, require gcd(u,U)=1.
Set r=(V+v)/u modulo U; determinant ratio is r's x^2 coefficient.
Then eta_P(X)=r_2^3 * Norm_{K[x]/U}(a+b*V). K=F(127^13).
Compute norm from multiplication-matrix determinant. No branch-point roots
or extension beyond K are constructed. This is unnormalized eta, not a
quotient theta section or quotient map. Input zeros/poles explicitly counted.

Frozen gate:
- Consume the existing54point certificate and pinned original curve fixture;
  identity h=1, all52nonzero inputs must meet generic conditions or report
  unsupported cases; do not regenerate or choose new kernel points.
- Exact principal-function identity for every h; Frobenius propagation of
  canonical h through both kernels; one-coefficient corruption rejected.
- On first16distinct base-field affine places (ascending x, canonical y),
  use first16lexicographic distinct-x triples. For both kernels and every h,
  compare quotient-algebra evaluation against direct determinant/Vandermonde
  and product at all triples. Retain zeros and skip counts, no replacements.
- Verify conjugation eta over Frobenius at every triple and cyclic orbit sums
  land in F127; these sums are only arithmetic descent checks, NOT invariant
  under kernel translation or proven quotient functions.
- On first8public multiples of D1, compare norm determinant with polynomial
  resultant recurrence, retain unsupported/zero/pole cases. DLP answer unused.
- Reject public D1 as 3-torsion and corrupted h; controls include identity,
  norm of1, constants and U evaluated against itself (zero).
600second owned watchdog. Rust science only. No new timing comparison,
performance record, prior closed-record/default/leaderboard changes.
Commit/checkpoint protocol and source before executing frozen gate; retain
raw/source/binary hashes. Report source/sign/normalization/regularity limits.
Next dependency: normalized translation cocycle and invariant theta section,
then same-field quotient, transport and complete costs; other genera retained.
