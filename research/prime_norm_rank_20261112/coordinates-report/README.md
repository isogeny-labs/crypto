# Public branch coordinates and derived cofactor guards

Later continuation: [NormCertificateRecovery](../recovery-report/README.md)
now derives the full polynomial norm and its nonzero normalization from the
actual cofactor equations. Its checked native F17 control demonstrates the
target distinction at fixed rank. This report retains its original25-statement
scope and receipts; efficient finding and the divisor/scalar application remain
separate [pcc:unverified].

2026-11-13. Pure proof continuation, source01e312a94d29edd1b7ce6e86cfe51a6a5ee5d9de.
Unsigned producer standing [pcc:unverified]. Exact source bindings and final
kernel receipts accompany this report; no new solver or timing experiment.

Central unsigned bindings: actual radical PCC-20261113-fb631f, numerical
corank PCC-20261113-41e3e6, actual nonzero cofactor PCC-20261113-3e0d14, and
actual coefficient recovery PCC-20261113-20da19. All25 exact statements and
claim paths are in [claim-pins.json](../coordinates-evidence/claim-pins.json).

The actual polynomial branch now has explicit public coordinates. Restricting
the actual moment matrix through them gives numerical corank one and a
nonzero diagonal adjugate entry under the prior quotient-moment hypotheses.
Those structural conditions are derived, not numerical rank assumptions.
The existing guarded recovery theorem is instantiated with the actual reduced
residual coefficient matrices, conditional on their cofactor equations.

## Public construction and exact attachment

Let a=deg F, b=deg H, N=(a+1)+(b+1), n=N-1. The full coordinate families are
exactly the pinned product monomial basis for degree-bounded polynomial pairs.
Set ell_i=A_i(q)-y B_i(q). Its first coefficient is1 because the first basis
pair is(1,0). For z with n coordinates, define

    Tz = (-sum_j ell_(j+1) z_j, z_0, ..., z_(n-1)).

Thus column j is e_(j+1)-ell_(j+1)e_0. T uses only the public branch
parameters and families, not a witness, quotient moment or basis oracle.
Tail deletion L satisfies LTz=z; TLw=w whenever ell(w)=0. Every inserted
vector lies in the branch. The map is injective and respects scaling.

Define S_red=T^transpose S T. The exact entry formula is

    S_red(i,j) = S(i+1,j+1) - ell_(i+1) S(0,j+1)
                - ell_(j+1) S(i+1,0)
                + ell_(i+1) ell_(j+1) S(0,0).

The Lean proof transfers arbitrary-pair evaluation through this actual
matrix congruence and connects the previous double-sum convention explicitly.
No diagonal-only polarization or characteristic restriction is used.
Writing dense S_red needs only a fixed number of field operations per entry;
this elementary component accounting is not a Lean runtime theorem and does
not charge or bound the missing search procedure.

## Derived guards

The field K is arbitrary. The actual theorem assumes F,H monic, deg H>0,
deg H+1<deg F, UF+V(fH)=1, F(q) nonzero, alpha nonzero, and
alpha F(q)-y beta H(q)=0. These are the prior exact quotient-moment chart
hypotheses. An elliptic application still needs its actual curve/divisor and
input distribution obligations; arbitrary polynomials are not a new ECDLP class.

Under them, for S built from the actual functional moment(F,V), the exact
reduced radical is

    {x : for all z, x^transpose S_red z=0} = span(Lw0).

Coordinates w0 of(alpha F,beta H) exist by actual family completeness. They
are nonzero, and Lw0 is nonzero by the branch inverse. Rank-nullity applied
to S_red^transpose proves rank(S_red)+1=n. In particular det(S_red)=0.
No unrelated literal matrix or supplied numeric rank is substituted
[pcc:unverified].

