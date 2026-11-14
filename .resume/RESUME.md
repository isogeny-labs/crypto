# Resume: crypto @ claude/loving-hawking-w2wv5p

## Goal
Owner (2026-11-14, "Yes to 1"): run panel A, the positive sparse PKM relation-finder panel at m = 8
(research/prime_leads_chase_20261113/CONTINUATION_PROTOCOL.md §A), on a large host. Protocol:
research/pkm_m8_sparse_panel_20261114/PROTOCOL.md (+ protocol.json). Outcome space: KILL this sparse
mechanism at measured scale, or INCONCLUSIVE; ADVANCE is impossible by construction (held-out t=4,5 out of reach).

## Lead / hypothesis
c(B) = B^gamma per solver call; gate gamma < 3.5 (all-output) / 3 (one-output). Expectation from D(8,2)=8,
D(8,3)>=9..10: gamma_lo well above 3.5 (KILL), with t=3 cells likely censored at the memory cap.

## Spec
Instrument: examples/pkm_m8_panel.rs (cell/targets/export/verify/native-solve/rho). Pod runner:
research/pkm_m8_sparse_panel_20261114/pod/{rp.sh,podserver.py,job.sh,launch.sh,parse_msolve_log.py,plan.json}.
msolve 0.6.5-1build2 from the Ubuntu 24.04 archive on the pod. Cells and caps: PROTOCOL.md §3.
Route: RunPod CPU pod (isolab monthly budget exhausted 1808.95/1300 USD); ceiling USD 60; deadline 26 h.

## State
2026-11-14 ~21:30Z: protocol drafted; instrument and pod runner being built (Sonnet workers); Opus red-team
of the protocol in flight. Nothing launched. Lead heat map landed in PR #4161 (merged).

## Runs in flight
None yet. When launched: pod name pkm-m8-panelA-<stamp>; state in research/pkm_m8_sparse_panel_20261114/pod/state/
(git-ignored); results pulled to research/pkm_m8_sparse_panel_20261114/results/<stamp>/; raw tarball to S3 via
tools/benchrec.py (record benchmarks/records/*-pkm-m8-panela.json). To check: pod/rp.sh ls <name>; done.json ends the plan.
Delete the pod with pod/rp.sh delete <name> after fetching results.

## Stop and success conditions
Deadline 26 h from pod creation or USD 60: fetch partials, delete pod, record the loss. Success = complete raw
records for every planned row that ran (censored rows included), the frozen analysis (gamma_lo/gamma_hi per
observable, controls), RESULTS.md + figure + PDF, benchrec record, session log, PR. No retry under this protocol.
