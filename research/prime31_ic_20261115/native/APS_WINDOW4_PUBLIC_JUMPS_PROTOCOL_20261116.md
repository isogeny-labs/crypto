# EXP-R31-APS-WIN4 — prospective four-bit public-jump window benchmark

2026-11-16 UTC. **Frozen before modifying Rust source or executing benchmarks.** Status: proposed; no completed calls, timing win, or non-generic novelty is claimed in this document. [pcc:unverified]

## Baseline and reason to test

The existing `research/prime31_ic_20261115/native/examples/prime31_aps_one_relation.rs` uses published Amadori–Pintore–Sala (FFA 2018) one-relation S3 methodology. Its explicit quadratic-root search across pairs is generic meet-in-the-middle equivalent. This experiment improves that **existing index-calculus implementation**, not its asymptotic complexity, while preserving the S3 oracle byte-for-byte. The incumbent draws 64 public additive-walk jumps `J_i=[a_i]G+[b_i]Q` for each round using 31-bit joint Shamir multiplication in Jacobian coordinates with one final Montgomery batch inversion. Across 64 jumps that performs roughly 1,984 doublings and 1,488 mixed additions, plus one `G+Q` construction, before all walks. The joint source optimization already passed a 24-target producer CI panel but the later 48-target fully cold result on this same curve recorded APS 109.734280 ms versus unfolded rho64 99.473873 ms; the difference was not statistically established. Those are historical [pcc:unverified] figures, not today's measurements.

**Hypothesis:** precompute affine `[d]G,[d]Q` for `d=1..15` with Jacobian repeated additions and one batch inversion over the 30 nonzero points; compute each `aG+bQ` from eight 4-bit windows, with 4 doublings and at most two mixed additions per window. For each jump, this preserves all labels and exact target-dependent public points, while replacing a roughly 23-addition binary Shamir joint ladder with about 15 mixed additions plus paid per-target precomputation. It may improve whole-call cold latency. The baseline is the **same** currently fastest in-repo APS Rust algorithm; the generic comparison is unfolded 32-/64-lane rho on the same curve and host, not plain BSGS.

## Curve, prior leads, and boundaries

Use only the existing nonsingular cofactor-one prime31 curve `p=2147483647`, `a=847514846`, `b=2087444724`, `n=2147446831` prime and the existing `PRIME31.g`. Retain current APS partitions `[384,384,16384]`, at most six complete rounds and identical public PRNG labels. No deliberately weak curve, cached secret scalar, table construction outside cold timing, or separate Python/C++ solver.

Prior checks: `APS_JOINT_SHAMIR_BATCH_RESULT_20261116.md` (binary joint source), `APS_RHO_FOLD_SENSITIVITY_48Q_RESULT_20261116.md` (stronger unfolded generic control), `RHO_JUMP_WIDTH_PROTOCOL_20261116.md` (generic jump-width control), `S9_ZERO_FIBRE_THEOREM_AND_PROTOCOL_20261116.md` and PR #4451 (no noncancelling eight-leaf zero relation if D4 injective), `M8_RATIONAL_QUOTIENT_BRIDGE_20261116.md` (#4228/#4349 certified quotient/root, target-dependent S9 still unsolved), and `research/norm_branch_rank_pilot_20261115/README.md` (Riemann–Roch norm is prior work, not an untried novelty). Also inspected `research/prime_one_shot_20261029/README.md` and PKM 2016 DOI 10.1007/978-3-662-49387-8_1, APS 2018 DOI 10.1016/j.ffa.2018.01.009, and Kudo–Yokota–Takahashi–Yasuda's hybrid/symmetry followup. The research-KB MCP catalog, wrapper and Conductor/EKS are not available to this GitHub-only environment: fallback was actual GitHub source and primary publication records. This is not a novelty-by-absence claim.

## Prospective finite sample and matched arms

Deterministically draw 24 **new** distinct nontrivial `Q=[k]G` using SplitMix64 seed `0x57494e3441505331`, with `k` held only by the independent caller/verifier. For each Q run in rotating order, all as fully cold independent calls:

1. `APS-BINARY`: current 31-bit joint-Jacobian 64 public jump routine;
2. `APS-WIN4`: new 4-bit unsigned joint-window 64 public jump routine;
3. `APS-WIN4-AA`: identical candidate repeated with same Q and public seeds;
4. `RHO32`: existing unfolded 32-lane r-adding rho with 32 jumps, DP bits 4, 2,000,000-step cap;
5. `RHO64`: existing unfolded 32-lane r-adding rho with 64 jumps, same cap.

Fixed rho seed base `0x57494e3452484f31`. The comparison must report failures/caps as failures, not include incomplete time as completed solve. Verify every recovered scalar `[k]G=Q` with the caller's independently held answer. APS-BINARY vs WIN4 must be **transcript-identical** for every round, pair-query count, rejected x, signed witnesses, scalar and signed congruence. Native unit tests include 64 public scalar pairs, adversarial zero-one and Q=-G, and complete partition walk equivalence; no other code path changes.

Report per-target complete-cold ns, source setup ns (including both precomputations), candidate/binary and candidate/faster-rho aggregate ratios, paired wins, candidate AA drift, total group ops and success status. Counts include field inversions, PRNG, failed rounds, signed verification, memory allocation, cold setup and all walk and rootfinding work. Preregistered **engineering success** requires all 72 APS recoveries, exact transcript equality, `candidate/binary <=0.95`, at least 18/24 paired wins, AA drift <= 5%. Stronger security comparison is provisional unless both rho arms finish 24/24 and case-paired confidence excludes parity. A candidate performance failure is a recorded negative result, never a reason to discard data.

Source-bound GitHub Actions Rust 1.100 release CI is a producer test, not independent signed PCC. Do not promote to a leaderboard, call fastest, claim a new non-generic asymptotic attack, or merge without full repo-specific gates. Next **non-generic** lead, independent of this source optimization: blinded target-conditioned S9/K961-S5 rational f-from-R extraction without enumerable 4+4 group support or materializing 961^4, first original-ideal-certified B4/B8 then B31 complete unknown logs versus mature external IC and optimized rho. Use verified D4 injectivity and explicit Dickson31 negative targets as falsifiers.
