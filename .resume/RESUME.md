# Resume: crypto @ claude/dreg-lfd-theory-scaling

## Goal
User request (2026-11-05): find ways to reduce the degree of regularity / total solve time of the
descended Semaev system. Three phases, chosen by the user: (1) theory and scaling survey, done
(PR #3512); (2) the l = 9 guess-saturation test; (3) a multilinear closure engine prototype.

## Lead / hypothesis
LEAD-ICE-descent-closure-degree-at-scale / LEAD-ICE-hybrid-guess-determine-l7. The recorded k*
(guesses that hold degree 4) is 0, 1-2 and 4 at l = 6, 7 and 8. K_marginal predicts 7 at l = 9.
Saturation (k* <= 5) would revive the guess route.

## Spec
research/dreg_guess_scaling_20261106/PREREGISTRATION.md (frozen at commit 37129b4253).

## State
Gate A passed. The 4 positive controls refute at degree 4. Whole-summand runs: l = 7 and 8 done; l = 9 has 14 of 16 done, all degree 4 for nonzero x1. S5.d1.b0 refutes at degree 4 (a surprise against K_marginal). The remaining 129 branches are moving to a RunPod pod (PREREGISTRATION Amendment 1, owner chose the cloud route; $30 cap).

## Next steps
1. Launch: research/koblitz_ic_leads_build_20261024/pod/upload_launch.py run --name dreg-s5-l9 --vcpu 64 --flavors cpu3g --disk 40 --max-seconds 18000 <the 4 input files>, then push them.
2. Fetch https://<pod>-8000.proxy.runpod.net/runs/ when STATUS reads ALLDONE; copy the records into runs/; stop the pod.
3. Score both preregistrations, then write RESULTS, the graph and the PDF, the session log, and update PR #3512.

## Runs in flight
RunPod pod dreg-s5-l9 (id recorded in runs/queue.log once created).

## Stop and success conditions
About 12 h of container time at most. The decision rule is in the PREREGISTRATION.

## Open questions
Whether pe_closure fits about 196k columns in 15 GB.
