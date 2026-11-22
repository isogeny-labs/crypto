# Prime31 run58 — early individual-log stop (2026-11-22)

**Scope:** Exact target-specific rank criterion and conditional Monte Carlo on the unchanged ordinary Prime31 elliptic curve: p=2147483647; E: y²=x³+847514846x+2087444724; n=2147446831. No actual public-only S3/F4 extraction, new complete ECDLP log, or speed advantage is claimed.

## Previous leads checked

- This PR #4901 runs 55–57: 509-point Dickson factor base with 518085 distinct signed pair sums / 13 free homogeneous ranks; 677-point Dickson+y⁷ hybrid with 916469 distinct sums / 88 free ranks. Source-linked rank57 Rust CI passed at https://github.com/isogeny-labs/crypto/actions/runs/45003247773. Reuse exact run57_basis.json and the existing original-Rust signed DSU, not a new arithmetic implementation.
- Prior graph-cycle/one-target work: research/graph_cycle_ic_20261007/cyc.py, examples/ic_m3_cycle.rs, research/prime31_ic_20261115/native/M3_RANK_EARLY.md. Target early stopping is already known and is NOT claimed as a novel general algorithm.
- Amadori–Pintore–Sala, Finite Fields and Their Applications 51 (2018), DOI https://doi.org/10.1016/j.ffa.2018.01.009, studied prime-field one-relation index calculus. Current RR9/Horner/F4 lanes are #4692, #4812 and #4822; S3 and batch-root fast paths #4809 and #4846. Do not duplicate their unknown-index extraction engines. Shared research-KB MCP not accessible in this runtime; this is a limited repository/primary-paper audit.

## Exact mathematical sufficient criterion

Let L_i=log_G(P_i), the signed affine graph represent L_i = s_i U_component+d_i, with sign s_i=±1, known offset d_i, and an unknown component parameter U except in anchored components. Every valid known-scalar relation [k]G=a P_i+b P_j gives a L_i+b L_j=k mod n. A non-balanced cycle (or double-point row) fixes one component parameter, since n is odd.

For target Q=[t]G, a public shifted relation Q+[u]G=a P_i+b P_j yields t=a L_i+b L_j-u, immediately computable if either: (1) both endpoints are anchored or (2) they are in the same unanchored component and a*s_i+b*s_j=0. A candidate t must be independently verified by [t]G=Q.

Among the exactly 2B² nonidentity unordered signed pair templates, let A be the number of factor-base indices in anchored components, and let s_j be the sizes of all unanchored components. The exact conditional **sufficient-event** probability is:

    p_one = [2*A^2 + SUM_j s_j*(s_j-1)] / [2*B^2].

Proof: 2*A² signed templates with both endpoints anchored are usable (including doubled points). For each pair of distinct indices in the same unanchored component exactly 2 of the 4 signed choices cancel its free-root coefficient; this adds s_j(s_j-1) templates. Other templates leave an unknown root. Reproduced by exhaustive enumeration of every nonidentity signed template across 51 random small graphs (B=4..20, three seeds each). Extra frozen 13/88 homogeneous equations and combinations of multiple target rows can only improve on this sufficient event; they are intentionally not used here.

## Preregistered conditional experiment

- Frozen original-curve nonidentity two-point support S=518084 / 916468, group order n=2147446831. The positive known-scalar rows are **synthetically sampled uniformly over signed pair templates**. This is a statistical oracle model, not an actual Semaev solve; rare repeated group-sum representations are not modeled exactly.
- Independent Python script run58_early_target_model.py: 256 deterministic streams per arm, 51 exhaustive checks. Mersenne Twister seed 0x58ac16e3+0x9e3779b1*trial+[0 or 104729]. Base positive-hit grid m=0,100,200,300,400,500,600,700,800,900,1000,1200,1500,1800,2300,2800,3500; batch size T=1,32,128,1024.
- Exact p_one per graph. The correct average target-positive wait is E[1/p_one], **not** 1/E[p_one]. If any trial has zero sufficient probability, that grid row is ineligible. Expected uniform probes: Q_T(m)=[m+T*E(1/p_one)]*n/S. This includes expected failed support probes but EXCLUDES real S3/F4 runtime, scalar multiplication, precompute, linear algebra, verification, memory, exceptions and real query-output bias. No overall ECDLP performance follows from it.

