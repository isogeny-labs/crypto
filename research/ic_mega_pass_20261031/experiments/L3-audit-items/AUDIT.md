# AUDIT: L3-audit-items (2026-11-01)

Verdict: **partially_valid.** The numbers L3 reports reproduce exactly. Three things are wrong or over-stated:
the C1b status and the "blocker" behind it, the "isolated to that one curve" claim for N3 m=3, and the
unreported partial failure of the pre-registered C2 prediction. No security claim is made or implied; none found.

CPU used by this audit: about 2 CPU-min, 1 thread. Scripts and verbatim outputs: `audit_scripts/` (OUTPUTS.txt).

## Reproduced exactly
- `diag_n3_p13.py`, re-run in scratch: (npts, kernel dim) = (149,1), (181,3), (277,1) for p = 11, 13, 17. `raw_n3_p13_diag.json` is identical to the re-run.
- Independent re-check with my own point enumeration and PARI `matrank` (no n3lib.evalf/nullspace): p=13 seed 0 has 181 points and kernel dim 3. C1a (exhaustive dim 3) holds.
- `diag_n3_p13_fp2.gp` re-run: 1. I also drew a fresh sample (403 points of V(F_169), none in F_13) and expressed f in the kernel basis: f = 2*K1 + 12*K2 + 1*K3, and the F_169 kernel vector is exactly [2,12,1]. So the surviving element is f. The claim "only f survives on V(F_169)" is correct.
- N1: `l3_validation.py` re-run (festest 30/30, brute-force n=19 36/36, 0 mismatches). The output JSON is identical to `crosscheck/l3_validation.json`. My own tabulation of raw n=19..31 matches the README Addendum table (n=29 D1 7/8 with one record at low_4=2536; n=31 A 9/9, C1/C2/D1 0/8 stalled at 2821, B/RM at n(N+1)=1023, RM 0/9). Total child CPU 3794.8 s matches `run_main.log`.
- N3: `cpu_seconds_approx` sums to 4192. N2: this run's closure validation values (0.089996/0.116066/0.128883, 0.018776/0.025589) match `raw/closure_validate*.json`. N4: every cited number (0.010871 vs 0.010886; 0.103-0.187 at n=13 j=3; 0.0715/0.0744/0.0770 and 0.1364 at n=17 j=5) matches `raw/armA_n17_*.json` and `raw/armB_*_P.json`.

## Findings

### 1. C1b is falsified, not "not confirmed", and the stated blocker is false (must fix)
README/RESULTS say the factorisation of f "was not computed because there is no multivariate F_p factoring in the tools", and record C1b as "not confirmed".
PROTOCOL says C1b is "Falsified if f is absolutely irreducible over F_13 (checked over F_13 and F_169)".
A cheap sufficient test exists: restrict f to a random plane z = c0 + c1 x + c2 y, apply the Kronecker substitution y -> X^9, x -> X, and test the univariate image for irreducibility. An irreducible image proves the restriction is irreducible, and hence that f has no non-trivial factorisation over that field.
Results (`audit_scripts/pl13.gp`, `pk2.gp`, `pk3.gp`): the image is irreducible for f at p=13 over F_13 (220 tries), F_169 (98) and F_13^3 (44). It is also irreducible for the p=11 and p=17 controls. The point count (181 against p^2 = 169) is consistent with one geometric component.
Consequence per PROTOCOL: C1b is falsified over F_13 and F_169 (not shown absolutely irreducible: I checked k = 1, 2, 3 only). The protocol's fork then says the cause is "undiagnosed". The kernel basis is saved, so that clause is satisfied.
Required edits: RESULTS `C1b_reducibility` and README ("C1b was not confirmed") should read "falsified (f irreducible over F_13, F_169, F_13^3 by plane-section + Kronecker test)". Remove the "no factoring in the tools" blocker.

### 2. "Point-set artifact" is a description, not a diagnosis (must fix wording)
The observation is real: the 181 F_13-points impose only 122 independent conditions on the 124-dim restriction space, and the two extra elements do not vanish on V(F_169). That restates the failure; it does not explain it.
- RESULTS `diagnosis` and N3 `outcome` present "F_13 point-set artifact" as a diagnosis.
- N3 README's "same kind as the m=2 Riemann-Roch excess" is an analogy. No degree or Riemann-Roch argument is given for m=3.
- With C1b falsified, no tested hypothesis remains. Record the cause as undiagnosed and the F_169 result as a property of the failure.

