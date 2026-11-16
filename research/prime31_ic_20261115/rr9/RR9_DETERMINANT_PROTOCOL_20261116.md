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
