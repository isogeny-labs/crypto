# AUDIT of L12-symbolic-4n (independent auditor, 2026-11-02)

Verdict: **partially_valid**. The computations are real and reproduce. The four relations, the inclusion R_i in W3 (given
the certificate), the 4(L-3) model counts for L<=8 and the three bit-level dump checks are all confirmed. The README
headline overstates one thing ("proven in the generic model" for the 4n count), and a few smaller wording and accounting
points need fixing. No security claims are made, and none were found.

Conductor: `conductor_check_conflicts` (MCP) returned `allow` for `path:.../L12-symbolic-4n/AUDIT.md`. The CLI could not reach
its local port from the sandbox, so the MCP tool was used instead. Only AUDIT.md was written in the experiment directory. Scratch
files were under the session scratchpad and $TMPDIR; the dumps I generated were deleted.

## What was re-run (all < 10 CPU-min)

| item | method | result |
|---|---|---|
| exact symbolic closure, window {0..3}, with certificate | original `src/sym.py out 0 4 0 4 cert` (105 CPU-s) | output JSON **identical** to `analysis/sym_rref_cert.json` (dims 37/41, 4 new rows, lcm denominators c*z^7, 4 certs of 10 rows) |
| P1 vs L9 | original `cmpl9.py 31 2`, `31 3` | 37/41, 4 relations identical to L9 `newrel_n31_A_s2/s3` |
| P2 | original `chk_sym_vs_num.py` | all 15 draws (n=29..37) give 37/41 and symbolic = numeric; both L9 comparisons true |
| shifts | original `shifts.py` logic, n=33, L=4..7 (output redirected to scratch, not over the auditee's file) | 4/8/12/16 shifts, in W3, independent mod W2, complete; same as `p3_shifts.json` |
| dump check n=31 s2 | fresh unmodified `macdump` via L9 `run9.py 31 A 2 3` (119 CPU-s), then original `dumpchk.py` | round log 1022/2821/2945, new 124; every R_i trace comp in W3 (31/31), 0/31 in W2; span mod W2 = 124. Matches `dumpchk_n31_s2.json` and `raw/` |
| dump-check negative control | same `dumpchk.py` with z flipped by one bit | in_W3 = 0/31 for all four. So the check discriminates. (Note `span_mod_W2` alone does NOT discriminate; wrong rows also give 124. The evidence is "in W3" + "span = new".) |
| README relations vs `relations_cleared.json` | sympy parse of the four README blocks, mod 2 | all 4 x 14 coefficients equal; every monomial contains a0 or b0; no a0*b0, no constant (claim true) |
| README/RESULTS numbers vs raw logs and analysis files | by hand | table dims 2821/2945/124, 3201/3333/132, 3605/3745/140 match `raw/*.json` round logs; exact counts 16/16, 37/41, 66/74, 103/115 match `sym_L*_p0.json`; W2 formula 4L^2-7L+1 gives 16, 37, 66, 103, 148, 201 as stated; macdump CPU 120+194+309 = 10.4 min |
| macdump.c hash | `shasum` | starts 400655e0 as stated |

## Independent re-implementation (written by the auditor, shares no code with the author's)

Own GF(2^16) arithmetic (log tables), own enumeration of multipliers, numpy Gauss-Jordan; model exactly as in PROTOCOL.
- Window {0..3}: dim W2 = 52, dim(W2 in Win) = 37, dim(W3 in Win) = 41 at random (z,c). Matches.
- Cleared relations R_1..R_4 from `relations_cleared.json`, evaluated at random (z,c): all four lie in rowspace(gens + Var*W2) and
  none lies in rowspace(gens). Matches the claim "in W3, not in W2".
- Translation count: (L,p) = (3,0) 0; (4,0),(4,1),(4,2) 4; (5,0),(5,1) 8; (6,0),(6,1) 12; (7,0) 16. All equal 4(L-3). Window dims
  16, 37, 66, 103, 148 match.
- Controls: independent coefficients give 39/73 (matches the 39/73 claim); c=0 gives 37/41 (matches).
- Special-point test of the "every z*c != 0" claim: 676 (z,c) pairs in GF(2^16), including all 225 pairs of GF(16)^*, 150 pairs in
  GF(256)^*, (1,1), and curves c = z^k for k in {1,2,3,4,5,6,8,10,12,16}. R_1..R_4 lie in W3 at every point and in W2 at none.
  This is evidence that no exceptional locus other than c*z = 0 exists, not a proof.
- Factor-curve test (added after review): the 15 non-monomial irreducible factors appearing in the cleared coefficients (from
  `relations_cleared.json`, e.g. c+z^2, c+z^4, c^2+c*z^2+z^8, c^3+c^2*z^2+c*z^8+c*z^6+z^10, c^4+c*z^8+c*z^6+z^16, ...) each define a
  curve in the (z,c)-plane. For each, 12 points were taken (random z in GF(2^16), c a root found by brute force over all of GF(2^16)):
  180 points in all. At every point dim(W2 in Win)/dim(W3 in Win) = 37/41 (no dimension jump), R_1..R_4 lie in W3 and none lies in W2.
  No exceptional locus was found on any factor curve. A point test remains a test, not a proof.

## Findings

### F1. README overclaims the model count (main issue)
- The README Answer says: "**Proven in the generic model, not in the real system:** the 4n count, i.e. that the shifts are
  independent mod W2 and that nothing else falls in round 3."
- What was actually done:
  - Exact over F2(z,c): L<=6, padding p=0 only.
  - Numeric at specific (z,c): L<=8, p<=2 for L<=6 and p<=1 for L=7,8, for one (z,a6) in the padding sweep.
  - Shifts independence: L<=7, three instances.
  - Padding p=1 exact was NOT completed.
  - Nothing is proved for all L, and the model has no cyclic Z/n structure at all.
- `RESULTS.json` is correct and more careful: `proof_status.count_in_model` = "exact for windows L<=6 (p=0);
  translation-free-rank-4 conjectured for all L".
- The README sub-heading "Count in the model (exact over F2(z,c))" also covers statements that are numeric (shifts, L=7,8,
  padding).
- Correct wording: "verified by computer for finite windows (exact L<=6 at p=0; numeric L<=8); extension to all L, and to the cyclic case,
  is conjectured (H1/H2 of the PROTOCOL)."

### F2. Certificate is not archived; "proven" inclusion is computer-assisted and reproducible, not line-checkable from the files
- `analysis/sym_rref_cert.json` stores only nterms, positions, ok and the lcm of denominators, not the coefficients lambda of the
  10-row certificates or the W2 combinations used.
- The README says "10 rows of the form g*F_j or v*h". No auditor can check these line by line without re-running `sym.py` (2.5 min).
  I did re-run it and it reproduces bit for bit.
- The mathematical argument itself is sound (checked):
  - The ring homomorphism K[a,b] -> functions, a -> a, b -> b, with a_i = a^(2^i), does not raise bit-degree.
  - It sends model M4 into M4 (x) K, model W2 into W2 (x) K, v*h into Var*W2 (x) K.
  - Specialising a polynomial identity over F2[z,c,1/(cz)] is legitimate.
  - W3 (x) K is Frobenius-stable, because g^2 = g for Boolean g.
  - "Trace components" is the right notion: coordinates of K-valued functions, via the dual basis Tr(theta_t x).
- Residual gap: the lcm c*z^7 is taken by PARI `denominator(content(..))` and `denominator(..)` on bivariate rational functions.
  I did not re-derive it symbolically. It is independently spot-checked: 676 special points plus 180 points on every factor curve
  of the cleared coefficients, with no failure and no dimension change. Not a symbolic proof of "every z*c != 0".
- The bound "n >= 5" is never justified. The homomorphism works for any n, so the bound is harmless but unexplained.

### F3. P4 "stronger form" is partly an artifact of the search range
- `sym.py` was run with the position range [0,4). Variables a_4,b_4 and F_3 do not exist in that run, so
  `positions = [0,1,2,3]` and "only F_0,F_1,F_2" are forced by construction.
- What is genuinely shown: a certificate exists with no carry slot. That is a real strengthening of the prediction.
- The "positions" field itself carries no information.

### F4. Minor wording and accounting
- README says "R1 = c*(RREF row with pivot a0a1b3)". The displayed leading coefficient is c*z^3, and `RESULTS.json`/`relations_cleared.json` give
  multiplier **c*z^3**. The README line should read c*z^3.
- README "CPU used: ... everything else under 10 min" does not account for the aborted exact p=1 run (about 25 min wall, README's own
  figure). This is above the 20-CPU-min single-run budget if it was CPU-bound; the CPU time was not recorded.
- P2 "V-independence" is true by construction in the model, since the Z-model contains no V. The only real V-dependence evidence is the three dump
  checks, each with a different V. The scorecard line "P2 held" should say so.
- P2 also pre-registers "a random unrelated field". L12 used only the `modulus(n)` family (n = 29..37). Re-scored: P2 held, with the
  unrelated-field leg supplied by the auditor's GF(2^16) re-implementation (37/41), not by L12.
- "P2: 15 draws": 5 of the 15 are the fixed L9 `base` draw (s=0), the other 10 are random. All fields use the same `modulus(n)` family.
- The `W2 = 4L^2-7L+1` formula is a fit to six points, not derived.
- brief rule 5 (non-genericity certificate B*D^2 vs N) is not addressed. This is an explanation experiment, not a proposal, so
  this is acceptable, but a one-line "not applicable" would be cleaner.

## Checks that passed (no issue)
- PROTOCOL.md (mtime 17:35:14) predates every code and result file (first is `kech` 17:35:31); never modified afterwards.
- Predictions are scored honestly: P5 is reported as FAILED; padding p=1 exact as NOT done; the n=29 and n=27 failure of completeness is
  stated.
- The "4n is a gap, not a theorem" framing for the real system is correct: the dump checks confirm W3 = W2 + span(Tr R_i) at n=31,
  33, 35 only. The claim that this equality follows from "span = new" plus "each R_i in W3" is logically valid.
- Setup is faithful: F_j = a_{j+1}b_{j+1} + z_j a_j b_j + z_{j+1}(a_{j+1}+b_{j+1}) + c_j matches `sys9.variant` (bil via L(x)=x^2+zx, affine
  terms z^2 a^2, constant a6). The round structure (M4; Var*W2) matches the L9 log (rank 37169, low 2945).
- `kech.c` field arithmetic is confirmed indirectly: my independent GF(2^16) implementation gives identical dimensions and relation membership.
- Outputs 292 KB; dumps deleted (`$TMPDIR/l12d` absent). No security claims. Classification "accounting/explanation, no exponent change"
  is appropriate. novelty_unverified is stated.

## Corrected outcome (what the data support)
1. Four explicit K-relations R_1..R_4 (polynomial coefficients in z, c after clearing c*z^7) lie in W3 (x) K for all z*c != 0 and every
   n, V (computer-assisted certificate; reproduced; 676-point spot check). Their Frobenius shifts too.
2. In the generic model, dim(W3 in Win) - dim(W2 in Win) = 4(L-3) for L=3..8, exact over F2(z,c) for L<=6 at p=0, numeric beyond;
   the all-L statement and the 4n count are conjectural.
3. In the real system at (n,s) = (31,2), (33,0), (35,0): W3 = W2 + span(Tr R_i), 4n new falls, confirmed on bit-level dumps
   (re-confirmed at n=31). Not shown for other n or in general; fails at n=29 (82 extra) and n=27.
