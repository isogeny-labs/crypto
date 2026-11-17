# Verification of LEAD-17 (mechanical tier)

Dossier: leads/LEAD-17.md. Procedure: BRIEF_HAIKU.md. Date: 2026-10-22.

## Citations
| # | citation as written | route checked (URL or query) | outcome | page shows |
|---|---|---|---|---|
| 1 | Kosters, Yeo, *Notes on summation polynomials*, arXiv:1503.08001 | https://arxiv.org/abs/1503.08001 (WebFetch) | VERIFIED | Title "Notes on summation polynomials"; authors Michiel Kosters, Sze Ling Yeo; submitted 27 Mar 2015; no venue shown |
| 2 | Solovyev, `alekssolov/weil-descent-s3-dreg`, `paper/paper.tex` (2026) | https://github.com/alekssolov/weil-descent-s3-dreg (WebFetch; identifier is the repository slug) | VERIFIED | Repository "weil-descent-s3-dreg", owner alekssolov; paper title "The degree of regularity of Weil-descent systems for the third summation polynomial over F_{2^n}: exact closure dimensions, thresholds and a growth law"; author "A. Solovyev"; "A. Solovyev, 2026"; paper/paper.tex not listed on the page; dossier gives no title for this entry |
| 3 | Kousidis, Wiemers, arXiv:1906.05594 | https://arxiv.org/abs/1906.05594 (WebFetch) | VERIFIED | Title "On the first fall degree of summation polynomials"; authors Stavros Kousidis, Andreas Wiemers; "Submitted on 13 Jun 2019"; "Journal of Mathematical Cryptology, 2019, 13(3-4), 229-237"; dossier gives no title for this entry |
| 4 | Huang–Kosters–Yang–Yeo, arXiv:1505.02532 | https://arxiv.org/abs/1505.02532 (WebFetch) | VERIFIED | Title "On the last fall degree of zero-dimensional Weil descent systems"; authors Ming-Deh A. Huang, Michiel Kosters, Yun Yang, Sze Ling Yeo; "Submitted on 11 May 2015"; no venue shown |
| 5 | Caminata, Gorla, solving degree versus degree of regularity; identifier arXiv:2202.05175 (guessed in the dossier) | https://arxiv.org/abs/2202.05175 (WebFetch) | MISMATCH | Title "Application of the Affinity Propagation Clustering Technique to obtain traffic accident clusters at macro, meso, and micro levels"; authors Fagner Sutel de Moura, Christine Tessele Nodari; "Submitted on 9 Feb 2022"; no venue shown |

