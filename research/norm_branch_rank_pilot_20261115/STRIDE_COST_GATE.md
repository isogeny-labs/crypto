# Prime-field ECDLP: complete cold-cost gate and rank-six matched null (2026-11-15)

**Standing:** exploratory producer-local exact arithmetic `[pcc:unverified]`; no independent signed replay, speedup claim, leaderboard promotion, or cryptographic-size result.

## Reproduce

```sh
python3 research/norm_branch_rank_pilot_20261115/stride_complete_cost.py > /tmp/stride-cost.json
python3 -m py_compile research/norm_branch_rank_pilot_20261115/stride_complete_cost.py
```

Source: `stride_complete_cost.py`; locally recorded JSON: `stride_complete_cost.results.json`.
Two local executions produced byte-identical JSON; `python -O` reproduced that JSON, and `py_compile` passed. Source SHA-256 (tested local copy): `75591a54290c6fe6aaead2393ccac208ec2e37741efdc8119b63a33145e19965`; output SHA-256: `4f76c931a17b2e6995d4272fda662e2d3507c3979bd75c19412e73438bf0483`. GitHub exact-head CI and independent PCC standing remain pending.

## New exact collision criterion

Let `S7={sum(eps_i*P_i):eps_i in {+1,-1}}` for seven fixed points in an additive abelian group. For an eighth point T,

```text
S8 = (S7+T) union (S7-T).
|S8| = 2|S7| - |(S7+T) intersect (S7-T)|.
(S7+T) intersect (S7-T) is nonempty iff 2T lies in D=S7-S7.
```

Thus, when `|S7|=128`, all 256 signed sums are distinct **if and only if** `2T` is outside `D`. This statement is elementary group theory: it makes **no reference to squared-Vandermonde coefficient rank**. In a prime-order group doubling is bijective, so precisely `r-|D|` group points T have maximal support.

On `E/F_1019: y²=x³+46x+158`, the group has prime order 977, `|S7|=128`, and `|D|=879`. Hence exactly 98 points T, or 49 opposite pairs, give collision-free eight-sign support. The previous conditioned root `x=764` is one such rank-4/6 case. **Forty-eight other distinct-x eligible eighth points have full 6/6 coefficient rank and the same maximal 256 support.** The first is `T=(34,48)`. The local script recomputes all 480 alternatives and their block ranks.

This falsifies the claim that the 6x6 rank defect is *necessary* for maximal signed coverage. It does not rule out a different target-conditioned positive-witness algorithm, a growing-k compression law, or an arithmetic attack.

## All-target complete toy DLP comparison

Both methods solve **977/977** public target logarithms `[d]G=Q` with `G=(944,936)`, and recheck every recovered `d` by scalar multiplication.

| Metric | Shared-table BSGS | Rank-4 signed | Rank-6 signed |
|---|---:|---:|---:|
| Table/support entries | 32 | 256 | 256 |
| Setup point-add calls | 32 | 429 | 409 |
| Optimized stride | 32 (giant step) | 379 | 125 |
| Average online point-add calls | 14.769703 | 1.915046 | 1.807574 |
| **Mean cold calls** | **46.769703** | **430.915046** | **410.807574** |
| Verified targets | 977 | 977 | 977 |

The rank-4 signed setup consists of 32 BSGS baby additions, 111 additional BSGS additions to obtain the **seven unknown factor-base logarithms**, 271 Gray-table additions, and 15 to compute the chosen stride. Choosing stride 379 instead of 1 drops average online search from 4.051177 to 1.915046 additions, but the mean cold cost is still about 9.21 times that of BSGS. The full-rank maximal-support control has *lower* online and cold counts than the rank-deficient base in this test.

**Optimism and accounting limitations:** Both counts omit field-operation weighting, memory, output verification overhead, subgroup/curve selection and factor-base/locus selection. The signed approaches further receive a **free** brute-force search over all 976 possible stride scalars using recovered group logarithms. This scan is infeasible at cryptographic field sizes and is uncharged, biasing the comparison in favor of the signed approach. No matched optimized Pollard-rho execution was available. The 31-target nominal amortized crossover for rank-four signed versus reused BSGS precomputation does not charge this stride scan or selection cost and is **not** a measured amortized win.

