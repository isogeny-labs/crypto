# Lead queue, 2026-11-14: turning the open IC leads into concrete experiments

Owner's request (2026-11-14): "Can you work on these? There seems like a lot of leads in that artifact page, and we can
probably turn those into concrete experiments." The page is the lead queue of 2026-11-13
(https://claude.ai/artifact/Gss1TVNACwQd9MXidoz4hR, private): three unowned leads to chase, three waiting on owner
decisions. Every number here is `[pcc:unverified]`. Nothing in this directory is a measured result yet.

## Tracks and status

| track | lead | what was done | state | next |
|:--|:--|:--|:--|:--|
| A | class-representative IC selection (`../ic-representative-selection_20261105/`, frozen 2026-11-05, never run) | drift check since the freeze (`A_repsel/DRIFT.md`); Stage 0 ran locally (24 classes, 205 job specs, 16 native tests pass); amendment A1 filed in the study's `INPUTS.md`; protocol hash re-checked with the lab's tool (equal to the recorded value) | ready except the image; not launchable while the launcher quota is 0 | image publish (`isolab-experiment.yml`, `publish_only`), mask nonce, cards; about 247 jobs, 75 to 155 job-hours on the L2 pool, about USD 7 to 25 (estimate, `A_repsel/READINESS.md`) |
| B | the Nov 10 sweep's unfinished leads (`../lead_sweep_20261110/`) | 107 prime-field leads decided at the desk (`B_sweep_followups/EXPERIMENTS.md`, `decisions.json`): 30 already tested for real by a native study, 47 closed at the desk, 27 need new native code, 3 runnable now | B1 and B2 frozen 2026-11-14T20:00:39Z (protocol tool, 0 findings); records opened; not launched (launcher quota is 0) | B1: factor-base size above 45 at 20 bits (under 0.1 CPU-hour); B2: kangaroo against rho on the six medium-tier curves (about 0.5 CPU-hour); B3 (first counted algebraic row) is blocked on a route and an image |
| C | global-lift height family (`LEAD-EXO-global-lift-height` and three related ids) | desk analysis (`C_global_lift/DESK.md`) and an independent review (`C_global_lift/REVIEW.md`): "holds with corrections". The exponent closure holds only in a narrow setting and duplicates the closure committed and checker-reviewed on 2026-11-12 (`../prime_field_ic_closure_20261112/`); its one addition is that Masser 1989 needs no prime of bad reduction. The leads stay open on section-finding cost (R2) and adaptive trials (R3) | done; no lead disposition changes | R2 needs an instrument the repository lacks (a general F_p F4 and a builder for surfaces with planted sections) |

## Owner decisions taken today (`../open_leads_execution_20261108/DECISIONS.md`)

- 9: the two lost `pfd-b` attempts count toward no stage B unit. Amendment 15 was frozen and unit job 1 prepared.
- 10: PKM t = 14 and further t = 13 draw 0 attempts run on non-Spot capacity the owner arranges. The lab's EKS pools are
  Spot-only by policy (`taskq/deploy/eks-karpenter/spot-only-policy.yaml`), so this needs a change on the owner's side or
  another host.
- 11: about 6 H200-hours approved for stage 2 of the P1 quotient-law test. Preflight in `ml-cryptanalysis`
  (`experiments/ic-bounds-k15-stage2/PREREG.md`, branch `claude/ic-bounds-k15-stage2`): D = 6 at k = 15 needs about
  145.5 GB on the card for the target part; an H200 has 141 GB. Not launched, nothing spent. A retry needs a larger card
  or engine work, and a new decision.

## Sources checked

The knowledge base (`research-kb`) is unavailable in this session (`scripts/research-kb status`: no executable). The
lead atlas in `crypto-autoresearcher` is now vault-encrypted and this session has no key. Fallback: the repository paths
named in each track's files, and the two inventories of 2026-11-12 (sweep follow-ups and leads outside the atlas).

## Launch path blocked (2026-11-14)

Every AWS CodeBuild "Concurrently running builds" quota in us-east-2 reads 0 (Service Quotas; first recorded by
another session at about 00:27Z on 2026-11-11). All Iso Lab cluster launches and status reads go through the CodeBuild
project `isolab-experiment-launcher`, so no job can be submitted from any session. Stage B unit job 1 stopped at its
pool check with `AccountLimitExceededException`; nothing ran. Raising the quota is an owner action. The image builds run
on GitHub Actions and are not affected.

## B1/B2 benchmark records

The bench records named in the frozen B1 and B2 protocols (`benchmarks/records/20261114T1957Z-b1-prime-ic-n-ext-fp20.json`,
`…-b2-kangaroo-medium-tier.json`) were opened on 2026-11-14 and then taken out of the pull request again, because the
`records` check fails on records that are open with no run. Their exact files are in git history at commit `9278a12fb9`
(sha256 `a11f8d91…` and `80068f10…`). They are restored unchanged, at the same paths, in the pull request that lands the
runs, and closed there.
