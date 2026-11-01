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
| L2a | INCOMPLETE, no report | none | preliminary fit from attempts.csv (1472 rows, partial run): e = 1.97 (arm G), 2.21 (S), 3.21 (T). Beating rho needs e < m/2-1 = 0.5 at m=3, so the lead does not reopen on this data. Unreviewed. |
| L2b | INCOMPLETE | none | PREREG.md frozen; ladder produced 1 cell. No conclusion. |
| C2 | INCOMPLETE | none | REPORT.md says IN PROGRESS; msolve leg timed out at 420 s on all M cells under load. Counted-work leg only; no decision. |

Rerun needs: a quiet machine (cap parallelism; load was 137-430), L2b ladder, C2 msolve leg on S only.
Not committed: `.tmp/` scratch, `L2b_dreg_trajectory/work/` (raw logs).