For any symmetric matrix with this radical line, choose k with(Lw0)_k nonzero.
Replace row k by e_k. A vector in the updated matrix's kernel has coordinate
k zero and is annihilated by every other original row. Pairing with the
nonzero radical coordinate forces the missing original row to vanish too.
Symmetry then places the vector in the radical line, and its zero coordinate
forces it to be zero. Thus the updated matrix is invertible. Its determinant
is exactly adj(S_red)_(k,k), proving a nonzero cofactor without a minor oracle.
This is standard linear algebra; no priority or algorithmic novelty claim.

## Recovery and remaining search

For j<deg G, use the actual residual functional
P -> coeff_j(P)-coeff_degG(P) coeff_j(G) and reduce its actual coefficient
matrix through the same public T. If all its pairings with adj(S_red) vanish,
actualCoefficientRecovery supplies a nonzero reduced vector annihilating all
these quadratic forms. Symmetry, determinant zero, numerical corank and
nonzero adjugate are derived inside that application of prior recovery.

The conclusion here is quadratic conditions. It does not itself prove a
nonzero normalized norm scalar, final divisor relation, or recovered discrete
logarithm. Degree/leading-coefficient and full recovery attachment remain
explicit further steps. The moments used in this construction depend on F,H,V;
an efficient public-input method that finds suitable moments and satisfies
the cofactor equations is still missing. Conditional certificate existence
must not be mistaken for a sub-exponential finder [pcc:unverified].

The annotated nonscalar-origin/jet completion W_base+C(T)H(T) remains a
separate reachability question. The norm polynomial named H here is not a
proof that the permitted jet corrections realize these moments or a useful
crossing rank.

## Verification and reproducibility

There are25 new theorem statements in NormBranchCoordinates. The tracked-source
gate audits201 statements in the whole study with0 additional trusted axioms.
All25 print only propext, Classical.choice and Quot.sound. The import-inclusive
fresh replay has a separate completion receipt. These are producer checks;
only a verifier signature would supply independent signed PCC standing.

The maintained PCC source-only checks report not_covered for all25 bindings
because Lean is absent from their default PATH. The explicit toolchain source
gate and fresh replay are separate, successful kernel receipts. None is
misrepresented as an independent signed verdict.

```sh
lake -d research/prime_norm_rank_20261112/lean build
bash tools/lean/proof-gate.sh research/prime_norm_rank_20261112/lean
cd research/prime_norm_rank_20261112/lean
lake env leanchecker --fresh --verbose NormBranchCoordinates
```

Builds001-009 failed on finite-index annotations, partial application rewriting,
matrix constructor recognition, module-export opacity and quantified dimension
conversion. Build010 first passed the actual radical bridge;011 rank-nullity,
012 actual corank/singularity,013 cofactor pivot,014 actual cofactor,015 final
coefficient recovery. All logs and available process receipts are retained.
No failed printed sorryAx diagnostic counts as a proof. No new module warning
remains in the final build. No benchmark, leaderboard or ecdb row is added.

Sources checked (KB unavailable): prior actual family, moment and branch
proofs; faithful-report/NEXT_CORANK.md; pinned Mathlib DegreeLT basis values,
Fin tuple/sum APIs, Matrix Mul/Rank/RowCol/Adjugate/NonsingularInverse and
FiniteDimensional rank-nullity/span lemmas. Read the actual sources before
their use. No claim of exhaustive cross-repository or literature review.
The old report's exhausted-budget language is historical; live admission
remains unobserved, and no paid run is launched here.

For a concrete next implementation control, reuse the already checked
NormNonconstantControl example: F=normA, H=lowerMonic, f=curve, U=bezoutU,
V=inverseV, q=0, y=6, alpha=1, beta=6 over F17. The prior source supplies
monicity, degrees3/1, Bezout, nonzero evaluation and branch identities.
The reduced chart is five-dimensional, so the new corank theorem predicts
rank4 and a nonzero diagonal cofactor. No new numerical matrix row or solver
measurement is reported for this prospective control.

[Report PDF](report.pdf), [TeX](report.tex), [DOT](coordinates.dot),
[SVG](coordinates.svg), [gate receipt](../coordinates-evidence/gate-001/status),
[fresh receipt](../coordinates-evidence/fresh-status.json).
