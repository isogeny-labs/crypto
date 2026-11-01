# Prime-field leads 20261031: synthesis

Workflow wf_c23b552f-88c stopped at ~17:25 local on 2026-10-31 (host load average above 400); its synthesis stage never ran.
A resuming session wrote the first version of this file by hand on 2026-10-31; the synthesis step of 2026-11-01 (cloud
session, no reruns) completed the three missing reports (C3, L2a, L2b) from the data already on disk, rendered their PDFs,
drew the L2b graph from the recorded cells, and folded the outcomes into section 5 of the leads note. Nothing below is a
speedup over rho.

| Lead | Status | Decision | One line |
|---|---|---|---|
| L1 | done (writeup) | writeup | cost >= c_m*r under hypotheses H_C, H_E; tower presentation gives p^(1/2-o(1)) iff c >= (m-1)/2; four gaps named; no experiment |
| L3 + C5 | done (writeup) | writeup | precomputation ~2^170.7 (P-256), ~2^168 (Curve25519), ~2^86.8 (ECCp-131) vs rho 2^128/2^126/2^65.1; special-prime bases not closed under the group law |
| C1 | done | closes | prime-to-p torsion section unique on every lift; lead's "only N = p non-split" claim is wrong |
| C3 | done (table; REPORT.md written by the synthesis step from the agent's CSVs) | writeup | f must be <= p^(-1/m); screening budget leaves an empty window at m=3, one boundary point at m=4; 6 of 18 mechanisms close, 6 sieve-pricing only, 3 unaffected, 3 not scored |
| C4 | done | null_confirmed | energy ratio 0.993-1.013, no growth with log p |
| L2a | partial (s=6..11 of 6..12; REPORT.md written by the synthesis step) | does not reopen; G closes | e = 1.98 [1.94, 2.01] (G), 2.21 [2.16, 2.24] (S), 3.21 [3.00, 3.40] (T, 3 points). Reopening needs e < 0.5 at m=3. The frozen "interval contains 2" clause fires for G, not for S (above 2). s=12 rows (one curve, G only) were removed for a clean rerun that did not happen. |
| L2b | INCOMPLETE (REPORT.md written by the synthesis step) | none | PREREG.md frozen; ladder produced 1 cell (p=65521 ctrl) plus 5 real-arm probe cells at seed 9 (2 timeouts at 600 s). Max step degree N+8 in every completed cell. No conclusion. |
| C2 | done | null_confirmed | counted-work crater/floor ratio 0.9998 (M), 1.000 (S), spreads overlap. msolve: S all 32 cells finish, max degree 20 both sides; M 11/16 timed out, common 95-round prefix identical. Weak: ~3/4 of work_total is fixed by B. S_ctrl_a0/b0 not interpreted. |

Deviations from the lead text, recorded: L2a ran a group-MITM arm, an S_4 root-finding arm and a triple scan, not the
SAT/SMT decomposers the note named. L2b's probe cells used curve/target seed 9, outside the frozen ladder's seeds 1 and 2.

Rerun needs: a quiet machine (cap parallelism; load was 137-430), L2a s=12 both arms, the L2b ladder as frozen
(`run_ladder.sh`), the C2 msolve leg on M to completion. The L2b ladder is the only one of these whose outcome could change a
reading (its reopen clause).
Not committed: `.tmp/` scratch, `L2b_dreg_trajectory/work/` (raw msolve logs, on the original host only).
Rendering on 2026-11-01: Typst 0.13 wheel and matplotlib in a scratch virtualenv outside the repository (pandoc for
Markdown to Typst), per the research-visuals skill's cloud-container rule.
