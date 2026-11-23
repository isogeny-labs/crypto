# AUDIT N2-d4-s5-lattice (2026-11-01)

audit = partially_valid. Reran = true (cheap parts only, in a scratch copy; nothing under this directory was modified except this file).
Numbers in README/RESULTS match raw/. Code does what PROTOCOL says. The engineering/no-lever conclusion is not contradicted.
Several statements are stronger than the evidence, the controls do not calibrate what they are presented as calibrating, and two required items are missing.

## Re-run (about 6 CPU-min total, scratch copy)
- rank_test.py: reproduces {rank 59 over random (A,c,xR), rank 9 at fixed curve}.
- closure_val2.py (S3 validation): 0.089996 / 0.116066 / 0.128883, same as raw (lit2026: 0.09 / 0.116071 / 0.128889).
- closure.py d4 1 boxes [4,4,4,8], [6,6,6,12], [8,8,8,16]: 0.000595 (dim 105), 0.003044 (797), 0.004578 (1688); identical to raw.
- run_fplll.py 4096 seed 4: bit-identical to the stored record (below v1/v2/v3 vanish, log2norm 4038.0; above 3851.5; null 4063.5 not vanishing; armX 0/1). Runs are deterministic from the seed.
- Independent check: Semaev S3 in the a2-model, 200/200 sums vanish at p=1000003, 0/50 for a random third x. 1/sum(WDEG) = 1/1680 with 105 monomials; log2 X* formula gives -0.359, 0.707, 1.926, 3.146, 4.365 at 2^256, 2^2048, 2^4096, 2^6144, 2^8192 (matches README/RESULTS).
- NOT re-run: the 2^6144 block (about 3 LLL x 150 s per seed) and the far-above arm. Their raw/fplll_L6144_s*.json flags were read only; they are internally consistent with fplll_summary.json.

## What checks out
- build_h interpolation is a real test: 220 fit points plus 40 fresh points, with an overdetermined consistency requirement; full support 105; even-flip and S4 scalar 1; odd flip not invariant. run_fplll asserts H(planted Ys) = 0 mod p, so the planted root really is a root of the S5-based H.
- Bounds on arm Y (|s_k| <= C(4,k) X^(2k), |t| <= X^4) are correct and are what lattice.bounds() uses. Lattice rows and the unscaling/Z-evaluation in check() are right: "vanish" means exact equality over Z, which is a meaningful event.
- xstar_bits() correctly solves det^(1/n) sqrt(n) = p including the C(4,k) constants.
- Closure code is the unchanged MP-lattices-3 lp.py; validation reproduces lit2026 for S3 (3 rows) and S4 (2 rows). The D4 path is checked against the analytic 1/1680.
- Deviations and the failed "above" prediction are disclosed (Amendments A, B). The arithmetic 5.5/6144 = 9e-4 and the norm shortfalls are right.

## Findings
1. Headline reach claim is not supported as worded. README says "Every reach measured or predicted here is at most about 0.0057."
   - Same README: the l=1 closure series 0.00304, 0.00458, 0.00527, 0.00558, 0.00568 "is not yet bounded"; l=2 and l=3 were not run.
   - The "asymptote <= 1/80" is imported from MP-lattices-4's Monte Carlo, whose own AUDIT (items 4, 5) says it used a weak JM strategy and an identity of normalisation. It is not computed here.
   - Increments are shrinking (1.5, 0.7, 0.3, 0.1 e-3), so the l=1 limit is probably about 0.0058. That is an extrapolation, not a bound.
   - All reaches (determinant and closure) assume generic coefficients, but this H is measured non-generic (finding 5). They are therefore not ceilings for this H.
