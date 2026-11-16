# N4-char2-xfilter: outcome (REPAIRED, see Repair section): no growth seen; powered only for j=1 and for the visited j>=2 tuples; inconclusive for rare structured subspaces. Class: accounting. Toy only.
[SUPERSEDED original headline: "outcome SUPPORTS H0 within range (no growth of G(M) seen on <P>)"]
Target: KN-OPEN-91d4f6 (characteristic 2, F_2-linear x-functional filters, single-bucket gain G(M)=M*max_{c,d} pi_c(d) on the prime-order subgroup).
REPORT section 6 step 4. Novelty: novelty_unverified (no literature search). No security claim of any kind.

Files: PROTOCOL.md (pre-registered, plus amendments 1-3 made during calibration), src/ (enum2.c, lib.py, validate.py, calib.py,
calib_K3b.py, arm_A.py, arm_B.py, lanes.sh, analyze.py, curves.gp), raw/ (all JSON/logs), RESULTS.json (aggregated by analyze.py).
Tools: C enumerator (-O3 -march=native), numpy/scipy FFT, PARI (cypari2/gp) for point counts and checks. At most 3 processes at once,
plus calibration overlap (see amendment 3). The machine was heavily loaded (load average ~45).

## Instrument (validated before any real run; raw/validate.json)
- Enumeration of x(kG) checked against PARI ellmul: 0 mismatches. Checked n=7,13, both domains, ord(G) exact, x(kG)=x(-kG).
- Exact G statistic: WHT over the alphabet x cyclic convolution over Z/L, with XOR combiner. It equals brute force over all L^2 pairs (max |diff| = 0.0).
- Positive control: Tr(x) filter on E(F_{2^n}) gives G=2.000 (h=4) and G=1.9995 (h=2). The deviation is x(O)'s label, an O(1/L) effect.
  On <P> the same filter is exactly constant, so degenerate, as the card states.
- Koblitz n=13: G(beta)=G(beta^2) on 200 random beta, max |diff| = 0.0 (P2 confirmed).

## Calibration (planted dose-response, before the real arms)
- K1 (statistic): labels forced to 0 on a dlog interval of fraction rho. Detected above the max of 8 PRF nulls at every (n,j) cell.
  Smallest detected planted excess E*: j=1: 0.071 (n=13) down to 0.004 (n=23). j_{1/4}: 0.21 (n=13), 0.12 (17), 0.07 (19), 0.08 (21), 0.17 (23).
  j_{1/3}: 0.33 (13), 0.23 (17), 0.39 (19), 0.37 (21). For n>=17, rho=0.005 (the lowest dose) was already detected, so E* there is an upper bound on the floor.
  No K1 cell for j=2, or for j values specific to the h=4 curves. Those cells are compared only against the nulls.
- K2 (exhaustive j=1 pipeline, plant through the x-data): the scan's argmax = m* and exceeds the NULL-1sym family max at rho>=0.05 (n=13,15). It misses at rho<=0.02.
  AMENDMENT 1: the first K2 plant broke Tr-constancy, and the scan correctly flagged the trace mask with E=0.997. The plant was fixed. That flawed run is kept.
- K3/K3b (climber pipeline): the oracle (planted tuple evaluated) detects at rho>=0.02 (n=13) and rho>=0.005 (n=17).
  The 24-evaluation climber found the plant at NO dose, as pre-registered. Its power is zero against a planted j-tuple it does not hit.
  K3, the single-functional plant, did not transfer (amendment 2). K3b first hung on a dependent planted tuple (amendment 3).

## Real results on <P> (RESULTS.json)
- Arm A, exhaustive j=1 over all 2^n-1 functionals:
  - Exactly 1 degenerate functional, the trace, on every curve: h=2, h=4 and Koblitz. P1 confirmed.
  - The max excess on REAL is at or below the NULL-1sym family max:
    - n=13: 0.0439 vs 0.0441-0.0446 (random1), 0.0424 vs 0.043, 0.0776 vs 0.079-0.081 (h=4), 0.0747 vs 0.079 (Koblitz).
    - n=15: 0.0221 vs 0.0220-0.0221, 0.0216 vs 0.022, 0.0408 vs 0.041 (h=4).
    - n=17 (random1, all 131071 functionals): 0.010871 (REAL) vs 0.010886 (null) (1 null replicate). [L3: was "0.01091 vs 0.01091".]
  - REAL/null-max ratio 0.93-1.00 (the 0.932 cell is Koblitz, unmatched null; non-Koblitz 0.957-1.000). No functional stands out. The K2 floor is E~0.05 at n=13,15.
