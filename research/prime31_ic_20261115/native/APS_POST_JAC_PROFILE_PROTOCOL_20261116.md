# Pre-registered post-Jacobian APS partition profile, prime31 (2026-11-16)

**Scope:** fully cold Rust source-specific development experiment; no claim to a novel non-generic algorithm, independent PCC, or fastest-world prime-field ECDLP. The public, ordinary prime-order curve stays fixed at p=2147483647, n=2147446831, cofactor=1, with the existing Rust curve and source generator. No EKS/paid experiment or benchmark-signing event.

## Why now and prior-lead decision

Read `AGENTS.md`, latest `docs/session-logs/2026-11-16.md`, existing Rust example and exact-source #4391 results (Jacobian 72/72 and 1.696x versus affine), earlier #4383's failed T=1024 speed claim, #4391 asymmetric 384-profile fresh success, #4373 m8 F4 censorship, B8 symmetric correctness gate, and #4228/#4349 certified quotient. Publication constraints: PKM 2016 DOI 10.1007/978-3-662-49387-8_1 and APS 2018 DOI 10.1016/j.ffa.2018.01.009: published one-relation IC; explicit S3 finite pair scanning is MITM-equivalent. No novelty claim. KB MCP tools not loaded in this session; fallback is accessible source/primary literature, not a comprehensive novelty certification.

The old unequal optimum was selected when label construction used affine walk. Jacobian batch-normalization reduced its relative cost by 2.72x on 24 exact-source targets. The optimum may therefore move to **larger third partitions/smaller first and second partitions**. Test prospectively, not by recycling previously reported data.

## Frozen experimental design, before code/runs

- 32 fresh deterministic public `Q=[k]G`, derived from seed `0x5052_4f46_5f4a_3332`, with k used solely by caller post-solve known-answer check. Each arm independently regenerates **all** labels and points for every trial; no shared reusable preprocessing across arms.
- Five arms, indexed by declaration: A `[256,256,32768]`, B `[320,320,24576]`, C `[384,384,16384]` incumbent, D `[512,512,8192]`, and C' incumbent A/A repeat. Each `execute_unequal_profile_source(Q,sizes,6,true,true)` uses exactly the current Jacobian batch public source and S3 x-fibre Mersenne field oracle, all ordinary `Arithmetic` and curve replay; six deterministic rounds maximum per arm. Partition sizes always pass the existing `sum<=40000` admission.
- Rotate call order through predetermined five cyclic permutations by target index, retaining `position` and original arm identity. Keep all complete successes, bounded misses, setup costs, actual number of full/partial S3 quadratics, candidate-replay work, and per-round transcript and interval. No replacements of hard targets.
- True full-cold in-process cost: source construction + failed rounds + S3 candidate finding + signed validation + final caller-independent solver verification. Caller known-answer generation/checking outside each solver's timer. Sum costs across the same 32 Q, no ratio of unmatched old CI runs.
- A/A stability `abs(total_C_repeat/total_C-1) <= 0.05`; improvement screen requires **32/32** correct scalar recovery on all candidate/control arms and `best/median(C,C')<=0.90` and candidate paired wins >=24/32 over C. The sample is exploratory; a surviving profile needs **another fresh 32-target confirmation** before replacing the default. If controls or any profile miss, print all receipts and fail admission (not necessarily the Rust process). Don't force output of a winning arm.
- Falsification: possible source collisions/identity resets, larger source construction swamping fewer S3 queries, different number of failed rounds, between-arm thermal/runner bias, and no measured advantage over optimized rho or mature PKM/APS. Report observed full costs and total work, not hypothetical asymptotic gains or timing of an incomplete branch.

## Successful/negative execution requirements

Run Rust 1.100 locked release example tests and `--post-jac-profile` through the existing dedicated `.github/workflows/prime31-s4-split-20261116.yml` on the exact commit. Preserve original output and entire per-case JSON in `native/APS_POST_JAC_PROFILE_CI_20261116.json`, including all misses. Report the winning/losing profiles and independent A/A drift in `native/APS_POST_JAC_PROFILE_REPORT_20261116.md`. The executed source and job must be pinned; if CI is unavailable, leave the result explicitly source-only. No silent censoring or leaderboard/PCC promotion.

## Non-generic follow-up (separate science)

After measured APS control, pursue an *actual target-conditioned* symmetric m8 (S_9)/Vieta polynomial root finder, starting with B4/B8 public natural negative/positive root coverage and original-ideal certificates, then B31 natural relations and complete logs. The B8 validator solves **verification of given f**, not extraction of f. No enumeration of all `binomial(38,8)=48,903,492` multisets, and no transfer of performance claims from #4349's small native quotient worker.
