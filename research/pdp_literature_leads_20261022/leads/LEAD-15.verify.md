# Verification of LEAD-15 (mechanical tier)

Dossier: leads/LEAD-15.md. Procedure: BRIEF_HAIKU.md. Date: 2026-10-22.

## Citations
| # | citation as written | route checked (URL or query) | outcome | page shows |
|---|---|---|---|---|
| 1 | Diem, "Index Calculus in Class Groups of Plane Curves of Small Degree", ECC 2005 slides, cr.yp.to URL | WebFetch https://cr.yp.to/conferences/2005-ecc/www.cacr.math.uwaterloo.ca/conferences/2005/ecc2005/diem.pdf; WebSearch "Index Calculus in Class Groups of Plane Curves of Small Degree" | MISMATCH | WebFetch: HTTP 502 Bad Gateway. Top search result: arxiv.org/pdf/math/0606607, title "Index calculus with double large prime variation for curves of small genus with cyclic class group" |
| 2 | Diem, "An index calculus algorithm for plane curves of small degree", ANTS VII, LNCS 4076 (2006) 543–557, doi 10.1007/11792086_38 | WebFetch https://doi.org/10.1007/11792086_38: 302 to link.springer.com, then 303 to idp.springer.com login, not followed; WebFetch of the link.springer.com URL: same login redirect, not followed; WebSearch "An index calculus algorithm for plane curves of small degree" | MISMATCH | Top search result: eprint.iacr.org/2005/119, link text "Paper 2005/119"; search summary gives its title as "Index Calculus in Class Groups of Plane Curves of Small Degree" (2005). Other results include doi.org/10.1007/11792086_38 and dblp conf/ants/Diem06 (no titles shown) |
| 3 | Diem–Thomé, "Index calculus in class groups of non-hyperelliptic curves of genus three", J. Cryptology 21(4) (2008) 593–611, doi 10.1007/s00145-007-9014-6; hal.inria.fr/inria-00107290/document | WebFetch https://hal.inria.fr/inria-00107290/document; WebSearch "Index calculus in class groups of non-hyperelliptic curves of genus three" | MISMATCH | WebFetch: Anubis access-denied page, no title, authors or year shown. Top search result: hal-univ-tlse3.archives-ouvertes.fr/INRIA/inria-00107290v1, title "Index calculus for non-hyperelliptic curves of genus 3". Search summary: Diem and Thomé, "Index Calculus in Class Groups of Non-hyperelliptic Curves of Genus Three", J. Cryptology 21(4) (2008) 593–611 |
| 4 | Diem, "On the discrete logarithm problem for plane curves", JTNB 24(3) (2012) 639–667, doi 10.5802/jtnb.815; jtnb.centre-mersenne.org/item/10.5802/jtnb.815.pdf | WebFetch https://jtnb.centre-mersenne.org/item/10.5802/jtnb.815.pdf; WebSearch "On the discrete logarithm problem for plane curves" | MISMATCH | WebFetch: binary PDF (922.9KB), title page is an image, no title, authors or year in text. Top search result: arxiv.org/pdf/cs/0703032, title "An $L (1/3 + \epsilon)$ Algorithm for the Discrete Logarithm Problem for Low Degree Curves". Results 2 and 3 (numdam.org and jtnb.centre-mersenne.org articles/10.5802/jtnb.815) carry link text "On the discrete logarithm problem for plane curves" |
| 5 | Diem, "Index calculus with double large prime variation for curves of small genus with cyclic class group", arXiv math/0606607 | WebFetch https://arxiv.org/abs/math/0606607 | VERIFIED | Title "Index calculus with double large prime variation for curves of small genus with cyclic class group"; author Claus Diem; submitted 23 Jun 2006; arXiv math.NT math/0606607; no journal reference |
| 6 | Diem, Math. Comp. 80 (2011) 443–475 (preprint question in row 5) | WebFetch https://www.ams.org/mcom/2011-80-273/S0025-5718-2010-02281-1/S0025-5718-2010-02281-1.pdf; no WebSearch (no exact title in dossier) | UNREACHABLE | HTTP 403 Forbidden; response body not retrieved |
| 7 | Flon–Oyono–Ritzenthaler, "Fast addition on non-hyperelliptic genus 3 curves", CORR 2007-10, uwaterloo.ca .../2007_10.pdf | WebFetch https://uwaterloo.ca/combinatorics-and-optimization/sites/default/files/uploads/documents/2007_10.pdf; WebSearch "Fast addition on non-hyperelliptic genus 3 curves" | MISMATCH | WebFetch: raw compressed PDF (580.3KB), no title, authors or year readable. Top search result: eprint.iacr.org/2004/118, link text "Paper 2004/118"; search summary: Flon, Oyono and Ritzenthaler, 2004 ePrint 2004/118 (year 2004; dossier: CORR 2007-10). The uwaterloo 2007_10.pdf link is the second result, title not shown |
| 8 | Khuri-Makdisi, Math. Comp. 73 (2004) 333–357, arXiv math/0105182 | WebFetch https://arxiv.org/abs/math/0105182 | VERIFIED | Title "Linear algebra algorithms for divisors on an algebraic curve"; author Kamal Khuri-Makdisi; submitted 22 May 2001; journal ref Math. Comp. 73 (2004), no. 245, 333–357. Dossier gives no title for this entry |
| 9 | Khuri-Makdisi, Math. Comp. 76 (2007) 2213–2239, arXiv math/0409209 | WebFetch https://arxiv.org/abs/math/0409209 | VERIFIED | Title "Asymptotically fast group operations on Jacobians of general curves"; author Kamal Khuri-Makdisi; submitted 13 Sep 2004; journal ref Math. Comp. 76 (2007), 2213–2239 |
| 10 | Abu Salem–Khuri-Makdisi, LMS J. Comput. Math. 10 (2007) 307–328, arXiv math/0610121 | WebFetch https://arxiv.org/abs/math/0610121 | VERIFIED | Title "Fast Jacobian group operations for C_{3,4} curves over a large finite field"; authors Fatima K. Abu Salem and Kamal Khuri-Makdisi; submitted 3 Oct 2006; journal ref LMS J. Comput. Math. 10 (2007) 307-328 |
| 11 | Tian, "Cover attacks for elliptic curves with prime order", J. Cryptology 36 (2023), arXiv 2012.07173 | WebFetch https://arxiv.org/abs/2012.07173 | VERIFIED | Title "Cover attacks for elliptic curves with prime order"; author Song Tian; submitted 13 Dec 2020; journal ref Journal of Cryptology 36, 2023 |
| 12 | Hess, J. Symb. Comput. 33 (2002) 425–445 | none: no URL, identifier or exact title in dossier; no fetch or search run | UNREACHABLE | no route: dossier gives no URL or identifier |
| 13 | Hartshorne IV (cited as recalled) | none: no URL, identifier or exact title in dossier; no fetch or search run | UNREACHABLE | no route: dossier gives no URL or identifier |
| 14 | Oyono's thesis (not located) | none: no URL, identifier or exact title in dossier; no fetch or search run | UNREACHABLE | no route: dossier gives no URL or identifier |
| 15 | CGHW 2026/384 | WebFetch https://eprint.iacr.org/2026/384 (URL built from the dossier's ePrint id) | VERIFIED | Title "The Structured Generic-Group Model"; authors Henry Corrigan-Gibbs, Alexandra Henzinger, David J. Wu (initials match CGHW); year 2026; Cryptology ePrint Archive Paper 2026/384 |

