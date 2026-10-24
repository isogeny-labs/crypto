# Verification of LEAD-11 (mechanical tier)

Dossier: leads/LEAD-11.md. Procedure: BRIEF_HAIKU.md. Date: 2026-10-22.

## Citations
| # | citation as written | route checked (URL or query) | outcome | page shows |
|---|---|---|---|---|
| 1 | McGuire, Mueller, *A New Index Calculus Algorithm for the ECDLP and Summation Polynomial Evaluation*, IACR ePrint 2017/1262 (rev. 2018-08-08) | WebFetch https://eprint.iacr.org/2017/1262 | MISMATCH | title "A New Index Calculus Algorithm for the Elliptic Curve Discrete Logarithm Problem and Summation Polynomial Evaluation"; authors "Gary McGuire and Daniela Mueller"; 2017 (received 2017-12-31; last of 2 revisions 2018-08-08) |
| 2 | Mueller, *A few more index calculus algorithms for the ECDLP*, Caen 2018 (20–22 June), slides on nitaj.users.lmno.cnrs.fr | WebFetch https://nitaj.users.lmno.cnrs.fr/Caen2018/Caen2018Slides/Caen2018Mueller.pdf (slides; paper PDF DanielaMuellerCaen2018.pdf not fetched); then WebSearch '"A few more index calculus algorithms for the ECDLP" Mueller Caen 2018' | UNREACHABLE | WebFetch error: "timeout of 60000ms exceeded". WebSearch returned Caen host URLs and a summary but no page title for the talk; no page was read |
| 3 | Mueller, UCD algebra and number theory seminar, Nov 2015, "Summation polynomials and the elliptic curve discrete logarithm problem" | WebSearch '"Summation polynomials and the elliptic curve discrete logarithm problem" Mueller UCD seminar 2015' (page not fetched) | VERIFIED | top result title "Summation polynomials and the elliptic curve discrete logarithm problem" (ucd.ie seminar listing); summary: speaker Daniela Mueller, 2 November 2015 |
| 4 | McGuire, Mueller, *Some Results on Linearized Trinomials that Split Completely*, arXiv:1905.11755v3 (2019) | WebFetch https://arxiv.org/abs/1905.11755 | VERIFIED | title "Some Results on Linearized Trinomials that Split Completely"; authors "Gary McGuire" and "Daniela Mueller"; submitted 28 May 2019 |
| 5 | Semaev, ePrint 2004/031 | WebFetch https://eprint.iacr.org/2004/031 | VERIFIED | title "Summation polynomials and the discrete logarithm problem on elliptic curves" (no title given in dossier); author "Igor Semaev"; 2004 |
| 6 | Amadori–Pintore–Sala, *On the DLP for prime-field elliptic curves*, FFA 51 (2018), ePrint 2017/609 | WebFetch https://eprint.iacr.org/2017/609 | MISMATCH | title "On the discrete logarithm problem for prime-field elliptic curves"; authors "Alessandro Amadori, Federico Pintore, and Massimiliano Sala"; 2017 |
| 7 | Petit–Kosters–Messeng, PKC 2016 | WebSearch 'Petit Kosters Messeng PKC 2016 elliptic curve discrete logarithm summation polynomials' (no identifier or title in dossier; page not fetched) | VERIFIED | top result title "6 1 Algebraic approaches for the Elliptic Curve Discrete Logarithm Problem over Prime Fields" (slides, troll.iis.sinica.edu.tw/pkc16); summary: Christophe Petit, Michiel Kosters, Ange Messeng, PKC 2016 |
| 8 | Corrigan-Gibbs–Kogan (EUROCRYPT 2018) | WebSearch 'Corrigan-Gibbs Kogan EUROCRYPT 2018 time-space tradeoff discrete logarithm' (no identifier or title in dossier; page not fetched) | VERIFIED | top result title "eurocrypt18discrete slides" (people.csail.mit.edu); summary: Henry Corrigan-Gibbs and Dmitry Kogan, EUROCRYPT 2018 |
| 9 | Corrigan-Gibbs–Henzinger–Wu ePrint 2026/384 (SGGM) | WebFetch https://eprint.iacr.org/2026/384 | VERIFIED | title "The Structured Generic-Group Model"; authors "Henry Corrigan-Gibbs, Alexandra Henzinger, David J. Wu"; 2026 |
| 10 | Dinur–Keller–Marmor arXiv 2505.00894 | WebFetch https://arxiv.org/abs/2505.00894 | VERIFIED | title "Non-Adaptive Cryptanalytic Time-Space Lower Bounds via a Shearer-like Inequality for Permutations"; authors "Itai Dinur, Nathan Keller, Avichai Marmor"; submitted 1 May 2025 (v3 dated 20 May 2026) |
| 11 | Yokoyama–Yasuda–Takahashi(–Kogure), JMC 2020 | WebSearch 'Yokoyama Yasuda Takahashi Kogure Journal of Mathematical Cryptology 2020 index calculus elliptic curve' (no identifier or title in dossier; page not fetched) | VERIFIED | summary: Kazuhiro Yokoyama, Masaya Yasuda, Yasushi Takahashi, Jun Kogure, "Complexity bounds on Semaev's naive index calculus method for ECDLP", J. Math. Cryptology (Oct 2020), DOI 10.1515/jmc-2019-0029; top result title "Journal of Mathematical Cryptology (Oct 2020)" |
| 12 | Meneghetti–Sala–Taufer 2019/2020 | WebSearch 'Meneghetti Sala Taufer summation polynomials elliptic curve discrete logarithm 2019' (no identifier or title in dossier) | UNREACHABLE | no result for this paper; search summary: "The search didn't return the Meneghetti, Sala, and Taufer 2019 paper" |
| 13 | Schneider–Auer–Wagner, TCHES 2025 | WebSearch 'Schneider Auer Wagner TCHES 2025 elliptic curve discrete logarithm index calculus' (no identifier or title in dossier) | UNREACHABLE | no result for this paper; search summary: "The search did not find the Schneider, Auer, and Wagner TCHES 2025 paper" |
| 14 | Hu, ePrint 2024/1923 (EasyChair 15613) | WebFetch https://eprint.iacr.org/2024/1923 (EasyChair 15613 not fetched) | VERIFIED | title "Implementation analysis of index calculus method on elliptic curves over prime finite fields"; author "Jianjun HU"; 2024 |

