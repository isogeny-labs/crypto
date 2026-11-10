# Resume: crypto @ claude/best-leads-experiments-jikl0j

## Goal
Owner's ask (2026-11-10, ~21Z): "pick our best leads and launch experiments asap".
Selected (protocol §"Why these leads"): sibling points (`wwalk sib=8,pool=M`, PR #3847 smokes
0.775 / 0.691 bit-op ratio to tw2cj-lazy at k = 20 / 28) and `cv=0` on the two-seed walk
(engine items Addendum 1: twcj-cv0 / twcj 0.879 / 0.950). All numbers `[pcc:unverified]`.

## Lead / hypothesis
Expectations E1-E4 in research/curve25519_autolab_20261104/siblings/stage1-eks/PROTOCOL.md:
sib8-p64 / tw2cj-lazy 0.69 [0.62, 0.76] at k = 28, 0.66 [0.58, 0.74] at k = 32 (w = 0);
p256 below 1 at k = 32; cv0 vs lazy two-sided.

## Spec
research/curve25519_autolab_20261104/siblings/stage1-eks/spec-k{28,32}.json (L2, 6 rounds, 8 targets
x 5 SNC-panel curves, 7 arms incl. rho-neg-aa A/A).
Route: GitHub `isolab-experiment.yml` dispatched on main with source_revision = the frozen commit
on this branch, spec_path = each spec, instance c8a.2xlarge. Container AWS keys are rejected by STS
(placeholder), no CHECKPOINT_UPLOAD_TOKEN: S3 checkpoint push not possible from this session; Git
push of this branch is the checkpoint.

## State
Specs + protocol written; local ecbench smoke (plan acceptance of every arm) pending; then commit,
push, dispatch two runs, record run IDs in stage1-eks/LAUNCH.md and the session log.

## Next steps
1. When runs finish: download artifacts (S3 receipts via workflow artifact), verify, compare, band.
2. Report + figure + PDF (research-visuals), benchrec record close, PR.
