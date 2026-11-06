# Verification of LEAD-19 (mechanical tier)

Dossier: leads/LEAD-19.md. Procedure: BRIEF_HAIKU.md. Date: 2026-10-22.

## Citations
| # | citation as written | route checked (URL or query) | outcome | page shows |
|---|---|---|---|---|
| 1 | Kosters, Yeo, *Notes on summation polynomials*, arXiv:1503.08001v3, 2015 | WebFetch https://arxiv.org/pdf/1503.08001; then WebSearch `"Notes on summation polynomials" Kosters Yeo arXiv 1503.08001` | VERIFIED (via WebSearch; fetch gave no title or authors) | fetch: no title, authors or year readable; only `http://arxiv.org/abs/1503.08001v3`. search top result: "Notes on summation polynomials", Michiel Kosters and Sze Ling Yeo, 2015 (v3 dated 8 June 2015) |
| 2 | Fong, Hankerson, López, Menezes, *Field inversion and point halving revisited*, IEEE Trans. Computers 53(8), 2004 | WebFetch https://www.ece.uvic.ca/~fayez/research/gf3m/fong_j04.pdf; then WebSearch `"Field inversion and point halving revisited" Fong Hankerson López Menezes` | UNVERIFIED (no title on either route) | fetch: running header "IEEE TRANSACTIONS ON COMPUTERS", volume 53, issue 8, August 2004; no title or authors in excerpt. search top result: authors Kenny Fong, Darrel Hankerson, Julio López, Alfred Menezes; volume 53, issue 8, pages 1047–1059, 2004; no title in result text |
| 3 | Galbraith, Granger, Merz, Petit, *On index calculus algorithms for subfield curves*, ePrint 2020/1315 | WebFetch https://eprint.iacr.org/2020/1315 | VERIFIED | "On Index Calculus Algorithms for Subfield Curves"; Steven D. Galbraith, Robert Granger, Simon-Philipp Merz, Christophe Petit; 2020; Cryptology ePrint Archive, Paper 2020/1315 |
| 4 | Galbraith, Gaudry, *Recent progress on the elliptic curve discrete logarithm problem*, ePrint 2015/1022 | WebFetch https://eprint.iacr.org/2015/1022 | VERIFIED | "Recent progress on the elliptic curve discrete logarithm problem"; Steven D. Galbraith and Pierrick Gaudry; 2015; Cryptology ePrint Archive, Paper 2015/1022 |
| 5 | KN-FIND-007 (yield conservation) | none: dossier gives an identifier only, no URL or title | UNREACHABLE (no route applicable) | n/a |
| 6 | Corrigan-Gibbs–Kogan | none: dossier gives authors only, no URL, title, year, venue or identifier | UNREACHABLE (no route applicable) | n/a |
| 7 | Shoup | none: dossier gives an author only, no URL, title, year, venue or identifier | UNREACHABLE (no route applicable) | n/a |
| 8 | CGHW 2026/384 | WebFetch https://eprint.iacr.org/2026/384 (from the ePrint id) | UNVERIFIED (page fetched; dossier gives no title or author to match) | "The Structured Generic-Group Model"; Henry Corrigan-Gibbs, Alexandra Henzinger, David J. Wu; 2026; ePrint 2026/384, "A major revision of an IACR publication in EUROCRYPT 2026" |
| 9 | DKM 2025/783 | WebFetch https://eprint.iacr.org/2025/783 (from the ePrint id) | UNVERIFIED (page fetched; dossier gives no title or author to match) | "Non-Adaptive Cryptanalytic Time-Space Lower Bounds via a Shearer-like Inequality for Permutations"; Itai Dinur, Nathan Keller, Avichai Marmor; 2025; ePrint 2025/783, "Published elsewhere. Major revision. STOC 2026" |