- Arm B, climber, j in {2, j_{1/4}, j_{1/3}}, 53 <P> cells over n=13..23 and all curve types:
  - REAL E / NULL-1sym E: median 0.999, range 0.95-1.11. [SUPERSEDED as evidence: "REAL never exceeds max(NULL-1sym, PRF) by more than E*" holds by construction, see Repair item 1.]
  - The PRF (SHA-stand-in) null max is >= REAL in almost every cell.
  - At fixed theta=1/4, E falls with l: about 0.103-0.187 (n=13; L3: was "0.10-0.24", 0.239 is a theta=1/3 cell), 0.048 (17), 0.022 (19), 0.011 (21), 0.007 (n=23, M=32).
    [SUPERSEDED, claim withdrawn: "This decay matches the Weil-type heuristic E ~ M^2 sqrt(h/l) and the null equally."]
    E is dominated by the marginal bucket-imbalance floor, which NULL-1sym shares (see Repair item 2).
  - At theta=1/3: 0.16-0.24 (n=13), 0.0715-0.0770 for h=2 and 0.1364 for h=4 (17; L3: was "0.074-0.077"), 0.048-0.050 (19, M=64), 0.031 (21, M=64). It also falls. [L3: j = floor(theta*log2 l), so M drifts and theta_eff runs ~0.20-0.25 across n (e.g. n=21 M=16, n=23 M=32 vs l^{1/4}~45); the cross-n trends compare unequal theta_eff.]
- Arm C (E(F_{2^n}), random1, j_{1/4}, n=13..19): REAL tracks NULL-1sym. The climber did not draw the trace functional, which is the only known exact gain there.
- Observation: REAL and NULL-1sym climbers usually end on the same masks with E equal to 3-4 digits.
  The statistic at these sizes is dominated by the bucket-size (marginal) floor, which NULL-1sym preserves. The structure term is below resolution.

## Verdict, per the pre-registered rule
[SUPERSEDED: "SUPPORTS H0 within range: in every valid cell REAL <= max(nulls) + E*, and the exhaustive j=1 max is within the null family range." The E* rule is vacuous; the verdict now rests on same-mask z-scores, see Repair.]
Scope of the negative:
- (a) j=1: all F_2-linear functionals, exhaustively (n=13,15 all curves; n=17 random1).
- (b) j>=2: only a 24-evaluation random climber. K3b shows this has no power against a specific planted tuple.
  So the j>=2 arm excludes only "typical" filters. It does not exclude a rare good subspace.
- (c) l <= 2^22. This cannot say "bounded" asymptotically.
- (d) XOR combiner only. Other combining rules f are not tested.
- (e) Level 1 of a Wagner tree only.
[L3: "consistent with" withdrawn as vacuous; no bound is tested here.] A possibly relevant obstruction is the Weil/Bombieri bound on additive character sums Tr(beta*(x(R)+x(K-R))) on E. [SUPERSEDED: "This gives E <~ c M^2 sqrt(h/l), so it is bounded for M <= l^{1/4}." The constant c is unspecified and the data do not test this bound; observed E is the marginal floor.]
It is not a proof for M ~ l^{1/3}, and the Artin-Schreier-degenerate cases (only Tr on <P> was observed) are not classified beyond the j=1 scan.

## Not run / cut (reasons)
- j_{1/3} at n=23 (M=128 x 2^23-length FFTs per evaluation, over budget). h=4 and Koblitz curves at n=21,23, also budget.
- n=19+ exhaustive j=1 (2^19 functionals x 0.02-0.1 s each).
- Koblitz Frobenius-orbit-preserving null (not built). NULL-1sym does not preserve the Frobenius orbits.
- A real SHA-256 null: replaced by a keyed splitmix64 PRF for speed (declared in PROTOCOL).
## Next step
To give the j>=2 arm power, replace random climbing with an exhaustive or structured search over 2-dim subspaces at n=13 (~2^24 subspaces, needs C).
Or derive the Weil bound for the XOR single-bucket rate explicitly. With an explicit constant and the actual E-vs-M scaling this could bound the theta<=1/4 regime; that is not established here. [L3: was "Then the theta<=1/4 regime is a theorem".]

## Repair (2026-11-01, after AUDIT.md; PROTOCOL.md Amendment 4 written before the re-run)
Scope: audit must_fix items 1-4, as the user asked ("1-4"). Item 5 (README number slips: n=17 Arm A 0.010871 vs 0.010886;
theta=1/4 n=13 range 0.103-0.187; theta=1/3 n=17 range missing 0.0715 and 0.1364; theta_eff drift 0.2-0.25) is NOT fixed in this pass.
The numbers above are left as they were; superseded sentences are marked [SUPERSEDED].
New code: src/repair_samemask.py, src/repair_analyze.py. Raw: raw/repair_samemask_*.json, raw/repair_samemask.log, raw/repair_analyze.log.
RESULTS.json gains a "repair" block; summary.cells_exceeding_floor is marked as using the retired rule.

