# Prospective 48-Q same-host rho folding sensitivity and APS complete-cost confirmation (2026-11-16)

Status: preregistered before new Rust code or executing the confirmation. [pcc:unverified]

The distinct 24-target seed `0x52484f5f4e455733` produced 96/96 complete scalar recoveries, with 32-lane *unfolded* rho 86.369557ms versus APS 94.771239ms, but 13/24 individual APS wins and broad paired bootstrap confidence bands crossing 1. The previous 24-Q *folded* rho watchdog experiment used a DIFFERENT target panel, making direct folded/unfolded ratios invalid. Do not promote either partial result to general dominance or fastest-rho status.

### Immutable confirmation protocol

Use the identical ordinary cofactor-one prime31 curve and Rust arithmetic/solver sources from #4391. **48 fresh nontrivial distinct public Q**, fixed caller-only-scalar seed `0x52484f5f4e455734` (no overlap guarantee beyond disjoint deterministic seeds; check and report exact intersection), with frozen rho seed base `0x52484f5f43523031`, mixed per-target using `i*0xd1b54a32d192ed03`. No fitting on these unseen targets.

Five separate fully cold calls per target, chronological order rotated in all five permutations:
- A0: incumbent published APS S3 one-relation index-calculus, `[384,384,16384]`, at most six complete attempts, 64 public jointly computed jumps and batch inversions
- A1: **folded** 32-lane DP4 negation rho, existing 16-jump, canonical signed points, 512-step stagnation watchdog
- A2: **unfolded** 32-lane DP4 rho, same jump/DP settings and restart policy but no negation fold, signed x-collision recovery
- A3: **unfolded** 64-lane DP5 rho
- A4: unfolded 32-lane repeat with identical seed, independent fresh setup

Rho cap remains 2,000,000 mixed-Jacobian steps per target. Every arm includes public Q admission, jump creation, batch inversion, table, restarts/cycles, candidate scalar inversion, and final caller-verified scalar replay. Do not insert the caller-known k in any solver. Recompute every 48x5 recovered k and APS signed modular congruence, all timers, and paired target wins. Archive all raw cases, including censored/failed calls, preserving original JSON.

### Prospective gates and interpretation

Success for *measurement validity*: all 240 calls fully complete and correct, A4/A2 aggregate drift <=5%; no censored call counted as a solved sample. Report all five totals, paired wins, median target costs, 10,000 deterministic paired-case bootstrap 95% intervals for APS/unfolded32, folded32/unfolded32 and APS/folded32. Compare each method against its **own target-matched same-source** denominator; do not combine numerator from one Actions run with denominator from another. If confidence interval overlaps 1 or success is incomplete, report uncertain ranking (no overclaim). This confirmation is still development CI, not a signed independent PCC or fastest generic rho: the separate repository tuned WZ look-ahead implementation and external fastest rho remain to benchmark.

The APS method is established one-relation index calculus but explicit S3 pair MITM-equivalent (Amadori, Pintore, Sala 2018 DOI 10.1016/j.ffa.2018.01.009). Folded rho is standard Wiedner/Zuccherato; real-life fruitless cycles and observed 1.29x practical limits are discussed in Bos/Kleinjung/Lenstra ANTS 2010 DOI 10.1007/978-3-642-14518-6_9. Repository prior: `research/rho_walk_leads_20261014/README.md`; new result `APS_VS_UNFOLDED_RHO_RESULT_20261116.md`. Research-KB MCP and local Rust/Conductor unavailable; bounded repository/primary fallback, no novelty inference.

No paid EKS/GPU, merge, policy changes, PCC promotion, or leaderboard edits.
