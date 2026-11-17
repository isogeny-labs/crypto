# Verification of LEAD-10 (mechanical tier)

Dossier: leads/LEAD-10.md. Procedure: BRIEF_HAIKU.md. Date: 2026-10-22.

## Citations
| # | citation as written | route checked (URL or query) | outcome | page shows |
|---|---|---|---|---|
| 1 | Semaev, *Summation polynomials and the discrete logarithm problem on elliptic curves*, ePrint 2004/031 | https://eprint.iacr.org/2004/031 (WebFetch) | VERIFIED | Title "Summation polynomials and the discrete logarithm problem on elliptic curves"; author Igor Semaev; 2004; ePrint 2004/031 |
| 2 | Jochemsz, May, *A Strategy for Finding Roots of Multivariate Polynomials with New Applications in Attacking RSA Variants*, ASIACRYPT 2006, LNCS 4284, pp. 267–282 | https://www.iacr.org/archive/asiacrypt2006/42840270/42840270.pdf (WebFetch); then WebSearch `"A Strategy for Finding Roots of Multivariate Polynomials with New Applications in Attacking RSA Variants"` | UNREACHABLE | WebFetch: "The server returned HTTP 403 Forbidden", body not retrieved. WebSearch: no title text shown, links only (iacr.org PDF; IACR CryptoDB pubkey=219; win.tue.nl seminar "Finding Small Roots & Attacking RSA Variants"); search summary names Ellen Jochemsz and Alexander May, ASIACRYPT 2006, LNCS 4284, pp. 267–282 |
| 3 | Ryan, *Solving Multivariate Coppersmith Problems with Known Moduli* (cuso), ePrint 2024/1577, EUROCRYPT 2025 | https://eprint.iacr.org/2024/1577 (WebFetch) | VERIFIED | Title "Solving Multivariate Coppersmith Problems with Known Moduli"; author Keegan Ryan; 2024; ePrint 2024/1577; "A major revision of an IACR publication in EUROCRYPT 2025" |
| 4 | Feng, Luo, Chen, Nitaj, Pan, *Computing Asymptotic Bounds for Small Roots in Coppersmith's Method via Sumset Theory*, ePrint 2024/1330 | https://eprint.iacr.org/2024/1330 (WebFetch) | VERIFIED | Title "Computing Asymptotic Bounds for Small Roots in Coppersmith's Method via Sumset Theory"; authors Yansong Feng, Hengyi Luo, Qiyuan Chen, Abderrahmane Nitaj, Yanbin Pan; 2024; ePrint 2024/1330 |
| 5 | Corrigan-Gibbs, Kogan, *The Discrete-Logarithm Problem with Preprocessing*, EUROCRYPT 2018, ePrint 2017/1113 | https://eprint.iacr.org/2017/1113 (WebFetch) | VERIFIED | Title "The Discrete-Logarithm Problem with Preprocessing"; authors Henry Corrigan-Gibbs and Dmitry Kogan; 2017; ePrint 2017/1113; "A major revision of an IACR publication in EUROCRYPT 2018" |
| 6 | Petit, Kosters, Messeng, *Algebraic approaches for the ECDLP over prime fields*, PKC 2016 | https://christophe.petit.web.ulb.be/files/16PKC_primeECDLP.pdf (WebFetch); then WebSearch `"Algebraic approaches for the ECDLP over prime fields"` | VERIFIED | WebFetch: PDF returned (421KB); its text contains "Algebraic approaches for the Elliptic Curve Discrete Logarithm Problem over prime fields", which differs from the dossier only by spelling out ECDLP; authors and year not recoverable from the fetch. WebSearch summary: Christophe Petit, Michiel Kosters, Ange Messeng; PKC 2016 |
| 7 | Dossou-Yovo, Nitaj, Togbé, *Solving modular cubic equations with Coppersmith's method*, Information and Computation, 2024 | none (no URL or identifier in dossier); WebSearch `"Solving modular cubic equations with Coppersmith's method"` | VERIFIED | Top-ranked link was arXiv 2304.08718 "Generalized Implicit Factorization Problem" (not the cited paper). Search summary of the ma.x-mol.com listing: title "Solving modular cubic equations with Coppersmith's method", authors Virgile Dossou-Yovo, Abderrahmane Nitaj, Alain Togbé, Inf. Comput., pub date 2024-04-15; full text not read |

