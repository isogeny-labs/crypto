# EXP-R31-RR9 — elliptic L(9O) evaluation determinant as a compact S9 relation predicate

**Prospective protocol, 2026-11-16. Status [pcc:unverified].** Commit this *before* Rust implementation and Actions. This is a **new experiment with a classical Riemann–Roch identity**, not a claim of a new algorithm or complete ECDLP solve.

## Goal and mathematical statement

Curve: actual ordinary cofactor-one prime31 instance `p=2147483647`, `E: y²=x³+847514846x+2087444724`, `n=2147446831`, `G=(625037715,2100627251)`. No deliberately weak curve.

For **nine pairwise distinct affine points** `Z_1,...,Z_9` on an ordinary Weierstrass curve of characteristic other than 2, form the 9-by-9 evaluation matrix whose columns are the pole-order basis for `L(9O)`:
`[1, x, y, x², xy, x³, x²y, x⁴, x³y]`.
The proposed exact original-curve test is

`det(Eval_9(Z_1,...,Z_9)) = 0 iff Z_1+...+Z_9=O`.

Proof obligation: `dim L(9O)=9`, with the stated monomials of distinct pole orders at O; a nonzero linear combination has zeros at all nine distinct points iff its divisor is `sum Z_i - 9 O`, which is principal iff the elliptic Abel–Jacobi sum is O. Special cases at O, repeated signed points and characteristic 2 are excluded. For eight factor-base leaves `P_i` and public residual `R`, test the nine points `P_1,...,P_8,-R`, so a determinant zero encodes `sum P_i=R`.

**Critical false-positive control**: Naively repeating any point gives duplicate evaluation rows and a zero determinant **even when the group equation is false**. Strict pairwise distinctness is mandatory; repeated leaves require genuine Hermite-jet evaluation or original-ideal saturation. Never claim this simple determinant resolves the full signed S9 system.

## Measurement and falsification, fixed in advance

Reuse `prime31_algebraic_ic::{Arithmetic, Base, PRIME31}` in a **native Rust example**; no Python/C++ ECDLP code or new curve. Compare the first 31 rational x base and the *actual* Dickson-depth-six 31-point base. The test deterministically draws 32 reproducible eight-**distinct**-signed-leaf trials on **each** base. For each trial:
- Construct one original-curve positive `R=sum P_i` and a negative `R'=R+G`; discard/resample before testing if either produces an O/repeated row. The internal construction is a planted mathematical predicate fixture, **not** a solved unknown scalar.
- Compute modular 9-by-9 determinant only (no group operations inside the predicate), assert zero on positive and nonzero on negative; keep matrix, sign, curve/leaf metadata in output digest/counters sufficient to repeat. Independently check `sum P_i` with existing curve arithmetic *outside* determinant.
- Test the adversarial repeated-point case with an intentionally incorrect residual and record the **false zero** as a known limitation.
- Record number of rejected degeneracies; do not silently drop cases, run until 32 admissible with an explicit 10,000-draw cap per base.
- Run release-mode native Rust tests and CI; log diagnostic time for determinant and source point construction separately. This is a **predicate-only** experiment; no full target-conditioned polynomial root extraction, rank32 recovery, rho advantage or PCC is implied.

Pass criterion: 64/64 distinct planted positives and 64/64 distinct planted negatives agree with group arithmetic across both bases, duplicate-row negative control falsely returns zero as expected, all Rust tests pass. If not, preserve the failing fixture; fix source without rewriting this preregistration.

## Research significance and stop criteria

The attraction is a **compact 9x9 determinantal presentation of an original-curve S9 signed relation** with entries of x-degree at most four, in place of expanding a huge iterated resultant. However, the determinant requires the eight unknown signed **points**; simply evaluating it after enumerating support is generic and no more an attack than the existing 4+4 oracle. To become a credible non-generic index-calculus lead it must combine this identity with the actual Dickson31 factor-base polynomial, signed y-lifts, collision/diagonal saturation or Hermite jets, a target-conditioned *nonenumerative* elimination and certified recovery of the factor polynomial f. Check equivalence on B4/B8 planted positives, admissible negatives, the two recorded Dickson31 negative controls, infinity and repeats. Go to B31 only after the algebraic extractor produces f directly; charge all costs against same-host optimized rho, plus fastest external PKM/APS/F4.

## Sources and prior negatives checked

- Existing Rust exact witness controls and S9 net multiplicity: crypto #4391, #4451, `research/prime31_ic_20261115/native/S9_B31_MULTIPLICITY_PROTOCOL_20261116.md`.
- The certified quotient/subresultant line #4228/#4349; bounded/censored m6/m8 F4 #4373 and prior factor-base pitfalls; Vieta #4410, auxiliary IC cryptanalysis #1536, newer remainder graph #4453. Do not re-run unchanged generic enumeration or claim a theorem about new ECDLP complexity.
- Published Semaev/APS/PKM are prior art; the divisor/interpolation identity is classical genus-one Riemann–Roch, **not claimed novel**. GitHub text/code search for `Riemann Roch interpolation determinant`, `L(9O) determinant` and `evaluation matrix elliptic sum` did not locate this specific prime31 experiment; this bounded search is **not** evidence of worldwide novelty.
- Shared `research-kb` server and local wrapper/Conductor unavailable in this tool environment; scoped repository/publisher search used. Owner policy `AGENTS.md` read from immutable merged Rust ref due current main 404.