2. "D4 passes the Y-box at a far smaller dimension" is a comparison against the wrong object.
   - 0.005253 is lit2026 section 3, the closed-form JM-extended box strategy on S5 in x, shift t = 30, so bounding box 38, dimension 39^4 = 2.3M. Not a closure computed in this run, and not on Y.
   - "Y-box = x-box" is assumed, not computed; this is MP-lattices-4 AUDIT item 3, still unresolved.
   - The PROTOCOL's "matched l AND matched dim" table was not produced. The 62.5x dimension ratio is against the 6561-dim JM-basic box (reach 1e-5). The protocol predicted about 8x; this miss is not reported.
   - Either compute the Y-polynomial G(Y) closure with the same lp code at matched l and dim, or retract "passes the box" and the word "edge".
3. Controls are not calibrated.
   - The null (same Ys, random xR) cannot vanish by construction. Any lattice vector with norm < p has a != 0 mod p, so F(Ys) = a H(Ys) != 0 mod p; 0/8 and 0/3 carry no information about the false-positive rate of "v1 vanishes".
   - The only real false-positive probe is diag_degenerate.py: one seed, 2^4096, six vectors, random points in [2,40], and it does not address the +-pair anomaly mechanism.
   - Nothing equivalent exists at 2^6144, where the "above" arm failed. Without it, "8/8 vanish at X = 12" cannot be separated from "short vectors vanish on a structured small-integer locus".
   - The 3 vanishing vectors per instance were never checked for independence or common factors. The experiment tests the Howgrave-Graham vanishing criterion. It is not root recovery (needs 4 independent equations).
4. The empirical threshold "~5.5 bits, delta_emp ~ 9e-4" has no stated method.
   - From the stored v1 norms, extrapolating the two vanishing points (X = 6, 12) to the HG bound p/sqrt(105) gives log2 X about 5.0.
   - Using the X = 128 point gives about 6.6.
   - Only the bracket (3.58, 7), i.e. delta in (5.8e-4, 1.1e-3), is supported.
   - delta_emp is finite-p with constants, so it is not comparable to the asymptotic 1/1680.
5. Rank-59 explanation: plausible, and the README correctly says "not proved".
   - A back-of-envelope supports it: an effective dimension of 59 instead of 105 shortens the shortest vector by p^(104/105 - 58/59) = 2^45.6 at 2^6144; observed shortfall is 35-37 bits.
   - Consequence: the failure threshold is set by the shortest vector, not the determinant, and the observed reach (about 9e-4) exceeds the determinant prediction (5.1e-4).
   - This is exactly the non-full-rank / unravelled class that GAPS says the programme has no tool for. It should be stated as an untested lever, not as a cap. README's "reach stays below 0.006" is only for the generic closure.
6. Arm-X "falsifier" is not discriminating. It uses fiat X = 3 for plants with |x| up to 2^16, so 0/3 is guaranteed. Protocol said 8/8; 3 were run. README says it "failed ... as predicted". The valid evidence is only the log_p measurement (0.998-0.9997), which is expected because c is a random residue. Say that.
7. BRIEF section 0 items missing from README:
   - rule 5 (how, or whether, this could get below B*D^2 << N; here it is a constant-factor device);
   - rule 4 (novelty_unverified, what was searched).
   - README should also say outright that nothing here is root recovery.
8. Smaller items.
   - README's "validated" line quotes lit2026 reference values rather than this run's; use the run's (S4: 0.018776, 0.025589).
   - "about 2^35 under det" is 37 for the below arm.
   - raw/ stores only booleans and norms: no p, A, c, xR or reduced vectors, so the vanish flags cannot be re-checked without a 3-min rerun per instance.
   - CPU use (about 150 min per RESULTS) vs brief's 20 CPU-min/run and the protocol's 1 CPU-hour is not in README.
   - Amendments A/B were written after seeing results (acknowledged; no rescue occurred, since 2^6144 also failed).

## Verdict
Classification "engineering, no lever, no security claim" stands. Do not cite 0.0057 as a bound, do not cite the Y-box "edge", and do not use the null as a calibration.

