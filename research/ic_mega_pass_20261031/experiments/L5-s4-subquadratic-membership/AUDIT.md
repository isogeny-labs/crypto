# AUDIT: L5-s4-subquadratic-membership

Auditor verdict: **partially_valid**. The computation is correct and the negative result stands as a toy result. Several statements in README.md claim more than the data shows (items 1-4 below), and none of them changes the sign of the conclusion.

## What was re-run (about 1 CPU-minute total, single thread, scratch in the session scratchpad)

| check | outcome |
|---|---|
| PROTOCOL.md sha256 vs RESULTS.json | match (e440b24d...). File mtimes: PROTOCOL 18:43, memb.c 18:44, first raw output 18:55. Pre-registration order is consistent. |
| `python3 analyze.py` | regenerates RESULTS_grid.json byte-for-byte (the audit re-run rewrote the file with identical content, so its mtime is now 19:06). |
| `memb.c`, p=1048571, n=32, modes 0/1/2, seeds 1 and 2 | reproduces the raw/grid.jsonl.gz rows exactly (the RNG is deterministic). |
| Independent ground truth | I patched a scratch copy of memb.c to dump F and 700 target decisions, then recomputed X3 with PARI `elladd`/`ellneg` over all i<=j<=k and 8 sign patterns. |
| | |X3| = 21263 (RAND), 5484 (TCL), 1408 (E2CL), all equal to the C values. |
| | 0 decision mismatches in 700 targets per mode. All F points are on-curve, and E2CL F is closed under the (0,0) shift. |
| Mutation tests (are the controls able to fail?) | Restricting the quotient arms to one shift gives 113 and 159 mismatches. Deleting the "difference" lookup gives 53, 42 and 50 mismatches. The brute-force X3 control is sensitive. |
| `arm_c_pari.py` | re-run in 33 s wall: 0 mismatches, S3_validation_bad = 0, identical gcd-call counts. CPU per target agrees within about 10% (0.00074 s at n=8, 0.0431 s at n=64). |
| Primality | 1048571, 16777199 and 268435399 are prime and are 3 mod 4. |
| README numbers vs RESULTS | The four main-table rows match RESULTS_grid.json. The B/A list matches. The scorecard ranges match, except for the item 3 wording. |
| Saturation explanation | Expected distinct x = M(1-exp(-N/M)), with M=p/2 and N=(2/3)n^3. This gives 65.3M for p=2^28, n=512 against 65.3M measured. For p=2^20, n=64 it gives 148.6k against 148.0k measured. The README's explanation of the P2 excursions is numerically confirmed. |

Resources: the directory is 76 KB, raw logs are gzipped, and everything ran single-threaded. This complies.

## Findings (ordered by importance)

