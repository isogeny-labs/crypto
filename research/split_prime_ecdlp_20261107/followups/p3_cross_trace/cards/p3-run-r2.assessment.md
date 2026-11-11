card_id: CARD-p3-cross-trace-20261108-r2
card_sha256: 2d25bb725f660e7557c8ae75059ce4110d37d938cdd4ff7590d7a801a7cf9be8
relay_verbatim: not_applicable (harness-launched detached; the tool's own launch log is the record, no executor text relayed)
outcome: executed_as_specified
stopped_at_step: null
recommendation: proceed_to_analysis
interpretation: not_assessed

checks:
- Receipt (cards/p3-run-r2.receipt.json): schema task-card-receipt.v1, status completed, commands_total 10, card_sha256 equals the sha256 of the card on disk, commit 3c11bc13b9 (ancestor of HEAD c8c73d0a77). All 10 commands exit 0, attempts 1, no timeout; receipt argv equals card argv for every command, in order.
- Binds: all 23 bound files (protocol, PROTOCOL-R2, README, p3census sources/Cargo files, six round-1 inputs under runs/run-20261107T2220Z/p24 and p28: curves, cov, yield) hash to the card's sha256 now on disk. must_not_exist directory was absent before (dir created by the card's own commands).
- Output hashes: rhoA/rhoB/analysis.json for p24 and p28 recomputed from disk equal the receipt and launch-log digests (p24 rhoA 283b5199..., rhoB cf07b894..., analysis 78bb2f02...; p28 rhoA 80d23d0c..., rhoB 37217da9..., analysis 4aab0917...). Binary sha256 b4dcf0ad...55f9 (binary-sha256.stdout, matches receipt stdout digest 279b27f2...). Selftest 'ALL SELFTESTS PASSED' including 'rho v2' line.
- Analyze arguments vs round 1 (cards/p3-run-02.yaml): identical option set; every option equal (seeds p24 9456636048090206450, p28 14287345612908892213; covariates p24 z_691,logL1_200000,psi3_roots,psi5_roots, p28 z_691,logL1_200000,psi3_roots; family-size 16; perms 1000; alpha 0.05; planted 0.02) except the rho-a/rho-b/out paths, which point at the r2 run dir. Rho seeds, --targets 256 and --run A/B also equal round 1.
- Walk version: all 2000 lines of each of the four rho files carry "walk_version":2 (8000/8000; zero lines without it). Every line verified==T=256, mismatch 0, unsolved 0.
- Write scope: only the declared run-r2 directory and cards/ files (receipt, launch log, card) were written by the card. Other untracked/modified items in git status are not from this card: benchmarks/records/20261108T0545Z-split-prime-p3-r2-walk-v2.json (declared BENCHREC), p1_rr_dickson/* (separate p1 card), and, written after the run finished (06:43, after the last card command at 06:42), p3_cross_trace/RESULTS.md (modified) and analysis/derived_r2/p3_descriptive.txt, which are main-session post-run work outside the card.
- No check-relay run (no executor text). Build used nice-d cargo against a scratchpad target dir; warnings only (4 dead-code), exit 0.
