# Faithful coordinates and actual matrix branch radical

2026-11-13. Symbolic finite-coordinate transfer, unsigned producer standing
[pcc:unverified].

The actual product-basis coordinates are injective and degree bounded. The
actual moment matrix gives the polynomial pairing for two arbitrary vectors.
Transferring the polynomial branch theorem through these maps identifies its
coefficient-space branch radical with the supplied witness vector's scalar
line. Complete coordinates supply a nonzero witness vector, so its existence
is not assumed as an independent oracle [pcc:unverified].

## Actual maps, not a rank hypothesis

For upper/lower degree caps a,b, let e be the actual Mathlib basis

    degreeLT.basisProd K (a+1) (b+1),

and E=e.equivFun.symm. Its output is a pair of bounded polynomials. The two
polynomial projections of E(w) are exactly the existing assembled polynomials
A_w,B_w from NormMomentMatrix.upperFamily/lowerFamily. Consequently,

    deg A_w<=a,  deg B_w<=b,
    (A_w=A_z and B_w=B_z) implies w=z.

Earlier familiesComplete supplies surjectivity onto every bounded pair. No
arbitrary matrix rank, random-basis assumption or coordinate independence
heuristic enters these facts [pcc:unverified].

Let the actual entry matrix be

    S_ij = mu(A_i A_j-f B_i B_j).

For arbitrary coefficient vectors w,z, the exact identity is

    sum_ij S_ij w_i z_j = mu(A_w A_z-f B_w B_z).

The proof expands both vectors and exchanges the finite sums explicitly. It
does not infer an arbitrary-pair identity from its diagonal and does not divide
by2 [pcc:unverified].

## Actual coefficient-space branch radical

Use all explicit assumptions of
NormBranchRadical.restrictedRadicalWitnessLine: F,H monic,
0<deg H, deg H+1<deg F, UF+VfH=1, F(q)!=0, alpha!=0, and the actual supplied
witness branch equation for (alpha F,beta H). Set a=deg F,b=deg H. Choose
actual coordinates w0 for that pair; witnessCoordinates proves a nonzero
such vector exists [pcc:unverified].

For every actual branch vector w, the exact theorem is

    [sum_ij S_ij w_i z_j=0 for all z with A_z(q)-y B_z(q)=0]
       iff
    w=t*w0 for some t in K.

Forward: use familiesComplete to turn every polynomial branch test into an
actual vector z; the arbitrary-pair identity then invokes the polynomial
radical theorem. Both polynomials become t times the witness pair, so faithful
coordinate injectivity gives w=t*w0. Reverse: scalar assembly and the same
polynomial theorem give every required zero pairing [pcc:unverified].

This identifies the radical of the form restricted to the branch hyperplane.
It is not the kernel of the full unrestricted matrix S, whose polynomial
radical has two directions. A reduced branch matrix still needs a genuine
basis and its rank-nullity/adjugate transfer. No corank for an unrelated literal
matrix is asserted [pcc:unverified].

## Formal map, failures and provenance

[NormMatrixFaithful.lean](../lean/NormMatrixFaithful.lean), frozen source
69c822e1c; ten preferred declaration bindings and two compatibility bindings in
[faithful-evidence](../faithful-evidence/) [pcc:unverified].

| Declaration | Exact scope |
|---|---|
| upperProjection, lowerProjection | Actual basis projections equal actual assembly |
| coordinatesInjective | Both polynomials determine the vector |
| assembledUpperDegree, assembledLowerDegree | Actual assembled degree caps |
| assembledPairing | Arbitrary-vector polynomial double-sum identity |
| bilinearMoment | Actual matrix bilinear expression equals moment pairing |
| assembleSmul | Actual coefficient scaling transfers to polynomials |
| coordinateBranchRadical | Branch radical is exactly the witness coefficient line |
| witnessCoordinates | Actual nonzero witness coordinates exist |
| upperDegree, lowerDegree | Statement-compatible wrappers for the initial records |

Pinned Lean4.35.0, CryptoLean148f5dc, Mathlibcfc55fc. Builds007-009
complete3151 jobs. Build008 has unique preferred degree names;009 adds the
original-name compatibility wrappers. The current
tracked-source gate and import-inclusive fresh replay have attributable
completion receipts; no additional trusted axiom is admitted. All producer
standing remains unsigned [pcc:unverified].

Draft001 proves the initial coordinate bridge. Draft002's arbitrary-vector
expansion fails on swapped summation indices; a diagonal identity would hide
that error. An explicit sum exchange resolves it in003. Draft004 fails on
scalar smul versus field multiplication;005 resolves it. Draft006 adds actual
nonzero witness coordinates. Failed and successful logs are preserved
[pcc:unverified].

The initial registration appended ten unsigned claims and then failed its
preservation audit: maintained PCC lookup matches short names, so new
lowerDegree shadowed the older nonconstant control's theorem. The Lean
namespaces were distinct and older source unchanged. Rename the new degree
lemmas to assembledUpperDegree/assembledLowerDegree. Two new unsigned bindings
provide preferred citations; original files/ledger entries and a note are
retained. The workflow also checks retained records, so replacement alone was
insufficient. The canonical crypto-owned PCC lookup now resolves qualified
namespaces exactly, rejects ambiguous unqualified matches and fails closed on
unsupported scope layouts. Original-name wrappers preserve initial statement
hashes. All127 stored study bindings then match, including the initial records.
All25 PCC tests and the62-class canary pass. Shared lock digests are updated;
downstream identical-copy rollout follows the canonical owner change.
See replaced-degree-bindings.json; no signed verdict or retraction authority
is invented [pcc:unverified].

Fresh001 was deliberately terminated with143 before renaming the source;
it is not a pass. Fresh002 completed on the renamed source before compatibility
wrappers; its historical receipt is retained. Fresh003 checks the final source.
No mathematical
Python research or timing campaign was run. The only Python use here was the
maintained PCC administration and its regression tests.

The repository-wide source-only check in this sparse workspace exits1 on151
other records; the old verifier exits1 on152. The comparison introduces no
new failures and removes the old nonconstant degree-binding failure. All21
new claim records are not_covered with Lean absent from the default PATH;
the actual theorem modules have separate toolchain-specific gate/kernel
receipts. No clean repository-wide verdict or signed standing is claimed
[pcc:unverified].

## Sources and next step

Read pinned Mathlib LinearAlgebra/Basis/Defs.lean, Basis.equivFun,
equivFun_symm_apply and the existing degreeLT.basisProd before implementation.
Read the prior assemble and actual moment-matrix definitions, actual polynomial
branch theorem and continuation plans. KB MCP/wrapper unavailable; bounded
fallback only. This is a transfer of standard basis/form arguments, with no
priority claim. The primary summation-polynomial context remains Galbraith
15.8.1-2, source-pinned in the adjacent cost audit.

Next: choose a genuine basis of the actual branch kernel, transfer its radical
line to matrix kernel dimension1, and derive exact corank and nonzero adjugate
before invoking guarded recovery. This remains separate from an efficient
public-input method that finds F,H,V/moments and from its uniform bit cost.
The annotated jet family's reachability is not established here. Constant-H
and degree-drop charts remain separate [pcc:unverified].

Monthly isolab budget remains exhausted. Local proof correctness checks only;
no new paid job, experiment timing, benchmark, leaderboard or ecdb edit.
Canonical measured figures and scoreboards remain unchanged.

Editable [DOT](faithful.dot), [SVG](faithful.svg), [diagram PDF](faithful.pdf),
[report PDF](report.pdf) and [LaTeX](report.tex) accompany this result.
