# Verification of LEAD-13 (mechanical tier)

Dossier: leads/LEAD-13.md. Procedure: BRIEF_HAIKU.md. Date: 2026-10-22.

## Citations
| # | citation as written | route checked (URL or query) | outcome | page shows |
|---|---|---|---|---|
| 1 | J.-C. Faugère, A. Wallet, The point decomposition problem over hyperelliptic curves: toward efficient computation of discrete logarithms in even characteristic, Des. Codes Cryptogr. 86(10), 2018, DOI 10.1007/s10623-017-0449-y, HAL hal-01658573 | https://researchportal.ip-paris.fr/en/publications/the-point-decomposition-problem-over-hyperelliptic-curves-toward-/ (WebFetch) | VERIFIED | Title "The point decomposition problem over hyperelliptic curves:" "Toward efficient computation of discrete logarithms in even characteristic"; authors "Jean Charles Faugère" and "Alexandre Wallet"; "Published - 1 Oct 2018"; "Designs, Codes, and Cryptography," vol. 86, no. 10, pp. 2279–2314 |
| 2 | A. Wallet, Le problème de décompositions de points dans les variétés jacobiennes (talk), GT-BAC, Télécom ParisTech, 2016-12-08 | https://gtbac.telecom-paristech.fr/slides/161208wallet.pdf (WebFetch); then WebSearch "Le problème de décompositions de points dans les variétés jacobiennes" Wallet | UNREACHABLE | PDF fetch: "The content provided doesn't show the title, author(s), date, or venue from the first page. It's a raw PDF dump." WebSearch: no talk page returned; one result is a Mathematics Genealogy entry (mathgenealogy.org/id.php?id=280141) listing Alexandre Wallet's 2016 PhD dissertation under this exact title; the search gave no abstract or full text |
| 3 | K. Nagao, Decomposed attack for the Jacobian of a hyperelliptic curve over an extension field, ePrint 2007/112 (ANTS-IX 2010, LNCS 6197) | https://eprint.iacr.org/2007/112 (WebFetch) | VERIFIED | Title "Decomposed Attack for the Jacobian of a Hyperelliptic Curve over an Extension Field"; author "Koh-ichi Nagao"; 2007; venue "Published elsewhere. Unknown where it was published"; BibTeX host "Cryptology ePrint Archive, Paper 2007/112" |
| 4 | A. Joux, V. Vitse, Cover and decomposition index calculus on elliptic curves made practical, Eurocrypt 2012; ePrint 2011/020 | https://eprint.iacr.org/2011/020 (WebFetch) | VERIFIED | Title "Cover and Decomposition Index Calculus on Elliptic Curves made practical." "Application to a seemingly secure curve over F_{p^6}"; authors "Antoine Joux and Vanessa Vitse"; 2011, ePrint Paper 2011/020; abstract: "Extended version of the accepted paper at Eurocrypt 2012." |
| 5 | Faugère–Gaudry–Huot–Renault (J. Cryptology 2014) | no URL or identifier in dossier; WebSearch "Faugère Gaudry Huot Renault J. Cryptology 2014" | VERIFIED (dossier gives no title; title test not run) | DBLP record: Jean-Charles Faugère, Pierrick Gaudry, Louise Huot, Guénaël Renault, J. Cryptol. 27(4): 595–635 (2014), "Using Symmetries in the Index Calculus for Elliptic Curves Discrete Logarithm" |
| 6 | Faugère–Huot–Joux–Renault–Vitse (Eurocrypt 2014) | no URL or identifier in dossier; WebSearch "Faugère Huot Joux Renault Vitse Eurocrypt 2014" | VERIFIED (dossier gives no title; title test not run) | "Symmetrized Summation Polynomials: Using Small Order Torsion Points to Speed Up Elliptic Curve Index Calculus"; five authors Jean-Charles Faugère, Louise Huot, Antoine Joux, Guénaël Renault, Vanessa Vitse; EUROCRYPT 2014; LNCS 8441, pp. 40–57 (2014); author order differs between sources |

