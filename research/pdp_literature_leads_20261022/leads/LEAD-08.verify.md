# Verification of LEAD-08 (mechanical tier)

Dossier: leads/LEAD-08.md. Procedure: BRIEF_HAIKU.md. Date: 2026-10-22.

## Citations
| # | citation as written | route checked (URL or query) | outcome | page shows |
|---|---|---|---|---|
| 1 | Kera–Ishihara–Kambe–Vaccon–Yokoyama, *Learning to Compute Gröbner Bases*, NeurIPS 2024, arXiv 2311.12904 | WebFetch https://arxiv.org/abs/2311.12904 | VERIFIED | Title "Learning to Compute Gröbner Bases"; authors Hiroshi Kera, Yuki Ishihara, Yuta Kambe, Tristan Vaccon, Kazuhiro Yokoyama; first submitted 2023 (revision 6 Nov 2024); venue "NeurIPS 2024" (listed under Comments) |
| 2 | Malhou–Perret–Lauter, *HATSolver*, arXiv 2512.14722 (ePrint 2025/2219) | WebFetch https://arxiv.org/abs/2512.14722 | VERIFIED | Title "HATSolver: Learning Groebner Bases with Hierarchical Attention Transformers"; authors Mohamed Malhou, Ludovic Perret, Kristin Lauter; 2025 (submitted 9 Dec 2025); arXiv cs.LG, no journal or conference venue |
| 3 | Bunch–Ergür–Golestani–Tong–Walewski–Zeytuncu, *Learning Fast Monomial Orders for Gröbner Basis Computations*, arXiv 2602.02972v2 (Aug 2026) | WebFetch https://arxiv.org/abs/2602.02972 | VERIFIED | Title "Learning Fast Monomial Orders for Gröbner Basis Computations"; authors R. Caleb Bunch, Alperen A. Ergür, Melika Golestani, Jessie Tong, Malia Walewski, Yunus E. Zeytuncu; 2026 (v1 3 Feb 2026, v2 28 Aug 2026); arXiv preprint, no venue |
| 4 | Peifer–Stillman–Halpern-Leistner, *Learning Selection Strategies in Buchberger's Algorithm*, ICML 2020, arXiv 2005.01917 | WebFetch https://arxiv.org/abs/2005.01917 | VERIFIED | Title "Learning selection strategies in Buchberger's algorithm"; authors Dylan Peifer, Michael Stillman, Daniel Halpern-Leistner; 2020 (submitted 5 May 2020); "to appear in Proceedings of the 37th International Conference on Machine Learning (ICML 2020)" |
| 5 | Caminata–Gorla, *Solving degree, last fall degree, and related invariants*, J. Symb. Comput. 2023, arXiv 2112.05579 | WebFetch https://arxiv.org/abs/2112.05579 | VERIFIED | Title "Solving degree, last fall degree, and related invariants"; authors Alessio Caminata, Elisa Gorla; 2021 (submitted 10 Dec 2021, revised 1 Jun 2022); "To appear in Journal of Symbolic Computation"; year 2023 not shown on page |
| 6 | Trimoska–Ionica–Dequen, *Parity (XOR) Reasoning for the Index Calculus Attack*, CP 2020, arXiv 2001.11229 | WebFetch https://arxiv.org/abs/2001.11229 | VERIFIED | Title "Parity (XOR) Reasoning for the Index Calculus Attack"; authors Monika Trimoska, Sorina Ionica, Gilles Dequen; 2020; journal reference "CP 2020, LNCS vol 12333, Springer" |
| 7 | Trimoska, thesis (Université de Picardie Jules Verne, 2021), https://mtrimoska.com/Monika_Trimoska_these.pdf | WebFetch https://mtrimoska.com/Monika_Trimoska_these.pdf; no WebSearch (dossier gives no exact title) | UNREACHABLE | Fetch returned a binary PDF with no readable title or author text; PDF metadata title "logo UPJV bis", no author, date 2015; nothing to match, so not verified |
| 8 | Trimoska–Ionica–Dequen, *A SAT-based approach for index calculus on binary elliptic curves*, AFRICACRYPT 2020, ePrint 2019/313 | WebFetch https://eprint.iacr.org/2019/313 | VERIFIED | Title "A SAT-based approach for index calculus on binary elliptic curves"; authors Monika Trimoska, Sorina Ionica, and Gilles Dequen; ePrint 2019/313 (2019); "Published elsewhere. Progress in Cryptology - AFRICACRYPT 2020" |
| 9 | Bayer–Stillman 1987 | none: no URL or identifier in dossier; not fetched or searched | UNREACHABLE | no page |
| 10 | Galligo (generic coordinates minimise grevlex regularity) | none: no URL or identifier in dossier; not fetched or searched | UNREACHABLE | no page |
| 11 | SAT4CryptoBench (NeurIPS 2025) | none: no URL or identifier in dossier; not fetched or searched | UNREACHABLE | no page |
| 12 | ePrint 2026/384 (§5.3, "per the card") | WebFetch https://eprint.iacr.org/2026/384 (identifier only) | UNREACHABLE | Page fetched: title "The Structured Generic-Group Model"; authors Henry Corrigan-Gibbs, Alexandra Henzinger, David J. Wu; 2026; "major revision of an IACR publication in EUROCRYPT 2026"; dossier gives no title or author to match, so not verified |

