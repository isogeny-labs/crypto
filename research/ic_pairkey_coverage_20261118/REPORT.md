# R4 distinct-key pair-table selection — negative result (2026-11-18)

**EMPIRICAL, toy only; [pcc:unverified].** Frozen protocol commit 10e6338b123e563c4359e2f66a783f32157f66a8, prior to R4 execution. Reproducer run_r4.py copies the exact R2 run.py and uses the same E(F1009) prime-order-1069 generator, targets, factor bases, pair shuffle, cyclic relation walk, graph rank/solve and individual descent. The new arm scans extra shuffled unordered factor-base pairs to fill ceil(M/2) **distinct** pair-sum keys, charging **all** evaluated additions including collisions. Comparator is the original ceil(M/2) **raw** pairs. Full and A/A full controls are retained. No secret-adaptive selection.

## Frozen criterion and verdict

Primary B24, seeds 0x1c2026111880 through 0x1c2026111897 (24 seeds): at least 3% lower median cold EC additions (unique/raw <= 0.9700), with 24/24 verified complete logs. **FALSE:** 24/24 completed but median ratio **1.007752**, i.e. 0.8% worse. Paired seed bootstrap 95% CI [0.983586,1.012777]; 10,000 samples, fixed bootstrap seed.

| B | seeds | median unique/raw cold EC-add ratio | w=.1 ratio | w=1 ratio | median unique / matched rho EC-add ratio |
|--:|--:|--:|--:|--:|--:|
| 24 (primary) | 24 | 1.007752 | 1.009309 | 1.013358 | 9.508 |
| 32 (sensitivity) | 8 | 1.023234 | 1.026427 | 1.037068 | 12.873 |
| 40 (sensitivity) | 8 | 1.049927 | 1.049348 | 1.045960 | 18.468 |

Paired bootstrap 95% CIs for B32 [0.953191,1.034965] and B40 [0.909297,1.084011]. 160/160 tested arms recovered all planted logs; zero censored/failed. Independent local affine/dense-rank audit checked each row and factor-base target, pair sums, rank, complete target, phases, and executed rho walk; intentional wrong-row-scalar mutation was rejected (exit 1). No signed PCC certificate.

## Phase accounting: means per solve, operations (not seconds)

| B | arm | pair-build EC adds | collection EC adds | LA mod-r ops | descent EC adds | total EC adds |
|--:|:--|--:|--:|--:|--:|--:|
| 24 | half raw | 150.0 | 329.29 | 248.79 | 6.50 | 774.13 |
| 24 | half distinct | 160.96 | 298.42 | 237.96 | 6.46 | 754.17 |
| 32 | half raw | 264.0 | 328.75 | 434.75 | 4.25 | 988.75 |
| 32 | half distinct | 297.13 | 297.38 | 429.38 | 4.25 | 990.50 |
| 40 | half raw | 410.0 | 310.50 | 603.50 | 2.88 | 1204.13 |
| 40 | half distinct | 504.25 | 214.0 | 472.75 | 1.25 | 1200.25 |

Factor-base costs identical within each matched seed and charged in total. Per-arm raw outputs also record separate matrix construction rows/columns/wall, LA time, relation attempts, rank updates and descent attempts. Unisolated Python ns are diagnostic only. The ratio of means is NOT the median of paired per-seed ratios; the preregistration uses the latter. Weights w=0,.1,1 combine EC-adds and mod-r arithmetic only as arbitrary toy sensitivities.

Matched same-target toy 4-partition Pollard rho counted cold starts/restarts, independently replayed. The birthday heuristic sqrt(pi*1069/2)=40.978 steps is a reference, not a theorem for this rho implementation. The toy rho is not guaranteed globally strongest.

## Prior art and leaderboard disposition

Semaev (ePrint 2004/031); Gaudry/Diem IC; Shoup generic-group lower bound; closure research/ic_mega_pass_20261031/brief/BRIEF.md and report/REPORT.md; atlas research/prime_field_lead_assessment_20261107/atlas_survey.md; existing native prime-field collection baseline research/prime_ic_collection_frontier_20261117/README.md; prior toy PR #4763. Generic pair table coverage is bounded by preprocessing/online advice tradeoffs. No change in exponent or evidence of non-generic attack. docs/ic/LEADERBOARD.md compares different cost units across tables A/B/C; R4 is a separate toy-only unit and does not qualify for a leaderboard row.

## Reproduce

    python3 run_r4.py --base 24 --seeds 24 --seed0 0x1c2026111880 --out raw/r4_primary_b24.json
    python3 run_r4.py --base 32 --seeds 8 --seed0 0x1c20261118a0 --out raw/r4_sensitivity_b32.json
    python3 run_r4.py --base 40 --seeds 8 --seed0 0x1c20261118b0 --out raw/r4_sensitivity_b40.json
    python3 audit_r4.py raw/r4_primary_b24.json raw/r4_sensitivity_b32.json raw/r4_sensitivity_b40.json

R2 source run.py and independent audit_base.py vendored; compact all-40-seed rows under raw/PAIRED_SEED_COSTS.csv. Complete raw JSON witnesses, checksums, diagnostic times, per-run failure logs and additional local audit are in downloadable conversation archive; NOT committed to GitHub (local DNS blocked checkout, Library transfer denied). R4 runner/verifier source and compact per-seed cost records ARE committed. Important: the R4 source was transported via GitHub connector and not rerun from that remote checkout; local run source SHA in RUN_MANIFEST. No claim of signed independent remote replay.

Workspace: upstream trunk inaccessible; labeled local offline fallback ran bin/ws route before bin/ws need crypto. Need attempted blobless depth-one no-checkout clone and failed resolving github.com. Conductor and research-kb tools not exposed; ownership not claimed. No EKS/isolated Rust run, no cargo test, no leader-board promotion.

**Next:** Do not expand this rejected overfill strategy. Preregister a truly fixed-setup-budget collision-aware selection or move to previously queued native prime_fast 24/28-bit comparison, after official workspace routing, Conductor claim, independently reproducible evidence and matched strong rho.