## Repository paths
| # | path as written | tested as | outcome |
|---|---|---|---|
| 1 | research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md | /home/user/crypto/research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md | EXISTS |
| 2 | /home/user/crypto/research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md | /home/user/crypto/research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md | EXISTS |
| 3 | research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md | /home/user/crypto/research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md | EXISTS |
| 4 | research/ic_next10_20261014/07-cubic-ghs-genus3/README.md | /home/user/crypto/research/ic_next10_20261014/07-cubic-ghs-genus3/README.md | EXISTS |
| 5 | /home/user/crypto/research/ic_next10_20261014/07-cubic-ghs-genus3/README.md | /home/user/crypto/research/ic_next10_20261014/07-cubic-ghs-genus3/README.md | EXISTS |
| 6 | src/cryptanalysis/jv_cover.rs | /home/user/crypto/src/cryptanalysis/jv_cover.rs | EXISTS |
| 7 | /home/user/crypto/src/cryptanalysis/jv_cover.rs | /home/user/crypto/src/cryptanalysis/jv_cover.rs | EXISTS |
| 8 | src/cryptanalysis/hyperelliptic_index_calculus.rs | /home/user/crypto/src/cryptanalysis/hyperelliptic_index_calculus.rs | EXISTS |
| 9 | experiments/30_jv_cover_ccov.json | /home/user/crypto/experiments/30_jv_cover_ccov.json | EXISTS |
| 10 | /home/user/crypto/experiments/30_jv_cover_ccov.json | /home/user/crypto/experiments/30_jv_cover_ccov.json | EXISTS |
| 11 | research/notes/index-calculus/RESEARCH_K5_TORSION_JOUX_VITSE.md | /home/user/crypto/research/notes/index-calculus/RESEARCH_K5_TORSION_JOUX_VITSE.md | EXISTS |
| 12 | research/notes/ecc2k130/RESEARCH_ECC2K130_HYPERELLIPTIC.md | /home/user/crypto/research/notes/ecc2k130/RESEARCH_ECC2K130_HYPERELLIPTIC.md | EXISTS |
| 13 | /home/user/crypto/research/notes/ecc2k130/RESEARCH_ECC2K130_HYPERELLIPTIC.md | /home/user/crypto/research/notes/ecc2k130/RESEARCH_ECC2K130_HYPERELLIPTIC.md | EXISTS |
| 14 | /home/user/cryptanalysis/ecc2k130/runner/codegen/nagaodecomp.py | /home/user/cryptanalysis/ecc2k130/runner/codegen/nagaodecomp.py | EXISTS |
| 15 | research/ic_rr_norm_ladder_20260930/RESULTS.md | /home/user/crypto/research/ic_rr_norm_ladder_20260930/RESULTS.md | EXISTS |
| 16 | /home/user/crypto/research/ic_rr_norm_ladder_20260930/RESULTS.md | /home/user/crypto/research/ic_rr_norm_ladder_20260930/RESULTS.md | EXISTS |
| 17 | src/cryptanalysis/ghs_descent.rs | /home/user/crypto/src/cryptanalysis/ghs_descent.rs | EXISTS |
| 18 | src/ | /home/user/crypto/src/ | EXISTS |
| 19 | /home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md | /home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md | EXISTS |

## Quoted numbers
| # | number | file | grep outcome (line) |
|---|---|---|---|
| 1 | 5.15·10⁶ | research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md | FOUND (114, 176, 422) |
| 2 | 3.19·10⁶ | research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md | FOUND (176, 213, 415) |
| 3 | 1.62·10⁶ | research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md | FOUND (415) |
| 4 | 566 | research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md | FOUND (148, 361) |
| 5 | 0.0122 | research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md | FOUND (324, 462) |
| 6 | 0.0137 | research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md | FOUND (324, 462) |
| 7 | 0.0075 | research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md | FOUND (462, 682) |
| 8 | 0.0086 | research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md | FOUND (462, 683) |
| 9 | 53–56 | research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md | NOT FOUND |
| 10 | 70–80 | research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md | FOUND (374, 563) |
| 11 | 1,440 | research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md | FOUND (261) |
| 12 | 3,360 | research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md | FOUND (163, 178, 417) |
| 13 | 8,000 | research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md | FOUND (142, 146, 542, 554, 891, 957, 1120) |
| 14 | 63.99 | experiments/30_jv_cover_ccov.json | FOUND (1868, 7475, 8894, 10669, 14292, 19311, 24291, 26074; unescaped '.' in pattern) |
| 15 | 1,200 | experiments/30_jv_cover_ccov.json | NOT FOUND |
| 16 | 61–64 | experiments/30_jv_cover_ccov.json | NOT FOUND |
| 17 | 1.300 | research/ic_next10_20261014/07-cubic-ghs-genus3/README.md | FOUND (23) |
| 18 | 1.292 | research/ic_next10_20261014/07-cubic-ghs-genus3/README.md | FOUND (23) |
| 19 | 1.308 | research/ic_next10_20261014/07-cubic-ghs-genus3/README.md | FOUND (23) |
| 20 | 1.377 | research/ic_next10_20261014/07-cubic-ghs-genus3/README.md | FOUND (27) |
| 21 | 1.62 | research/ic_next10_20261014/07-cubic-ghs-genus3/README.md | FOUND (37, 72) |
| 22 | 416 | research/notes/index-calculus/RESEARCH_K5_TORSION_JOUX_VITSE.md | FOUND (17, 206) |
| 23 | 3.0·10¹⁰ | research/notes/index-calculus/RESEARCH_K5_TORSION_JOUX_VITSE.md | FOUND (12, 197, 253) |
| 24 | 7.3·10⁷ | research/notes/index-calculus/RESEARCH_K5_TORSION_JOUX_VITSE.md | FOUND (14, 205, 240, 255, 269, 285) |
| 25 | 2^{233} | research/notes/index-calculus/RESEARCH_K5_TORSION_JOUX_VITSE.md | NOT FOUND |
| 26 | 2^{130} | research/notes/ecc2k130/RESEARCH_ECC2K130_HYPERELLIPTIC.md | NOT FOUND |
| 27 | 2^{131} | research/notes/ecc2k130/RESEARCH_ECC2K130_HYPERELLIPTIC.md | NOT FOUND |
| 28 | 1/720 | src/cryptanalysis/jv_cover.rs | FOUND (17) |

## Totals
citations: 5 verified / 0 mismatch / 1 unreachable; paths: 19 exist / 0 missing; numbers: 22 found / 6 not found.
