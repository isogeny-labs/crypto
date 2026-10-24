# Verification of LEAD-07 (mechanical tier)

Dossier: leads/LEAD-07.md. Procedure: BRIEF_HAIKU.md. Date: 2026-10-22.

## Citations
| # | citation as written | route checked (URL or query) | outcome | page shows |
|---|---|---|---|---|
| 1 | Joux, Vitse, *A crossbred algorithm for solving Boolean polynomial systems*, NuTMiC 2017 (LNCS 10737), ePrint 2017/372 | https://eprint.iacr.org/2017/372 (WebFetch) | VERIFIED | Title "A crossbred algorithm for solving Boolean polynomial systems"; authors "Antoine Joux and Vanessa Vitse"; 2017; Cryptology ePrint Archive, Paper 2017/372 |
| 2 | Vidal, Delaplace, Ionica, *An analysis of the Crossbred Algorithm for the MQ Problem*, IACR Commun. Cryptol. 1(3), 2024, doi 10.62056/ak86cy7qiu | https://cic.iacr.org/p/1/3/36/pdf (WebFetch: binary PDF saved, no readable text); then WebSearch "An analysis of the Crossbred Algorithm for the MQ Problem" | VERIFIED | WebSearch top result: title "An analysis of the Crossbred Algorithm for the MQ Problem"; authors Damien Vidal, Claire Delaplace, Sorina Ionica; published 2024-10-07 in IACR Communications in Cryptology |
| 3 | Vidal, Delaplace, Ionica, *Improved preprocessing for the Crossbred algorithm and application to the MQ problem*, ePrint 2026/360 | https://eprint.iacr.org/2026/360 (WebFetch) | VERIFIED | Title "Improved preprocessing for the Crossbred algorithm and application to the MQ problem"; authors Damien Vidal, Claire Delaplace, Sorina Ionica; 2026; Cryptology ePrint Archive, Paper 2026/360 |
| 4 | Duarte, *On the Complexity and Admissible Parameters of the Crossbred Algorithm*, ePrint 2023/1664 | https://eprint.iacr.org/2023/1664 (WebFetch; URL taken from the ePrint id) | MISMATCH | Title "On the Complexity and Admissible Parameters of the Crossbred Algorithm in F_{q≥2}"; author "João Diogo Duarte"; 2023; page states the paper was withdrawn on 2025-07-14 |
| 5 | cryptographic_estimators Crossbred page | https://cryptographicestimators.readthedocs.io/en/latest/MQEstimator/MQAlgorithms/crossbred.html (WebFetch) | MISMATCH | Page title "crossbred" (site "Cryptographic Estimators documentation"); no authors, year or venue shown |
| 6 | HAL record hal-04729683 | https://hal.science/hal-04729683 (WebFetch; URL taken from the HAL id); WebSearch not run, dossier gives no title to quote | UNREACHABLE | "Access Denied: error code 9e4edb5b6b850c41." |

## Repository paths
| # | path as written | tested as | outcome |
|---|---|---|---|
| 1 | /home/user/crypto/src/cryptanalysis/crossbred.rs | as written | EXISTS |
| 2 | /home/user/crypto/research/crossbred_20260920/ | as written | EXISTS |
| 3 | /home/user/crypto/research/crossbred_20260920/e2e_panel.txt | as written (expanded from brace group in §9) | EXISTS |
| 4 | /home/user/crypto/research/crossbred_20260920/oracle_ladder.txt | as written (expanded from brace group in §9) | EXISTS |
| 5 | /home/user/crypto/research/dreg_closure_driver_20261016/RESULTS.md | as written | EXISTS |
| 6 | /home/user/crypto/research/dreg_closure_driver_20261016/PREREGISTRATION.md | as written (expanded from brace group in §9) | EXISTS |
| 7 | /home/user/crypto/research/dreg_closure_mutantxl_20261016/RESULTS.md | as written | EXISTS |
| 8 | /home/user/crypto/research/ic_next10_20261014/06-equivariant-f4-m4/README.md | as written | EXISTS |
| 9 | /home/user/crypto/research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md | as written | EXISTS |
| 10 | /home/user/crypto/research/linearization_reach_20260930/README.md | as written | EXISTS |
| 11 | /home/user/crypto/research/ic_leads_review_20261018/README.md | as written | EXISTS |
| 12 | /home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md | as written (expanded from brace group in §9) | EXISTS |
| 13 | /home/user/crypto/research/pdp_literature_leads_20261022/LEADS.md | as written (expanded from brace group in §9) | EXISTS |
| 14 | examples/ic_next10_06_chain_export.rs | relative; tested under /home/user/crypto first: /home/user/crypto/examples/ic_next10_06_chain_export.rs | EXISTS |

