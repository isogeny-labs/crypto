# Verification of LEAD-03 (mechanical tier)

Dossier: leads/LEAD-03.md. Procedure: BRIEF_HAIKU.md. Date: 2026-10-22.

## Citations
| # | citation as written | route checked (URL or query) | outcome | page shows |
|---|---|---|---|---|
| 1 | Faugère–Huot–Joux–Renault–Vitse, *Symmetrized Summation Polynomials: Using Small Order Torsion Points to Speed up Elliptic Curve Index Calculus*, Eurocrypt 2014, https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/Semaev2torsion.pdf | WebFetch https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/Semaev2torsion.pdf; then WebSearch "Symmetrized Summation Polynomials: Using Small Order Torsion Points to Speed up Elliptic Curve Index Calculus" | VERIFIED | WebFetch: "compressed PDF byte stream ... not readable text"; no title or authors read. WebSearch top result title "# Symmetrized Summation Polynomials: Using Small Order Torsion Points to Speed up Elliptic Curve Index Calculus"; search summary: Jean-Charles Faugère, Louise Huot, Antoine Joux, Guénaël Renault, Vanessa Vitse; EUROCRYPT 2014, LNCS 8441 |
| 2 | Faugère–Gaudry–Huot–Renault, *Using symmetries in the index calculus for elliptic curves discrete logarithm*, J. Cryptology 2014 | WebSearch "Using symmetries in the index calculus for elliptic curves discrete logarithm" (no URL or identifier in dossier) | VERIFIED | top result title "Using symmetries in the index calculus for elliptic curves discrete logarithm" (researchportal.ip-paris.fr); search summary: Faugère, Gaudry, Huot and Renault; 2014 Journal of Cryptology |
| 3 | Barbulescu–Robert–Sarkis, *Models of Kummer lines and Galois representations*, ePrint 2025/543, https://eprint.iacr.org/2025/543 | WebFetch https://eprint.iacr.org/2025/543 | VERIFIED | Title "Models of Kummer lines and Galois representations"; Authors Razvan Barbulescu, Damien Robert, Nicolas Sarkis; Year 2025; Venue "Cryptology ePrint Archive" (Paper 2025/543) |
| 4 | Galbraith–Gaudry, *Recent progress on the elliptic curve discrete logarithm problem*, DCC 78 (2016), ePrint 2015/1022 | WebFetch https://eprint.iacr.org/2015/1022 | MISMATCH | Title "Recent progress on the elliptic curve discrete logarithm problem"; Authors "Steven D. Galbraith and Pierrick Gaudry"; Year "2015" (dossier year 2016, DCC 78); Venue "Cryptology ePrint Archive, Paper 2015/1022" |
| 5 | Galbraith–Gebregiyorgis. INDOCRYPT 2014, ePrint 2014/806 | WebFetch https://eprint.iacr.org/2014/806 | VERIFIED (dossier gives no title; authors and year match) | Title "Summation polynomial algorithms for elliptic curves in characteristic two"; Authors Steven D. Galbraith and Shishay W. Gebregiyorgis; Year 2014; Venue "INDOCRYPT 2014" |
| 6 | HAL id hal-01215623, https://hal.archives-ouvertes.fr/hal-01215623 (no authors or title in dossier) | WebFetch https://hal.archives-ouvertes.fr/hal-01215623; no WebSearch (no title in dossier) | UNREACHABLE | "Access Denied: error code 9e4edb5b6b850c41" (Anubis page, heading "Oh noes!") |
| 7 | Shephard–Todd 1954 / Chevalley | none (no URL, identifier or title in dossier) | UNREACHABLE | no route run; no page |
| 8 | Smart (CHES 2001) | none (no URL, identifier or title in dossier) | UNREACHABLE | no route run; no page |
| 9 | Joye–Quisquater (CHES 2001) | none (no URL, identifier or title in dossier) | UNREACHABLE | no route run; no page |
| 10 | Farashahi–Joye (PKC 2010) | none (no URL, identifier or title in dossier) | UNREACHABLE | no route run; no page |
| 11 | Vitse's DLP 2014 talk | none (no URL, identifier or title in dossier) | UNREACHABLE | no route run; no page |
| 12 | Huot's thesis | none (no URL, identifier or title in dossier) | UNREACHABLE | no route run; no page |

