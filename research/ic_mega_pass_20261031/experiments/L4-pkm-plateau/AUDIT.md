# AUDIT: L4-pkm-plateau (auditor pass, 2026-11-01 ~18:48-18:53 PST)

Scope: snapshot audit of an experiment that was still running (nohup chains A/B). At audit time
RESULTS.json did NOT exist, README.md was marked "interim", and the null arm, the P2=998244353 arm,
msolve null and msolve t=10/11 had produced no rows. Everything below concerns what existed then.
Re-check list for finalisation is at the end.

## Verdict
partially_valid: the code and the rows that exist are correct and independently reproduced; controls
are real but the original planted control is weaker than the README wording for t>=4; README numbers match
raw rows (README is stale, not wrong); final deliverables are not yet present. No security claims, no
C-step overclaim found.

## What I re-ran (all single-thread, ~1-2 CPU-min total; my scratch is outside the experiment dir)
1. Rebuilt closure.c (`cc -O3`, md5 of source identical to the audited file) and re-ran the full
   Kummer P1 closure ladder t=1..6, seeds 1..4 (24 cells) through the experiment's own gen.py/run_closure.py.
   Result: coefficients, per-c (ncols, dimV, dimLow, unit) and RESULT lines identical to raw/closure_kummer_P1.jsonl
   in 24/24 cells (only wall-time differs).
2. Independent re-implementation of V_{F,c} (audit/indep.py: dense numpy mod p, fixed-point iteration instead
   of the C queue, degree computed with Python popcount): kummer t=2,3,4 seeds 1,2 and t=5 seed 1 give the same
   (c, ncols, dimV, dimLow, unit) tuples as the C code, e.g. t=5: (3,176,11,1), (4,386,386,176,unit).
3. Semaev check (audit/semaev_check.py): gen.S3coeffs evaluated at x_i = g*z_i vanishes on 1000/1000 genuine
   (P,Q,P+Q) triples on random curves mod 10007 (and a random wrong x_R did not vanish). So the polynomial
   encodes S_3 correctly, including the g^(e1+e2) rescaling.
4. mk_ms.py: evaluated the generated msolve input at random points: the y-relations vanish on mu_K x mu_K and the f line
   equals sum c_ij z1^i z2^j (0 mismatches for t=1,2,3,5). The msolve D parser (regex on the "deg sel pairs mat" table)
   reads the right column: my own awk parse of fresh msolve runs gives D=4,4,5 for t=2,5,6, equal to the rows.
5. Controls, stronger than the originals: planted (consistent) targets re-run with cmax = 2t+2 (the full ring, since
   max degree is 2t) for t=2..5: dimV = 2^(2t)-2 and unit never appears (conclusive: the two grid zeros survive).

## Findings
F1 (code vs PROTOCOL) OK. Closure = exact V_{F,c} as specified, d_F = least c with unit, ffd by dimLow(c) > dimV(c-1),
   cap/time limits give censored output ("dF>c" or "dF>=c"). Decomposable draws (grid zero) are excluded, not redrawn
   (none so far). gen.py grid-zero test, planted construction and S_3 are right.
F2 (controls) Real and discriminating, with caveats:
   - Gc summand d_F=2 (t=2..7): real but a weak sanity check (f is degree-1 in the y variables).
   - Gd planted: as recorded in gates.jsonl the cap was c<=6 (and 60 s at t=7), which is conclusive only for t=2,3
     (2t<=6). For t=4..7 the recorded evidence is only "no unit up to c=6". Item 5 above closes t=4,5; t=6,7 remain
     cap-limited. README's "planted never reaches 1 (t=2..7)" should read "up to c=6".
   - Ga/Gb/Ge hold on all rows seen: ffd=4 for t>=2 (t=1 gives 3, outside the gate), d_F<=2t+2, d_F<=D_msolve.
   - Closure vs msolve agreement (D=d_F on every common cell, t=1..8) is the best independent evidence; the two codes share
     no implementation.
