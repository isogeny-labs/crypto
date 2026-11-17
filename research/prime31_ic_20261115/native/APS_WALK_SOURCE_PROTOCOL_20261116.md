# EXP-R31-APS-WALK: walk-generated public labels versus independent-scalar labels (2026-11-16)

**Standing:** experiment specification frozen before the new Rust measurement; [pcc:unverified]. This is incremental engineering of the **published APS 2018 one-relation** prime-field index-calculus control, **generic pair-MITM equivalent**, not a claimed new non-generic complexity breakthrough. The target curve is ordinary, full prime subgroup and cofactor one, not selected for weak group order.

## Frozen prior results and literature review

- Curve `Fp`, `p=2147483647`; `E:y²=x³+847514846x+2087444724`, `n=2147446831` prime, `h=1`, `G=(625037715,2100627251)`.
- Published APS one-relation method: Amadori–Pintore–Sala, *On the discrete logarithm problem for prime-field elliptic curves*, Finite Fields Appl. 51 (2018) 168–182, DOI https://doi.org/10.1016/j.ffa.2018.01.009. Standard PKM comparator: Petit–Kosters–Messeng, PKC 2016, DOI https://doi.org/10.1007/978-3-662-49387-8_1.
- Merged #4374 proved a complete **producer-development Rust run** on 12 distinct prime31 public target logarithms using 3x1536 labels and S3 root enumeration; matched source-plus-algebraic cost on one Q 268.6 ms [pcc:unverified]. It is generic pair-MITM equivalent; separate direct EC join is not the IC frontier. Merged #4383's fresh 24-Q / 72-arm Rust confirmation reversed the exploratory T1024 win: full charged total T1024 5175.152 ms vs original T1536 3920.747 ms and repeated T1536 3913.221 ms [pcc:unverified]. **T1536 remains the control.**
- Merged #4373's balanced m8 S3-chain F4 stopped oversize, no extracted target roots. Original #4188 factorized m4 natural-target yield 0.02882% at B31; #4228 merged certified quotient and open #4349 compact certificates have no completed B31 target-dependent K/S5 relation oracle. No duplicate m8 F4 presentation is proposed here.
- The research KB MCP connection is not present in this runner, and repository wrapper/Conductor/EKS are unavailable through the GitHub-only interface. The referenced primary repo histories and publisher paper sources were inspected as bounded fallback; no absence/novelty claim.

## Why test this

The current APS Rust single-relation baseline builds `T` independent public points per each of three labelled partitions as `R=[a]G+[b]Q`, requiring two full scalar multiplications per accepted point. This dominates a material source-setup fraction of the recorded full cold cost. The proposed source constructs 64 uniformly labelled public *step points* per attempt, plus three independent labelled starting points, then makes a bounded random additive walk using a seeded random step index. Every new accepted point has exact recursively updated public labels `(a,b)`; a sample must pass `[a]G+[b]Q=R` independent replay.

This changes ONLY public point generation. It reuses the **existing** Rust S3 finite-field quadratic root oracle, signed full-curve verification, and one-relation scalar recovery. No secret target log is passed to the solver. It is a generic walk and an engineering component for the published APS control; never assert non-generic algebraic superiority.

## Frozen test and prospective acceptance

1. Rust unit test builds 3x128 walk-labelled points for the same ordinary prime31 public Q, checks global uniqueness of all x values, no identity, every `(a,b)` point replay and accepted coordinate on-curve. Existing field S3 and baseline source tests must remain unchanged.
2. `--walk-confirm` uses **24 fresh caller-side known-answer public Q=[k]G**, with the known scalar used *only after* each complete solver call to check exact log plus `[k]G=Q`. Three complete **cold** arms per Q: independent scalars / T1536; walk-labelled / T1536; walk-labelled / T1536 A/A repeat. Alternate arm execution order baseline-walk-walk / walk-walk-baseline. Preserve every completed and unsuccessful bounded round, all candidate roots, points per partition, label-source setup, algebraic root search, total cold wall, duplicated x, exceptional identities, source group additions and exact solver scalar. Cap all arms at 3 attempts, no unbounded retry or hidden fallback.
3. A candidate win requires 24/24 complete correct logs for **each** of all three arms and lower *aggregate fully paid cold wall* than the current independent-scalar arm. A/A repetition should be close enough to support timing stability; record individual targets and report uncertainty instead of asserting an arbitrary confidence interval. A failed or censored target is counted and charged. If either condition fails, retain as negative, no baseline ratchet or leaderboard.
4. Even if completed and faster, compare the IC-frontier **published APS one-relation** control only at this scope. Externally mature implementations (PKM/APS, F4/quotient, and CAS for the *same* algebraic call) are still required before claiming fastest; optimized rho separately after completing full cold security cost.
5. Standard isolation, independent verifier/PCC and L2/positive reference gates are NOT satisfied by a normal GitHub Actions development runner. No EKS paid compute is authorized in this run, particularly given the repo's recently recorded monthly spend above its gate.

## Falsifiers and caveats

- Walk points are correlated (shared 64 fixed step points); even with marginally uniform initial states, observed relation yield may differ materially. It is NOT scientifically valid to compare just the same `T` or just per-query field time; only complete, fully charged cost and verified target coverage count.
- A lucky short relation, label cancellation `B=0`, identity, duplicate x, short cycle, or a verified wrong scalar must be reported, never silently supplied with a generic fallback.
- Stochastic speed may vary by target. Do not convert a 12-Q exploratory advantage into an unconditional efficiency claim, as #4383 showed.
- The non-generic **m8 selected-root K/S5** experiment remains distinct and unsolved. If this walk control wins, it only improves an APS-style published IC pipeline implementation at a specific size.
