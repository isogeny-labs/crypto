# Verification of LEAD-02 (mechanical tier)

Dossier: leads/LEAD-02.md. Procedure: BRIEF_HAIKU.md. Date: 2026-10-22.

## Citations
| # | citation as written | route checked (URL or query) | outcome | page shows |
|---|---|---|---|---|
| 1 | Joux, Vitse, *A variant of the F4 algorithm*, CT-RSA 2011, LNCS 6558, pp. 356–375 | `https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/CT-RSA2011.pdf` (WebFetch: binary PDF, no title or authors returned); then WebSearch `"A variant of the F4 algorithm" Joux Vitse CT-RSA 2011` | MISMATCH | top result title "CT RSA2011"; search summary: Antoine Joux and Vanessa Vitse, Topics in Cryptology CT-RSA 2011, DBLP pp. 356-375 |
| 2 | Joux, Vitse, *Elliptic curve discrete logarithm problem over small degree extension fields. Application to the static Diffie–Hellman problem on E(F_{q⁵})*, J. Cryptology 26 (2013) (ePrint 2010/157) | `https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/DLFq5.pdf` (WebFetch: binary PDF, no title or authors returned); then WebSearch `"Elliptic curve discrete logarithm problem over small degree extension fields" Joux Vitse` | MISMATCH | top result title "Paper 2010/157" (eprint.iacr.org/2010/157); no paper title shown on that result; search summary (a different result, CNRS listing): J. Cryptology 26(1):119-143, 2013 |
| 3 | Vitse, *F4 traces and index calculus on elliptic curves over extension fields*, ECC 2010 slides | `https://www-fourier.univ-grenoble-alpes.fr/~viva/research/talks/ECC2010.pdf` (WebFetch: binary PDF, no title or authors returned); then WebSearch `"F4 traces and index calculus on elliptic curves over extension fields" Vitse` | VERIFIED | top result title "# F4 traces and index calculus on elliptic curves over extension fields"; search summary: talk by Vanessa Vitse, ECC 2010 workshop, Redmond, October 2010 |
| 4 | Weispfenning, *Comprehensive Gröbner bases*, J. Symb. Comput. 14 (1992) 1–29 | none: dossier gives no URL or identifier; no route run | UNREACHABLE | no page returned; no route run |
| 5 | Montes, Wibmer, *Gröbner bases for polynomial systems with parameters*, J. Symb. Comput. 45(12) (2010) 1391–1425 | none: dossier gives no URL or identifier; no route run | UNREACHABLE | no page returned; no route run |
| 6 | Faugère, Mou, *Sparse FGLM algorithms*, J. Symb. Comput. 80 (2017), arXiv:1304.1238, `https://arxiv.org/abs/1304.1238` | `https://arxiv.org/abs/1304.1238` (WebFetch) | VERIFIED | title "Sparse FGLM algorithms"; authors "Jean-Charles Faugère", "Chenqi Mou"; "Submitted on 4 Apr 2013"; journal reference "Journal of Symbolic Computation, 2017, 80(3): 538-569" |
| 7 | Traverso, *Gröbner trace algorithms* (ISSAC 1988) | none: dossier gives no URL or identifier; no route run | UNREACHABLE | no page returned; no route run |
| 8 | Gaudry (J. Symb. Comput. 44, 2008) | none: dossier gives no URL or identifier; no route run | UNREACHABLE | no page returned; no route run |
| 9 | Diem (Compositio 147, 2011) | none: dossier gives no URL or identifier; no route run | UNREACHABLE | no page returned; no route run |
| 10 | Faugère–Gaudry–Huot–Renault (J. Cryptology 27, 2014) | none: dossier gives no URL or identifier; no route run | UNREACHABLE | no page returned; no route run |

