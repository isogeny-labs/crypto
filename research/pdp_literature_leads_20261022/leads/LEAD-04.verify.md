# Verification of LEAD-04 (mechanical tier)

Dossier: leads/LEAD-04.md. Procedure: BRIEF_HAIKU.md. Date: 2026-10-22.

## Citations
| # | citation as written | route checked (URL or query) | outcome | page shows |
|---|---|---|---|---|
| 1 | Kosters, Yeo, *Notes on summation polynomials*, arXiv:1503.08001v3 (2015) | WebFetch https://arxiv.org/abs/1503.08001 | VERIFIED | Title "Notes on summation polynomials"; authors "Michiel Kosters", "Sze Ling Yeo"; 2015 ("Submitted on 27 Mar 2015"); arXiv:1503.08001 |
| 2 | Fong, Hankerson, López, Menezes, *Field inversion and point halving revisited*, IEEE Trans. Computers 53(8):1047–1059 (2004) | WebFetch https://www.ece.uvic.ca/~fayez/research/gf3m/fong_j04.pdf (URL as given in dossier §9) | NOT VERIFIED (title and authors not shown on fetched page) | No title and no authors in the fetched text; year 2004; running header "IEEE TRANSACTIONS ON COMPUTERS, VOL. 53, NO. 8, AUGUST 2004" |
| 3 | Semaev, arXiv:1504.01175 | WebFetch https://arxiv.org/abs/1504.01175 | NOT VERIFIED (dossier gives no title to compare) | Title "New algorithm for the discrete logarithm problem on elliptic curves"; author "Igor Semaev"; "Submitted on 6 Apr 2015"; arXiv:1504.01175 |
| 4 | Karabina, arXiv:1504.02347 | WebFetch https://arxiv.org/abs/1504.02347 | NOT VERIFIED (dossier gives no title to compare) | Title "Point Decomposition Problem in Binary Elliptic Curves"; author "Koray Karabina"; "Submitted on 9 Apr 2015"; arXiv:1504.02347 |
| 5 | Galbraith, Granger, Merz, Petit, *On index calculus algorithms for subfield curves*, ePrint 2020/1315 (SAC 2020) | WebFetch https://eprint.iacr.org/2020/1315 | VERIFIED (venue SAC 2020 not shown on page) | Title "On Index Calculus Algorithms for Subfield Curves"; authors "Steven D. Galbraith, Robert Granger, Simon-Philipp Merz, and Christophe Petit"; year 2020; "Cryptology ePrint Archive, Paper 2020/1315"; "Preprint. MINOR revision." |
| 6 | Silverman, *AEC* 2nd ed., X.1 | WebSearch "AEC" Silverman elliptic curves (no URL or identifier in dossier) | NOT VERIFIED (dossier title is the abbreviation "AEC") | Top result title "The Arithmetic of Elliptic Curves"; snippet gives "Springer-Verlag, 2009" for the second edition; the search states the "AEC" abbreviation is "unconfirmed here" |
| 7 | Washington, *Elliptic Curves* ch. 8 | WebSearch "Elliptic Curves" Washington (no URL or identifier in dossier) | NOT VERIFIED (dossier title is a fragment of the page title) | Top results: "Elliptic Curves: Number Theory and Cryptography", author "Lawrence C. Washington", second edition Chapman & Hall/CRC 2008 (per snippet) |
| 8 | Frey–Rück, Math. Comp. 62 (1994) (Tate pairing non-degenerate over finite fields) | none (no URL, identifier or title in dossier) | NOT CHECKED | no page fetched |
| 9 | Corrigan-Gibbs–Kogan, Shoup (generic bounds) | none (no URL, identifier, title or year in dossier) | NOT CHECKED | no page fetched |
| 10 | CGHW ePrint 2026/384 | WebFetch https://eprint.iacr.org/2026/384 | NOT VERIFIED (dossier gives initials and no title) | Title "The Structured Generic-Group Model"; authors "Henry Corrigan-Gibbs, Alexandra Henzinger, David J. Wu"; 2026; "Cryptology ePrint Archive, Paper 2026/384"; "A major revision of an IACR publication in EUROCRYPT 2026." |
| 11 | DKM ePrint 2025/783 | WebFetch https://eprint.iacr.org/2025/783 | NOT VERIFIED (dossier gives initials and no title) | Title "Non-Adaptive Cryptanalytic Time-Space Lower Bounds via a Shearer-like Inequality for Permutations"; authors "Itai Dinur, Nathan Keller, Avichai Marmor"; 2025; "Published elsewhere. Major revision. STOC 2026." |
| 12 | Sutherland, 18.783 lecture 23 | WebFetch https://math.mit.edu/classes/18.783/2015/LectureNotes23.pdf (URL as given in dossier §9) | NOT VERIFIED (fetched content unreadable; dossier gives no title) | Returned compressed PDF data with no readable text; no title, author or year shown |