## Repository paths
| # | path as written | tested as | outcome |
|---|---|---|---|
| 1 | examples/pdp_perm_f4.rs | /home/user/crypto/examples/pdp_perm_f4.rs MISSING; /home/user/crypto-autoresearcher/... MISSING; /home/user/cryptanalysis/... MISSING; /home/user/ml-cryptanalysis/... MISSING | MISSING |
| 2 | research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md | /home/user/crypto/... EXISTS; other three roots MISSING | EXISTS |
| 3 | research/index_calculus_baseline_20260914/regression/{README,RESULTS}.md (README.md) | /home/user/crypto/... EXISTS; other three roots MISSING | EXISTS |
| 4 | research/index_calculus_baseline_20260914/regression/{README,RESULTS}.md (RESULTS.md) | /home/user/crypto/... EXISTS; other three roots MISSING | EXISTS |
| 5 | src/cryptanalysis/wdsat_oracle.rs | /home/user/crypto/... EXISTS; other three roots MISSING | EXISTS |
| 6 | src/cryptanalysis/semaev_corpus.rs | /home/user/crypto/... EXISTS; other three roots MISSING | EXISTS |
| 7 | src/cryptanalysis/inherited_f4.rs | /home/user/crypto/... EXISTS; other three roots MISSING | EXISTS |
| 8 | research/notes/ecc2k130/RESEARCH_CHAIN_SPLIT_ORDER.md | /home/user/crypto/... EXISTS; other three roots MISSING | EXISTS |
| 9 | research/notes/index-calculus/RESEARCH_SAT_SEMAEV.md | /home/user/crypto/... EXISTS; other three roots MISSING | EXISTS |
| 10 | research/pdp_split_dreg_20261011/{README,ANALYSIS.md} (README) | research/pdp_split_dreg_20261011/README under all four roots MISSING | MISSING |
| 11 | research/pdp_split_dreg_20261011/{README,ANALYSIS.md} (ANALYSIS.md) | /home/user/crypto/... EXISTS; other three roots MISSING | EXISTS |
| 12 | research/dreg_ell_grid_20260925/RESULTS.md | /home/user/crypto/... EXISTS; other three roots MISSING | EXISTS |
| 13 | research/dreg_m4_chain_20261006/RESULTS.md | /home/user/crypto/... EXISTS; other three roots MISSING | EXISTS |
| 14 | research/notes/index-calculus/RESEARCH_FALL_DEGREE_BOUNDS.md | /home/user/crypto/... EXISTS; other three roots MISSING | EXISTS |
| 15 | research/fall_degree_bounds_20261008/RESULTS.md | /home/user/crypto/... EXISTS; other three roots MISSING | EXISTS |
| 16 | research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md | /home/user/crypto/... EXISTS; other three roots MISSING | EXISTS |
| 17 | research/ic_next10_20261014/03-pkm-degree/{results.csv,systems/,run.sh,parse.sh} (results.csv) | /home/user/crypto/... EXISTS; other three roots MISSING | EXISTS |
| 18 | research/ic_next10_20261014/03-pkm-degree/{results.csv,systems/,run.sh,parse.sh} (systems/) | /home/user/crypto/... EXISTS; other three roots MISSING | EXISTS |
| 19 | research/ic_next10_20261014/03-pkm-degree/{results.csv,systems/,run.sh,parse.sh} (run.sh) | /home/user/crypto/... EXISTS; other three roots MISSING | EXISTS |
| 20 | research/ic_next10_20261014/03-pkm-degree/{results.csv,systems/,run.sh,parse.sh} (parse.sh) | /home/user/crypto/... EXISTS; other three roots MISSING | EXISTS |
| 21 | experiments/closure-degree-predictor/README.md | /home/user/crypto/..., /home/user/crypto-autoresearcher/..., /home/user/cryptanalysis/... MISSING; /home/user/ml-cryptanalysis/... EXISTS | EXISTS |
| 22 | experiments/hardness/report.md | /home/user/crypto/..., /home/user/crypto-autoresearcher/..., /home/user/cryptanalysis/... MISSING; /home/user/ml-cryptanalysis/... EXISTS | EXISTS |
| 23 | csd/fpn_degrees.py | /home/user/crypto/..., /home/user/crypto-autoresearcher/..., /home/user/cryptanalysis/... MISSING; /home/user/ml-cryptanalysis/... EXISTS | EXISTS |
| 24 | csd/order_probe.py | /home/user/crypto/..., /home/user/crypto-autoresearcher/..., /home/user/cryptanalysis/..., /home/user/ml-cryptanalysis/... MISSING | MISSING |

