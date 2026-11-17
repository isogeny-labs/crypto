# Literature sweep, 2026-11-07: arXiv and ePrint around the `openai/math` results

Full report: [`report.pdf`](report.pdf) (source [`report.typ`](report.typ)).

Figures:
- [`figures/crossovers.svg`](figures/crossovers.svg). Data: `figures/crossovers.json`; script: `figures/plot_crossovers.py`.
- [`figures/source_map.svg`](figures/source_map.svg). Script: `figures/source_map.py`.

Every source claim below is reported by its paper and not checked here [pcc:unverified].

## Scope and method

- arXiv and ePrint, posted 2026-09-08 to 2026-11-07, swept with `cryptanalysis/experiments/lit-watch`. The configuration is reproduced in `report.typ`.
- 1,601 items fetched. 16 papers read at abstract or HTML level.
- The `research-kb` server was not reachable in this session.

## Findings

- **ETH disproof** (`KN-LIT-caa51d`). Dias and Lima (arXiv 2611.00517) put OpenAI's crossover against brute force at n ≈ 10^32 to 10^3517 variables, for hidden constant C = 2 to 10. Majdoddin (arXiv 2610.41622) puts Anthropic's #3-SAT crossover at about 2^420 clauses at density 1. The assessment that the result is galactic for relation finding stands. `LEAD-ICB-decomposition-encoding-and-sat` is unchanged.
- **ω ≤ 9/4** (`KN-LIT-58d42d`). Lü (arXiv 2610.18914) gives it over every algebraically closed field. ω depends only on the characteristic, so this covers GF(2^k): a third reported route. Sun (arXiv 2610.13971) claims ω = 2 [pcc:unverified]. It is an outlier, not used as an input.
- **Algebra.** Xiong and Wang (ePrint 2026/2625): factorization over F_q in n^1.5532 log q. Asymptotic only.
- **Negative result.** No new paper in the window targets ECDLP index calculus, summation polynomials or binary Weil descent directly.

No cost-model number changes.