## Repository paths
| # | path as written | tested as | outcome |
|---|---|---|---|
| 1 | docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json | /home/user/crypto/docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json | EXISTS |
| 2 | src/cryptanalysis/koblitz_index_calculus.rs | /home/user/crypto/src/cryptanalysis/koblitz_index_calculus.rs | EXISTS |
| 3 | src/cryptanalysis/ic_framework/plugins.rs | /home/user/crypto/src/cryptanalysis/ic_framework/plugins.rs | EXISTS |
| 4 | src/cryptanalysis/ic_framework/fiber.rs | /home/user/crypto/src/cryptanalysis/ic_framework/fiber.rs | EXISTS |
| 5 | research/isogeny_conductor_gap_ic_20261010/README.md | /home/user/crypto/research/isogeny_conductor_gap_ic_20261010/README.md | EXISTS |
| 6 | src/cryptanalysis/ic_boundary.rs | /home/user/crypto/src/cryptanalysis/ic_boundary.rs | EXISTS |
| 7 | research/notes/index-calculus/RESEARCH_IC_BOUNDARY_LEDGER.md | /home/user/crypto/research/notes/index-calculus/RESEARCH_IC_BOUNDARY_LEDGER.md | EXISTS |
| 8 | docs/ic/LEADERBOARD.md | /home/user/crypto/docs/ic/LEADERBOARD.md | EXISTS |
| 9 | docs/ic/boundary_targets.json | /home/user/crypto/docs/ic/boundary_targets.json | EXISTS |
| 10 | research/pdp_literature_leads_20261022/leads/LEAD-04.md | /home/user/crypto/research/pdp_literature_leads_20261022/leads/LEAD-04.md | EXISTS |
| 11 | docs/VALIDATION_GATE.md | /home/user/crypto/docs/VALIDATION_GATE.md | EXISTS |
| 12 | docs/ic/tools/ | /home/user/crypto/docs/ic/tools/ | EXISTS |
| 13 | src/bin/ic/boundary.rs | /home/user/crypto/src/bin/ic/boundary.rs | EXISTS |
| 14 | research/lead19_projected_base/char2.json | /home/user/crypto/…, /home/user/crypto-autoresearcher/…, /home/user/cryptanalysis/…, /home/user/ml-cryptanalysis/… (all four) | MISSING (all four bases) |
| 15 | research/lead19_projected_base/prime24.json | /home/user/crypto/…, /home/user/crypto-autoresearcher/…, /home/user/cryptanalysis/…, /home/user/ml-cryptanalysis/… (all four) | MISSING (all four bases) |
| 16 | research/pdp_literature_leads_20261022/BRIEF_SONNET.md | /home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md | EXISTS |
| 17 | research/pdp_literature_leads_20261022/LEADS.md | /home/user/crypto/research/pdp_literature_leads_20261022/LEADS.md | EXISTS |
| 18 | research/pdp_literature_leads_20261022/leads/LEAD-04.verify.md | /home/user/crypto/research/pdp_literature_leads_20261022/leads/LEAD-04.verify.md | EXISTS |
| 19 | docs/ic/tools/boundary_round_compare.py | /home/user/crypto/docs/ic/tools/boundary_round_compare.py | EXISTS |
| 20 | research/ic_tool_program/rule/v3/README.md | /home/user/crypto/research/ic_tool_program/rule/v3/README.md | EXISTS |

## Quoted numbers
| # | number | file | grep outcome (line) |
|---|---|---|---|
| 1 | 3.5 | research/isogeny_conductor_gap_ic_20261010/README.md | FOUND (104, 105, 106) |
| 2 | 0.99 | research/isogeny_conductor_gap_ic_20261010/README.md | FOUND (105, 106, 108, 109, 163) |
| 3 | 0.65 | research/isogeny_conductor_gap_ic_20261010/README.md | FOUND (104) |
| 4 | 5.8 | research/isogeny_conductor_gap_ic_20261010/README.md | FOUND (104) |
| 5 | 1.4–1.6 | research/pdp_literature_leads_20261022/leads/LEAD-04.md | FOUND (238) |
| 6 | 2/3 | research/pdp_literature_leads_20261022/leads/LEAD-04.md | FOUND (160, 238) |

## Totals
citations: 3 verified / 0 mismatch / 3 unreachable (plus 3 unverified: rows 2, 8, 9); paths: 18 exist / 2 missing; numbers: 6 found / 0 not found.
