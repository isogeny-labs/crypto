# Resume: crypto @ claude/f4-f5-algorithm-speed-bvp7qd

## Goal
Lower the cold one-target cost of the plane-quartic double-large-prime index calculus (arms D, DL1,
DL of `research/quartic_n4v2_20261020`, the GHS-weak family, the only IC leaderboard section below
its matched rho) by two changes, measured against the frozen grid on the same instances.

## Lead / hypothesis
`LEAD-J-dl1-percolation-fb-size` (`research/leads_20261112`, lane J): the frozen factor-base constant
c_D = 2.0 sits far above the large-prime-graph percolation threshold (about 1.55); plus a new lever,
a fixed-exponent chain for the per-pair square root. Confirmed if BOTH / BASE for DL is below 1 at
every k = 18..22 (all seeds, w = 0, 0.1, 1); killed if not.

## Spec
`research/quartic_dl1_cd_sqrt_20261113/PROTOCOL.md` (frozen in the commit that adds it); units
`launch/units.tsv` (116 units: BASE, CHAIN, CRULE, BOTH; k = 12..22; frozen N4 v2 seeds); sources
`src/` (v3 = frozen v2 + chain); pod runner `launch/job.sh`. Baseline instrument: frozen
`research/quartic_n4v2_20261020/v2.rs`. Frozen grid units: `research/quartic_n4v2_20261020/results/grid-20261025/units.tar.gz`
(artifact store, `ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store`).

## State
Local pilots (not data, 2026-11-13, documented in PROTOCOL.md and docs/session-logs/2026-11-13.md):
c = 1.55 succeeded on every unit at k = 18..22; combined estimate DL 0.68 / 0.61 / 0.56 of BASE at
k = 18 / 20 / 22. A pilot BASE unit reproduced every counted field of the frozen grid unit.

## Next steps
1. Commit and push the study; `check-experiment-git` receipt into `launch/`.
2. Build v2 (from `research/quartic_n4v2_20261020/v2.rs`) and v3 (from `src/v3.rs`) with
   `rustc --edition 2021 -O`; image layer = binaries, job.sh, units.tsv, AWS CLI v2; publish with
   `taskq/deploy/eks-karpenter/oci-layer-publish.py`; receipt `launch/image.json`.
3. `launch/request.json` (isolab.eks.command-request/v1) and `remote/launch-remote-command`.
4. Collect `s3://isolab-results-104186811093-us-east-2/runs/<job>/<pod>/`, run the analysis (H0, H1,
   H2, ratios), report + PDF + figure, bench record, VALIDATION.json, leaderboard, PR.

## Runs in flight
none (to be filled at submission).

## Stop and success conditions
Budget ceiling USD 15 (charged incl. failures); one submission; success per PROTOCOL.md decision rule.

## Open questions
None blocking. Incremental factor-base growth (instead of the charged restart) is a later, smaller lever.
