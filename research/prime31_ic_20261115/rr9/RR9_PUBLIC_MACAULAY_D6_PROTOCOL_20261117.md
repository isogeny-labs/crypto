# EXP-R31-RR9-MACAULAY-D6 — separately prospective follow-up
Date: 2026-11-17 UTC. Source: stacked draft PR #4549 on #4548. [pcc:unverified].

## Frozen hypothesis and scope
Repeat exactly the previously published public-residual-only RR9 B8 coefficient ideal over the **ordinary cofactor-one** E/F_2147483647 with a=847514846, b=2087444724, n=2147446831, G=(625037715,2100627251). Use the existing Rust native library and exact q_0..q_7 from #4548; normalize V_3=1 and require M=H_B, with H_B from x=[0,1,2,3,5,6,8,9].
Fix the three public R before any execution:
- positive: R=(1911579686,548820096), whose eight signed points sum to R (classified **only** by independent test oracle);
- negative1: R=(625037715,2100627251);
- negative2: R=(1923055775,307731458).
Both negatives were independently excluded from all 256 B8 sign sums in #4548. Pass *only* public R, H_B, curve constants to the candidate, no signs, scalar, precomputed group lookup, or secret side channel.

## Single new bound / falsifier
Compute Macaulay row span of m*q_i for all monomials m of degree <=4, with total degree <=6 in seven coefficient variables.
Predicted dimensions, not yet measurements: C(13,6)=1716 columns, 8*C(11,4)=2640 generated rows, maximum dense pivot storage approx 1716²*8=23.56MB; the worker process may consume more.
Use the exact same modular pivot algorithm as #4548; no algorithm rewrite, random fixtures, modification of prior protocol, or post-data degree increase. A nonzero constant unit certificate 1=sum c_mi m*q_i must be replayed exactly against original quadrics. A returned positive coefficient vector must satisfy all original q_i and independently orient and sum eight signed curve points to R; no unverified positive inference from a dimension count. If neither appears: **inconclusive**, including negative cases.
Every case reports columns, rows, rank, multiplications, inversions, reductions, wall and pivot storage. Exit on assertion failure. Restrict CI runtime to <=15 min for the degree6 example and <=1GiB address space for candidate binary; retain raw JSON artifact even on failure. No paid compute. No rerun with a raised degree or changed target.

## Acceptance / interpretation
- Source-specific pinned Rust 1.100.0 workflow must run existing library tests, earlier degree5 controls and new degree6 positive/negative tests; raw JSON schema and target identities checked.
- An ideal unit certificate is a mathematically sound **negative** only for the fixed squarefree B8 chart. A positive solution is chart-specific, not a complete B31 solver. No speedup claim from a certificate or rank.
- Historical #4532 one-run same-cohort 1024Q mixed IC 2221.474 ms vs cached generic BSGS 170.113 ms, not rerun here; compare absolute phase cost descriptively, never infer matched full-solver speedup.
- Follow-up: prove or refute the degree5 56 free-coordinate pattern using independent exact syzygy/rank audit and structurally generic quadrics, then test *public-only* B31 unknown-target complete solver only after positive coefficient extraction. No leaderboard update or signed PCC absent independent validation.

## Prior-art receipt
Repository evidence #4466, #4373, #4228/#4349, #4513, #4532, #4539, #4541, #4545, #4546, #4547, #4548; crypto-autoresearcher #3301/#3305. Relevant published work: Amadori–Pintore–Sala, FFA 51 (2018) 168–182, DOI 10.1016/j.ffa.2018.01.009; Kudo–Yokota–Takahashi–Yasuda, CANS 2018, DOI 10.1007/978-3-030-00434-7_19. Their prime-field index-calculus and hybrid/Gröbner investigations limit any novelty claim. Research KB MCP and wrapper are inaccessible here; fallback was published prior source, original PR chain and relevant journal listings, not exhaustive literature search.