F3 (README vs raw) README numbers are accurate for what they claim: closure d_F t=1..6 = 3,4,4,4,4,5 (4/4 draws each,
   all identical); ffd=4 at t>=2; msolve D = 3,4,4,4,4,5 and t=7 D=5. The README is stale relative to raw at audit time:
   closure now also has t=7 (d_F=5, 4/4) and t=8 (d_F=5, 1 seed); msolve has t=7 (5, 4/4), t=8 (5, 4/4), t=9 (5, 1 seed).
   The "no decomposable draws" statement is true (0 'excluded' rows).
F4 (unsupported number) README's "t=12..14 need 56-112 GiB and hours-days per system" has no source in this lane's
   data or in any file under report/ or this directory (grep). Cite the source or label it an estimate. The "t=10, c=6 needs
   ~60k columns" figure is correct (sum_{d<=6} C(20,d) = 60460).
F5 (draw independence) For a given seed the (a,b,g,xR) parameters and hence the coefficient matrix are identical at every t
   (verified for seeds 1,2; t-independent by construction of gen.draw). So "4 random targets per t" are the SAME 4 systems on
   growing grids, not independent samples per t, and every cell returned 4/4 identical dims. The medians are therefore
   robust-looking but carry less information than 4 independent draws; say so in the README.
F6 (P-repro scope) closure is only run to t=9 (chainA), so P-repro at t=10 (prediction 6) rests on msolve D at t=10 only.
   D is not d_F; note however that if gate Ge (d_F<=D) is accepted, a measured D<=5 at t=10 would contradict the
   registered d_F=6 at t=10 and must be reported as such, not as "within noise". t=10 is still inside the "fit" range,
   so it would be a reproduction discrepancy, not a blind refutation.
F7 (reproducibility) The closure binary is built into $TMPDIR; no build line is recorded. `cc -O3 -o closure closure.c`
   reproduces the rows exactly. Record it in README. mtimes: PROTOCOL.md 18:42 precedes gen.py 18:43 and all raw rows
   (>=18:47), consistent with "protocol before rows"; closure.c mtime 18:47 is later than run_closure.py (18:44), so I
   cannot prove which closure.c produced the earliest rows, but the current source reproduces them bit-for-bit.
F8 (load) System load average was ~380 during the audit and wall-clock limits (closure `tl`, msolve `timeout`) drive
   censoring. Any censored/timeout cell at t>=8 should be labelled "wall-limited under heavy load", not as an intrinsic cost.
F9 (hygiene) raw/ is ~160 KB, directory <250 KB; fine. `__pycache__/` in the experiment dir is scratch, delete it.
   Disk had ~4 GB free and falling during the audit (not caused by this lane).
F10 (deviations, correctly disclosed) msolve -t 1 instead of -t 2; own C closure instead of the registered Rust pilot;
   extra flag `-g 2` (prints the reduced GB, harmless). The new engine is validated by items 1-2 and msolve agreement.

## Overclaim check
None found. README states "reproduction, not a C-step test" and C-step "UNDECIDED, blocked". The PROTOCOL's own warning that a clean
t<=11 match is not "C-step holds" must be kept in the final README. Nothing is said about deployed curves.

## To re-verify when RESULTS.json / final README appear
- RESULTS.json medians equal the raw rows (recompute from raw/*.jsonl; do not trust copied numbers).
- Null arm: P-null compares medians at the two largest common t within 1; confirm the null generator keeps a nonsingular
  bilinear part and no grid zero (gen.py does), and that null arms with grid zero are reported as excluded.
- P2 arm uses 3 seeds, t<=8, nothing else changed; msolve null t=2..9; msolve t=10,11 (timeouts labelled).
- Gate Ge on every common cell; P-D differences in {0,1,2}.
- Any cell where closure and msolve disagree, or where a median differs from 3,4,4,4,4,5,5,5,5,6 at t<=10, is reported as discrepancy.
Audit scripts: audit/semaev_check.py, audit/indep.py (copy next to gen.py to run; run from outside this dir to avoid __pycache__).