## Repository paths
| # | path as written | tested as | outcome |
|---|---|---|---|
| 1 | /home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md (§9) | /home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md | EXISTS |
| 2 | /home/user/crypto/problems/plane-quartic-jacobian-class-arithmetic-missing.md (§9) | /home/user/crypto/problems/plane-quartic-jacobian-class-arithmetic-missing.md | EXISTS |
| 3 | /home/user/ml-cryptanalysis/experiments/smith-codomain/report.md (§3, §9) | /home/user/ml-cryptanalysis/experiments/smith-codomain/report.md | EXISTS |
| 4 | /home/user/crypto/research/ghs_cubic_quartic_ic_20261012/quartic_ic.rs (§9 brace; line range dropped) | /home/user/crypto/research/ghs_cubic_quartic_ic_20261012/quartic_ic.rs | EXISTS |
| 5 | /home/user/crypto/AGENTS.md (§9) | /home/user/crypto/AGENTS.md | EXISTS |
| 6 | research/ghs_cubic_quartic_ic_20261012/README.md (§3) | /home/user/crypto/research/ghs_cubic_quartic_ic_20261012/README.md | EXISTS |
| 7 | research/ghs_cubic_quartic_ic_20261012/PROTOCOL.md (§3 as PROTOCOL.md; §9 brace) | /home/user/crypto/research/ghs_cubic_quartic_ic_20261012/PROTOCOL.md | EXISTS |
| 8 | research/ic_next10_20261014/07-cubic-ghs-genus3/README.md (§3) | /home/user/crypto/research/ic_next10_20261014/07-cubic-ghs-genus3/README.md | EXISTS |
| 9 | research/ic_next10_20261014/07-cubic-ghs-genus3/PROTOCOL.md (§3 as PROTOCOL.md; §9 brace) | /home/user/crypto/research/ic_next10_20261014/07-cubic-ghs-genus3/PROTOCOL.md | EXISTS |
| 10 | research/ghs_cubic_hyperelliptic_genus3_20261013/README.md (§3, §9) | /home/user/crypto/research/ghs_cubic_hyperelliptic_genus3_20261013/README.md | EXISTS |
| 11 | research/ghs_cubic_cover_census_20261012/README.md (§3, §9) | /home/user/crypto/research/ghs_cubic_cover_census_20261012/README.md | EXISTS |
| 12 | research/ghs_cubic_genus3_build_20261012/README.md (§3; §9 with line range dropped) | /home/user/crypto/research/ghs_cubic_genus3_build_20261012/README.md | EXISTS |
| 13 | src/cryptanalysis/hyperelliptic_index_calculus.rs (§3; §9 absolute) | /home/user/crypto/src/cryptanalysis/hyperelliptic_index_calculus.rs | EXISTS |

