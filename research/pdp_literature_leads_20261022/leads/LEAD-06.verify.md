# Verification of LEAD-06 (mechanical tier)

Dossier: leads/LEAD-06.md. Procedure: BRIEF_HAIKU.md. Date: 2026-10-22.

## Citations
| # | citation as written | route checked (URL or query) | outcome | page shows |
|---|---|---|---|---|
| 1 | [GM-IC] Gorla, Massierer, *Index calculus in the trace zero variety*, Adv. Math. Commun. 9(4) (2015) 515–539; arXiv 1405.1059 v2; ePrint 2014/318 | WebFetch https://arxiv.org/abs/1405.1059 | MISMATCH (year) | title "Index Calculus in the Trace Zero Variety"; authors Elisa Gorla, Maike Massierer; year 2014 (submitted 5 May 2014; v2 revised 23 Feb 2015); no journal shown; dossier year 2015 |
| 2 | [GM-PC] Gorla, Massierer, *Point compression for the trace zero subgroup over a small degree extension field*, Des. Codes Cryptogr. (2015), DOI 10.1007/s10623-014-9921-0; arXiv 1403.0126 | WebFetch https://arxiv.org/abs/1403.0126 | MISMATCH (year) | title "Point compression for the trace zero subgroup over a small degree extension field"; authors Elisa Gorla, Maike Massierer; year 2014 (submitted 1 Mar 2014); comment "to appear in Designs, Codes and Cryptography"; dossier year 2015 |
| 3 | [GM-OR] Gorla, Massierer, *An optimal representation for the trace zero subgroup*, arXiv 1405.2733 v4, ePrint 2014/333 | WebFetch https://arxiv.org/abs/1405.2733 | VERIFIED | title "An optimal representation for the trace zero subgroup"; authors Elisa Gorla, Maike Massierer; year 2014 (first submitted 12 May 2014; v4 revised 15 Jun 2016); no venue shown; dossier gives no year |
| 4 | Massierer, Basel thesis 2014, *Trace zero varieties in cryptography: optimal representation and index calculus* | WebFetch https://edoc-vmtest.ub.unibas.ch/32498 (URL from dossier §9): getaddrinfo ENOTFOUND edoc-vmtest.ub.unibas.ch; then WebSearch "Trace zero varieties in cryptography: optimal representation and index calculus" | MISMATCH | top result title "Point compression for the trace zero subgroup over a small degree extension field" (link.springer.com/article/10.1007/s10623-014-9921-0); authors and year not shown; no result link carries the thesis title |
| 5 | Gaudry, J. Symb. Comput. 44 (2009) | none: no URL, identifier or exact title in dossier | UNREACHABLE | no page fetched (no route available) |
| 6 | Diem, Compositio Math. 147 (2011) | none: no URL, identifier or exact title in dossier | UNREACHABLE | no page fetched (no route available) |
| 7 | Joux–Vitse, J. Cryptology (2012/13) | none: no URL, identifier or exact title in dossier | UNREACHABLE | no page fetched (no route available) |
| 8 | Diem–Scholten, trace-zero cover attacks (genus 4 for n = 5, 1 curve found in the AREHCC report) | none: no URL, identifier or exact title in dossier | UNREACHABLE | no page fetched (no route available) |
| 9 | Galbraith–Gaudry survey, Des. Codes Cryptogr. 78 (2016), ePrint 2015/1022 | WebFetch https://eprint.iacr.org/2015/1022 | MISMATCH (year) | title "Recent progress on the elliptic curve discrete logarithm problem"; authors Steven D. Galbraith and Pierrick Gaudry; year 2015; venue Cryptology ePrint Archive, Paper 2015/1022; no journal listed; dossier year 2016; dossier gives no title |
| 10 | ePrint 2026/384 (dossier §8; identifier only) | WebFetch https://eprint.iacr.org/2026/384 | NOT COMPARABLE (dossier gives identifier only) | title "The Structured Generic-Group Model"; authors Henry Corrigan-Gibbs, Alexandra Henzinger, David J. Wu; year 2026; venue "A major revision of an IACR publication in EUROCRYPT 2026" |

