# L3: closure of open audit items for N1..N4 (2026-11-01)

No security claims. PROTOCOL.md (pre-registered C1/C2) was written before the runs. Total CPU < 5 min at 1 thread.

## N1 (gf2-d4-profile): see its README "Closure of audit items"
- Item 5: done. festest 30/30. Brute-force nsol n=19 36/36. Python closure n=19 2/2 identical. Scripts and json are in N1/crosscheck/l3_*.
- Item 6: done. The CPU is child clock() only, 3,795 s total, within the 3 CPU-h budget, and both cost-rule readings are stated.
- n>=27: the independent recount equals the Addendum table: n=31 A 9/9, and C1/C2/D1 0/8 stalled at low_4=2821.
- Blocked: an independent N=32 closure (msolve did not finish; Python is too slow for a cheap check). The n=31 stall stays unverified.

## N2 (d4-s5-lattice)
- Item 6 (arm-X non-discriminating): done.
- Item 7 (rule 4/5): done.
- Item 5: the wording is done. The rank-59 structure test is blocked. The smallest decisive test (a lattice on the 59-dim span vs the observed delta) needs new code and a new pre-registration.
- Item 8: partial. Own validation values are quoted, 2^35 is corrected to 2^37, and the CPU overrun is disclosed. Raw p/A/c/xR for the 8-seed run needs a rerun.

## N3 (capacity-lp-converse)
- Items 5-8: done. The B2 SA started at the optimum (0.0 gap by construction) and l=8 was not run. CPU is corrected to ~70 min (4192 s). The 3-thread use is disclosed. The e-variable 1/32 claim is withdrawn. The diag.out labels (m2hp seed1 p41, seed0 p47), the m2hp grid and __pycache__ are fixed.
- m=3 p=13 seed 0: an exhaustive kernel over all 181 F_13-points has dim 3 (controls p=11 and 17: dim 1). On 300 points of V(F_169) only f survives (dim 1). So the excess is an F_13 point-set artifact, not a failure of P on the surface. Pre-registered C1a held. C1b (reducible f) was not confirmed: the factorisation over F_13 was not computed, because there is no multivariate F_p factoring in the tools. Still open: why this curve differs from the other p=13 curves.

## N4 (char2-xfilter)
- Item 5: done. The fixes are 0.010871 vs 0.010886, 0.103-0.187, 0.0715/0.0744/0.0770 and 0.1364, and the theta_eff drift note. All were checked against raw by grep.
- Wording: "consistent with Weil" is withdrawn and "a theorem" is softened.
- Partial: the RESULTS summary ratio ranges are left as-is (already marked superseded).

## Blockers
- The Conductor check could not run: the sandbox denied localhost:8491 and the unsandboxed retry timed out.

Files: PROTOCOL.md, diag_n3_p13.py, diag_n3_p13_fp2.gp, raw_n3_p13_diag.json, raw_n3_p13_fp2.out, RESULTS.json.