## Repository paths
| # | path as written | tested as | outcome |
|---|---|---|---|
| 1 | `/home/user/crypto` | `test -e /home/user/crypto` | EXISTS |
| 2 | `research/linearization_reach_20260930/README.md` | `/home/user/crypto/research/linearization_reach_20260930/README.md` | EXISTS |
| 3 | `src/cryptanalysis/koblitz_groebner.rs` | `/home/user/crypto/src/cryptanalysis/koblitz_groebner.rs` | EXISTS |
| 4 | `src/cryptanalysis/{binary_semaev.rs (header),koblitz_groebner.rs (builder, Macaulay rows),koblitz_bench.rs (closure scan, solution counts)}`, expanded: `koblitz_bench.rs` | `/home/user/crypto/src/cryptanalysis/koblitz_bench.rs` | EXISTS |
| 5 | `src/cryptanalysis/{...}`, expanded: `binary_semaev.rs` | `/home/user/crypto/src/cryptanalysis/binary_semaev.rs` | EXISTS |
| 6 | `research/pdp_literature_leads_20261022/{BRIEF_SONNET.md,LEADS.md (Standing constraints, LEAD-17),leads/LEAD-05.md}`, expanded: `BRIEF_SONNET.md` | `/home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md` | EXISTS |
| 7 | `research/pdp_literature_leads_20261022/{...}`, expanded: `LEADS.md` (annotation stripped) | `/home/user/crypto/research/pdp_literature_leads_20261022/LEADS.md` | EXISTS |
| 8 | `research/pdp_literature_leads_20261022/{...}`, expanded: `leads/LEAD-05.md` | `/home/user/crypto/research/pdp_literature_leads_20261022/leads/LEAD-05.md` | EXISTS |
| 9 | `research/dreg_closure_{scale,driver,mutantxl}_20261016/{RESULTS.md,PREREGISTRATION.md (scale),runs/*.jsonl,score-output.txt (scale)}`, expanded: `research/dreg_closure_scale_20261016/RESULTS.md` | `/home/user/crypto/research/dreg_closure_scale_20261016/RESULTS.md` | EXISTS |
| 10 | same brace form, expanded: `research/dreg_closure_scale_20261016/PREREGISTRATION.md` | `/home/user/crypto/research/dreg_closure_scale_20261016/PREREGISTRATION.md` | EXISTS |
| 11 | same brace form, expanded: `research/dreg_closure_scale_20261016/runs/*.jsonl` | `compgen -G "/home/user/crypto/research/dreg_closure_scale_20261016/runs/*.jsonl"` | EXISTS |
| 12 | same brace form, expanded: `research/dreg_closure_scale_20261016/score-output.txt` | `/home/user/crypto/research/dreg_closure_scale_20261016/score-output.txt` | EXISTS |
| 13 | same brace form, expanded: `research/dreg_closure_driver_20261016/RESULTS.md` | `/home/user/crypto/research/dreg_closure_driver_20261016/RESULTS.md` | EXISTS |
| 14 | same brace form, expanded: `research/dreg_closure_driver_20261016/runs/*.jsonl` | `compgen -G "/home/user/crypto/research/dreg_closure_driver_20261016/runs/*.jsonl"` | EXISTS |
| 15 | same brace form, expanded: `research/dreg_closure_mutantxl_20261016/RESULTS.md` | `/home/user/crypto/research/dreg_closure_mutantxl_20261016/RESULTS.md` | EXISTS |
| 16 | same brace form, expanded: `research/dreg_closure_mutantxl_20261016/runs/*.jsonl` | `compgen -G "/home/user/crypto/research/dreg_closure_mutantxl_20261016/runs/*.jsonl"` | EXISTS |
| 17 | `research/notes/ecc2k130/{RESEARCH_CHAIN_SPLIT_ORDER.md,RESEARCH_ECC2K130_DESCENT_DEGREE.md} (grep)`, expanded: `RESEARCH_CHAIN_SPLIT_ORDER.md` | `/home/user/crypto/research/notes/ecc2k130/RESEARCH_CHAIN_SPLIT_ORDER.md` | EXISTS |
| 18 | same brace form, expanded: `RESEARCH_ECC2K130_DESCENT_DEGREE.md` | `/home/user/crypto/research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md` | EXISTS |

## Quoted numbers
| # | number | file | grep outcome (line) |
|---|---|---|---|
| 1 | 82.27 | /home/user/crypto/research/dreg_closure_scale_20261016/RESULTS.md | FOUND (line 79) |
| 2 | 21.46 | /home/user/crypto/research/dreg_closure_scale_20261016/RESULTS.md | FOUND (line 79) |
| 3 | 60.81 | /home/user/crypto/research/dreg_closure_scale_20261016/RESULTS.md | NOT FOUND |
| 4 | 68.43 | /home/user/crypto/research/dreg_closure_scale_20261016/RESULTS.md | FOUND (line 81) |
| 5 | 7.62 | /home/user/crypto/research/dreg_closure_scale_20261016/RESULTS.md | FOUND (line 81) |
| 6 | 24.37 | /home/user/crypto/research/dreg_closure_scale_20261016/RESULTS.md | FOUND (line 80) |
| 7 | 10.32 | /home/user/crypto/research/dreg_closure_scale_20261016/RESULTS.md | FOUND (line 82) |
| 8 | 92 | /home/user/crypto/research/dreg_closure_mutantxl_20261016/RESULTS.md | FOUND (lines 14, 17, 99, 174) |
| 9 | 3–4 | /home/user/crypto/research/dreg_closure_mutantxl_20261016/RESULTS.md | FOUND (lines 1, 72) |
| 10 | 5–7 | /home/user/crypto/research/dreg_closure_mutantxl_20261016/RESULTS.md | FOUND (line 1) |
| 11 | 40 | /home/user/crypto/research/dreg_closure_scale_20261016/RESULTS.md | FOUND (lines 7, 14, 25, 28, 35, 38, 46, 79, 87, 104) |
| 12 | 40 | /home/user/crypto/research/dreg_closure_scale_20261016/PREREGISTRATION.md | FOUND (lines 15, 53, 56, 59) |
| 13 | 42 | /home/user/crypto/research/dreg_closure_scale_20261016/RESULTS.md | FOUND (lines 1, 26, 29, 46, 72, 94) |
| 14 | 42 | /home/user/crypto/research/dreg_closure_scale_20261016/PREREGISTRATION.md | FOUND (lines 54, 57, 128) |

## Totals
citations: 4 verified / 1 mismatch / 0 unreachable; paths: 18 exist / 0 missing;
numbers: 13 found / 1 not found.