## Repository paths
| # | path as written | tested as | outcome |
|---|---|---|---|
| 1 | /home/user/crypto-autoresearcher/knowledge/findings/KN-FIND-14706d.md | /home/user/crypto-autoresearcher/knowledge/findings/KN-FIND-14706d.md | EXISTS |
| 2 | …/ledger/evidence/EV-SEMBIN-0c8bf4.yaml | ledger/evidence/EV-SEMBIN-0c8bf4.yaml under /home/user/crypto: MISSING; under /home/user/crypto-autoresearcher: EXISTS | EXISTS (/home/user/crypto-autoresearcher) |
| 3 | …/ledger/hypotheses/H-SEMBIN-83a856.yaml | ledger/hypotheses/H-SEMBIN-83a856.yaml under /home/user/crypto: MISSING; under /home/user/crypto-autoresearcher: EXISTS | EXISTS (/home/user/crypto-autoresearcher) |
| 4 | src/cryptanalysis/ic_boundary.rs | /home/user/crypto/src/cryptanalysis/ic_boundary.rs | EXISTS (/home/user/crypto) |
| 5 | src/cryptanalysis/koblitz_index_calculus.rs | /home/user/crypto/src/cryptanalysis/koblitz_index_calculus.rs | EXISTS (/home/user/crypto) |
| 6 | research/notes/ecc2k130/RESEARCH_ECC2K130_DECOMPOSITION.md | /home/user/crypto/research/notes/ecc2k130/RESEARCH_ECC2K130_DECOMPOSITION.md | EXISTS (/home/user/crypto) |
| 7 | research/notes/index-calculus/RESEARCH_IC_BOUNDARY_LEDGER.md | /home/user/crypto/research/notes/index-calculus/RESEARCH_IC_BOUNDARY_LEDGER.md | EXISTS (/home/user/crypto) |
| 8 | docs/ic/boundary_targets.json | /home/user/crypto/docs/ic/boundary_targets.json | EXISTS (/home/user/crypto) |
| 9 | research/ic_leads_review_20261018/README.md | /home/user/crypto/research/ic_leads_review_20261018/README.md | EXISTS (/home/user/crypto) |
| 10 | research/fall_degree_bounds_20261008/RESULTS.md | /home/user/crypto/research/fall_degree_bounds_20261008/RESULTS.md | EXISTS (/home/user/crypto) |
| 11 | docs/ic/LEADERBOARD.md | /home/user/crypto/docs/ic/LEADERBOARD.md | EXISTS (/home/user/crypto) |
| 12 | crypto-autoresearcher/knowledge/findings/{KN-FIND-14706d,KN-FIND-007}.md (KN-FIND-007) | /home/user/crypto-autoresearcher/knowledge/findings/KN-FIND-007.md | EXISTS |
| 13 | crypto-autoresearcher/ledger/evidence/EV-SEMBIN-0c8bf4.yaml | /home/user/crypto-autoresearcher/ledger/evidence/EV-SEMBIN-0c8bf4.yaml | EXISTS |
| 14 | crypto/src/cryptanalysis/orbit_pair_table.rs | /home/user/crypto/src/cryptanalysis/orbit_pair_table.rs | EXISTS |
| 15 | crypto/src/cryptanalysis/native_signed_mitm.rs | /home/user/crypto/src/cryptanalysis/native_signed_mitm.rs | EXISTS |
| 16 | crypto/src/cryptanalysis/ic_boundary.rs | /home/user/crypto/src/cryptanalysis/ic_boundary.rs | EXISTS |
| 17 | crypto/src/cryptanalysis/koblitz_index_calculus.rs | /home/user/crypto/src/cryptanalysis/koblitz_index_calculus.rs | EXISTS |
| 18 | crypto/src/cryptanalysis/koblitz_factor_base_search.rs | /home/user/crypto/src/cryptanalysis/koblitz_factor_base_search.rs | EXISTS |
| 19 | src/bin/ic/workflow.rs | /home/user/crypto/src/bin/ic/workflow.rs | EXISTS (/home/user/crypto) |
| 20 | docs/ic/leaderboard.json | /home/user/crypto/docs/ic/leaderboard.json | EXISTS (/home/user/crypto) |
| 21 | docs/curves/registry.json | /home/user/crypto/docs/curves/registry.json | EXISTS (/home/user/crypto) |
| 22 | docs/curves/ICV1.md | /home/user/crypto/docs/curves/ICV1.md | EXISTS (/home/user/crypto) |
| 23 | research/notes/index-calculus/RESEARCH_FACTOR_BASE_SHAPE_SEARCH.md | /home/user/crypto/research/notes/index-calculus/RESEARCH_FACTOR_BASE_SHAPE_SEARCH.md | EXISTS (/home/user/crypto) |
| 24 | research/notes/index-calculus/RESEARCH_FACTOR_BASE_SOLVE_COST.md | /home/user/crypto/research/notes/index-calculus/RESEARCH_FACTOR_BASE_SOLVE_COST.md | EXISTS (/home/user/crypto) |
| 25 | research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md | /home/user/crypto/research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md | EXISTS (/home/user/crypto) |
| 26 | research/{ic_leads_review_20261018,…}/… (ic_leads_review_20261018) | /home/user/crypto/research/ic_leads_review_20261018 | EXISTS (/home/user/crypto) |
| 27 | research/{…,fall_degree_bounds_20261008,…}/… (fall_degree_bounds_20261008) | /home/user/crypto/research/fall_degree_bounds_20261008 | EXISTS (/home/user/crypto) |
| 28 | research/{…,ic_tree_split_cutout_20260930,…}/… | /home/user/crypto/research/ic_tree_split_cutout_20260930 | EXISTS (/home/user/crypto) |
| 29 | research/{…,isogeny_fibers_20261010}/… | /home/user/crypto/research/isogeny_fibers_20261010 | EXISTS (/home/user/crypto) |
| 30 | research/ic_tool_program/rule/v3/README.md | /home/user/crypto/research/ic_tool_program/rule/v3/README.md | EXISTS (/home/user/crypto) |

