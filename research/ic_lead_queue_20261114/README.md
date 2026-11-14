# Lead queue, 2026-11-14: turning the open IC leads into concrete experiments

Owner's request (2026-11-14): "Can you work on these? There seems like a lot of leads in that artifact page, and we can
probably turn those into concrete experiments." The page is the lead queue of 2026-11-13
(https://claude.ai/artifact/Gss1TVNACwQd9MXidoz4hR, private): three unowned leads to chase, three waiting on owner
decisions. Every number here is `[pcc:unverified]`. Nothing in this directory is a measured result yet.

## Tracks and status

| track | lead | what was done | state | next |
|:--|:--|:--|:--|:--|
| A | class-representative IC selection (`../ic-representative-selection_20261105/`, frozen 2026-11-05, never run) | drift check since the freeze (`A_repsel/DRIFT.md`); Stage 0 ran locally (24 classes, 205 job specs, 16 native tests pass); amendment A1 filed in the study's `INPUTS.md`; protocol hash re-checked with the lab's tool (equal to the recorded value) | ready except the image | image publish (`isolab-experiment.yml`, `publish_only`), mask nonce, cards; about 247 jobs, 75 to 155 job-hours on the L2 pool, about USD 7 to 25 (estimate, `A_repsel/READINESS.md`) |
| B | the Nov 10 sweep's unfinished leads (`../lead_sweep_20261110/`) | 107 prime-field leads decided at the desk (`B_sweep_followups/EXPERIMENTS.md`, `decisions.json`): 30 already tested for real by a native study, 47 closed at the desk, 27 need new native code, 3 runnable now | B1 and B2 being made freeze-ready | B1: factor-base size above 45 at 20 bits (under 0.1 CPU-hour); B2: kangaroo against rho on the six medium-tier curves (about 0.5 CPU-hour); B3 (first counted algebraic row) is blocked on a route and an image |
| C | global-lift height family (`LEAD-EXO-global-lift-height` and three related ids) | desk analysis (`C_global_lift/DESK.md`): closed in the exponent for every family with fresh random points per trial, by two kernel-checked Lean results plus a published height bound; asymptotic only, nothing quantitative at deployed sizes; open residue: section-finding cost on surfaces of rank 9 or more, for which the repository has no instrument | under independent review | the closure is recorded only if the review holds |

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