## Repository paths
| # | path as written | tested as | outcome |
|---|---|---|---|
| 1 | research/gaudry_quartic_replay_20261013/README.md | /home/user/crypto/research/gaudry_quartic_replay_20261013/README.md | EXISTS |
| 2 | research/pdp_root_finder_requirements_20261012/README.md | /home/user/crypto/research/pdp_root_finder_requirements_20261012/README.md | EXISTS |
| 3 | research/gaudry_quartic_d4_20261013/README.md | /home/user/crypto/research/gaudry_quartic_d4_20261013/README.md | EXISTS |
| 4 | research/ic_next10_20261014/01-fp4-ladder/README.md | /home/user/crypto/research/ic_next10_20261014/01-fp4-ladder/README.md | EXISTS |
| 5 | src/cryptanalysis/gaudry_cubic.rs | /home/user/crypto/src/cryptanalysis/gaudry_cubic.rs | EXISTS |
| 6 | src/cryptanalysis/gaudry_quartic_d4.rs | /home/user/crypto/src/cryptanalysis/gaudry_quartic_d4.rs | EXISTS |
| 7 | research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | /home/user/crypto/research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | EXISTS |
| 8 | research/pdp_literature_leads_20261022/BRIEF_SONNET.md | /home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md | EXISTS |

## Quoted numbers
| # | number | file | grep outcome (line) |
|---|---|---|---|
| 1 | 7.0 % | research/gaudry_quartic_replay_20261013/README.md | FOUND (line 80) |
| 2 | 14.5 % | research/gaudry_quartic_replay_20261013/README.md | FOUND (line 78) |
| 3 | 4,956 | research/gaudry_quartic_replay_20261013/README.md | FOUND (line 72) |
| 4 | 41,780 | research/gaudry_quartic_replay_20261013/README.md | FOUND (line 72) |
| 5 | 3.1 | research/gaudry_quartic_replay_20261013/README.md | FOUND (line 52) |
| 6 | 4.3 % | research/pdp_root_finder_requirements_20261012/README.md | FOUND (line 47) |
| 7 | 58 % | research/pdp_root_finder_requirements_20261012/README.md | FOUND (line 280) |
| 8 | 2.535–2.552·10¹⁰ | research/gaudry_quartic_d4_20261013/README.md | FOUND (line 33) |
| 9 | 1.9·10¹⁰ | research/gaudry_quartic_d4_20261013/README.md | NOT FOUND |
| 10 | 6.2·10⁹ | research/gaudry_quartic_d4_20261013/README.md | NOT FOUND |
| 11 | 1.4·10⁸ | research/gaudry_quartic_d4_20261013/README.md | NOT FOUND |
| 12 | 512 | research/gaudry_quartic_d4_20261013/README.md | FOUND (line 21) |
| 13 | 19 | research/gaudry_quartic_d4_20261013/README.md | FOUND (line 18) |
| 14 | 20 | research/gaudry_quartic_d4_20261013/README.md | FOUND (line 3) |
| 15 | 517 | research/gaudry_quartic_d4_20261013/README.md | FOUND (line 76) |
| 16 | 24,830 | research/ic_next10_20261014/01-fp4-ladder/README.md | FOUND (line 101) |
| 17 | 1033 | research/ic_next10_20261014/01-fp4-ladder/README.md | FOUND (line 34) |
| 18 | 1.36·10⁷ | research/ic_next10_20261014/01-fp4-ladder/README.md | FOUND (line 98) |
| 19 | 0.217 | research/ic_next10_20261014/01-fp4-ladder/README.md | FOUND (line 115) |
| 20 | 105 | src/cryptanalysis/gaudry_quartic_d4.rs | FOUND (line 26) |
| 21 | 512 | src/cryptanalysis/gaudry_quartic_d4.rs | FOUND (line 24) |
| 22 | 8 | src/cryptanalysis/gaudry_quartic_d4.rs | FOUND (line 2) |
| 23 | 0.879·10⁶ | research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | FOUND (line 2197) |
| 24 | 507k | research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | NOT FOUND |
| 25 | 153k | research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | NOT FOUND |
| 26 | 354k | research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | NOT FOUND |
| 27 | 273k | research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | NOT FOUND |
| 28 | 58k | research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | NOT FOUND |
| 29 | 33k | research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | FOUND (line 348) |
| 30 | 0.904·10⁶ | research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | NOT FOUND |
| 31 | 1039 | research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | FOUND (line 1062) |
| 32 | 64 | research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | FOUND (line 61) |
| 33 | 5.1 | research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | FOUND (line 188) |
| 34 | 1.213·10¹² | research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | FOUND (line 2406) |
| 35 | 3.1 | research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | FOUND (line 158) |
| 36 | 495 | research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | FOUND (line 2359) |
| 37 | 9 | research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md | FOUND (line 7) |

## Totals
citations: 2 verified / 2 mismatch / 6 unreachable; paths: 8 exist / 0 missing;
numbers: 28 found / 9 not found.
