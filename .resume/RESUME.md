# Resume: crypto @ claude/serene-davinci-ujoi58

## Goal

User request (2026-11-08), verbatim: "Yes to all open leads". These are the open items left after PR
#3651 merged. Mapping and options: research/open_leads_execution_20261108/DECISIONS.md.

1. PKM fall-degree study (`research/pkm_falldegree_unblock_20261107`). Image, 17 command requests,
   serial on EKS.
2. Medium-tier rho follow-ups:
   - independent validator report (`research/prime_medium_tier_rho_20261107/validation/`);
   - new P4 confirmatory protocol (`research/prime_medium_rho_p4_confirm_20261108`), frozen before
     data, then run.
3. Study images and runs:
   - 3a: minors_guess_determine_20261107;
   - 3b: elliptic_code_list_decoding_20261107;
   - 3c: ml_learnability_control_20261107;
   - 5: split_graph_secant_cost_20261107.
4. `main` defect: `evidence_audit_pr` on `research/curve25519_f1_walk_20261107` sessions (fix in the next PR).

## Lead / hypothesis

Per study, from each frozen PROTOCOL.md (unchanged). Item 2 P4: is the pooled rho-plain / rho-neg
ratio's interval above 1.2 (J1 alone: 1.285 [1.177, 1.404], [pcc:unverified])?

## Spec

Each study directory's PROTOCOL.md / protocol.json, frozen on main at f13a690df0. The new item 2 P4
protocol is to be frozen after commit. The branch was reset to origin/main f13a690df016d0a457bd40579fbb9d054f486adf.

## State

Five preparation agents are running:
- Rust images for 3b, 3a and 5;
- the mllc image (3c);
- the audit fix;
- the validator for item 2;
- P4 protocol design.

The PKM image build is next, after the Rust images, because disk is about 7 GB free. No job has been
launched in this round. ECR push is authorized. Docker has no daemon, so images go through
`taskq/deploy/eks-karpenter/oci-layer-publish.py`. The pool allows at most 16 vCPU / 128 GiB on
`cpu-spot` and 8 / 32 on `cpu-l2-spot`.

## Next steps

1. Collect each agent's R2_READY.md, then commit and push.
2. Run `scripts/checkpoint.sh push`, then `tools/benchrec.py init` per benchmark run.
3. experiment-director compile writes a card per study. Relay it verbatim to executor-mechanical, then
   run director assess.
4. Launch order on `cpu-spot`: 5 (split-graph), then 3b (ecld), then 3c (mllc), then 3a (minors), then
   PKM serial: pfd-a first, then pfd-c and pfd-b, then the x / m4 cells. P4 confirm runs on `cpu-l2-spot`.
5. Analyse each study with its frozen rule. Each gets a report, a figure and a PDF; then PR and track it.

## Runs in flight

None.

## Stop and success conditions

Each study's own decision rules. Budget: PKM at most US$40 per job and US$400 in total; the other
studies at most a few dollars each. Stop a study on its control-failure rule. A lost Spot cell is
reported as lost, with at most one relaunch.

## Open questions

The PKM large host (80-510 GB) needs a capacity change outside the current cost controls. It is
prepared and reported, not forced.
