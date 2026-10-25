# Verification of LEAD-18 (mechanical tier)

Dossier: leads/LEAD-18.md. Procedure: BRIEF_HAIKU.md. Date: 2026-10-22.

## Citations
| # | citation as written | route checked (URL or query) | outcome | page shows |
|---|---|---|---|---|
| 1 | Faugère, Mou, *Sparse FGLM algorithms*, J. Symb. Comput. 80 (2017), arXiv:1304.1238 | WebFetch https://arxiv.org/pdf/1304.1238 (returned unreadable compressed PDF bytes, no title); WebSearch `"Sparse FGLM algorithms" Faugère Mou` | VERIFIED | result title "Sparse FGLM algorithms" at arxiv.org/pdf/1304.1238; search summary: Jean-Charles Faugère, Chenqi Mou, J. Symb. Comput. 2017, vol. 80, pp. 538–569 |
| 2 | Berthomieu, Eder, Safey El Din, *msolve: a library for solving polynomial systems*, ISSAC 2021, arXiv:2104.03572 | WebFetch https://arxiv.org/pdf/2104.03572 (returned unreadable compressed PDF bytes, no title); WebSearch `"msolve: a library for solving polynomial systems" Berthomieu Eder Safey El Din` | VERIFIED | result title "msolve: A Library for Solving Polynomial Systems" at arxiv.org/pdf/2104.03572; search summary: ISSAC 2021, pp. 51–58, authors Berthomieu, Eder, Safey El Din |
| 3 | msolve README, https://github.com/algebraic-solving/msolve | WebFetch https://github.com/algebraic-solving/msolve | VERIFIED | page title "GitHub - algebraic-solving/msolve ..."; heading "MSOLVE: Multivariate polynomial system solver"; citation section names J. Berthomieu, C. Eder, M. Safey El Din, 2021, ISSAC (the dossier entry names no author) |
| 4 | Berthomieu, Neiger, Safey El Din, *Faster change of order algorithm for Gröbner bases under shape and stability assumptions*, ISSAC 2022, arXiv:2202.09226 | WebFetch https://arxiv.org/abs/2202.09226 | VERIFIED | title "Faster change of order algorithm for Gröbner bases under shape and stability assumptions"; authors Jérémy Berthomieu, Vincent Neiger, Mohab Safey El Din; 2022; venue shown: arXiv preprint cs.SC, no journal or conference |
| 5 | Joux, Vitse, *Elliptic curve discrete logarithm problem over small degree extension fields*, J. Cryptology 26 (2013), https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/DLFq5.pdf | WebFetch on that URL (returned unreadable compressed PDF bytes, no title); WebSearch `"Elliptic curve discrete logarithm problem over small degree extension fields" Joux Vitse` | VERIFIED | search summary: ePrint 2010/157, J. Cryptology 2013, title "Elliptic Curve Discrete Logarithm Problem over Small Degree Extension Fields – Application to the Static Diffie-Hellman Problem on E(F_q^5)"; authors appear in the summary only as Joux–Vitse |
| 6 | Faugère, Gianni, Lazard, Mora, *Efficient computation of zero-dimensional Gröbner bases by change of ordering*, J. Symb. Comput. 16 (1993) 329–344 | no URL or identifier in dossier; WebSearch `"Efficient computation of zero-dimensional Gröbner bases by change of ordering" Faugère Gianni Lazard Mora` | UNREACHABLE | no result page carries this title; search summary, not a page: J.C. Faugère, P. Gianni, D. Lazard, T. Mora, J. Symbolic Comp. 16:329–344, 1993 (from snippets) |
| 7 | Cantor, Zassenhaus, *A new algorithm for factoring polynomials over finite fields*, Math. Comp. 36 (1981) 587–592 | WebFetch https://www.ams.org/journals/mcom/1981-36-154/S0025-5718-1981-0606517-5/ (HTTP 403); WebSearch `"A new algorithm for factoring polynomials over finite fields" Cantor Zassenhaus` | VERIFIED | search link title "A new algorithm for factoring polynomials over finite fields" at ams.org/mcom/1981-36-154/S0025-5718-1981-0606517-5/ (AMS page itself not read); search summary: David G. Cantor and Hans Zassenhaus, Math. Comp. 36 (1981), 587–592 |
| 8 | von zur Gathen, Gerhard, *Modern Computer Algebra* | no URL or identifier in dossier; WebSearch `"Modern Computer Algebra" von zur Gathen Gerhard` | VERIFIED | listing title "Modern Computer Algebra - Joachim von zur Gathen, Jurgen Gerhard" (Librairie Eyrolles); search summary: Cambridge University Press, 3rd ed. 2013 |
| 9 | Faugère (F4, 1999) | none: no URL, identifier or exact title in dossier | UNREACHABLE | no route applicable; no page |
| 10 | Bardet–Faugère–Salvy (F5 complexity, 2015) | none: no URL, identifier or exact title in dossier | UNREACHABLE | no route applicable; no page |
| 11 | Gaudry (2009) | none: no URL, identifier or exact title in dossier | UNREACHABLE | no route applicable; no page |
| 12 | Faugère–Gaudry–Huot–Renault (J. Cryptol. 2014) | none: no URL, identifier or exact title in dossier | UNREACHABLE | no route applicable; no page |

