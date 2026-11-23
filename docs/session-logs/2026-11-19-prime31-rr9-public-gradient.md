# 2026-11-19 UTC — Prime31 public RR9 factor-resultant/Jacobian gate (#4772)

## What was done (commands, files, features)

- Audited open and historical GitHub lanes #4692, #4728, #4770, #4532, #4714, #4722, #4720; consulted isogeny-labs/crypto-autoresearcher SA-04/SA-18/SA-19 and cryptanalysis binary Semaev/SAT direction. Shared KB tooling was not available in this runtime. Reviewed GitHub AGENTS.md and main ACTIVE and newest bounded session log; did not overwrite the large actively edited shared daily log.
- Created draft PR #4772 directly against main with prospective protocol committed before implementation.
- Implemented a self-contained original-crate Rust example: public R, arbitrary 7-coefficient candidate c, and public eight-by-eight weighted slot support produce exact eight norm-resultants H_j and 8x7 Jacobian via inversion-free prefix/suffix products.
- Added an independently formulated truncated-dual-number Rust differential evaluator, 16 planted signed-witness graders per arm, target group-sum replay, 7/7 Jacobian rank audit, and predetermined mutated-chart negative controls.
- Added separately authored original-curve Python BigInt group, polynomial, derivative and rank verifier. Full source-bound GitHub Actions gate uses original Rust EC arithmetic pinned at 1df714645aa683cf975bc76fcacb9842cb4780fd and Rust 1.100.0.
- Independently preflighted exact affine arithmetic in Python on the unchanged curve before GitHub submission: 16/16 planted H=0, 16/16 rank=7, 16/16 z0+1 mutants H !=0 in BOTH old and ternary arms (32 total each type). This is local preflight, not CI/PCC signed validation.
- Established exact necessary support bound for any fixed B,m signed representation: P_uniform[hit] <= min(1,(2B)^m/n). For B8, m8, a single window covers <=2^32 target group elements, four windows <=2^34. For ~2^256 group, even free uniform-shift membership calls need >=2^222 on average across the four windows. It does NOT bound arbitrary non-generic algorithms.

## Decisions made (and why)

- Focused the new implementation on a public-only algebraic **front end** (H and dH) instead of another optimized generic group-sum oracle, which cannot establish the requested new solver.
- Kept signed witnesses exclusively inside the grader, not in the public polynomial evaluator; explicitly report that no unknown chart has been solved.
- Kept existing #4692 untouched to avoid overwriting concurrent work. New PR #4772 is a reusable, independent feeder to that lane.
- No paid compute, merges, leaderboard promotion or claimed full-solver speedup.

## Open threads (unfinished, pick up next time)

- Confirm exact-source GitHub CI, inspect raw results, and commit source-specific results report with receipts and any fixes.
- Implement true **unknown-chart extraction** for public R from H_0=...=H_7=0 without supplied indices, planted c, or generic 4+4 tables; test 46 positive / 18 negative public B8 frozen panel, with failure/timeouts recorded.
- Charge Gröbner/F4 setup and per-target time, integrate rank-31/1024 unknown logs, and compare *matched* strongest index calculus/shared BSGS/optimized rho on the same runner.
- Scaling theorem requires B and/or m to grow with subgroup order, which also grows resultant degree and coefficient complexity. Quantify that cost in a frozen multi-prime ladder.

## Errors (what went wrong, how it was handled)

- Local runtime has no Rust compiler, Cargo or outbound Git network; source-specific GitHub Actions is used as the compilation/test route instead.
- No claimed CI/PCC result until exact-source run completes; no false whole-solver advantage claim.
