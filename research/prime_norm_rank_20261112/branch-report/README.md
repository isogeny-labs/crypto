# Actual target-branch radical is the witness line

2026-11-13. Symbolic polynomial transfer; unsigned producer evidence
[pcc:unverified].

The explicit target-branch projection preserves the bounded polynomial test
space and its actual quotient pairing. Combined with the previous full-block
radical theorem, it identifies the branch-restricted radical with exactly the
supplied witness line. No literal matrix rank or characteristic-two
polarization argument is assumed [pcc:unverified].

## Defined objects and hypotheses

Work over any field K. Let F,H,f,U,V be polynomials, with F,H monic,

    0 < deg H,   deg H+1 < deg F,   UF+VfH=1.

Define the actual functional

    mu(P) = coeff_(deg F-1)((P V) mod F),

and pairing on deg A<=deg F, deg B<=deg H by

    B((A,B),(Q,T)) = mu(AQ-fBT).

The target branch is l(A,B)=A(q)-yB(q). Assume F(q)!=0 and alpha!=0, and
that the supplied witness direction (alpha F,beta H) satisfies l=0. For every
admissible (A,B) with l(A,B)=0, the exact result is

    [B((A,B),(Q,T))=0 for every admissible (Q,T) with l(Q,T)=0]
       iff
    (A,B) = t*(alpha F,beta H) for some t in K.

The witness direction is nonzero. If its actual norm is cG, the norm of its
scalar multiple is t^2 cG. The radical theorem itself does not need a curve
hypothesis or a supplied norm identity; the norm-transfer theorem states that
additional hypothesis separately [pcc:unverified].

## Proof and construction

For any admissible test (Q,T), set

    s = (Q(q)-yT(q))/F(q),   Q' = Q-sF.

Then l(Q',T)=0. The polynomial Q' stays within the upper degree cap because
both Q and sF do. The quotient moment annihilates every multiple of F, so

    mu(AQ'-fBT) = mu(AQ-fBT).

The exact universal ring identity behind this equality is

    A(Q-sF)-fBT = (AQ-fBT)-F(sA).

Thus testing only the branch detects exactly the full radical. The existing
NormPairingRadical.normBlockRadical theorem identifies that full radical as
(A,B)=(cF,dH). The two branch equations are

    cF(q)-ydH(q)=0,   alpha F(q)-y beta H(q)=0.

Subtracting beta times the first from d times the second and cancelling
F(q) gives alpha d=beta c. Since alpha!=0, t=c/alpha supplies precisely the
witness line. This proof does not divide by y, H(q), beta or2; all cancellation
guards appear in the theorem [pcc:unverified].

## What this adds to the certificate route

The earlier CRT and matrix reports left the actual polynomial branch transfer
open. This result supplies that transfer before finite-coordinate rank
arguments. It establishes no efficient procedure for finding F,H,V or their
moments from the public target polynomial G. A known witness supplying a
certificate is distinct from a public-input algorithm finding one.

The annotated nonscalar-origin/jet-completion proposal remains a separate
reachability question: this proof does not show that its moments occur in
W_base+C(T)H(T), or that an efficient jet-selection rule reaches them. Here H
is the lower norm polynomial; it is not a construction of that proposed jet
correction [pcc:unverified].

Next obligations are an injective finite coordinate map, the arbitrary-pair
bilinear matrix identity, actual branch restriction and exact corank/adjugate
transfer. The constant-H and degree-drop charts remain separate. See
[NEXT_MATRIX.md](NEXT_MATRIX.md).

## Formal map and checks

[NormBranchRadical.lean](../lean/NormBranchRadical.lean), frozen source
38b91e915. Nine declaration bindings are recorded in
[branch-evidence](../branch-evidence/) [pcc:unverified].

| Declaration | Exact scope |
|---|---|
| projectionIdentity | Universal CommRing identity |
| momentMultiple | Actual moment annihilates modulus multiples |
| projectedBranch | Explicit projection enforces the branch |
| projectedPairing | Actual quotient pairing is preserved |
| restrictedTestsDetectFull | Branch tests equal full tests, with degree caps |
| branchScalarLine | Scalar elimination with explicit nonzero guards |
| restrictedRadicalWitnessLine | Actual bounded polynomial radical equals witness line |
| witnessNonzero | Actual upper witness polynomial is nonzero |
| normOfWitnessLine | Supplied actual norm scales by t^2 |

Build004 completes3148 jobs with the unchanged pinned Lean4.35.0,
CryptoLean148f5dc and Mathlibcfc55fc. The tracked-source gate and fresh
import-inclusive replay have attributable completion receipts. No additional
trusted axiom is admitted; all producer PCC standing remains unsigned
[pcc:unverified].

Draft001 failed because the moment definition is opaque across module
boundaries and a scalar division rewrite used the wrong implication. Draft002
uses exported momentApply and direct division equality. Builds003/004 add the
universal projection identity, nonzero witness and actual norm scaling.
Retain failed and successful logs; failed elaborations are not proofs.
Previous control/matrix/functional/cost declaration hashes remain unchanged
[pcc:unverified].

Native record IDC1h75cd0c4ee7db5e5d checks only projectionIdentity at32
statement-derived and32 extra-stream points through the unchanged maintained
checker. It pins the universal source, does not execute Lean, and does not
prove branch guards, the radical theorem or a search complexity. Its22 inherited
checker tests pass. No new mathematical research was run in Python and no
runtime claim is inferred [pcc:unverified].

## Sources and resources

Read prior local results NormPairingRadical.normBlockRadical,
NormPairingRadical.branchTestsDetectFull, NormPencilFunctional.momentApply and
the matrix/functional continuation plans before implementing. Primary
summation-polynomial context remains Galbraith15.8.1-2, source-pinned in the
[cost audit](../cost-floor-report/README.md). This elementary radical projection
has no priority claim. Pinned Mathlib polynomial degree sources
Algebra/Polynomial/Degree/Lemmas.lean (natDegree_C_mul_le) and
Degree/Defs.lean (natDegree_sub_le) were read for the degree-preserving step.

KB MCP absent and wrapper unavailable; bounded fallback only. Conductor
command exits127. Local proof/identity correctness checks only; no new paid
job or timing/scaling campaign. Monthly isolab budget remains exhausted. No
benchmark, leaderboard or ecdb result is added; canonical scoreboards and
historical measured figures remain unchanged.

Editable [DOT](branch.dot), [SVG](branch.svg), [diagram PDF](branch.pdf),
[report PDF](report.pdf) and [LaTeX](report.tex) accompany this report.
