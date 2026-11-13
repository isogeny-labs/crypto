# Exact nonconstant lower-chart control from two lines

2026-11-13. Known-answer correctness control; unsigned producer standing
[pcc:unverified].

This validates the previously proposed a3,b1 control. It exercises the
nonconstant lower polynomial chart that the earlier a2,constant-B control did
not cover. It is algebraically constructed from known lines, not a random
search or runtime/scaling experiment [pcc:unverified].

## Exact inputs

Reuse the cubic f=X^3+2X+2 over F17 from the existing NormRank fixture.
That earlier source contains its nonsingularity and rational point-count
checks. Take

    L=2X+8, M=4X+6,
    A=f+LM=X^3+8X^2+12X+16,
    B=L+M=6X+14=6H, H=X+8.

The universal CommRing polynomial identity

    (f+LM)^2-f(L+M)^2=(f-L^2)(f-M^2)

is proved separately before specialization [pcc:unverified]. Exact F17 factor checks give

    f-L^2=(X-5)(X-6)(X-10),
    f-M^2=X(X-3)(X-13).

Therefore the actual norm is the monic six-factor target

    A^2-fB^2=G=(X-5)(X-6)(X-10)X(X-3)(X-13).

The six x-nodes are distinct. The supplied points(5,1),(6,3),(10,11),
(0,6),(3,1),(13,7) each satisfy the actual cubic curve equation. At target
Q=(0,6), A(0)-6B(0)=0 and A(0) is nonzero. Every statement in this fixture is
an exact Lean component rather than an assumed numeric-rank condition
[pcc:unverified].

## Explicit nonconstant-chart certificate

Let F=A, alpha1,beta6,c1. The explicit polynomials

    U=16X^3+11X^2+6X+12,
    V=X^2+6X+4

satisfy UF+V(fH)=1 exactly over F17. The degree and monicity checks establish
F monic degree3, H monic degree1, G monic degree6, f degree3 and B degree1.
Thus B is nonconstant; this supplies a full nonconstant lower chart in the
study's planted controls [pcc:unverified].

For the actual quotient functional mu(P)=coeff2((PV) mod F), the actual norm
equation yields mu(G)=0. The general canonicalMatrixPencil theorem is
instantiated with all its monicity, positive-degree, gap, curve degree, Bezout
and scaled-norm hypotheses supplied. The canonical product coordinate dimension
is6. The source keeps families indexed by their actual degrees and separately
proves that dimension equals6, avoiding unsafe dependent-index rewrites
[pcc:unverified].

The result is actual matrix-pencil membership in the nonconstant chart.
Neither restricted matrix rank4, corank1, adjugate recovery nor a public-input
search procedure is claimed by this control. Those remain separate checks
[pcc:unverified].

## Why this follow-up is informative

Constant B does not exercise a positive-degree quotient modulo H. Moving to
B degree1 covers that missing mechanism using explicit inverse and norm data.
The fixture was chosen by an algebraic construction before verification;
there is no failed random-search run discarded or timing result selected.
Its equalities and counterexample guards are independent of host CPU
contention. A future speed claim would still require the prescribed isolated
measurement protocol and full cost accounting [pcc:unverified].

This is a correctness control with a supplied witness. It is not evidence of
out-of-sample success, scaling or sub-exponential search complexity.

## Source and proof map

[NormNonconstantControl.lean](../lean/NormNonconstantControl.lean), frozen
sourcecbfdaad42. Twenty-two scoped interpretive producer records bind the
reported declarations. All standing is unsigned [pcc:unverified].

| Declarations | Exact role |
|---|---|
| twoLineNorm | Universal CommRing identity |
| construction, lineFactors, actualNorm | Exact chosen lines, factors and prescribed norm |
| lowerScale, actualScaledNorm | Actual alpha1,beta6,c1 chart |
| bezoutIdentity | Actual quotient inverse data |
| targetBranch, branchNonzero | Actual target condition and nonzero evaluation |
| normAMonic, lowerMonicMonic, targetMonic | Actual monicity |
| normADegree, lowerDegree, curveDegree, targetDegree, normBDegree | Actual degrees |
| nodesDistinct, supportPointsOnCurve | Six distinct fibres and supplied points |
| targetMomentZero | Derived actual quotient annihilation |
| coordinateDimension, actualCanonicalPencil | Exact six-dimensional canonical pencil instantiation |

The new source reuses existing general theorems; earlier matrix/functional
declaration hashes remain unchanged. The older curve source is
[NormRank](../lean/NormRank.lean), especially fixtureNonsingular and
fixturePointCount; no new group-order or discrete-log solver theorem is claimed.

## Checks and failures

Lean4.35.0 with unchanged pinned CryptoLean/Mathlib revisions. Build004
passes3150 jobs without new-source warnings. Actual tracked-source audit
covers143 declarations, zero failing and zero additional trusted axioms.
Separate fresh kernel replay including imports exits0; its completed receipt is retained.
Only propext, Classical.choice and Quot.sound are admitted. No signed verdict
is claimed [pcc:unverified].

Build001 left degree/monicity arithmetic goals and a dependent-index rewrite
problem. The actual remaining goals were discharged; the canonical matrix is
stated with its actual degree-indexed families and a separate dimension proof.
Build002 left nonzero6 over F17, resolved with kernel decide. Build003 passed;
build004 removed unused new tactics. Failed and completed logs are retained.
No native_decide, extra axioms or failed source is counted as proof
[pcc:unverified].

Native record **IDC1h36c0f0c618539c35** checks only universal twoLineNorm over
CommRing, using the unchanged maintained issuer/checker at32 statement-derived
and32 extra-stream points. It pins the exact universal source rather than
claiming that a F17-only equality holds over the integers. The native checker
does not execute Lean or establish finite-field, matrix or search conclusions.
Its adapter's22 inherited tests pass, including tamper/statement-swapping
controls. These are correctness checks, not runtime measurements
[pcc:unverified].

## Remaining dependencies and resources

Actual branch/bilinear coordinate transfer, injectivity and exact corank,
then guarded adjugate recovery; possible explicit restricted-matrix control;
exceptional charts; public-input moment finding with accounted2^{o(n)} bit
cost for n=ceil(log2 r). Known witness-dependent data here does not supply that
finder. Reachability through the earlier W_base+C(T)H(T) completion proposal
also remains open [pcc:unverified].

Sources: preceding proposed control, actual matrix/functional source, existing
F17 curve fixture, pinned compute_degree/monicity and determinant APIs inspected
for the next restricted check. KB/conductor unavailable; commands attempted,
bounded fallback, no priority or exhaustive-search claim for elementary line
norms. No paid job, empirical timing/scaling campaign or budget override;
monthly isolab budget exhausted. No benchmark/leaderboard/ecdb changes.
Canonical scoreboards, progress timeline, curve diagrams and historical figures
gain no measured fact and remain unchanged.

Editable [DOT](control.dot), [SVG](control.svg), [diagram PDF](control.pdf),
[report PDF](report.pdf), [LaTeX](report.tex) and
[raw evidence](../nonconstant-evidence/) accompany this report.
