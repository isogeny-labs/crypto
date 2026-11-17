# Source audit — 2026-10-19

Primary publications were read via the web; local files were inspected at
`555eb93adc0d097aa9c2a702effa86110084d927`. Search scope is bounded, not an
exhaustive survey or proof that no newer algorithm exists.

| Primary source | Portion inspected | Use and limit |
| --- | --- | --- |
| [Tian, Cover attacks for elliptic curves with prime order, arXiv:2012.07173v1](https://arxiv.org/html/2012.07173) | Introduction, construction outline, section 3.4, conclusion | Established cubic Weil-restriction odd-isogeny mechanism; existence on selected inputs is not universal coverage or a local measured speedup |
| [Tian, Translating the DLP on genus-3 Jacobians, arXiv:2007.03172](https://arxiv.org/html/2007.03172) | Construction outline and section 6 fixture | Established odd-torsion hyperelliptic-to-quartic route; no claim that all Smith-negative inputs work |
| [Smith, Isogenies and the DLP in genus 3, arXiv:0806.2995](https://arxiv.org/abs/0806.2995) | Abstract | Two-torsion comparator mechanism only |
| [Petit, Kosters, Messeng, PKC 2016 author slides](https://www.iacr.org/workshops/pkc2016/troll.iis.sinica.edu.tw/pkc16/slides/6-1--Algebraic_approaches_for_the_Elliptic_Curve_Discrete_Logarithm_Problem_over_Prime_Fields.pdf) | Presentation text | PKM background; mixed-fiber proposal is our untested design |
| [Chung, Costello, Smith, arXiv:1510.03174](https://arxiv.org/abs/1510.03174) | Abstract | Kummer-to-Jacobian recovery exists in specified settings; applicability and arbitrary addition still need checking |
| [Complete group law for genus 2 Jacobians on Jacobian coordinates, DOI 10.1007/s13389-024-00351-7](https://link.springer.com/article/10.1007/s13389-024-00351-7) | Publisher article overview | Candidate arithmetic reference; no formula port or validation executed here |

Local audit: `ic_leads_review_20261018`, `three_leads_e2e_20261018`,
`ghs_cubic_quartic_ic_20261012`, `ic_improve4_20261015/01-genus3-relations`,
`pkm_supersingular_aux_20261015`, `pkm_m3_n21_msolve_20261018`,
`ic_next10_20261014/04-kummer-fourier`, the problem index, and the tracked
backlog list. Exact-term searches included `2012.07173`, `normal Weil`,
`cofactor two`, and `quasi-subfield` in tracked research Markdown, sources
and examples. Tian's route is already cited in the state-of-the-art and
weak-curve audits; this report expressly does not claim to discover it.

Web searches included the exact paper titles above, PKM author/title queries,
quasi-subfield polynomial papers, and ECDLP/index-calculus queries restricted
to 2025/2026 ePrints. Search-result recency is not publication recency. Search
hits were used for discovery only; third-party aggregators and unverified
recent algorithm claims do not support this report's conclusions.