## Quoted numbers
| # | number | file | grep outcome (line) |
|---|---|---|---|
| 1 | 0.97 | research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md | FOUND (line 24) |
| 2 | 1.02 | research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md | FOUND (line 24) |
| 3 | 60 | research/index_calculus_baseline_20260914/regression/README.md | FOUND (line 3) |
| 4 | 60 | research/index_calculus_baseline_20260914/regression/RESULTS.md | FOUND (line 8) |
| 5 | 1,280,281 | research/index_calculus_baseline_20260914/regression/README.md | NOT FOUND |
| 6 | 1,280,281 | research/index_calculus_baseline_20260914/regression/RESULTS.md | FOUND (line 34) |
| 7 | 6,246,481 | research/index_calculus_baseline_20260914/regression/README.md | NOT FOUND |
| 8 | 6,246,481 | research/index_calculus_baseline_20260914/regression/RESULTS.md | FOUND (line 35) |
| 9 | 4.88 | research/index_calculus_baseline_20260914/regression/README.md | NOT FOUND |
| 10 | 4.88 | research/index_calculus_baseline_20260914/regression/RESULTS.md | NOT FOUND |
| 11 | 20 | research/index_calculus_baseline_20260914/regression/README.md | FOUND (line 44) |
| 12 | 20 | research/index_calculus_baseline_20260914/regression/RESULTS.md | FOUND (line 25) |
| 13 | 10 | research/index_calculus_baseline_20260914/regression/README.md | FOUND (line 17) |
| 14 | 10 | research/index_calculus_baseline_20260914/regression/RESULTS.md | FOUND (line 72) |
| 15 | 60 | src/cryptanalysis/semaev_corpus.rs | FOUND (line 738) |
| 16 | 15 | src/cryptanalysis/semaev_corpus.rs | FOUND (line 124) |
| 17 | 5 | src/cryptanalysis/semaev_corpus.rs | FOUND (line 124) |
| 18 | 17 | src/cryptanalysis/semaev_corpus.rs | FOUND (line 125) |
| 19 | 6 | src/cryptanalysis/semaev_corpus.rs | FOUND (line 30) |
| 20 | 19 | src/cryptanalysis/semaev_corpus.rs | FOUND (line 30) |
| 21 | 4.91 | research/notes/ecc2k130/RESEARCH_CHAIN_SPLIT_ORDER.md | FOUND (line 346) |
| 22 | 9 | research/notes/ecc2k130/RESEARCH_CHAIN_SPLIT_ORDER.md | FOUND (line 12) |
| 23 | 1.68 | research/notes/ecc2k130/RESEARCH_CHAIN_SPLIT_ORDER.md | FOUND (line 348) |
| 24 | 33.3 | research/notes/ecc2k130/RESEARCH_CHAIN_SPLIT_ORDER.md | FOUND (line 348) |
| 25 | 7.25 | research/notes/ecc2k130/RESEARCH_CHAIN_SPLIT_ORDER.md | FOUND (line 349) |
| 26 | 0.19 | research/notes/ecc2k130/RESEARCH_CHAIN_SPLIT_ORDER.md | FOUND (line 379) |
| 27 | 1.54 | research/notes/ecc2k130/RESEARCH_CHAIN_SPLIT_ORDER.md | FOUND (line 484) |
| 28 | 767 | research/notes/index-calculus/RESEARCH_SAT_SEMAEV.md | FOUND (line 87) |
| 29 | 18 | research/notes/index-calculus/RESEARCH_SAT_SEMAEV.md | FOUND (line 88) |
| 30 | 11 | research/notes/index-calculus/RESEARCH_SAT_SEMAEV.md | FOUND (line 221) |
| 31 | 4 | research/pdp_split_dreg_20261011/README | FILE MISSING |
| 32 | 4 | research/pdp_split_dreg_20261011/ANALYSIS.md | FOUND (line 1) |
| 33 | (3,4,3,3) | research/pdp_split_dreg_20261011/README | FILE MISSING |
| 34 | (3,4,3,3) | research/pdp_split_dreg_20261011/ANALYSIS.md | NOT FOUND |
| 35 | 0 | research/pdp_split_dreg_20261011/README | FILE MISSING |
| 36 | 0 | research/pdp_split_dreg_20261011/ANALYSIS.md | FOUND (line 3) |
| 37 | 7 | research/pdp_split_dreg_20261011/README | FILE MISSING |
| 38 | 7 | research/pdp_split_dreg_20261011/ANALYSIS.md | FOUND (line 3) |
| 39 | 3 | research/dreg_ell_grid_20260925/RESULTS.md | FOUND (line 1) |
| 40 | 3 | research/dreg_m4_chain_20261006/RESULTS.md | FOUND (line 11) |
| 41 | 2 | research/notes/index-calculus/RESEARCH_FALL_DEGREE_BOUNDS.md | FOUND (line 3) |
| 42 | 2 | research/fall_degree_bounds_20261008/RESULTS.md | FOUND (line 1) |
| 43 | 3 | research/notes/index-calculus/RESEARCH_FALL_DEGREE_BOUNDS.md | FOUND (line 14) |
| 44 | 3 | research/fall_degree_bounds_20261008/RESULTS.md | FOUND (line 5) |
| 45 | 4 | research/notes/index-calculus/RESEARCH_FALL_DEGREE_BOUNDS.md | FOUND (line 53) |
| 46 | 4 | research/fall_degree_bounds_20261008/RESULTS.md | FOUND (line 5) |
| 47 | 5 | research/notes/index-calculus/RESEARCH_FALL_DEGREE_BOUNDS.md | FOUND (line 14) |
| 48 | 5 | research/fall_degree_bounds_20261008/RESULTS.md | FOUND (line 5) |
| 49 | 0.6.5 | research/ic_next10_20261014/03-pkm-degree/results.csv | FOUND (line 4; regex match, literal 0.6.5 not on that line) |
| 50 | 0.6.5 | research/ic_next10_20261014/03-pkm-degree/systems/ | FOUND (line 3 of pkm_kummer_p2013265921_m4_t4_x1647248299.ms; regex match, literal not on that line) |
| 51 | 0.6.5 | research/ic_next10_20261014/03-pkm-degree/run.sh | FOUND (line 2) |
| 52 | 0.6.5 | research/ic_next10_20261014/03-pkm-degree/parse.sh | NOT FOUND |
| 53 | 2 | research/ic_next10_20261014/03-pkm-degree/results.csv | FOUND (line 2) |
| 54 | 2 | research/ic_next10_20261014/03-pkm-degree/systems/ | FOUND (line 1 of pkm_kummer_p2013265921_m4_t4_x1647248299.ms) |
| 55 | 2 | research/ic_next10_20261014/03-pkm-degree/run.sh | FOUND (line 4) |
| 56 | 2 | research/ic_next10_20261014/03-pkm-degree/parse.sh | FOUND (line 11) |
| 57 | 3 | research/ic_next10_20261014/03-pkm-degree/results.csv | FOUND (line 2) |
| 58 | 3 | research/ic_next10_20261014/03-pkm-degree/systems/ | FOUND (line 1 of pkm_kummer_p2013265921_m4_t4_x1647248299.ms) |
| 59 | 3 | research/ic_next10_20261014/03-pkm-degree/run.sh | FOUND (line 2) |
| 60 | 3 | research/ic_next10_20261014/03-pkm-degree/parse.sh | NOT FOUND |
| 61 | 4 | research/ic_next10_20261014/03-pkm-degree/results.csv | FOUND (line 2) |
| 62 | 4 | research/ic_next10_20261014/03-pkm-degree/systems/ | FOUND (line 3 of pkm_kummer_p2013265921_m4_t4_x1647248299.ms) |
| 63 | 4 | research/ic_next10_20261014/03-pkm-degree/run.sh | FOUND (line 5) |
| 64 | 4 | research/ic_next10_20261014/03-pkm-degree/parse.sh | FOUND (line 9) |
| 65 | 6 | research/ic_next10_20261014/03-pkm-degree/results.csv | FOUND (line 2) |
| 66 | 6 | research/ic_next10_20261014/03-pkm-degree/systems/ | FOUND (line 2 of pkm_kummer_p2013265921_m4_t4_x1647248299.ms) |
| 67 | 6 | research/ic_next10_20261014/03-pkm-degree/run.sh | FOUND (line 2) |
| 68 | 6 | research/ic_next10_20261014/03-pkm-degree/parse.sh | FOUND (line 9) |
| 69 | 9 | research/ic_next10_20261014/03-pkm-degree/results.csv | FOUND (line 2) |
| 70 | 9 | research/ic_next10_20261014/03-pkm-degree/systems/ | FOUND (line 2 of pkm_kummer_p2013265921_m4_t4_x1647248299.ms) |
| 71 | 9 | research/ic_next10_20261014/03-pkm-degree/run.sh | NOT FOUND |
| 72 | 9 | research/ic_next10_20261014/03-pkm-degree/parse.sh | FOUND (line 8) |
| 73 | 11 | research/ic_next10_20261014/03-pkm-degree/results.csv | FOUND (line 6) |
| 74 | 11 | research/ic_next10_20261014/03-pkm-degree/systems/ | FOUND (line 4 of pkm_kummer_p2013265921_m3_t5_x1610027479.ms) |
| 75 | 11 | research/ic_next10_20261014/03-pkm-degree/run.sh | NOT FOUND |
| 76 | 11 | research/ic_next10_20261014/03-pkm-degree/parse.sh | NOT FOUND |
| 77 | 1 | research/ic_next10_20261014/03-pkm-degree/results.csv | FOUND (line 2) |
| 78 | 1 | research/ic_next10_20261014/03-pkm-degree/systems/ | FOUND (line 1 of pkm_kummer_p2013265921_m4_t4_x1647248299.ms) |
| 79 | 1 | research/ic_next10_20261014/03-pkm-degree/run.sh | FOUND (line 11) |
| 80 | 1 | research/ic_next10_20261014/03-pkm-degree/parse.sh | FOUND (line 8) |
| 81 | 116 | research/ic_next10_20261014/03-pkm-degree/results.csv | FOUND (line 6) |
| 82 | 116 | research/ic_next10_20261014/03-pkm-degree/systems/ | FOUND (line 3 of pkm_kummer_p2013265921_m2_t8_x1312284326.ms) |
| 83 | 116 | research/ic_next10_20261014/03-pkm-degree/run.sh | NOT FOUND |
| 84 | 116 | research/ic_next10_20261014/03-pkm-degree/parse.sh | NOT FOUND |
| 85 | 5 GB | research/ic_next10_20261014/03-pkm-degree/run.sh | FOUND (line 4) |
| 86 | -t 1 -v 2 | research/ic_next10_20261014/03-pkm-degree/run.sh | FOUND (line 19) |
| 87 | 37 | ml-cryptanalysis/experiments/closure-degree-predictor/README.md | FOUND (line 48) |
| 88 | 0 | ml-cryptanalysis/experiments/closure-degree-predictor/README.md | FOUND (line 19) |
| 89 | 8 | ml-cryptanalysis/experiments/closure-degree-predictor/README.md | FOUND (line 3) |
| 90 | 0.02 | ml-cryptanalysis/experiments/hardness/report.md | FOUND (line 36) |
| 91 | 0.08 | ml-cryptanalysis/experiments/hardness/report.md | FOUND (line 35; regex match, literal 0.08 not on that line) |
| 92 | 0.15 | ml-cryptanalysis/experiments/hardness/report.md | FOUND (line 35; regex match, literal 0.15 not on that line) |
| 93 | 5,592 | ml-cryptanalysis/csd/fpn_degrees.py | NOT FOUND |
| 94 | 3 | ml-cryptanalysis/csd/fpn_degrees.py | FOUND (line 3) |
| 95 | 2013265921 | research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md | FOUND (line 1303) |

## Totals
citations: 7 verified / 0 mismatch / 5 unreachable; paths: 21 exist / 3 missing; numbers: 78 found / 17 not found.