## Repository paths
| # | path as written | tested as | outcome |
|---|---|---|---|
| 1 | research/pdp_literature_leads_20261022/BRIEF_SONNET.md (§9 brace list) | /home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md | EXISTS |
| 2 | research/pdp_literature_leads_20261022/LEADS.md (§9 brace list) | /home/user/crypto/research/pdp_literature_leads_20261022/LEADS.md | EXISTS |
| 3 | research/pdp_literature_leads_20261022/leads/LEAD-12.md (§2, §9) | /home/user/crypto/research/pdp_literature_leads_20261022/leads/LEAD-12.md | EXISTS |
| 4 | research/lit2026_w10_s5_coppersmith_20261011/README.md (§3) | /home/user/crypto/research/lit2026_w10_s5_coppersmith_20261011/README.md | EXISTS |
| 5 | research/lit2026_w10_s5_coppersmith_20261011/reports/run-20261011T2118Z/run.md (§9 brace list) | /home/user/crypto/research/lit2026_w10_s5_coppersmith_20261011/reports/run-20261011T2118Z/run.md | EXISTS |
| 6 | examples/lit2026_w10_coppersmith_bound.rs (§3, §9) | /home/user/crypto/examples/lit2026_w10_coppersmith_bound.rs | EXISTS |
| 7 | research/ic_next10_20261014/05-multipoly-lattice/README.md (§3, §9 brace list) | /home/user/crypto/research/ic_next10_20261014/05-multipoly-lattice/README.md | EXISTS |
| 8 | research/ic_next10_20261014/05-multipoly-lattice/milp_multi.py (§3, §6) | /home/user/crypto/research/ic_next10_20261014/05-multipoly-lattice/milp_multi.py | EXISTS |
| 9 | research/ic_next10_20261014/05-multipoly-lattice/milp_v2.md (§9 brace list) | /home/user/crypto/research/ic_next10_20261014/05-multipoly-lattice/milp_v2.md | EXISTS |
| 10 | src/cryptanalysis/semaev_higher.rs (§3, §9 brace list) | /home/user/crypto/src/cryptanalysis/semaev_higher.rs | EXISTS |
| 11 | src/cryptanalysis/prime_small_x.rs (§3, §9 brace list) | /home/user/crypto/src/cryptanalysis/prime_small_x.rs | EXISTS |
| 12 | research/prime_field_smt_20261005/FOLLOWUP_S3_PROTOCOL.md (§3) | /home/user/crypto/research/prime_field_smt_20261005/FOLLOWUP_S3_PROTOCOL.md | EXISTS |

## Quoted numbers
| # | number | file | grep outcome (line) |
|---|---|---|---|
| 1 | 1/6 | /home/user/crypto/research/lit2026_w10_s5_coppersmith_20261011/README.md | FOUND (line 10) |
| 2 | 1/24 | /home/user/crypto/research/lit2026_w10_s5_coppersmith_20261011/README.md | FOUND (line 10) |
| 3 | 1/80 | /home/user/crypto/research/lit2026_w10_s5_coppersmith_20261011/README.md | FOUND (line 10) |
| 4 | 0.018176 | /home/user/crypto/research/lit2026_w10_s5_coppersmith_20261011/README.md | NOT FOUND |
| 5 | 0.024903 | /home/user/crypto/research/lit2026_w10_s5_coppersmith_20261011/README.md | FOUND (line 178) |
| 6 | 0.125 | /home/user/crypto/research/lit2026_w10_s5_coppersmith_20261011/README.md | FOUND (line 212; line 176 matched only as an unescaped-dot hit on 0.03125) |
| 7 | 0.1361 | /home/user/crypto/research/lit2026_w10_s5_coppersmith_20261011/README.md | FOUND (line 178) |
| 8 | 1.12 | /home/user/crypto/research/ic_next10_20261014/05-multipoly-lattice/README.md | FOUND (line 15) |
| 9 | 1331/70875 | /home/user/crypto/research/ic_next10_20261014/05-multipoly-lattice/README.md | FOUND (line 41, 127) |
| 10 | 0.018780 | /home/user/crypto/research/ic_next10_20261014/05-multipoly-lattice/README.md | FOUND (line 127) |
| 11 | 0.025591 | /home/user/crypto/research/ic_next10_20261014/05-multipoly-lattice/README.md | FOUND (line 127) |
| 12 | 16 | /home/user/crypto/research/ic_next10_20261014/05-multipoly-lattice/README.md | FOUND (line 28) |
| 13 | 1/8 | /home/user/crypto/research/pdp_literature_leads_20261022/leads/LEAD-12.md | FOUND (line 36) |
| 14 | 1/9 | /home/user/crypto/RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md | FOUND (line 359) |
| 15 | 0.111 | /home/user/crypto/RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md | NOT FOUND |
| 16 | 1/2 | /home/user/crypto/RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md | FOUND (line 75) |
| 17 | 1/4 | /home/user/crypto/RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md | FOUND (line 75) |
| 18 | 1/5 | /home/user/crypto/RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md | FOUND (line 132) |

## Totals
citations: 6 verified / 0 mismatch / 1 unreachable; paths: 12 exist / 0 missing; numbers: 16 found / 2 not found.