1. **E* rule retired.** In all 29 cells that have an E*, E* > E_real (1.11x to 22.9x), so "REAL <= max(nulls)+E*" cannot fail.
   24 of 53 cells had no E* and were scored "not exceeding" by construction. New decision statistic (pre-registered in Amendment 4):
   same-mask z. The saved REAL best tuple of every Arm B <P> cell is re-evaluated on 10 fresh NULL-1sym permutations of the same x-data
   (seeds 9000-9009). The REAL E re-evaluates exactly (max diff 0.0). Results for the 42 non-Koblitz cells: z from -2.62 to +2.58.
   No cell has z > 3.5 (the threshold), so the pre-registered P-R1 holds. The largest values are n=13 random1 a=0 j=2 (+2.58) and
   n=17 random1 a=0 j=2 (+2.46), both h=4. They are within what 42 draws produce and are not followed up. The climber REAL/NULL-1sym ratio
   (median 0.999, range 0.95-1.11, 1 null climber per cell) stays as a description only. Arm A non-Koblitz REAL/null-max ratio: 0.957-1.000.
2. **Weil-heuristic sentence withdrawn.** With M ~ l^{1/4}, M^2 sqrt(h/l) is flat in l, and the constant is unspecified, so it cannot explain a decay.
   Measured: marginal-only E_marg = M_eff*max_c n_c/L - 1 on the REAL tuple divided by E_real has median 0.982 over 53 cells (range 0.61-0.999).
   The low ratios are the n=13 cells with j=4 and the h=4 j=5 cell (n=17: 0.108 vs 0.136). In those cells the same-mask null mean matches
   E_real too (n=17 h=4 j=5: null mean 0.1394 vs REAL 0.1364). E is dominated by the marginal bucket-imbalance floor, which NULL-1sym shares.
   The decay of E with l is the decay of that floor. It is not evidence about Weil-type structure.
3. **Koblitz cells flagged and excluded.** Arm A n=13 Koblitz and Arm B n=13, 17, 19 Koblitz (11 cells) have an unmatched null. NULL-1sym
   destroys Frobenius orbits, which biases REAL low (toward H0). Audit figure, quoted not re-run: Arm A n=13 REAL 0.0747 vs 12-replicate null
   mean 0.0788, z=-4.8. Same-mask z for the Koblitz Arm B cells: -2.74 to +0.74 (prediction P-R3: negative tendency; 9 of 11 are negative).
   They are excluded from the tally. No Frobenius-orbit-preserving null was built (not in this pass).
   P2 (G(beta)=G(beta^2)) is an identity from the Frobenius automorphism. It validates the code and is not evidence about growth.
4. **Headline rescoped.** The outcome is: no growth seen. It is powered for j=1 (exhaustive) and, for j>=2, only on the 24 tuples each climber
   visited. The same-mask test checks only the final best tuple, with null sd 0.1-5% of E. It is inconclusive for rare structured subspaces
   and for theta > ~0.25. Calibration limits: the K3b oracle margins against NULL-1sym at the lowest detected doses are small
   (n=13 rho=0.02: 0.1803 vs 0.1691, 1.07x; n=17 rho=0.005: 0.1027 vs 0.0976, 1.05x; re-read from RESULTS.json calib_K3b).
   "Detected" there mostly means exceeding the PRF K1 max. K2 ran only at n=13 and 15, and it compared against the unplanted data's null
   family, so it mostly tests the marginal change. Arm A n=17 has no K2 calibration. The climber had zero power against a planted tuple at every dose.
No security claim. Not a bound.

## Closure of audit items (L3, 2026-11-01)
- Item 5a (0.010871 vs 0.010886): done (README line 35; value grep-confirmed in raw/armA_n17_random1_a1.json).
- Item 5b (theta=1/4 n=13 range 0.103-0.187): done.
- Item 5c (theta=1/3 n=17: 0.0715, 0.0744, 0.0770, h=4 0.1364): done.
- Item 5d (theta_eff drift 0.20-0.25): done (note added at the theta=1/3 line).
- Re-audit residual, line 57 "consistent with" Weil: done (withdrawn).
- Re-audit residual, line 67 "a theorem": done (softened).
- Re-audit residual, Arm A ratio 0.93 includes Koblitz: done (flagged inline).
- RESULTS.json summary ratio ranges still include Koblitz cells: partially (already marked superseded; not rewritten, to keep the generated file unchanged).
- Next-step items (Frobenius-orbit-preserving null; exhaustive 2-dim subspace search at n=13): not audit items; not done.
