# Verification of LEAD-05 (mechanical tier)

Dossier: leads/LEAD-05.md. Procedure: BRIEF_HAIKU.md. Date: 2026-10-22.

## Citations
| # | citation as written | route checked (URL or query) | outcome | page shows |
|---|---|---|---|---|
| 1 | A. Solovyev, repo `alekssolov/weil-descent-s3-dreg`, `paper/paper.tex` (September 2026) | https://github.com/alekssolov/weil-descent-s3-dreg (WebFetch) | VERIFIED | repo `weil-descent-s3-dreg`, owner alekssolov; author A. Solovyev; paper title "The degree of regularity of Weil-descent systems for the third summation polynomial over F_{2^n}: exact closure dimensions, thresholds and a growth law"; year 2026; no month shown |
| 2 | Pull request #14 of `cr-marcstevens/m4gb` ("Fix Gebauer-Moeller F-criterion in update(): equal-lcm pairs discarded each other"), opened 18 Sep 2026 by `alekssolov`, merged 25 Sep 2026 | https://github.com/cr-marcstevens/m4gb/pull/14 (WebFetch) | VERIFIED | title "Fix Gebauer-Moeller F-criterion in update(): equal-lcm pairs discarded each other" (#14); author alekssolov; opened Sep 18, 2026; merged Sep 25, 2026 |
| 3 | Kosters–Yeo, *Notes on summation polynomials*, arXiv:1503.08001 (v3) | https://arxiv.org/abs/1503.08001 (WebFetch) | VERIFIED | title "Notes on summation polynomials"; Michiel Kosters, Sze Ling Yeo; submitted 2015 (27 Mar 2015, last revised 8 Jun 2015); no journal shown |
| 4 | Kousidis–Wiemers, J. Math. Cryptol. 13 (2019), arXiv:1906.05594 | https://arxiv.org/abs/1906.05594 (WebFetch) | VERIFIED | title "On the first fall degree of summation polynomials"; Stavros Kousidis, Andreas Wiemers; submitted 13 Jun 2019; Journal of Mathematical Cryptology 2019, 13(3-4), 229-237 |
| 5 | Huang–Kosters–Yeo, CRYPTO 2015, ePrint 2015/573 | https://eprint.iacr.org/2015/573 (WebFetch) | VERIFIED | title "Last fall degree, HFE, and Weil descent attacks on ECDLP"; Ming-Deh A. Huang, Michiel Kosters, Sze Ling Yeo; 2015; published by the IACR in CRYPTO 2015 |
| 6 | Huang–Kosters–Yang–Yeo, JSC 87 (2018), arXiv:1505.02532 | https://arxiv.org/abs/1505.02532 (WebFetch) | VERIFIED | title "On the last fall degree of zero-dimensional Weil descent systems"; Ming-Deh A. Huang, Michiel Kosters, Yun Yang, Sze Ling Yeo; first submitted 11 May 2015 (page year is arXiv submission; JSC 2018 not shown on page) |
| 7 | Petit–Quisquater (ASIACRYPT 2012) | query "Petit Quisquater ASIACRYPT 2012 polynomial systems Weil descent" (WebSearch; dossier gives no URL, identifier or title) | VERIFIED (no title in dossier; authors, venue and year compared) | "On Polynomial Systems Arising from a Weil Descent"; Christophe Petit and Jean-Jacques Quisquater; ASIACRYPT 2012; ePrint 2012/146 |
| 8 | Semaev-2015 (chain of S₃ equations) | query "Semaev 2015 summation polynomials elliptic curve discrete logarithm index calculus" (WebSearch; dossier gives no URL, identifier or title) | VERIFIED (no title in dossier; author and year compared) | top 2015 result arXiv 1504.01175, "New algorithm for the discrete logarithm problem on elliptic curves"; search summary attributes it to Semaev (author list not returned verbatim); year 2015 |

