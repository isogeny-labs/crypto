# EXP-R31-APS-UNEQUAL-20261116 — Rust one-relation unequal labelled partitions

**Preregistered before implementing/launching this exact prime31 variant.** Standing proposed, no timings or CI outputs at this point. This is **published APS single-relation index-calculus methodology using S3 pair-root enumeration, mathematically generic MITM-equivalent**; it is not a claimed new non-generic prime-field attack. Its purpose is a falsifiable complete-cold solver optimization, not a claim against the external PKM/APS/CAS frontier.

## Literature, repository exclusions, and scope

- Amadori–Pintore–Sala (2018), FFA 51, 168–182, DOI 10.1016/j.ffa.2018.01.009: one-relation algebraic DLP; the present Rust S3 finite-list search is a specialization with generic pair-support complexity.
- Petit–Kosters–Messeng (2016), PKC 2016, DOI 10.1007/978-3-662-49387-8_1: separate compositional factor-base prime-field IC baseline.
- Repo `research/aps_cost_leads_20261106/PROTOCOL.md` has an **unequal profile** arm for *small-field* APS and a separate elimination-choice experiment; no prime31 24-Q complete-cost result there. Reuse conceptual experimental controls, do not claim first priority.
- `#4383`: equal T1024 exploratory win reversed on 24 fresh Q, so use matched heldouts and A/A. `#4391`: complete T1536 Rust walk+Mersenne source and 24-Q field speed control; preserve it unchanged as primary complete-solving incumbent. `#4373` m8 S3-chain and `#4188` F4 remain censored; `#4391` anchored m6 has <=3.7407% support ceiling; `#4228/#4349` certified quotient at wide prime remain a different mathematical task. The cryptanalysis repo already has symmetrized Semaev machinery. Never repackage generic list collision as a new non-generic complexity result.
- KB MCP not loaded in this tool runtime; no local repo wrapper/isolab-EKS access. Bounded current GitHub primary source and publisher pages used. This is not an exhaustive novelty clearance.

## Deterministic curve, data and arms

Same ordinary cofactor-one curve `p=2147483647`, `E:y²=x³+847514846*x+2087444724`, `n=2147446831` prime and existing Rust `PRIME31` generator. Compare exactly the same 24 **fresh** caller-generated nontrivial `Q=[k]G`, with each `k` kept outside the unknown-log solver, from a preregistered non-historical seed `0x554e455155414c32`. Use existing three-list S3 and full signed `A+B*k=0` replay. No generic point-sum-table fallback or hidden oracle.

| Arm | (|A|,|B|,|C|) | Max public rounds | Hypothesis |
|---|---:|---:|---|
| control | (1536,1536,1536) | 6 | Established fully cold Mersenne+walk incumbent |
| unequal-256 | (256,256,32768) | 6 | Fewer S3 quadratic tests at greater walk-source cost |
| unequal-384 | (384,384,16384) | 6 | Intermediate source/query balance |
| unequal-256-repeat | (256,256,32768) | 6 | A/A drift control, identical source law |

For even-index Q execute control, 256, 384, 256-repeat; for odd-index Q execute 256, 384, 256-repeat, control. Source walks use **one** 64-step public table per round, a global dedup set of x abscissae across all three parts, and independent per-part seeds and point labels using the existing pseudorandom stream. Equal (1536,1536,1536) generation and S3 solver must remain unchanged; tests check exact source equality against the prior equal-T method (test-only).

Every arm charges cold curve check, 64 step-table building, labelled-point setup, all retries, every algebraic S3 discriminant/square root/batch inversion, candidate signed y/full-point verification, and final log replay. Count actual pair queries, source additions, duplicated x, failed rounds and wall. Record stdout of every Q/arm, including incomplete cases. Do not drop failed cases from cost ratios or call a censored experiment faster.

## Mathematical model, before observing outputs

For list sizes A,B,C, the *raw ordered signed triple* mean under uniform group placement is 8ABC/n, but global negation and label correlations reduce unique relations. For coordinate-root discovery, at most 2ABC S3 root-candidate checks are available; the actual useful B!=0 yield is empirical. The asymmetric profiles trade O(AB) S3 pair work against O(A+B+C) labelled public point generation. The same randomized 64-step walk is **not** an independent uniform draw: no Poisson success probability is assumed as a conclusion.

## Success/falsification criteria

- Each candidate must recover and independently replay **all 24 unknown Q scalars**, without a precomputed group pair-index or target-secret hint, with every failed attempt charged.
- Primary metric: sum of 24 fully cold in-process solver walls, also report each paired target, median, 24/24 wins, total group additions and S3 quadratics. A/A candidate aggregate drift and execution-order effect must be reported. No sole reliance on a smaller algebraic substage.
- *Provisional engineering win only* if 24/24 recover, cold aggregate <=0.8 times the incumbent and 256-repeat stays within 5% aggregate; require independent L2/PCC and external mature PKM/APS/CAS comparison before scientific promotion.
- If a profile fails a Q in six rounds, it is **not** a complete solver for this panel: report bounded failure and no all-correct speed ratio. Even if it wins, it remains generic MITM-equivalent and cannot satisfy the separate novel non-generic complexity goal.
- Independently continue selected-root nonenumerative target-conditioned m8 K/S5 with exact original-ideal certificates. Do not use these algebraic-stage development timings as a substitute for that work.

## Reproduction

Existing Rust-only `research/prime31_ic_20261115/native/Cargo.toml`, locked Rust 1.100.0, release example tests and `--unequal-confirm` mode, same host CI. This protocol does not authorize paid EKS and records no source-execution result until a specific commit's job completes.
