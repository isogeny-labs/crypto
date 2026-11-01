# Prime-field leads 20261031: synthesis (written 2026-10-31 18:5x by the resuming session)

Workflow wf_c23b552f-88c stopped at ~17:25 (host load average above 400); its synthesis stage never ran.
This file is the manual replacement. Nothing below is a speedup over rho.

| Lead | Status | Decision | One line |
|---|---|---|---|
| L1 | done (writeup) | writeup | cost >= c_m*r under hypotheses H_C, H_E; tower presentation gives p^(1/2-o(1)) iff c >= (m-1)/2; one open gap; no experiment |
| L3 + C5 | done (writeup) | writeup | precomputation ~2^170.7 (P-256), ~2^168 (Curve25519), ~2^86.8 (ECCp-131) vs rho 2^128/2^126/2^65.1; special-prime bases not closed under the group law |
| C1 | done | closes | prime-to-p torsion section unique; lead's "only N = p non-split" claim is wrong |
| C3 | partial | writeup | f must be < p^(-1/m); screening budget leaves empty window at m=3, one boundary point at m=4 |
| C4 | done | null_confirmed | energy ratio 0.993-1.013, no growth with log p |
| L2a | INCOMPLETE, no report | none | fit over s=6..11 (s=12 rows removed for a clean rerun): e = 1.98 [1.94, 2.01] (G), 2.21 [2.16, 2.24] (S), 3.21 [3.00, 3.40] (T, 3 points). Reopening needs e < 0.5 at m=3, so it does not reopen on this data. No REPORT.md. |
| L2b | INCOMPLETE | none | PREREG.md frozen; ladder produced 1 cell. No conclusion. |
| C2 | done (REPORT.md has results) | null_confirmed | counted-work crater/floor ratio 0.9998 (M), 1.000 (S), spreads overlap. msolve: S all 32 cells finish, max degree 20 both sides; M 11/16 timed out, common 95-round prefix identical. Weak: ~3/4 of work_total is fixed by B. S_ctrl_a0/b0 not interpreted. |

Rerun needs: a quiet machine (cap parallelism; load was 137-430), L2b ladder, C2 msolve leg on S only.
Not committed: `.tmp/` scratch, `L2b_dreg_trajectory/work/` (raw logs).