## Repository paths
| # | path as written | tested as | outcome |
|---|---|---|---|
| 1 | src/cryptanalysis/gaudry_quartic_d4.rs | /home/user/crypto/src/cryptanalysis/gaudry_quartic_d4.rs | EXISTS |
| 2 | research/gaudry_quartic_d4_20261013/README.md | /home/user/crypto/research/gaudry_quartic_d4_20261013/README.md | EXISTS |
| 3 | research/gaudry_quartic_d4_cost_20261014/README.md | /home/user/crypto/research/gaudry_quartic_d4_cost_20261014/README.md | EXISTS |
| 4 | research/gaudry_quartic_replay_20261013/README.md | /home/user/crypto/research/gaudry_quartic_replay_20261013/README.md | EXISTS |
| 5 | research/ic_speed_and_rho_scope_20261013/README.md | /home/user/crypto/research/ic_speed_and_rho_scope_20261013/README.md | EXISTS |
| 6 | research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | /home/user/crypto/research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | EXISTS |
| 7 | research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md | /home/user/crypto/research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md | EXISTS |
| 8 | groebner_compare/README.md | /home/user/crypto/groebner_compare/README.md | EXISTS |
| 9 | research/gb_cas_comparison_20261011/README.md | /home/user/crypto/research/gb_cas_comparison_20261011/README.md | EXISTS |
| 10 | research/f4_fp_msolve_parity_20261011/ | /home/user/crypto/research/f4_fp_msolve_parity_20261011/ | EXISTS |
| 11 | research/ic_next10_20261014/02-solution-count/scripts/run_msolve.sh | /home/user/crypto/research/ic_next10_20261014/02-solution-count/scripts/run_msolve.sh | EXISTS |
| 12 | research/magma_f4_comparison_20261013/ | /home/user/crypto/research/magma_f4_comparison_20261013/ | EXISTS |
| 13 | examples/gaudry_quartic_d4.rs | /home/user/crypto/examples/gaudry_quartic_d4.rs | EXISTS |
| 14 | examples/gb_cas_export.rs | /home/user/crypto/examples/gb_cas_export.rs | EXISTS |
| 15 | research/magma_f4_comparison_20261013/scripts/d4_p269_seed1.magma | /home/user/crypto/research/magma_f4_comparison_20261013/scripts/d4_p269_seed1.magma | EXISTS |
| 16 | research/pdp_literature_leads_20261022/BRIEF_SONNET.md | /home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md | EXISTS |
| 17 | docs/ic/LEADERBOARD.md | /home/user/crypto/docs/ic/LEADERBOARD.md | EXISTS |