### 3. "The p=13 excess is isolated to that one curve" is contradicted (must fix; N3 README, not L3)
N3's extra check (raw/armA_m3_extra.json) used one seed at p = 19, 23, 29 and two at p=13. Both p=13 entries show the same npts = 253 and identical gains. The generating script is not saved, so a duplicate curve cannot be excluded.
I ran the same instrument (m=3, l=1, box4, exhaustive F_p points) on 4 curves per prime (seed 0 = Random(p), seed s = Random(5000*s+p)). Entries are (npts, kernel dim); f always contributes 1.
- p=11: 1,1,1,1
- p=13: (181,3), (101,45), (161,27), (193,27)
- p=17: 1,1,1,1
- p=19: (569,1), (199,1), **(235,2), (275,2)**
- p=23: 1,1,1,1
Notes:
- p=13 s=2 and s=3 have rhs(xR)=0 (degenerate, the m=2 exclusion applies). s=1 has 101 < 125 points, so excess is forced.
- The two p=19 cases are non-degenerate with 235 and 275 points, well above the 124 conditions. They were confirmed by the independent PARI rank (`indep.py`).
- Among non-degenerate cells with npts >= 125 (17 cells), 3 have excess (18%), at p = 13 and 19. This is a small sample with no trend in p over 11..23.
Consequence: the failure of c = Sum k at l=1, m=3 is not a one-off curve. N3's ceiling stays conditional on P, and "artifact at small p" must not be read as "vanishes at larger p". L3 did not make that claim, but its closure of the p=13 item inherits "isolated".

### 4. C2: the pre-registered prediction partly failed and this is not reported (must fix)
PROTOCOL C2 predicted n=27 and n=29 keep A/C1/C2/D1 with 1 in W_4 on all UNSAT draws. Raw shows n=29 D1 7/8 (one UNSAT draw at low_4=2536 without 1 in W_4). That is a partial falsification of the prediction. The n=31 stall prediction held. L3 README/RESULTS say "equals the Addendum table" and never compare it with the prediction. State: "C2 prediction partly falsified (n=29 D1 7/8); n=31 stall reproduced from raw".
Also: this recount reads the same raw fields with the same selection rule as the author's analysis. It is a recount, not an independent derivation, and the n=31 stall remains unverified by any N=32 closure (L3 says so). One thing worth noting: low_4 = 2821 is identical on all 21 C1, 84 C2 records; D1 spreads over 2821..2839 across 81. A constant that does not depend on the instance is a signature of a structural or instrument limit, and no test separates the two here.

### 5. Process and smaller items
- Conductor: BRIEF rule 6 says Conductor T-5736 holds the directory and agents must not run `conductor check`. L3 ran it and retried it unsandboxed (the retry timed out). It is disclosed under Blockers, but the retry was against the brief. I did not run it. The root CLAUDE.md asks for a check before code changes, and I followed the BRIEF and the task's write-only-under-experiments rule instead; my only write in the tree is this file plus `audit_scripts/`.
- "Total CPU < 5 min at 1 thread": no timing log is saved. My re-runs are consistent with it, so it is plausible but unrecorded.
- N3 `n3lib.py` default threads were lowered after the recorded runs (comment in the file; disclosed in N3 README). Acceptable, but it means the saved code differs from the code that produced the raw data.
- `__pycache__` in N3: L3 removed it. My re-run recreated it (my import of n3lib); I removed it again and verified it is absent.
- N3 `cpu_seconds_approx` entries are round numbers (1200 is the timeout, 700, 600, 200). The 4192 s total is an estimate, not a measurement.
- N2 "2^37 under det^(1/n)" I could not re-derive without rebuilding the lattice. Not checked. N2 item 5 (rank-59 test) and item 8 (raw for the 8-seed run) remain open as L3 states.
- N4: ratio ranges in RESULTS left as is (already marked superseded). Fine.

## Controls
- p=11 and p=17 controls in C1 are real: they use the same code path and give dim 1, and I confirmed this independently. They cannot show that the p=13 excess is a "point-set artifact" rather than another cause, since they differ in both p and curve. My 4-curve scan is a better control and contradicts "isolated".
- No planted-positive control was needed for rank computations. I cross-checked the rank with two separate implementations.

## Corrected outcome
- C1a: confirmed. C1b: falsified (f irreducible over F_13, F_169, F_13^3). Cause of the m=3 p=13 seed-0 excess: undiagnosed. Descriptive facts only: the extra kernel elements vanish on V(F_13) and not on V(F_169) (sample); similar excess occurs at p=19 on two non-degenerate curves; it is not an isolated curve at the sizes tested.
- C2: n=31 stall reproduced from raw (A 9/9, C1/C2/D1 0/8 at low_4=2821). n=29 D1 is 7/8, so the "all UNSAT draws" prediction failed in one cell. The stall is not independently verified.
- N1/N2/N4 closures: verified as stated, with the partials L3 itself lists. Classification unchanged (accounting/engineering, no advance). No security claim.
