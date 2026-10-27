# Verification of LEAD-16 (mechanical tier)

Dossier: leads/LEAD-16.md. Procedure: BRIEF_HAIKU.md. Date: 2026-10-22.

## Citations
| # | citation as written | route checked (URL or query) | outcome | page shows |
|---|---|---|---|---|
| 1 | K. Nagao, *Decomposed attack for the Jacobian of a hyperelliptic curve over an extension field*, ePrint 2007/112, https://eprint.iacr.org/2007/112 | WebFetch https://eprint.iacr.org/2007/112 | VERIFIED | "Decomposed Attack for the Jacobian of a Hyperelliptic Curve over an Extension Field"; "Koh-ichi Nagao"; 2007; "Cryptology ePrint Archive, Paper 2007/112" |
| 2 | K. Nagao, *Decomposition attack for the Jacobian of a hyperelliptic curve over an extension field*, ANTS-IX, LNCS 6197, pp. 285–300, 2010, https://link.springer.com/chapter/10.1007/978-3-642-14518-6_23 | WebFetch https://link.springer.com/chapter/10.1007/978-3-642-14518-6_23; then one WebSearch "Decomposition attack for the Jacobian of a hyperelliptic curve over an extension field" | UNREACHABLE | WebFetch: "REDIRECT DETECTED ... Redirect URL ... https://idp.springer.com/authorize ... Status: 303 See Other" (redirect not followed). WebSearch top result link title "Paper 2007/112" (https://eprint.iacr.org/2007/112), a different record; the springerprofessional.de link for the 2010 chapter was not the top result |
| 3 | K. Nagao, *Index calculus attack for Jacobian of hyperelliptic curves of small genus using two large primes*, Japan J. Indust. Appl. Math. 24(3), 289–305, 2007 (recalled; URL in dossier section 9: https://projecteuclid.org/euclid.jjiam/1197909114) | WebFetch https://projecteuclid.org/euclid.jjiam/1197909114; then one WebSearch "Index calculus attack for Jacobian of hyperelliptic curves of small genus using two large primes" | UNREACHABLE | WebFetch: "The content you provided is empty, so I can't find a title, author names, year, or journal/volume." WebSearch top result link title "Open Access" (https://projecteuclid.org/euclid.jjiam/1197909114); no title or author text on that result |
| 4 | A. Joux, V. Vitse, *Cover and decomposition index calculus on elliptic curves made practical*, Eurocrypt 2012 (ePrint 2011/020), https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/CoverDecomp.pdf | WebFetch https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/CoverDecomp.pdf; then one WebSearch "Cover and decomposition index calculus on elliptic curves made practical" | UNREACHABLE | WebFetch: "It's a raw PDF file with compressed (FlateDecode) streams, so the first page's text isn't readable here." WebSearch top result link title "www-fourier.univ-grenoble-alpes.fr"; no title or author text on that result |
| 5 | J.-C. Faugère, A. Wallet, *The point decomposition problem over hyperelliptic curves*, Des. Codes Cryptogr. 86(10), 2279–2314, 2018, https://researchportal.ip-paris.fr/en/publications/the-point-decomposition-problem-over-hyperelliptic-curves-toward-/ | WebFetch https://researchportal.ip-paris.fr/en/publications/the-point-decomposition-problem-over-hyperelliptic-curves-toward-/ | VERIFIED | Title "The point decomposition problem over hyperelliptic curves:" "Toward efficient computation of discrete logarithms in even characteristic"; authors "Jean Charles Faugère" and "Alexandre Wallet"; "Published - 1 Oct 2018"; "Designs, Codes, and Cryptography," Vol. 86, No. 10, pp. 2279-2314 |
| 6 | A. Wallet, talk, GT-BAC 2016-12-08, https://gtbac.telecom-paristech.fr/slides/161208wallet.pdf | WebFetch https://gtbac.telecom-paristech.fr/slides/161208wallet.pdf; WebSearch not run (no exact title in dossier) | UNREACHABLE | WebFetch: "The content doesn't include the first slide's title, author(s), date, or event." |
| 7 | Gaudry (JSC 2009), recalled | none (no URL, identifier or title in dossier) | UNREACHABLE | no route run: dossier line gives no URL, identifier or title |
| 8 | Diem (Compositio 2011), recalled | none (no URL, identifier or title in dossier) | UNREACHABLE | no route run: dossier line gives no URL, identifier or title |
| 9 | Galbraith–Gaudry (DCC 2016), recalled | none (no URL, identifier or title in dossier) | UNREACHABLE | no route run: dossier line gives no URL, identifier or title |
| 10 | "A New Perspective on the Decomposition in the Jacobian of Small Genus Hyperelliptic Curve" (section 8, "seen, not fetched"; no authors, year, venue or URL in dossier) | WebSearch "A New Perspective on the Decomposition in the Jacobian of Small Genus Hyperelliptic Curve" | MISMATCH | top result title "On Jacobians of curves with superelliptic components" (https://arxiv.org/pdf/1310.7241); a cispa.de page with the exact title is listed second, not the top result |

## Repository paths
| # | path as written | tested as | outcome |
|---|---|---|---|
| 1 | research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md | /home/user/crypto/research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md | EXISTS |
| 2 | research/pdp_literature_leads_20261022/leads/LEAD-13.md | /home/user/crypto/research/pdp_literature_leads_20261022/leads/LEAD-13.md | EXISTS |
| 3 | research/gaudry_quartic_d4_20261013/README.md | /home/user/crypto/research/gaudry_quartic_d4_20261013/README.md | EXISTS |
| 4 | research/gaudry_quartic_d4_cost_20261014/README.md | /home/user/crypto/research/gaudry_quartic_d4_cost_20261014/README.md | EXISTS |
| 5 | research/d4_symmetrised_s5_20261015/README.md | /home/user/crypto/research/d4_symmetrised_s5_20261015/README.md | EXISTS |
| 6 | research/ic_next10_20261014/01-fp4-ladder/README.md | /home/user/crypto/research/ic_next10_20261014/01-fp4-ladder/README.md | EXISTS |
| 7 | src/cryptanalysis/gaudry_quartic_d4.rs | /home/user/crypto/src/cryptanalysis/gaudry_quartic_d4.rs | EXISTS |
| 8 | src/cryptanalysis/jv_cover.rs | /home/user/crypto/src/cryptanalysis/jv_cover.rs | EXISTS |
| 9 | research/jv_cover_sparse_fglm_20261018/README.md | /home/user/crypto/research/jv_cover_sparse_fglm_20261018/README.md | EXISTS |
| 10 | src/cryptanalysis/glv_gaudry.rs (lines 1643–1714, tested without suffix) | /home/user/crypto/src/cryptanalysis/glv_gaudry.rs | EXISTS |
| 11 | research/notes/index-calculus/RESEARCH_GLV_INDEX_CALCULUS.md | /home/user/crypto/research/notes/index-calculus/RESEARCH_GLV_INDEX_CALCULUS.md | EXISTS |
| 12 | research/f4_fp_msolve_parity_20261011/ | /home/user/crypto/research/f4_fp_msolve_parity_20261011/ | EXISTS |
| 13 | research/magma_f4_comparison_20261013/ | /home/user/crypto/research/magma_f4_comparison_20261013/ | EXISTS |
| 14 | research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md (line 58, tested without suffix) | /home/user/crypto/research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md | EXISTS |
| 15 | src/cryptanalysis/jv_quartic.rs | /home/user/crypto/src/cryptanalysis/jv_quartic.rs | EXISTS |
| 16 | research/pdp_literature_leads_20261022/BRIEF_SONNET.md | /home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md | EXISTS |
| 17 | /home/user/cryptanalysis/ecc2k130/runner/codegen/nagaodecomp.py | /home/user/cryptanalysis/ecc2k130/runner/codegen/nagaodecomp.py (absolute) | EXISTS |
| 18 | /home/user/crypto | /home/user/crypto (absolute) | EXISTS |

## Quoted numbers
| # | number | file | grep outcome (line) |
|---|---|---|---|
| 1 | 512 | research/gaudry_quartic_d4_20261013/README.md | FOUND (line 21) |
| 2 | 4096 | research/gaudry_quartic_d4_20261013/README.md | NOT FOUND |
| 3 | 8⁴/8 | research/d4_symmetrised_s5_20261015/README.md | NOT FOUND |
| 4 | 8 | research/d4_symmetrised_s5_20261015/README.md | FOUND (line 5) |
| 5 | 2.535–2.552·10¹⁰ | research/gaudry_quartic_d4_20261013/README.md | FOUND (line 33) |
| 6 | 7.06·10¹⁰ | research/gaudry_quartic_d4_20261013/README.md | FOUND (line 34) |
| 7 | 1/8 | research/gaudry_quartic_d4_20261013/README.md | FOUND (line 46) |
| 8 | 517 | research/gaudry_quartic_d4_20261013/README.md | FOUND (line 76) |
| 9 | 20 | research/gaudry_quartic_d4_20261013/README.md | FOUND (line 3) |
| 10 | 6.9·10⁷ | research/gaudry_quartic_d4_cost_20261014/README.md | FOUND (line 1) |
| 11 | 1.4·10⁸ | research/gaudry_quartic_d4_cost_20261014/README.md | FOUND (line 163) |
| 12 | 11× | research/gaudry_quartic_d4_cost_20261014/README.md | FOUND (line 61) |
| 13 | 512 | research/d4_symmetrised_s5_20261015/README.md | FOUND (line 58) |
| 14 | 1.4·10⁹ | research/d4_symmetrised_s5_20261015/README.md | NOT FOUND |
| 15 | 4 (cofactor h = 4) | research/ic_next10_20261014/01-fp4-ladder/README.md | FOUND (line 4) |
| 16 | 2^{126} | research/ic_next10_20261014/01-fp4-ladder/README.md | NOT FOUND |
| 17 | 64 | research/jv_cover_sparse_fglm_20261018/README.md | FOUND (line 11) |
| 18 | 6 (6 quadrics) | research/jv_cover_sparse_fglm_20261018/README.md | FOUND (line 1) |
| 19 | 3,110× | research/notes/index-calculus/RESEARCH_GLV_INDEX_CALCULUS.md | NOT FOUND |
| 20 | 64 | research/notes/index-calculus/RESEARCH_GLV_INDEX_CALCULUS.md | FOUND (line 32) |
| 21 | 9 (9 unknowns) | src/cryptanalysis/glv_gaudry.rs | FOUND (line 376) |
| 22 | 0.96–1.22× | research/f4_fp_msolve_parity_20261011/ (grep -r) | NOT FOUND |
| 23 | 0.44 | research/f4_fp_msolve_parity_20261011/ (grep -r) | FOUND (ratios.svg:695) |
| 24 | 2.4 | research/f4_fp_msolve_parity_20261011/ (grep -r) | FOUND (ratios.svg:4) |
| 25 | 17.8 | research/f4_fp_msolve_parity_20261011/ (grep -r) | FOUND (ratios.svg:62) |
| 26 | 0.6.5 (msolve version) | research/f4_fp_msolve_parity_20261011/ (grep -r) | FOUND (ratios.svg:1049) |
| 27 | 150,000 | research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md | FOUND (line 58) |

## Totals
citations: 2 verified / 1 mismatch / 7 unreachable; paths: 18 exist / 0 missing; numbers: 21 found / 6 not found.