1. **"Measured S*D/|X3| = 2.9-3.5" is partly an identity, and the falsifier cannot trigger by construction.**
   - In analyze.py, `D_lookups = 2*n` is hard-coded for every arm. Only S and |X3| are measured.
   - The ratio is therefore (2n * #distinct pair x-values) / |X3|, a combinatorial constant (about 2n^3 / (2/3 n^3) = 3) that is flat by arithmetic.
   - The no-table loop (S=n, D~n^2) and the full-X3 table (D=1) are not in the grid. README says "Three methods sit on the line ... measured ... for every arm", but those two are analytic.
   - Every lookup-model arm is an instance of the same scheme, so the pre-registered falsifier ("S*D/|X3| decreasing with n_F") can only fire on a different algorithm. Only arm C is a different algorithm, and its S*D was not computed.
   - The substance of the result is the counting argument plus the operation counts. It is not an empirical test of the bound.

2. **The headline "Answer: negative" is ambiguous, and the counting argument is close to a tautology inside the model.**
   - The pair table already gives online D = 2n_F, which is sub-n_F^2 per target, with S = n_F^2.
   - What is excluded is S*D = o(n_F^3) in a non-adaptive equality-lookup model.
   - README should say which reading of "sub-n_F^2 structure" (Lever 2) is being answered. As written, a reader may take "negative" to mean that no sub-n_F^2 online cost exists. The pair table contradicts that reading.
   - The model excludes batching and sorting across targets, adaptive chains (acknowledged in scope 1), and algebraic steps (acknowledged).

3. **P2 scorecard.**
   - The "below saturation" qualifier was not pre-registered. PROTOCOL has only a parenthetical on RAND's |X3|, and the pre-registered bands were [0.20, 0.30] and [0.05, 0.08]. The scorecard should read "PASS in 12 of 16 arm-cells, FAIL in 4 saturated arm-cells (explained)", i.e. partial. The explanation is correct (see the saturation row above), but the verdict was scoped after the fact.
   - README says the near-saturation values "0.284-0.384 and 0.074-0.109" are "outside the band". Two of those values (TCL 0.284 and E2CL 0.074, both at p=2^20, n=64) are inside the pre-registered bands. The outside values are TCL 0.315 and 0.384, and E2CL 0.084 and 0.109.
   - The same cell (p=2^20, n=64) is treated as unsaturated for the S*D range (3.52 is quoted inside "2.9-3.5") and as saturated for P2.

4. **Arm C "fitted exponent 2.01" does not support the asymptotic claim.**
   - The 2.01 comes from two endpoints (n=8 at p=2^20, n=64 at p=2^24) over a factor-of-8 range of n, with one seed and 50 targets. A least-squares fit over the five points gives 2.02, and the p=2^20-only fit gives 1.88.
   - The code builds the product with `g *= S3(...)` in a loop. That is naive and O(n^2) per gcd call, so the algorithm as run is asymptotically cubic per target. The `formula_M_per_target` string (O~(n^2)) describes a product-tree implementation that was not run.
   - At n<=64 the PARI interpreter loop dominates, which mimics n^2.
   - My probe of per-gcd-call time at n=32, 64, 128, 256 gave local exponents of 1.64, 1.30 and 1.87 (noisy). The per-target exponent would then be about 2.3-2.9, not 2.0.
   - So P5 "PASS" supports only "about n^2 over n<=64". The Õ(n^2) claim rests on the formula. Neither the arm C S*D claim nor its "dominated" label depends on the exponent, because the argument in (a) does the work.
   - P5's second clause (wall time >> A) is not measured. A is reported in M-equivalents and C in seconds, on different n. It is obviously true but not shown.

5. **"Relabelling with a constant-factor table saving" is not what the arithmetic gives at equal yield.**
   - At equal |X3|, TCL needs n_TCL = 4^(1/3) n, which gives a table about 0.63x and online D about 1.59x. S*D is unchanged. This is a table-versus-online trade, not a saving.
   - The equal-|X3| comparison was not measured. Only equal-n_F cells exist, and the claim follows from the constancy of S*D/|X3| (which is item 1).

6. **Citation provenance.**
   - README presents the bound as "the standard 3SUM-indexing counting bound (Demaine-Vadhan; Golovnev-Guo-Horel-Park-Vaikuntanathan 2020)".
   - No search was run in the task, so the attribution is from memory and unverified. The `novelty_unverified` tag is present but covers only novelty, not the attribution.
   - Known 3SUM-indexing algorithms are adaptive and sit outside this counting bound, which scope 1 does say.

7. **Smaller points.**
   - The B/A list in README omits the saturated cell (592 at p=2^24, n=256). It is in RESULTS.json. The ratios double with 2.07-2.2x per doubling of n, slightly super-linear, which is consistent with early exit.
   - Method B ran on only 120 targets per run (100 random and 20 planted). "0 mismatches for A, quotient and B across 48 runs" is accurate, but B has much thinner coverage than A.
   - In the positive-decision verifier, the `fk==-2` branch (x(R+T) in x(G)) sets ok=1 without an explicit check. For the quotient arms the verifier checks R = +-Q +- P3 + t, not the F-element decomposition. The ground-truth comparison is independent of this, and my PARI check covers it.
   - memb.c and the ground truth share one group-law implementation. My PARI recomputation removes that concern.
   - Per-relation cost uses the predicted hit rate |X3|/(p/2), not the measured one. This is fine, but the near-saturation cells are affected, and README does note them.
   - The Rule-5 certificate is arithmetically correct (1.5p/n^2 per relation, so B*D_rel^2 about 2.25 p^2/n^3, which is >= r for n <= r^(1/3) taking p about r). It is a statement about this scheme only.

## What is solid

- The code does what PROTOCOL.md says: methods A, B, Q_T, Q_E2 and C, the targets, and the p and n_F grid. No protocol deviation was found.
- The v1 convention bug in arm C was caught by the brute-force control, fixed, and the buggy output kept. The raw file confirms 1 mismatch in v1 and 0 in the final run.
- The controls are real: brute-force X3, planted positives, PARI independent recomputation, and mutation sensitivity.
- The yield ratios (1/4 and 1/16 below saturation), the 4x and 17x per-relation penalties, and the linear growth of B/A are reproduced.
- Classification (relabelling, not advance) and "no security claims" are appropriate.

## Suggested corrections

- Item 1: say "S and |X3| measured, D = 2n set by construction". Drop "measured" for the B and full-table arms.
- Item 2: state the exact question answered (S*D = Omega(n^3) in the non-adaptive lookup model), and say plainly that the pair table gives D = O(n) per target.
- Item 3: P2 = partial pass, with the saturation formula given.
- Item 4: "about n^2 over n<=64; the code is a naive product, asymptotically worse; Õ(n^2) is a formula for an unrun variant".
- Item 5: "table x0.63, online x1.59 at equal yield; S*D unchanged".
- Item 6: mark the attribution as unverified.
