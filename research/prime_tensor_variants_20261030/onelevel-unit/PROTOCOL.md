# Frozen development protocol: one ternary contraction call

Frozen before native arithmetic implementation, 2026-10-30 UTC.

This is a finite exact-arithmetic development unit. It is not an ECDLP solver,
benchmark, asymptotic-parameter admission test, general contraction certificate,
or novelty claim. There are no timing acceptance criteria or performance metrics.
All work and artifacts are confined to this directory.

## Sources read before choosing the implementation

- `/workspace/audit/tensor-primary/openai-oct25.txt`, Appendix D.4.1--D.4.7:
  coefficient extension and entrywise canonical lifts, Cauchy charts, Hermite
  formula D.9, one-source/one-target mode factorization and direct leaf semantics.
  The source is the pinned October 25 manuscript already recorded by the parent.
- `/workspace/audit/next_ternary_packet_review.md` and
  `/workspace/audit/next_ternary_precision.md`: corrected target-specific ternary
  modes and conservative three-adic precision. In particular a source tile may
  cross to TWO target groups; an unsplit source variable is invalid.

## Fixed instance and exhaustive panel

Use one packet with M=L=3, a=u=1 and label field F5. Its base class has
`(R1,C1)=(0,1)` and `(R2,C2)=(0,1)`. The slot action is
`S(a,b)=(-a-b,a)` on component 1 and its inverse on component 2. Thus:

| selected component | row groups | column groups |
| --- | --- | --- |
| 1 | 0,4,1 | 1,0,4 |
| 2 | 0,1,4 | 1,4,0 |

Each of the three nonempty group children has exactly two Boolean edges, one
original row edge and one original column edge, and is contracted by its four
assignments. The two remaining group factors are 1. Original unary tensors pin
all three X bits and all three Y bits. Test all 8*8 pin pairs at root precision
p=1 and p=2: exactly 128 root cases. Original contraction is 1 iff X=Y, else 0.
These deliberately forced tiny parameters exercise finite identities and do not
satisfy or test the source's sufficiently-large asymptotic packing admission.

## Arithmetic and deformation

Use GMP integers/rationals in native C++. The coefficient ring is
`O=Z_(3)[U]/(U^3-U-1)`, represented in basis 1,U,U^2. The polynomial is irreducible
modulo 3 because it has no F3 root. For precision q, coordinates are canonical
integer residues modulo 3^q. Use all 27 canonical F27 lifts (base-3 coefficient
digits in that basis) for field controls and the first 25 as interpolation nodes.
Their pairwise differences are units. Unit inversion starts with exhaustive F27
inversion and uses Newton lifting; a zero residue is rejected.

The grid is S={-3,-2,-1,0,1,2,3,4,5}, C(T)=product(T-s), C'(0)=-720 and alpha=9.
Precompute exact rational polynomial kernels
`W_xy(t+9z)=product_(s != y-x)(t+9z-s)/(-720)`.
Assert coefficientwise three-integrality, cancel factors of 3 in each rational,
then invert the unit denominator modulo 3^q. Never invert -720 modulo 3^q.
At T=0 check W=I. Locality holds at the selected component of each active t.

There are six active nodes t=-2,-1,1,2,4,5, all of valuation i=0. Set h=6,
D=3, multiplicity q_t=6 and child precision
`p_t=p+4*h*D+8*M*(a+1)=p+120`. The degree bound is M*(3L-1)=24.

## Actual child evaluation and independent controls

For each active t and each of 25 residue-distinct evaluation nodes zeta:

1. Separate the six crossing b=0 modes by source column AND target group.
   Every tile contains one column with j=0. Therefore all b>0 source moments
   are identically zero; retain their exact zero proof rather than enumerate
   them. A row factor has its constant/local term and two incoming modes.
2. Enumerate all 3^3=27 choices of constant/one of two incoming modes, one
   choice per row. This covers every nonzero monomial; each target has one row,
   so no higher target degree occurs. Evaluate each group packet tensor by
   coefficient extraction and source-bit powers, retain the original pin unary
   tensors, and contract its four assignments. Sum the product of all five
   group children. No original pin substitution replaces these leaf sums.
3. Compare with an independently written direct 64-assignment deformed sum.
   Also compare with evaluation of the independently assembled exact rational
   global polynomial in T. All comparisons use complete ring coordinates.

Interpolate all 25 coefficients of each local chart using ring divided
differences and unit differences. Compare all coefficients against the exact
rational chart. Recover F(0) using six jets and exact coordinatewise rational
Hermite formula D.9. Canonical lifted jets are allowed to differ from exact
coefficients by multiples of 3^p_t; sum the exact rational Hermite contributions,
cancel powers of 3, and reduce to root precision. Compare with the original
64-assignment matching contraction and with exact rational Hermite recovery.

Required controls: all 26 nonzero F27 residues invert at p_t=121 and 122;
every pair of 25 interpolation nodes has unit difference; zero/nonunit inversion
is rejected; a non-three-integral rational is rejected; wrong component action
or locality table is detected; unsplit source-mode regression at t=1,zeta=1,
X=Y=(1,0,0), root modulus 27 has correct value 9 and faulty value 0.

## Preservation and acceptance

Compile with strict C++17 warnings and preserve every build log, including
failed builds. Preserve all 128 case rows, native control output, source and
protocol hashes, exact command/replay script, compiler version and summary.
Repeat the complete run and require byte-identical output. Any failure remains
in the evidence and is resolved explicitly; no silently dropped case or changed
panel is allowed. A follow-up protocol addendum must precede a substantive scope
change. If full Hermite integration cannot be completed, identify precisely
which earlier stages passed and what remains unimplemented.