## Follow-on independent verification

Even a 100% predicate pilot is not a cheaper **complete** ECDLP solver. Preserve source-exact CI, raw output, an independent original-curve replay (different arithmetic code or PCC signed verifier), compiler version/commit, and a genuine blinded unknown-scalar log panel before advancing a claim. No paid compute, merge, secrets, leaderboard edit or independent signed PCC requested.

## Prospective extension A — Hermite-jetted repeated leaves (committed before code)

The initial squarefree gate deliberately rejects a repeated point. After independently verifying the underlying `D_y=F'(x)/(2y)` tangent identity, add a second predicate for divisors where **each distinct affine point occurs at most twice and y≠0**. Use the unique local parameter `t=x-x(P)` at nonsingular non-2-torsion P. Replace the second identical evaluation row by the order-one jet `D=` d/dx along E, where `D(y)= (3x²+a)/(2y)`. Thus, for the L9 basis, the derivative row is
`[0,1,t,2x,y+xt,3x²,2xy+x²t,4x³,3x²y+x³t]` with `t=(3x²+a)/(2y)`.
Riemann–Roch then gives determinant zero iff the full **length-nine effective divisor counted with multiplicity** sums to O. Reject any triple occurrences or repeated rational y=0 points until higher jets / the y-local parameter are proved. This is classical confluent evaluation, not a novel attack.

Add a predeclared adversarial original-curve fixture with one duplicated signed factor-base point: the **naive** repeated-row determinant must spuriously vanish against a deliberately wrong target; the **Hermite-jetted** determinant must be nonzero against the same wrong target and zero for its true group residual. Run this fixture on both actual prime31 bases and record exact determinant values and all positivity/rejection flags. All distinct-point 128 checks remain unchanged; no hidden scalar or F4 root extraction is claimed.

If the root runner compilation or result fails, retain the negative evidence and correct the source via a separate commit; no magic `det==0` claim is made on diagonals without jet certification.

## Prospective extension B — all affine multiplicities through nine (frozen before code)

The two-occurrence limitation is artificial for the **actual cofactor-one prime31 curve**. If a rational affine point had y=0 it would have order two, impossible when #E(Fp)=2147446831 is odd prime. Thus x-x(P) is a local uniformizer for every affine rational P on this *particular* curve. This does not claim validity on other curves with rational 2-torsion or at O.

For each repeated P=(x0,y0), use t=x-x0 and the truncated formal expansion y(t)=sum_{j>=0} y_j t^j satisfying y(t)^2=(x0+t)^3+a(x0+t)+b and y_0=y0. For j>=1,

  2*y0*y_j = c_j - sum_{i=1}^{j-1} y_i*y_{j-i},

where c_1=3*x0^2+a, c_2=3*x0, c_3=1, c_j=0 for j>3, in Fp. Generate the t^j coefficients of all nine L(9O) basis monomials by truncated polynomial multiplication. For multiplicity m, feed the value, first, ..., (m-1)-th Taylor-*coefficient* rows (not factorial derivatives) to the same 9x9 determinant. This implements the correct confluent divisor condition for any m<=9 without enumerating signed leaves, but still requires supplied points.

**Predeclared tests (all on actual curve, using existing Rust Arithmetic):**
1. For the first 16 rational x points, order-zero series row must equal the old evaluation row, and order-one must equal the hand-derived elliptic tangent row.
2. For each of first31 and Dickson31 bases, form eight leaves with multiplicities m in {3,4,8} at P0 and the other 8-m leaves at P1. Set R=sum leaves *outside the determinant*. The confluent determinant must vanish for R and be nonzero for the wrong R+G, with both finite. This is 2 bases x 3 multiplicities x 2 signed decisions = 12 new checks; keep the original 128 distinct + 4 double controls unchanged.
3. Emit the per-base m, true/false determinant values, original-curve residual and unchanged complete_unknown_target_ecdlp_recoveries=0 into the raw JSON artifact; fail CI rather than discarding any counterexample.
4. Reject infinity explicitly; y=0 remains rejected in the generic function, though none exists on the checked prime31 curve. Do not generalize to unspecified curves.

The mathematical result, if tests pass, is **diagonal-correct S9 predicate completeness for all affine multiplicity patterns on this curve**, not an unknown-residual factor extractor or a cheaper index-calculus solver. No performance claim unless a fully cold unknown-scalar solver also succeeds. Existing #4228/#4349 quotient work, #4451 zero-fibre, #4454 APS and #4468 WZ control stay separate; do not duplicate their experiments.

## Prospective extension C — ninth-order jet falsifier (before code)

All prior B controls reach highest Taylor coefficient order 7 (eight copies of one signed leaf). Complete the last possible RR9 affine diagonal by choosing eight copies of a rational nonzero P as leaves and setting *wrong* residual R=-P. The ninth evaluation point -R=P, so all nine rows must be value/Taylor orders 0..8 at P. Their determinant must be nonzero, because nine copies sum to [9]P != O in the prime-order group (n=2147446831 not dividing 9). The naive repeated-value-row determinant is zero. Check this on both factor bases and emit the determinant and original-curve negative witness into the JSON; no successful ninth-order relation is possible on this curve except O, which is explicitly unsupported by the affine matrix. This guards the topmost coefficient recurrence with an adversarial source-exact test. No scalar search.