## Quoted numbers
| # | number | file | grep outcome (line) |
|---|---|---|---|
| 1 | 79,751 | /home/user/crypto-autoresearcher/ledger/evidence/EV-SEMBIN-0c8bf4.yaml | NOT FOUND |
| 2 | 1.97 | /home/user/crypto/research/notes/ecc2k130/RESEARCH_ECC2K130_DECOMPOSITION.md | FOUND (260) |
| 3 | 2.04 | /home/user/crypto/research/notes/ecc2k130/RESEARCH_ECC2K130_DECOMPOSITION.md | FOUND (258) |
| 4 | 1.92 | /home/user/crypto/docs/ic/boundary_targets.json | NOT FOUND (whole-token) |
| 5 | 1.92 | /home/user/crypto/research/notes/index-calculus/RESEARCH_IC_BOUNDARY_LEDGER.md | FOUND (1182) |
| 6 | 2.21 | /home/user/crypto/docs/ic/boundary_targets.json | NOT FOUND (whole-token) |
| 7 | 2.21 | /home/user/crypto/research/notes/index-calculus/RESEARCH_IC_BOUNDARY_LEDGER.md | FOUND (1182) |
| 8 | 3.04 | /home/user/crypto/docs/ic/boundary_targets.json | NOT FOUND (whole-token) |
| 9 | 3.04 | /home/user/crypto/research/notes/index-calculus/RESEARCH_IC_BOUNDARY_LEDGER.md | FOUND (1184) |
| 10 | 98.5 | /home/user/crypto/docs/ic/LEADERBOARD.md | FOUND (109) |
| 11 | 99.7 | /home/user/crypto/docs/ic/LEADERBOARD.md | FOUND (112) |
| 12 | 16 | /home/user/crypto-autoresearcher/knowledge/findings/KN-FIND-14706d.md | FOUND (69) |
| 13 | 3 | /home/user/crypto/research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md | FOUND (28; whole-token) |
| 14 | 5 | /home/user/crypto/research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md | FOUND (11; whole-token) |
| 15 | 6 | /home/user/crypto/research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md | FOUND (17; whole-token) |
| 16 | 7 | /home/user/crypto/research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md | FOUND (4; whole-token) |

## Totals
citations: 2 verified / 0 mismatch / 0 unreachable; paths: 30 exist / 0 missing;
numbers: 12 found / 4 not found.