## must_fix
1. Reword the README headline and RESULTS "gate": l=1 closure at bbox 14 is 0.00568 and still rising (extrapolated about 0.0058); l=2 and l=3 are not run; there is no asymptotic ceiling from this run; the closure and determinant reaches assume generic coefficients.
2. Retract or properly compute the Y-box comparison (finding 2): same-code closure for G(Y) at matched l and dim, or drop "passes the box" and "edge"; report the 8x vs 62.5x prediction miss.
3. State that the null cannot vanish by construction; add a non-root false-positive probe at 2^6144 (v1 at several other small Y-tuples), and check independence or common factors of the vanishing vectors; say explicitly that this is not recovery.
4. Replace "~5.5 / 9e-4" by the bracket log2 X in (3.58, 7), delta in (5.8e-4, 1.1e-3), or document the extrapolation.
5. Present rank-59 non-genericity as an untested lever (determinant and closure are not ceilings for this H), not as support for a cap.
6. Relabel the arm-X fiat falsifier as non-discriminating (X = 3 vs |x| <= 2^16, 3 seeds not 8).
7. Add the BRIEF rule 4 and rule 5 statements to README.
8. Save p, A, c, xR and the reduced vectors (or at least hashes) in raw/; quote this run's own validation values; mention CPU overrun.

---

# Re-audit (2026-11-01, after repair)

audit = partially_valid. Reran = true (cheap checks only, no LLL; only this section was appended to this directory).
Scope note: the user asked for repair items 1-4 only ("do 1-4"). Items 1-4 are addressed with small residuals below. Items 5-8 are open, as the README itself says.
Conductor: `conductor check` could not reach 127.0.0.1:8491 (sandbox "operation not permitted"); the edit is a docs-only append to AUDIT.md.

## Independent checks run
- raw/fp_probe_L6144_s{1 below, 2 below, 1 above}_vecs.json: sha256 of each vector list matches h_sha256 in the json (3/3).
- Re-evaluated v1-v3 over Z: planted invariant point vanishes (3/3 vectors, 3/3 instances); 0 vanishings at the other 139 / 139 / 2001 D4-invariant points in [-X,X]^4 (|Y_i| >= 2). This equals the README. Rank over Q = 3 in each instance. (gcd over Q[s1,s2,s3,t] was not recomputed; the stored value is 1.)
- Reproduction claim: the probe's log2 norms equal the stored main-run norms (s1 below 6094.0/6094.5/6094.5, s2 below 6094.5 x3, s1 above 6113.5/6113.5/6113.0). Confirmed.
- p, A, B, c, xR now stored for these 3 instances.