## Repository paths
| # | path as written | tested as | outcome |
|---|---|---|---|
| 1 | `src/m4gb.hpp` (lines 1682–1708) | /home/user/crypto/src/m4gb.hpp; /home/user/crypto-autoresearcher/src/m4gb.hpp; /home/user/cryptanalysis/src/m4gb.hpp; /home/user/ml-cryptanalysis/src/m4gb.hpp | MISSING (all four roots) |
| 2 | `research/dreg_closure_scale_20261016/RESULTS.md` (expanded from `{RESULTS,PREREGISTRATION}.md`) | /home/user/crypto/research/dreg_closure_scale_20261016/RESULTS.md | EXISTS |
| 3 | `research/dreg_closure_scale_20261016/PREREGISTRATION.md` | /home/user/crypto/research/dreg_closure_scale_20261016/PREREGISTRATION.md | EXISTS |
| 4 | `ledger/evidence/EV-SEMBIN-07960c.yaml` (base `crypto-autoresearcher`) | /home/user/crypto/ledger/evidence/EV-SEMBIN-07960c.yaml (MISSING); /home/user/crypto-autoresearcher/ledger/evidence/EV-SEMBIN-07960c.yaml | EXISTS (second root) |
| 5 | `examples/dreg_ladder.rs` | /home/user/crypto/examples/dreg_ladder.rs | EXISTS |
| 6 | `/home/user/crypto/research/` | /home/user/crypto/research/ | EXISTS |
| 7 | `pdp_literature_leads_20261022/BRIEF_SONNET.md` (base `/home/user/crypto/research/`) | /home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md | EXISTS |
| 8 | `pdp_literature_leads_20261022/LEADS.md` (base `/home/user/crypto/research/`) | /home/user/crypto/research/pdp_literature_leads_20261022/LEADS.md | EXISTS |
| 9 | `dreg_closure_driver_20261016/RESULTS.md` (base `/home/user/crypto/research/`) | /home/user/crypto/research/dreg_closure_driver_20261016/RESULTS.md | EXISTS |
| 10 | `dreg_closure_mutantxl_20261016/RESULTS.md` (base `/home/user/crypto/research/`) | /home/user/crypto/research/dreg_closure_mutantxl_20261016/RESULTS.md | EXISTS |
| 11 | `dreg_ell_grid_20260925/RESULTS.md` (base `/home/user/crypto/research/`) | /home/user/crypto/research/dreg_ell_grid_20260925/RESULTS.md | EXISTS |
| 12 | `notes/index-calculus/RESEARCH_DREG_MEASUREMENT.md` (base `/home/user/crypto/research/`) | /home/user/crypto/research/notes/index-calculus/RESEARCH_DREG_MEASUREMENT.md | EXISTS |
| 13 | `notes/index-calculus/RESEARCH_FALL_DEGREE_BOUNDS.md` (base `/home/user/crypto/research/`) | /home/user/crypto/research/notes/index-calculus/RESEARCH_FALL_DEGREE_BOUNDS.md | EXISTS |
| 14 | `notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md` (base `/home/user/crypto/research/`) | /home/user/crypto/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md | EXISTS |
| 15 | `fall_degree_bounds_20261008/RESULTS.md` (base `/home/user/crypto/research/`) | /home/user/crypto/research/fall_degree_bounds_20261008/RESULTS.md | EXISTS |
| 16 | `fall_degree_bounds_20261008/TABLE.md` (base `/home/user/crypto/research/`) | /home/user/crypto/research/fall_degree_bounds_20261008/TABLE.md | EXISTS |
| 17 | `ic_next10_20261014/README.md` (base `/home/user/crypto/research/`) | /home/user/crypto/research/ic_next10_20261014/README.md | EXISTS |
| 18 | `ic_next10_20261014/06-equivariant-f4-m4/README.md` (base `/home/user/crypto/research/`) | /home/user/crypto/research/ic_next10_20261014/06-equivariant-f4-m4/README.md | EXISTS |
| 19 | `notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md` (base `/home/user/crypto/research/`) | /home/user/crypto/research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md | EXISTS |
| 20 | `notes/ecc2k130/descent_degree_cost-output-20261016-scale.txt` (base `/home/user/crypto/research/`) | /home/user/crypto/research/notes/ecc2k130/descent_degree_cost-output-20261016-scale.txt | EXISTS |
| 21 | `/home/user/crypto/examples/dreg_closure.rs` | /home/user/crypto/examples/dreg_closure.rs | EXISTS |
| 22 | `/home/user/crypto/examples/dreg_sweep.rs` | /home/user/crypto/examples/dreg_sweep.rs | EXISTS |
| 23 | `/home/user/crypto/examples/assumption1_closure.rs` | /home/user/crypto/examples/assumption1_closure.rs | EXISTS |
| 24 | `/home/user/crypto-autoresearcher/research/dreg-linear-law/FINDING_v2.md` | /home/user/crypto-autoresearcher/research/dreg-linear-law/FINDING_v2.md | EXISTS |
| 25 | `/home/user/crypto-autoresearcher/research/dreg-linear-law/VALIDATION.md` | /home/user/crypto-autoresearcher/research/dreg-linear-law/VALIDATION.md | EXISTS |

## Quoted numbers
| # | number | file | grep outcome (line) |
|---|---|---|---|
| 1 | 37 of 38 | /home/user/crypto/research/fall_degree_bounds_20261008/RESULTS.md | NOT FOUND (grep output empty) |
| 2 | 0/345 | /home/user/crypto/research/dreg_closure_mutantxl_20261016/RESULTS.md | NOT FOUND (grep output empty) |
| 3 | 92 | /home/user/crypto/research/dreg_closure_mutantxl_20261016/RESULTS.md | FOUND (line 14; further matches at 17, 99, 174) |
| 4 | 6–7 | /home/user/crypto/research/dreg_ell_grid_20260925/RESULTS.md | NOT FOUND (grep output empty) |
| 5 | (15, 5) | /home/user/crypto/research/ic_next10_20261014/06-equivariant-f4-m4/README.md | FOUND (line 11) |
| 6 | (11, 4) | /home/user/crypto/research/ic_next10_20261014/06-equivariant-f4-m4/README.md | FOUND (line 17) |
| 7 | 0.2375 | /home/user/crypto-autoresearcher/research/dreg-linear-law/FINDING_v2.md | FOUND (line 29) |
| 8 | 2^1194 | /home/user/crypto-autoresearcher/research/dreg-linear-law/FINDING_v2.md | FOUND (line 40) |
| 9 | 40, 42 | /home/user/crypto/research/dreg_closure_driver_20261016/RESULTS.md | NOT FOUND (grep output empty) |
| 10 | 760k | /home/user/crypto/research/dreg_closure_scale_20261016/RESULTS.md | FOUND (line 87) |
| 11 | 8/8 | /home/user/crypto/research/dreg_closure_scale_20261016/RESULTS.md | NOT FOUND (grep output empty) |
| 12 | 68.58 | /home/user/crypto/research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md | FOUND (line 44) |
| 13 | 5.71 | /home/user/crypto/research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md | FOUND (line 45) |
| 14 | 60.81 | /home/user/crypto/research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md | FOUND (line 44) |
| 15 | 4 (phrase "Boolean degree 4") | /home/user/crypto/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md | FOUND (line 450) |

## Totals
citations: 8 verified / 0 mismatch / 0 unreachable; paths: 24 exist / 1 missing; numbers: 10 found / 5 not found.