## Quoted numbers
| # | number | file | grep outcome (line) |
|---|---|---|---|
| 1 | 90/90 | research/ghs_cubic_quartic_ic_20261012/README.md | FOUND (24) |
| 2 | 1.300 | research/ic_next10_20261014/07-cubic-ghs-genus3/README.md | FOUND (23) |
| 3 | 1.61e8 | research/ic_next10_20261014/07-cubic-ghs-genus3/README.md | FOUND (72) |
| 4 | 2.60e8 | research/ic_next10_20261014/07-cubic-ghs-genus3/README.md | FOUND (72) |
| 5 | 74.5 | research/ic_next10_20261014/07-cubic-ghs-genus3/README.md | FOUND (86) |
| 6 | 48% | research/ic_next10_20261014/07-cubic-ghs-genus3/README.md | FOUND (91) |
| 7 | 17 (LA share) | research/ic_next10_20261014/07-cubic-ghs-genus3/README.md | FOUND (39, 65, 91) |
| 8 | M_ℓ = 9 | research/ic_next10_20261014/07-cubic-ghs-genus3/README.md; PROTOCOL.md also grepped | NOT FOUND |
| 9 | N1 = q+1 | research/ghs_cubic_genus3_build_20261012/README.md | NOT FOUND |
| 10 | N2 = q²+1 | research/ghs_cubic_genus3_build_20261012/README.md | NOT FOUND |
| 11 | L(T) = 1 − t_E T³ + q³T⁶ | research/ghs_cubic_genus3_build_20261012/README.md | NOT FOUND |
| 12 | ker φ ⊂ E0[2] | research/ghs_cubic_genus3_build_20261012/README.md | FOUND (66) |
| 13 | 0.00496 | /home/user/ml-cryptanalysis/experiments/smith-codomain/report.md | FOUND (117) |
| 14 | 0.00499 | /home/user/ml-cryptanalysis/experiments/smith-codomain/report.md | FOUND (117) |
| 15 | f = 0.1 | /home/user/ml-cryptanalysis/experiments/smith-codomain/report.md | NOT FOUND |
| 16 | deg-3 | /home/user/crypto/problems/plane-quartic-jacobian-class-arithmetic-missing.md | NOT FOUND |
| 17 | deg-6 | /home/user/crypto/problems/plane-quartic-jacobian-class-arithmetic-missing.md | FOUND (19, 20, 30) |

## Totals
citations: 6 verified / 5 mismatch / 4 unreachable; paths: 13 exist / 0 missing;
numbers: 11 found / 6 not found.