## Item-by-item
1. Headline and gate reword: DONE. README headline now says 0.00568 at bbox 14, still rising, ~0.0058 extrapolated, l=2/3 not run, no asymptotic ceiling, generic-coefficient assumption vs rank 59. RESULTS.gate matches. Residuals: (a) README line 7 "Below 1/80 and 0.169 only as l=1, generic-model numbers" is still easy to misread as a claim; (b) RESULTS closure_crosscheck.note_superseded still carries "D4 l=1 exceeds Y-box l=1 (0.005253) from dim ~3.2k". It is labelled superseded, but sits inside the live crosscheck block, and no key says it was retracted (see item 2).
2. Y-box comparison: DONE by retraction (acceptable option). The same-code G(Y) closure was not run, and the README says so. 0.005253 is identified as the lit2026 JM-extended x-box at dim ~2.3M with Y-box = x-box assumed. "passes the box" and "edge" are withdrawn in the README. The 8x vs 62.5x miss is reported, with the explanation that 62.5x is against the 6561-dim JM-basic box. Residuals: the RESULTS note above, and README "Classification: Engineering, with the dimension cut real: 105 vs 6561" still leans on that unmatched ratio. Call it a dimension ratio against a different, weaker lattice, not a matched-reach saving.
3. Null, false-positive probe, independence: DONE and verified. The by-construction argument is stated (a != 0 mod p). The 2^6144 probe was pre-registered (Amendment C) before running; its predictions held. The "not root recovery" statement is explicit. Residuals: (a) "3 independent coprime equations" is only linear independence over Q plus pairwise/triple gcd 1. It is not algebraic independence, and 3 polynomials in 4 invariants always leave a positive-dimensional common variety, so reword to "linearly independent and coprime". (b) The probe covers 3 instances and small integer points only, and does not touch the +-pair mechanism of the 2^4096 anomaly. The README discloses both. (c) The probe lives in fp_probe.py and the logs, but the 8-seed table in the README still says "null 0/8" with the caveat inline; fine.
4. Bracket: DONE. log2 X in (3.58, 7), delta in (5.8e-4, 1.1e-3). Endpoints are checked: 3.58 = log2 12 (largest vanishing point), 7 = X=128 (failing, post hoc, v1 norm = p). The old figure is kept as SUPERSEDED in both README and RESULTS. The 5.0-6.6 extrapolation range is stated.
5. Rank-59 as an untested lever: PARTIAL. The headline now says the reaches are not ceilings for this H, but the README still frames rank 59 as a "consistent, not proved" explanation of the short vectors. It never calls the non-full-rank structure an untested lever, and does not say that the observed delta (~(5.8e-4, 1.1e-3)) exceeds the determinant prediction 5.1e-4. Open, outside the "do 1-4" scope.
6. Arm-X fiat falsifier relabel: NOT DONE. README line 14 still says the fiat lattice failed on small-x plants "as predicted", and RESULTS still lists arm_X_fiat_model_falsifier as a falsifier (X=3 vs |x| up to 2^16, 3 seeds vs 8 in the protocol). Open.
7. BRIEF rule 5 and rule 4 statements: NOT DONE. grep of README for rule 5 / B*D^2 / novelty finds nothing. Open.
8. Validation values, 2^35 -> 37, raw data, CPU: PARTIAL.
   - Done for the 3 probe instances only: p, A, B, c, xR, vectors and hashes saved. The main 8-seed runs still hold flags and norms only.
   - Not done: README line 24 still quotes lit2026 values (S4 0.018780 / 0.025591) as "validated in this run" instead of this run's 0.018776 / 0.025589 (and S3 0.089996 / 0.116066 / 0.128883); line 40 still says "about 2^35" (below arm is ~37); the roughly 150 CPU-min total (RESULTS.cpu_minutes_approx) vs the 20 CPU-min/run brief limit and 1 CPU-hour protocol limit is not disclosed in the README. The repair's own 12 CPU-min is disclosed.

## Verdict
The four requested items hold up under spot re-checks and no number I re-derived disagrees with the README. The classification "engineering only, no lever demonstrated, no security claim" still stands. Remaining weaknesses are wording (items 1-3 residuals) and the unrepaired items 6-8 (and the framing in 5).

## must_fix (remaining)
1. Remove or mark the "exceeds Y-box (0.005253)" text in RESULTS closure_crosscheck.note_superseded as retracted; stop using "dimension cut real: 105 vs 6561" as a saving (unmatched-reach comparison).
2. Reword "3 independent coprime equations" to "3 linearly independent, pairwise coprime polynomials"; rewrite README line 7 so it cannot be read as a ceiling.
3. Item 5: state the non-full-rank structure as an untested lever and note that observed reach exceeds the determinant prediction.
4. Item 6: relabel the arm-X fiat falsifier as non-discriminating; keep only the log_p measurement.
5. Item 7: add the BRIEF rule 5 (constant-factor device wrt B*D^2 << N) and rule 4 (novelty_unverified, with what was searched) statements.
6. Item 8: quote this run's own validation values; change 2^35 to ~37; save p/A/c/xR/vectors or hashes for the 8-seed main run (needs a rerun or the original seeds); disclose the ~150 CPU-min overrun.