| Base | Targets | Best positive base hits | Model uniform probes | Mean p_one |
|---|---:|---:|---:|---:|
| Dickson 509 | 1 | 300 | 1,335,005 | 0.074 |
| Dickson 509 | 32 | 400 | 2,000,851 | 0.390 |
| Dickson 509 | 128 | 500 | 2,928,025 | 0.621 |
| Dickson 509 | 1024 | 700 | 7,907,205 | 0.848 |
| Hybrid 677 | 1 | 400 | **988,031** | 0.077 |
| Hybrid 677 | 32 | 500 | **1,405,729** | 0.324 |
| Hybrid 677 | 128 | 600 | **1,980,307** | 0.524 |
| Hybrid 677 | 1024 | 900 | **5,002,567** | 0.830 |

Run57's fully populated hybrid factor-base model, **optimistically assuming one target hit per log after full rank**, requires ~5.375M model probes for one target and ~7.772M for 1024. The partial-rank model suggests a conditional **81.6% and 35.6% reduction**, respectively, relative ONLY to that model strategy. It is not a measured improvement over optimized BSGS, rho, or existing index calculus.

## Rust reuse, independent checks and next gates

The existing original-Rust research/prime31_ic_20261115/native/examples/prime31_rank_completion_gate.rs has been extended with a signed graph target-closure calculation, reusing its SignedDsu, SplitMix64 and the original Arithmetic/check_curve(PRIME31). The slim cached GitHub workflow runs both the standalone Python exact-gate and a separate 128-trial native Rust cohort. Do not mark native execution verified until a source-specific green CI run is confirmed.

1. Implement signed **affine offsets** and actual t recovery on valid public target relations in the existing native Rust collector, with independently replayed [t]G=Q.
2. Replace synthetic positive pair samples with **real unknown-index public-only S3** from #4809/#4846. Charge ALL negative probes, timeouts, rare multi-output cases, scalar shifts, field ops and memory. Test m=300,400,500,600,700,900 and batch sizes 1/32/128/1024 on the same public Prime31 targets.
3. Use the frozen support and homogeneous rows but quantify real oracle bias, target stopping accuracy, rank increments and extra free-row effects. Treat incomplete degree bound as unknown, not UNSAT.
4. Only if all ECDLP logs verify, compare fully charged end-to-end native Rust against best same-host negated BSGS, optimized rho and index-calculus frontier. A favorable reproducible confidence interval is required before claiming a cheaper complete solver.

**Result at this scope:** A rigorously checkable partial-rank stopping and conditional query-budget lead. The public-only algebraic root-extraction bottleneck remains unbroken.

## Source-specific CI evidence (2026-11-22 09:32 UTC)

[GitHub Actions run #45007882519](https://github.com/isogeny-labs/crypto/actions/runs/45007882519) **completed successfully** on source commit bcf71939742844563aeb28b67a69baff0f692a63. Its logs show independent 500/500 prior rank checks, 51/51 exhaustive run58 small-graph checks, 512/512 deterministic Python conditional trials, source-linked original Rust release build in 16.38s, original 509-point two-summand arithmetic census, and a separate 128-seed **native Rust** signed-target model. Native-Rust hybrid results: 985793 modeled uniform queries for T=1, 4994824 for T=1024 (the latter at m=900, mean single-positive target closure ~0.83190). Independently implemented Python found 988031 / 5002567 with different random generators. These are **model-based estimates** and do not constitute measured non-generic extraction or an ECDLP win.

Separately, the 64-fixture actual-prime31 affine-potential validation passed locally with an independently written Python EC group law. Its source was subsequently committed as run58_affine_recovery_verify.py and added to the cached Rust CI workflow; do not claim source-specific CI PASS for that subsequent change without observing it.
