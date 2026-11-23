# rho.negation_v5 seal audit (2026-11-23): results

Crypto PR (draft): https://github.com/isogeny-labs/crypto/pull/4993, branch `claude/rho-v5-seal-20261123`. Protocol and predictions: PROTOCOL.md (frozen before the fp36/fp40 data). Standing: `[pcc:unverified]`, L0 on a shared 4-CPU Xeon VM (wall indicative only).

## Instrument
`ecbench` built from the #4860 merge `8f33a645b` (the tip of main at 816f9c3a9 does not compile: unclosed delimiters in `binary_hyperelliptic_index_calculus.rs`, `efd_representations.rs`, `ecbench/spec.rs`, a duplicated `[[example]]` key in Cargo.toml, all from merge damage; no repair committed). Binary sha256 f273255f..., host class ECBENV3ha7153aea3de6 (Linux x86-64). The producer sessions were made on a different class (ECBENV3h32d7ea52a547, macOS, `git_commit` null).

## 1. Independent replay (PASS)
`ecbench verify --replay-all` on every session of the v5 evidence, each record re-executed: fp20 384/384, fp24 384/384, expanded fp28 1296/1296, expanded fp32 1296/1296, large fp32 3072/3072 identical, 0 problems. Replay of counts is host independent, so this is a cross-host, cross-OS reproduction of S. Receipts: audit-*.json. Not done: wall time does not replicate across hosts; no same-class L2 rerun (no EKS/S3/conductor/ecdb credentials were used or available here).

## 2. Integrity pass (PASS with findings)
Over the five cap64 sessions (python on records.jsonl):
- every row verified with matches_target; total_gae equals the phase sum on every row.
- Accounting: v5 lists extra unpriced work (DP table inserts/queries, about 1.6% of steps) that the baseline arm does not list; the baseline's own DP table traffic is not tallied, so the asymmetry is against v5's appearance and below 1% of cost. Canonicalisations per step are about 1.0 in both arms.
- Mechanism (large fp32, per solve): steps -4.9%, look-ahead additions -77%, cycle-escape doublings -89%, setup additions -56%, distinguished points x1.91 (dp bits 7 vs 8, each DP costs a charged start addition). Ratio of means S 0.926. Excluding the 21 pairs where the baseline capped a walk: 0.937, so the gain is not a capped-walk tail artefact.
- Cap behaviour: baseline walks_capped 22/1024 at fp32, v5 0; the weight-2 cap of 64 jumps binds from about fp28 (GAE-optimal k about 128 at fp28, 256 at fp32, about 450 at fp36, about 1000 at fp40).
- Finding (seed/selection): every v5 panel (pre-cut multicurve, pow2, cap64, expanded, large) reuses the same curve seeds (10007, 16180, ...), target_seed 727 and measurement seed 181. The pow2 and cap=64 choices were made after seeing wall CIs on those panels, so fp20..fp32 are in-sample for the tuning. The fp36/fp40 runs and the challenge verdict below are out of sample.
- Finding (A/A): the S of the A/A arm is identical to the reference in every pair (same seed, deterministic), so the S noise floor is vacuous (1.0000 [1.000, 1.000]); only the wall A/A is informative (0.9959 [0.988, 1.004] at fp40).
- Pairing: arms share the algorithm seed within (workload, round) (common random numbers); seeds differ across rounds and workloads; intervals cluster by workload.
Mean-of-ratios per pair is 1.38 (heavy tail), so only ratio of totals (the harness statistic) is quoted.

## 3. Sealed bounds and challenge verdict (records land; verdict does not advance)
Fitted and `bound check`-re-derived (docs/bounds/records/prime-rho-neg-v5*):
- toy fp20..32 (44 sizes, 1712 runs): v5 ECBND1h11f22b0448e7 ops 1.028 x floor [0.993, 1.066], alpha 0.481 [0.466, 0.498]; rho.negation ECBND1haa3253d7c6dc 1.148 [1.101, 1.201], alpha 0.457 [0.442, 0.472]. Memory per sqrt r: 0.0137 vs 0.0075.
- medium fp36..40 (32 sizes, 3072 runs): v5 ECBND1h9ad0da327bbc 1.029 [0.998, 1.059], alpha 0.511 [0.484, 0.537]; rho.negation ECBND1h0b3d84a07698 1.074 [1.048, 1.101], alpha 0.492 [0.471, 0.512]; memory 0.0026 vs 0.0013; field muls per sqrt r 1.83 vs 1.91, invs 0.912 vs 0.952.
- Alphas are pooled over curves of different size with set-up overhead at toy sizes; not an asymptotic claim.
Standing challenge prime-toy-reference (rho.negation incumbent), candidate committed first (ECCM1h24e7035ea316), beacon = git commit b07138dc3c..., epoch 7753219574668632271, 96 pairs on 4 sizes, replay-all, A/A determinism 1.0000: verdict ECVD1h9dced72c7ba9 **regresses (worse on memory)**: ops 0.811 [0.712, 0.924] but its Holm-adjusted verdict is indistinguishable, memory 1.44x [1.22, 1.71] worse. Under the repository's ops+memory acceptance rule v5 is a trade (about 1.4x to 2x table memory for 3-25% fewer ops), not an advance, and no baseline replacement is licensed by that rule. (Challenge isolation_required is L2; this ran at L0, counts only.)

## 4. fp36 / fp40 multi-curve (out of sample) vs rho.negation
16 PARI-generated cofactor-1 curves (prime_search stops at 32 bits), 8 targets, 12 rounds = 1536 pairs per size, 4608/4608 rows verified, replay-all identical (4608/4608 each).

| bits | S cand/rho-neg | 95% CI | end-to-end wall | 95% CI | ns/GAE | accept (both upper <= 1) |
|--:|--:|:--|--:|:--|--:|:--|
| 36 | 0.9449 | [0.908, 0.984] | 0.9552 | [0.920, 0.992] | 1.0065 [0.998, 1.015] | yes |
| 40 | 0.9721 | [0.933, 1.013] | 0.9768 | [0.938, 1.017] | 1.0030 [0.996, 1.010] | no (CIs straddle 1.0) |

Predictions (frozen): S point 0.93-0.98 at both sizes (confirmed), wall 0.93-1.03 (confirmed), falsifier S > 1 (not triggered). The gain shrinks with size: S by bits 20/24/28/32/36/40 = 0.750/0.781/0.920/0.928/0.945/0.972; a crude linear fit (slope 0.0115 per bit) reaches 1.0 near 40-42 bits, consistent with the cap-64 mechanism, so v5 as capped is not a size-robust baseline beyond about fp40 without lifting the cap (and its cache cost).

## Verdict
Replay: PASS. Integrity: PASS with two findings (in-sample tuning, vacuous S A/A). Sealed bound records: landed. Challenge verdict: regresses on memory (trade). fp36: accept; fp40: CIs open. Baseline/registry replacement PR: NOT prepared, because steps 3 and 4 did not both pass; the evidence PR stays draft.