## Quoted numbers
| # | number | file | grep outcome (line) |
|---|---|---|---|
| 1 | 4,000 | /home/user/crypto/src/cryptanalysis/crossbred.rs | NOT FOUND |
| 2 | 4000 | /home/user/crypto/src/cryptanalysis/crossbred.rs | NOT FOUND |
| 3 | 11.7 | /home/user/crypto/research/crossbred_20260920/e2e_panel.txt | FOUND (line 22) |
| 4 | 33.2 | /home/user/crypto/research/crossbred_20260920/e2e_panel.txt | FOUND (line 23) |
| 5 | 51.7 | /home/user/crypto/research/crossbred_20260920/e2e_panel.txt | FOUND (line 24) |
| 6 | 19,696 | /home/user/crypto/research/crossbred_20260920/e2e_panel.txt | NOT FOUND |
| 7 | 1,700 | /home/user/crypto/research/crossbred_20260920/e2e_panel.txt | NOT FOUND |
| 8 | 0 (filters = 0) | /home/user/crypto/research/crossbred_20260920/oracle_ladder.txt | NOT FOUND |
| 9 | 1.4 | /home/user/crypto/research/crossbred_20260920/oracle_ladder.txt | NOT FOUND |
| 10 | 16 of 16 | /home/user/crypto/research/dreg_closure_driver_20261016/RESULTS.md | NOT FOUND |
| 11 | 760k | /home/user/crypto/research/dreg_closure_driver_20261016/RESULTS.md | FOUND (lines 78, 117) |
| 12 | 70 GB | /home/user/crypto/research/dreg_closure_driver_20261016/RESULTS.md | FOUND (line 118) |
| 13 | 2^56.46 | /home/user/crypto/research/dreg_closure_mutantxl_20261016/RESULTS.md | FOUND (line 167) |
| 14 | 2.2 | /home/user/crypto/research/dreg_closure_mutantxl_20261016/RESULTS.md | FOUND (lines 128, 148) |
| 15 | 84k | /home/user/crypto/research/ic_next10_20261014/06-equivariant-f4-m4/README.md | FOUND (line 95) |
| 16 | 386–805 | /home/user/crypto/research/ic_next10_20261014/06-equivariant-f4-m4/README.md | NOT FOUND |
| 17 | 1.0–1.5 GB | /home/user/crypto/research/ic_next10_20261014/06-equivariant-f4-m4/README.md | FOUND (line 97) |
| 18 | 3 GB | /home/user/crypto/research/ic_next10_20261014/06-equivariant-f4-m4/README.md | FOUND (lines 12, 32, 84, 88, 209, 254, 276) |
| 19 | 1.6–1.8 GB | /home/user/crypto/research/ic_next10_20261014/06-equivariant-f4-m4/README.md | FOUND (line 34) |
| 20 | 1.39 | /home/user/crypto/research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md | FOUND (line 197) |
| 21 | 1.5 | /home/user/crypto/research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md | FOUND (lines 124, 194, 261) |
| 22 | 2^61 | /home/user/crypto/research/linearization_reach_20260930/README.md | FOUND (lines 1, 4, 14, 132) |
| 23 | 56 | /home/user/crypto/research/linearization_reach_20260930/README.md | FOUND (line 104) |
| 24 | 1,700 | /home/user/crypto/research/crossbred_20260920/e2e_panel.txt | NOT FOUND |
| 25 | 0 (filters = 0) | /home/user/crypto/research/crossbred_20260920/oracle_ladder.txt | NOT FOUND |

## Totals
citations: 3 verified / 2 mismatch / 1 unreachable; paths: 14 exist / 0 missing; numbers: 15 found / 10 not found.