## Lead assessment and next registered experiments

1. **Sparse PKM m=8 — highest priority.** Run the frozen natural-positive panel in `research/prime_leads_chase_20261113/CONTINUATION_PROTOCOL.md` with `log_B p` near 7, 8 and 9 and two held-out larger factor-base sizes. Record complete work per call (including failed/unsatisfiable targets), natural extracted positive witnesses, rank increments, memory and final scalar verification. The necessary optimistic all-output scaling bound is `gamma<3.5`; a one-output solver needs `gamma<3`. Compare full cold time and cost to optimized rho. PR #4091 supplies small-arity PKM tower tooling but its m=2/3 cost observations are not m=8 evidence.
2. **Target-conditioned rank locus.** Demand an explicit public target-dependent base selector that does not need an unknown-log oracle. Measure *independent useful relations per full invocation*, not rank-4 matrix incidence or planted witness counts. Blind the targets and reserve held-out larger primes and k; charge factoring, point lifting, sign finding, basis logs, linear algebra, memory and failed calls. Keep B1/B2 advancement gates from #4077 intact.
3. **Chained ECDLP transfers.** Continue #3877's scalar-blind public-map and complete arithmetic cost gate. No generic group support or toy rank result provides that map.
4. **Independent replay.** Run exact-head CI, PCC claim/signature and strongest available matched BSGS/rho performance on an authorized backend before promoting any quantitative observation.

## Sources and constraints

Internal: #4188, #4077, #4091, #3877, `research/prime_leads_chase_20261113/CONTINUATION_PROTOCOL.md`. Literature: Petit–Kosters–Messeng, PKC 2016, DOI [10.1007/978-3-662-49387-8_1](https://doi.org/10.1007/978-3-662-49387-8_1); Shoup, EUROCRYPT 1997, DOI [10.1007/3-540-69053-0_18](https://doi.org/10.1007/3-540-69053-0_18) (generic lower bound only). Shared research-KB, Conductor, full Rust checkout and EKS were unavailable in the tool environment; no novelty claim. [pcc:unverified]


## General fixed-table cyclic-shift lower bound (not an algebraic ECDLP lower bound)

Let H be a *cyclic* group of order r, S a fixed table of s distinct points, and U a generator of H chosen independently of the unknown target Q. Consider the **restricted** complete decoder testing only `Q, Q+U, Q+2U, ...` for membership in S, and returning an answer when a hit occurs, with the table values' logarithms known.

1. **Worst-case coverage:** after T table tests, at most sT distinct targets can be covered, since the successful Q lie in the union `S-jU` for `j=0,...,T-1`. A guaranteed complete decoder therefore requires `T >= ceil(r/s)` and at least `ceil(r/s)-1` shifts for some target.
2. **Exact average:** order S cyclically by U and let `d_1,...,d_s >= 1` be successive gaps with sum r. For uniform Q, the total number of additions needed across all r targets is exactly `sum_i d_i(d_i-1)/2`, so the mean is `sum_i d_i(d_i-1)/(2r)`. Cauchy yields the universal `mean >= (r/s-1)/2`; balanced gaps give the exact integer minimum.
3. **Explicit table work:** the Gray-walk implementation in this directory creates its s signed candidates using `s-1` transitions plus O(log s) initial point operations; when s is a tunable parameter, its cold expected call count is at least `s-1 + (r/s-1)/2` before basis logs and other costs. Minimizing this *particular explicit-table plus sequential-shift family* over s remains `Omega(sqrt(r))`. This is **not** a Shoup-type lower bound for arbitrary coordinate-aware or algebraic algorithms and does not constrain index calculus, a target-adaptive factor-base search, nonlinear extraction or transfer.

For r=977 and s=256, balanced gaps would consist of 47 gaps of length 3 and 209 of length 4, giving a theoretical best mean online `1395/977 = 1.4278403275` calls. The conditioned base with optimally scanned cyclic stride has mean `1.915046`, above the attainable bound. Even the unrealistically ideal 1.428 online count cannot offset 255 Gray transitions, let alone seven basis logs. These are elementary exact derived bounds, still awaiting independent PCC review under lab policy. [pcc:unverified]