## Repository paths
| # | path as written | tested as | outcome |
|---|---|---|---|
| 1 | /home/user/crypto/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md | as written (absolute) | EXISTS |
| 2 | /home/user/crypto-autoresearcher/docs/ecdlp-literature-review-20260926.md | as written (absolute) | EXISTS |
| 3 | /home/user/crypto-autoresearcher/analysis/ecdlp2m-iteration-20261009/m4-sat-cost-pilot/README.md | as written (absolute) | EXISTS |
| 4 | /home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md | as written (absolute) | EXISTS |
| 5 | /home/user/crypto/research/gaudry_quartic_d4_20261013/README.md | as written (absolute) | EXISTS |
| 6 | research/gaudry_quartic_d4_20261013/README.md | /home/user/crypto (base 1 of 4) | EXISTS |
| 7 | src/cryptanalysis/symmetrized_semaev.rs | /home/user/crypto (base 1 of 4) | EXISTS |
| 8 | src/cryptanalysis/fghr_full.rs | /home/user/crypto (base 1 of 4) | EXISTS |
| 9 | research/pair46_factor_bases_20261015/12-model-small-x/RESULTS.md | /home/user/crypto (base 1 of 4) | EXISTS |
| 10 | examples/three_torsion.rs | /home/user/crypto (base 1 of 4) | EXISTS |
| 11 | research/gaudry_quartic_d4_20261013 | /home/user/crypto (base 1 of 4) | EXISTS |

## Quoted numbers
| # | number | file | grep outcome (line) |
|---|---|---|---|
| 1 | 4096 | /home/user/crypto/research/gaudry_quartic_d4_20261013/README.md | NOT FOUND |
| 2 | 512 | /home/user/crypto/research/gaudry_quartic_d4_20261013/README.md | FOUND (line 21) |
| 3 | 14 | /home/user/crypto/research/gaudry_quartic_d4_20261013/README.md | FOUND (line 22) |
| 4 | 29 | /home/user/crypto/research/gaudry_quartic_d4_20261013/README.md | FOUND (line 22) |
| 5 | 47.5 | /home/user/crypto/research/gaudry_quartic_d4_20261013/README.md | FOUND (line 37) |
| 6 | 2¹⁵⁰ | /home/user/crypto/research/gaudry_quartic_d4_20261013/README.md | NOT FOUND |
| 7 | 2¹³⁵ | /home/user/crypto/research/gaudry_quartic_d4_20261013/README.md | NOT FOUND |
| 8 | 2¹⁶ | /home/user/crypto-autoresearcher/docs/ecdlp-literature-review-20260926.md | NOT FOUND |
| 9 | 240 | /home/user/crypto/research/pair46_factor_bases_20261015/12-model-small-x/RESULTS.md | FOUND (line 8) |
| 10 | 416 | /home/user/crypto/research/notes/index-calculus/RESEARCH_K5_TORSION_JOUX_VITSE.md (located by find) | FOUND (line 17) |
| 11 | 2.01 | /home/user/crypto/research/notes/index-calculus/RESEARCH_K5_TORSION_JOUX_VITSE.md | FOUND (line 33) |
| 12 | 0.99 | /home/user/crypto/research/notes/index-calculus/RESEARCH_K5_TORSION_JOUX_VITSE.md | FOUND (line 193) |
| 13 | 1513 | /home/user/crypto/research/notes/index-calculus/RESEARCH_K5_TORSION_JOUX_VITSE.md | NOT FOUND |
| 14 | 1633 | /home/user/crypto/research/notes/index-calculus/RESEARCH_K5_TORSION_JOUX_VITSE.md | NOT FOUND |
| 15 | 118 | /home/user/crypto/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md | FOUND (line 192) |
| 16 | 12 (solving degree) | /home/user/crypto/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md | FOUND (line 225, first of several) |
| 17 | 14 | /home/user/crypto/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md | FOUND (line 14, first of several) |
| 18 | 5953×6598 | /home/user/crypto/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md | NOT FOUND |
| 19 | 2166×2512 | /home/user/crypto/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md | NOT FOUND |
| 20 | 152 s | /home/user/crypto/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md | NOT FOUND |
| 21 | 19–35 | /home/user/crypto/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md | NOT FOUND |
| 22 | 740×822 | /home/user/crypto/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md | NOT FOUND |
| 23 | 30–60× | /home/user/crypto/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md | FOUND (line 1352) |
| 24 | solving degree 7 | /home/user/crypto/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md | NOT FOUND |
| 25 | 0.76–0.84 | /home/user/crypto/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md | FOUND (line 1943) |
| 26 | 4.0× | /home/user/crypto/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md | FOUND (line 1948) |
| 27 | 10³–10⁵× | /home/user/crypto/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md | NOT FOUND |
| 28 | 2.5× | /home/user/crypto/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md | NOT FOUND |
| 29 | 5–8× | /home/user/crypto/research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md | NOT FOUND |

## Totals
citations: 4 verified / 1 mismatch / 7 unreachable; paths: 11 exist / 0 missing; numbers: 14 found / 15 not found.
