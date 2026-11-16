#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#show link: underline

= Literature sweep, 2026-11-07: arXiv and ePrint around the `openai/math` results

*Question.* Which papers posted on arXiv and ePrint since 2026-09-08 bear on the program's knowledge-base entries about the ETH disproof (`KN-LIT-caa51d`) and the matrix-multiplication exponent (`KN-LIT-58d42d`, `KN-LIT-eef882`), and on ECDLP index-calculus leads?

*Status of every source claim:* reported by the cited paper; *not* checked here. The arithmetic in this report converts a stated $2^k$ to $log_10$ for plotting and nothing else.

== Method

- The `research-kb` MCP server and CLI were not available in this session, so the knowledge-base review used the lead atlas and repository files.
- `cryptanalysis/experiments/lit-watch` ran with a scratch configuration:
  - 60-day window, up to 50 results per query;
  - 15 arXiv queries over cs.CR, cs.CC, cs.DS, cs.SC, math.NT, math.AG and math.RA;
  - the ePrint RSS feed plus 13 ePrint searches (2 pages each).
- The fetch returned 1,601 items, with 0 failed sources.
- Ranking:
  - 225 arXiv items were dated in the window: 11 ranked as candidates, 41 as watch.
  - 1,043 ePrint search results carry no date. Their titles were scanned by hand from ID 2026/1700 onward, which roughly matches the window.
- Abstracts, and the HTML where available, were read for 16 papers. Two ePrint PDFs (2560, 2253) returned HTTP 403, so they were read from the abstract only.
- 2026/2560, 2621, 2253, 1969 and 2470 were already assessed in `research/ic_mega_pass_20261031/literature/`, and are cited from there.

== Findings

=== ETH disproof (bears on KN-LIT-caa51d and LEAD-ICB-decomposition-encoding-and-sat)

- *Dias and Lima, arXiv 2611.00517.* OpenAI's general bound beats brute force $2^n$ only beyond $n approx 10^32$ when the hidden constant is $C = 2$, $10^126$ when $C = 3$, and $10^3517$ when $C = 10$. The tensor-contraction route needs $log_2(n+m) > 10^9$.
- *Majdoddin, arXiv 2610.41622.* Anthropic's subexponential \#3-SAT algorithm crosses over at about $2^420$ clauses at density 1, lowered from $2^33858$, and at $2^(8.6 dot 10^9)$ variables in general. OpenAI's Appendix D route needs about $2^(4.4 dot 10^10)$ clauses.
- *Context, not affecting any entry:*
  - MIT Hardness Group (2611.02650): ETH is false, but the Satisfiability Hypothesis remains plausible.
  - Chan et al. (2611.06813): the 3-uniform hyperclique hypothesis is false, and Max-3-SAT runs in $O(2^(0.99 n))$.
  - Kozma (2610.41048): \#PPM from \#3SAT.
- None of these papers confirms or refutes either proof. Dias and Lima take the proofs at face value.
- *Effect.* The 2026-10-26 assessment, that the result is galactic for relation finding, now has published constants behind it. Every crossover is far above the scale of practical SAT and point-decomposition instances (@fig-x). `LEAD-ICB-decomposition-encoding-and-sat` stays `closed_obstructed`.

#figure(image("figures/crossovers.svg", width: 100%), caption: [Crossover points as stated by the cited papers (`figures/crossovers.json`). The green band is an indicative scale of practical instances, $10^2$ to $10^5$ variables; it was not measured here.]) <fig-x>

=== Matrix-multiplication exponent (bears on KN-LIT-58d42d, KN-LIT-eef882 and LEAD-ICE-first-fall-degree-assumptions)

- *Lü, arXiv 2610.18914.* Corollary 4.1 gives $omega_K <= 9/4$ for every algebraically closed field $K$: "no characteristic needs to be excluded". It also gives a cheaper shared-leg separation, with $2M - 1$ copies instead of $5M$.
  - *Derived here, standard fact:* $omega$ depends only on the characteristic, so this covers $"GF"(2^k)$.
  - This is a third route to $9/4$ in characteristic 2, alongside Amanov, van den Berg and Zuiddam (2610.30054) and Navot's Lean fork.
- *Li, arXiv 2611.04175.* An exposition of the OpenAI result; a summary reports that it argues no step needs characteristic 0.
- *Uznański, arXiv 2610.10494.* $omega(1, k, 1) <= 2$ for $k <= 1/2$, in characteristic 0 only.
- *Gao and Hu, ePrint 2026/2622.* Lattice sieving improves only if $omega <= 9/4 - eta$.
- *Sun, arXiv 2610.13971* (math.GM). Claims $omega_K <= 2 + 1/n$ for all $n >= 5$ over every field, i.e. $omega = 2$. Only the abstract was read. It is an *unverified outlier*: it changes no row and must not be used as an input.
- *Effect.* No number changes. The $9/4$ row stays at $2^68.96$ at $n = 131$, an existence-only exponent under the contested first-fall-degree model.

=== Classical ECDLP and algebraic solving

- *Xiong and Wang, ePrint 2026/2625.* Factorization over $F_q$ in $n^(1.5532 + o(1)) log q$, against the previous $n^1.80535$. It is asymptotic, the abstract does not discuss root finding, and no cost row changes.
- *Budroni et al., ePrint 2026/1711* (PKP/PEP, code-based schemes): no bearing on ECDLP; discarded.
- *ePrint 2026/2469* is the program's own GPU rho paper on ECC2K-130; already recorded.
- *Already assessed on 2026-10-31* (no new action): 2026/2560 (multi-user collision walks), 2621 (polynomial approximations of dlog), 2253 (multi-instance generic bounds), 1969 (Dixon resultants), 2470 (XL for RSDP).
- *Negative result:* no new paper in the window targets ECDLP index calculus, summation polynomials or Weil descent of binary curves directly.

#figure(image("figures/source_map.svg", width: 100%), caption: [Sources, the knowledge-base entries and leads they bear on, and their effect.]) <fig-map>

== Unresolved checks

- The OpenAI and Anthropic proofs: no third-party verdict yet.
- Build Navot's Lean fork, and check Lü's Corollary 4.1, before re-tagging `CORPUS_FAMILY_107` in `src/cryptanalysis/matmul_exponent.rs`.
- Sun's $omega = 2$ claim: no independent review found.
- Re-run the knowledge-base queries once `research-kb` is reachable.

== Sources

arXiv: 2611.00517, 2610.41622, 2611.02650, 2611.06813, 2610.41048, 2610.18914, 2611.04175, 2610.13971, 2610.10494, 2610.30054, 2610.28627. ePrint: 2026/2622, 2625, 1711, 2469, 2560, 2621, 2253, 1969, 2470. Lean: github.com/selanavot/matrix-multiplication-all-fields.