## Quoted numbers
| # | number | file | grep outcome (line) |
|---|---|---|---|
| 1 | 512 | src/cryptanalysis/gaudry_quartic_d4.rs | FOUND (line 24; 5 matches) |
| 2 | 85 | research/pdp_literature_leads_20261022/LEADS.md | FOUND (line 192; 6 matches) |
| 3 | 513 | src/cryptanalysis/gaudry_quartic.rs | NOT FOUND (0 matches) |
| 4 | 2.54·10¹⁰ | research/gaudry_quartic_d4_20261013/README.md | NOT FOUND (0 matches) |
| 5 | 512 | research/gaudry_quartic_d4_20261013/README.md | FOUND (line 21; 3 matches) |
| 6 | 19 of 20 | research/gaudry_quartic_d4_20261013/README.md | FOUND (line 74) |
| 7 | 2.547·10¹⁰ | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | FOUND (README line 42) |
| 8 | 6.89·10⁷ | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | FOUND (README line 50) |
| 9 | 8.21·10⁷ | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | FOUND (README line 70) |
| 10 | 8.06·10⁷ | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | FOUND (README line 71) |
| 11 | 370× | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | FOUND (README line 57; 2 matches) |
| 12 | 1.39·10⁶ | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | FOUND (README line 100) |
| 13 | 2^{103} | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | FOUND (README line 1) |
| 14 | 4.94·10⁷ | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | NOT FOUND (0 matches in each) |
| 15 | 72 % | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | NOT FOUND (0 matches in each) |
| 16 | 1.30·10⁷ | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | NOT FOUND (0 matches in each) |
| 17 | 6.3·10⁶ | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | FOUND (README line 23) |
| 18 | 1.41·10⁸ | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | FOUND (README line 50) |
| 19 | 4.88·10⁷ | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | NOT FOUND (0 matches in each) |
| 20 | 59 % | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | NOT FOUND (0 matches in each) |
| 21 | 2.52·10⁷ | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | NOT FOUND (0 matches in each) |
| 22 | 31 % | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | NOT FOUND (0 matches in each) |
| 23 | 6.4·10⁶ | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | NOT FOUND (0 matches in each) |
| 24 | 1.5·10⁶ | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | FOUND (README line 75) |
| 25 | 1.55·10⁸ | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | NOT FOUND (0 matches in each) |
| 26 | 4.52·10⁷ | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | NOT FOUND (0 matches in each) |
| 27 | 56 % | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | NOT FOUND (0 matches in each) |
| 28 | 6.5·10⁶ | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | NOT FOUND (0 matches in each) |
| 29 | 3.5·10⁶ | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | FOUND (README line 75) |
| 30 | 1.58·10⁸ | research/gaudry_quartic_d4_cost_20261014/{README.md, raw/summary.json, raw/table.md} | NOT FOUND (0 matches in each) |
| 31 | 2–7 % | research/gaudry_quartic_replay_20261013/README.md | NOT FOUND (0 matches) |
| 32 | 3.1 | research/gaudry_quartic_replay_20261013/README.md | FOUND (line 52) |
| 33 | 10× | research/ic_speed_and_rho_scope_20261013/README.md | FOUND (line 75; 3 matches) |
| 34 | 2^{13} | research/ic_speed_and_rho_scope_20261013/README.md | FOUND (line 76) |
| 35 | 1.21·10¹² | research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | FOUND (line 2449) |
| 36 | 0.518 | research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | FOUND (line 2548; 4 matches) |
| 37 | 16 | research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md | FOUND (line 5; 233 matches) |
| 38 | 33–41 | research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md | NOT FOUND (0 matches) |
| 39 | 15×15 | research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md | NOT FOUND (0 matches) |
| 40 | 0.6 % | research/pdp_literature_leads_20261022/leads/LEAD-02.md | FOUND (line 177) |
| 41 | 512 | research/pdp_literature_leads_20261022/leads/LEAD-02.md | FOUND (line 62; 6 matches) |
| 42 | 12–24× | research/gb_cas_comparison_20261011/README.md | FOUND (line 8) |
| 43 | 2.547·10¹⁰ | research/magma_f4_comparison_20261013/{PROTOCOL.md, scripts/d4_p269_seed1.magma, ours_d4.jsonl} | NOT FOUND (0 matches in each) |
| 44 | 4 solutions | research/magma_f4_comparison_20261013/{PROTOCOL.md, scripts/d4_p269_seed1.magma, ours_d4.jsonl} | NOT FOUND (0 matches in each) |

## Totals
citations: 7 verified / 0 mismatch / 5 unreachable; paths: 17 exist / 0 missing; numbers: 24 found / 20 not found.