## Repository paths
| # | path as written | tested as | outcome |
|---|---|---|---|
| 1 | papers/ecdlp-index-calculus-program/LITERATURE.md:51-67 | /home/user/crypto/papers/ecdlp-index-calculus-program/LITERATURE.md | EXISTS |
| 2 | research/lit2026_w3_ggm_audit_20261011/README.md | /home/user/crypto/research/lit2026_w3_ggm_audit_20261011/README.md | EXISTS |
| 3 | research/pdp_root_finder_20261012/README.md | /home/user/crypto/research/pdp_root_finder_20261012/README.md | EXISTS |
| 4 | research/pdp_root_finder_20261012/ | /home/user/crypto/research/pdp_root_finder_20261012/ | EXISTS |
| 5 | research/ecdlp_leads_20261018/README.md | /home/user/crypto/research/ecdlp_leads_20261018/README.md | EXISTS |
| 6 | research/notes/ecc2k130/RESEARCH_ECC2K130_ROUTE_TARGETS.md:1102-1106 | /home/user/crypto/research/notes/ecc2k130/RESEARCH_ECC2K130_ROUTE_TARGETS.md | EXISTS |
| 7 | research/elliptic_snfs_survey_20261016/README.md | /home/user/crypto/research/elliptic_snfs_survey_20261016/README.md | EXISTS |
| 8 | research/ic_next10_20261014/08-quasi-subfield/README.md | /home/user/crypto/research/ic_next10_20261014/08-quasi-subfield/README.md | EXISTS |
| 9 | docs/ic/LEADERBOARD.md | /home/user/crypto/docs/ic/LEADERBOARD.md | EXISTS |
| 10 | research/pdp_literature_leads_20261022/BRIEF_SONNET.md | /home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md | EXISTS |
| 11 | src/cryptanalysis/ic_framework/rootfind.rs | /home/user/crypto/src/cryptanalysis/ic_framework/rootfind.rs | EXISTS |
| 12 | knowledge/literature/KN-LIT-6966ea.md | /home/user/crypto/knowledge/literature/KN-LIT-6966ea.md: MISSING; /home/user/crypto-autoresearcher/knowledge/literature/KN-LIT-6966ea.md: EXISTS | EXISTS |
| 13 | research/sheriff/SHF-20261017-e4d7f6/scripts/readers/papers/pdftext/ic_program.txt | /home/user/crypto/...: MISSING; /home/user/crypto-autoresearcher/...: EXISTS (also run under /home/user/cryptanalysis/ and /home/user/ml-cryptanalysis/: both MISSING) | EXISTS |

## Quoted numbers
| # | number | file | grep outcome (line) |
|---|---|---|---|
| 1 | 0.91 | crypto/research/pdp_root_finder_20261012/README.md | FOUND (33) |
| 2 | 1.18 | crypto/research/pdp_root_finder_20261012/README.md | FOUND (33) |
| 3 | 8.7 | crypto/research/pdp_root_finder_20261012/README.md | FOUND (268, "." wildcard match in 28,798; literal 8.7 at 317) |
| 4 | 0.2 | crypto/research/pdp_root_finder_20261012/README.md | FOUND (3, "." wildcard match in 20261012; literal 0.2 at 152) |
| 5 | 43 | crypto/research/pdp_root_finder_20261012/README.md | FOUND (111, substring of 786433 only; 157 and 197 also substrings) |
| 6 | (23, 8) | crypto/research/pdp_root_finder_20261012/README.md | FOUND (21) |
| 7 | 10 | crypto/docs/ic/LEADERBOARD.md | FOUND (7, substring of 2026-10-14) |
| 8 | 23.4 | crypto/docs/ic/LEADERBOARD.md | FOUND (56) |
| 9 | 8.4 | crypto/docs/ic/LEADERBOARD.md | FOUND (13, "." wildcard match in 0.804582; literal 8.4 at 53 within 8.41) |
| 10 | 22.8 | crypto/docs/ic/LEADERBOARD.md | FOUND (49) |
| 11 | 5.5 | crypto/docs/ic/LEADERBOARD.md | FOUND (33, "." wildcard match in c545; literal 5.5 at 284 within 55.553) |
| 12 | 13.5 | crypto/docs/ic/LEADERBOARD.md | FOUND (56) |
| 13 | nine | crypto/src/cryptanalysis/line_oracle.rs | FOUND (26) |
| 14 | 4 × 4 | crypto/src/cryptanalysis/line_oracle.rs | FOUND (27) |
| 15 | three | crypto/src/cryptanalysis/line_s4_oracle.rs | FOUND (1) |

## Totals
citations: 9 verified / 2 mismatch / 3 unreachable; paths: 13 exist / 0 missing;
numbers: 15 found / 0 not found.
