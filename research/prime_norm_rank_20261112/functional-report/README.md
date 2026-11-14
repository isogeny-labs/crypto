# Actual quotient functional and coefficient-pencil membership

2026-11-13. Unsigned producer evidence [pcc:unverified].

The preceding CRT/radical round left mu(G)=0 and actual coefficient-pencil
membership as assembly obligations. This continuation supplies both at the
polynomial functional level. It does not yet assemble the finite matrix,
concrete branch intersection, or public-input finder [pcc:unverified].

## Exact setting and statement

Over any field K, let F,H be monic, deg H>0, deg H+1<deg F, and suppose
supplied U,V,f obey UF+V(fH)=1. Define the genuine K-linear map

    mu(P) = coeff_(deg F-1)((P V) mod F).

Suppose supplied scalars alpha,beta,c and target G satisfy

    (alpha F)^2 - f (beta H)^2 = cG,  c!=0.

Then mu(G)=0 is derived, not assumed. If G is monic and trial polynomials A,B
obey 2 natDegree A<=natDegree G and
natDegree f+2 natDegree B<=natDegree G, write N=A^2-fB^2, d=natDegree G.
The exact identity is

    mu(N) = sum_(j<d) (coeff_j(N)-coeff_d(N)*coeff_j(G))*mu(X^j).

Thus the actual leading-eliminated coefficient constraints are the polynomial
functional pencil for the supplied moments. The theorem does not hide norm
membership in an assumed rank or pencil hypothesis. It does use a supplied
norm witness to construct F,H,U,V. No algorithm for finding that witness is
supplied [pcc:unverified].

## Written proof and formal map

- `moment` constructs the K-linear map with ordinary multiplication, monic
  remainder and coefficient. `momentApply` exposes the exact formula.
- `annihilatesTarget`: mu((alpha F)^2)=0 by F-divisibility. The preceding
  lowerFullRadical, tested on H itself, gives mu(f H^2)=0; linearity gives
  mu(f(beta H)^2)=0. Apply mu to the actual norm equation and cancel nonzero c.
  Neither separability nor a characteristic restriction is required.
- `coefficientExpansion`: expand any bounded polynomial in monomials and
  apply an arbitrary linear functional. It equals the finite sum of its
  coefficients times the corresponding moments.
- `coefficientPencil`: expand both N and monic G through degree d. The leading
  coefficient of G is1; mu(G)=0 eliminates mu(X^d). Substitute in mu(N) and
  distribute the finite sum to get the actual residual coefficients.
- `normCoefficientPencil`: instantiate this general lemma using
  annihilatesTarget and the existing NormWronskian.normDegreeBound. The degree
  conclusion for the trial norm follows from the actual pole-degree bounds.
- `normPolarization`: over every commutative ring,

      Norm(A+Q,B+T)-Norm(A,B)-Norm(Q,T) = 2(AQ-fBT).

  This universal identity will support the later quadratic/bilinear matrix
  bridge; division by2 has not been used in this module.

Source: [NormPencilFunctional.lean](../lean/NormPencilFunctional.lean).
Frozen source commit78123b01d. Five interpretive producer PCC records bind
these substantive declarations; momentApply is audited as well. All standing
is unsigned [pcc:unverified].

## Checks, failures and limits

Pinned Lean4.35.0, CryptoLean148f5dc389b341401e06d76abe7783c4ccf42f2e and
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2 are unchanged. Build004
passes3147 jobs. Build001 failed an exported definitional-equality visibility
check for momentApply; an explicit simp proof resolves it. Build003 failed a
missing import for the reused norm degree theorem; public import NormWronskian
resolves it. Build002 was the successful pre-specialization scope. All raw
logs are retained, including failures [pcc:unverified].

Final actual-source audit covers110 declarations with zero failing and zero
additional trusted axioms; import-inclusive fresh replay exits0. Receipts are in
[functional-evidence](../functional-evidence/). These are unsigned producer
receipts, not signed PCC verification. Only propext, Classical.choice and
Quot.sound are admitted. Earlier CRT/radical/moment declaration hashes remain
unchanged [pcc:unverified].

The universal norm-polarization native certificate IDC1h345f3824264d232a is issued/checked using the
unchanged maintained implementation, with32 statement-derived and32 extra
stream points, bound to this module's exact universal CommRing theorem source.
The native tool does not execute Lean or prove the conditional pencil theorem.
The adapter's22 inherited tamper/control tests pass and are recorded separately.
No benchmark or empirical complexity measurement was run [pcc:unverified].

## Next assembly

1. Define bounded coefficient basis maps and identify the functional with
   their actual symmetric matrices; derive matrix-pencil equality from the
   now-established functional coefficient identity.
2. Instantiate branchTestsDetectFull for the actual evaluation branch, then
   compute its intersection with (cF,dH), including monic scaling factors.
3. Transfer exact radical dimension1 to corank1 and apply guarded adjugate
   recovery with all nonzero/stationarity conditions.
4. Handle constant/degree-drop charts; develop and account for a public-input
   finder. Current moments depend on supplied witness data. The required
   2^{o(n)} bit search bound, n=ceil(log2 r), does not follow from existence.

The earlier nonscalar origin/jet-completion suggestion motivates the
rank-directed certificate search. These functional results do not establish
reachability through W_base+C(T)H(T) or efficient selection of H
[pcc:unverified].

## Sources and resource boundary

Reviewed the previous NEXT_MATRIX_BRIDGE.md, exact pinned
Polynomial.RingDivision.smul_modByMonic/modByMonicHom,
Degree.Support.as_sum_range_C_mul_X_pow', the existing actual-degree lemma,
and lowerFullRadical. KB MCP absent; wrapper has no executable. Conductor also
absent. Bounded source fallback recorded; no novelty or exhaustive search
claim for these classical linear/coefficient arguments.

No paid job, runtime/scaling campaign, budget override, benchmark, leaderboard
or ecdb update. Monthly isolab budget remains exhausted. Source checks are
correctness work. Canonical scoreboards/progress timeline/curve diagrams and
frozen study figures gain no measured fact; they remain unchanged.

Editable [DOT](pencil.dot), [SVG](pencil.svg), [diagram PDF](pencil.pdf),
[report PDF](report.pdf) and [LaTeX](report.tex) accompany this report.