## Repository paths
| # | path as written | tested as | outcome |
|---|---|---|---|
| 1 | /home/user/crypto/src/cryptanalysis/gaudry_cubic.rs | /home/user/crypto/src/cryptanalysis/gaudry_cubic.rs | EXISTS |
| 2 | /home/user/crypto/research/ic_next10_20261014/01-fp4-ladder/README.md | /home/user/crypto/research/ic_next10_20261014/01-fp4-ladder/README.md | EXISTS |
| 3 | /home/user/crypto/RESEARCH_ISOGENY_FIBERS_ECDLP.md:1819 | /home/user/crypto/RESEARCH_ISOGENY_FIBERS_ECDLP.md | EXISTS |
| 4 | /home/user/crypto-autoresearcher/focus/current_plan.md:551–570, 2219–2241 | /home/user/crypto-autoresearcher/focus/current_plan.md | EXISTS |
| 5 | /home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md | /home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md | EXISTS |
| 6 | /home/user/crypto | /home/user/crypto | EXISTS |
| 7 | research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | /home/user/crypto/research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | EXISTS |
| 8 | research/notes/index-calculus/RESEARCH_IC_BOUNDARY_LEDGER.md:442 | /home/user/crypto/research/notes/index-calculus/RESEARCH_IC_BOUNDARY_LEDGER.md | EXISTS |
| 9 | docs/ic/ECDLP_RESEARCH_TECHNIQUES.md:64–78 | /home/user/crypto/docs/ic/ECDLP_RESEARCH_TECHNIQUES.md | EXISTS |
| 10 | docs/ic/factor-base-encyclopedia/REPORT.typ:1125–1132 | /home/user/crypto/docs/ic/factor-base-encyclopedia/REPORT.typ | EXISTS |
| 11 | src/cryptanalysis/gaudry_cubic.rs | /home/user/crypto/src/cryptanalysis/gaudry_cubic.rs | EXISTS |
| 12 | src/cryptanalysis/glv_gaudry.rs | /home/user/crypto/src/cryptanalysis/glv_gaudry.rs | EXISTS |
| 13 | research/weak_curve_leaderboard_20261012/COVERAGE_AUDIT.md | /home/user/crypto/research/weak_curve_leaderboard_20261012/COVERAGE_AUDIT.md | EXISTS |
| 14 | research/ic_next10_20261014/01-fp4-ladder/README.md | /home/user/crypto/research/ic_next10_20261014/01-fp4-ladder/README.md | EXISTS |
| 15 | examples/gaudry_cubic_bench.rs | /home/user/crypto/examples/gaudry_cubic_bench.rs | EXISTS |
| 16 | src/cryptanalysis/f4_fp.rs | /home/user/crypto/src/cryptanalysis/f4_fp.rs | EXISTS |

## Quoted numbers
| # | number | file | grep outcome (line) |
|---|---|---|---|
| 1 | 2^20 | /home/user/crypto/src/cryptanalysis/gaudry_cubic.rs | FOUND (line 529) |
| 2 | 2^14 | /home/user/crypto/RESEARCH_ISOGENY_FIBERS_ECDLP.md | NOT FOUND |
| 3 | 1.213·10¹² | /home/user/crypto/research/ic_next10_20261014/01-fp4-ladder/README.md | FOUND (line 69) |
| 4 | 2.54·10¹⁰ | /home/user/crypto/research/ic_next10_20261014/01-fp4-ladder/README.md | FOUND (line 71) |
| 5 | 0.333 | /home/user/crypto/research/ic_next10_20261014/01-fp4-ladder/README.md | FOUND (line 15; also 116, 134, 169, 171) |
| 6 | 0.464 | /home/user/crypto/research/ic_next10_20261014/01-fp4-ladder/README.md | FOUND (line 117; also 125, 252) |
| 7 | 1.44× | /home/user/crypto/research/ic_next10_20261014/01-fp4-ladder/README.md | FOUND (line 151) |
| 8 | 2^126–2^137 | /home/user/crypto/research/ic_next10_20261014/01-fp4-ladder/README.md | NOT FOUND |
| 9 | 64 | /home/user/crypto/research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | FOUND (line 61; many further matches) |
| 10 | 1/6 | /home/user/crypto/research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | FOUND (line 1089; further matches) |
| 11 | 4096 | /home/user/crypto/research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | NOT FOUND |
| 12 | 29 | /home/user/crypto/research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | FOUND (line 170; many further matches) |
| 13 | 17× | /home/user/crypto/research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | FOUND (line 849; also 2365, 2451) |
| 14 | 2^151 | /home/user/crypto/research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | NOT FOUND |
| 15 | 0.518 | /home/user/crypto/research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | FOUND (line 2548; also 2555, 2588, 2605) |
| 16 | 0.031 | /home/user/crypto/research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | FOUND (line 1760; also 1764, 1775, 2202, 2555, 2605) |
| 17 | 0.733 | /home/user/crypto/research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | FOUND (line 2569) |
| 18 | 0.036 | /home/user/crypto/research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | FOUND (line 2544; also 2569) |
| 19 | 2^156 | /home/user/crypto/research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | NOT FOUND |
| 20 | 1+o(1) | /home/user/crypto-autoresearcher/focus/current_plan.md | FOUND (line 559; also 2224) |

## Totals
citations: 1 verified / 4 mismatch / 4 unreachable; paths: 16 exist / 0 missing;
numbers: 15 found / 5 not found.
