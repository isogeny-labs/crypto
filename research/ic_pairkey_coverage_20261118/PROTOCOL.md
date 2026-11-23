# Preregistration R4 — distinct-key half pair table
Date: 2026-11-18. ID: EXP-IC-PAIRKEY-20261118-R4. Preregistered before local R4 execution; [pcc:unverified].

## Prior work
Base: R2/R3 PR #4763 and research/prime_ic_collection_frontier_20261117/README.md. ECDLP atlas research/prime_field_lead_assessment_20261107/atlas_survey.md; closure research/ic_mega_pass_20261031/brief/BRIEF.md. Literature Semaev 2004/031, Gaudry and Diem IC, Shoup 1997 generic lower bound. This is a generic constant-factor table/coverage tradeoff, NOT a proposed asymptotic new oracle.

## Frozen test
E/F1009: y²=x³+4x+10, prime order r=1069, first lexicographic generator. Standard half-table uses first ceil(0.5*B(B+1)/2) unordered pairs in deterministic seeded shuffle, keeps first witness per group-sum key. Candidate tests same pair order until *that many DISTINCT keys* have been found, counting all tested pairs including collisions. Neither strategy uses Q or known factor logs. Full table and full A/A controls. Same point stream, factor base, rank algorithm, relation set solver and descent rules.

**Hypothesis H1:** At B24, median (candidate / half baseline) **cold total EC group adds <= 0.9700**, and every planted target recovered with independently checked logs. Any wrong target, fewer than 24/24 passes, or median ratio above 0.9700 falsifies H1. Zero-hit and censored runs are retained.

Primary: B24, 24 seeds starting 0x1c2026111880. Sensitivity: B32, eight seeds starting 0x1c20261118a0, B40, eight seeds starting 0x1c20261118b0. Maximum 40 seeds * 4 arms; <=1069 targets per arm; 120 seconds CPU budget; <=10MiB raw output; zero cloud budget. No post-hoc tuning.

Record FB generation, table construction (tested pair additions and duplicate sums), collection attempts and accepted/dependent rows, matrix dimensions/construction time, linear algebra modular operations and time, target descent, total EC adds and field ops; rho measured on same target/seed, compare expected sqrt(pi*r/2)=40.978 walk steps. Cost sensitivity EC adds + w*(mod-r ops), w=0,0.1,1. Timings are nonisolated Python diagnostics, not native cold wall claims.

Correctness: independent affine curve arithmetic with negative checks, every relation recomputed, factor-base logs and target Q independently verified, full rank, rho recovery, full/full deterministic A/A comparison.

Environment: official isogeny-labs/workspace AGENTS, bin/ws and repos.json reviewed via GitHub; local **offline fallback**, not upstream ws, was used for bin/ws route before bin/ws need. Need did a depth-1 blobless no-checkout clone attempt which failed DNS. Conductor claim/check and research-kb unavailable. This protocol PR is not Conductor ownership; no native or signed PCC evidence and no leaderboard update.
