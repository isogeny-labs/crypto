# Sources checked and prior-work boundary

The repository references and current results were reviewed before integrating
or extending this study. The proof claims below are session derivations;
literature priority has not been established.

| Primary or repository source | Material checked | Consequence for this work |
|:--|:--|:--|
| Huang, Kosters, Yang, Yeo, [arXiv:1505.02532](https://arxiv.org/html/1505.02532), Definitions 2.1–2.2 and Section 6 | Ordinary bounded-degree closure; restrictions on applying Weil-descent bounds | Fixes `V_(F,D)` and prevents an automatic transfer from coefficient-field bounds to descended systems |
| Caminata, Gorla, [arXiv:2112.05579v2](https://arxiv.org/html/2112.05579v2), Sections 1–4 | Solving degree, last fall degree, and modified syzygy first fall | First fall is presentation- and convention-sensitive; no first-fall heuristic is used as an upper bound |
| Semaev, [arXiv:1504.01175](https://arxiv.org/html/1504.01175), Section 2 | The explicit odd and ordinary-binary third summation polynomials | Supplies the two model-specific polynomial formulas; the paper's complexity heuristic is not imported as a theorem |
| Kosters, Yeo, [arXiv:1503.08001](https://arxiv.org/html/1503.08001), Section 2/Proposition 2.1 and Section 4.2 | Summation-polynomial point interpretation and the binary trace relation | Rational-lift assumptions are explicit; trace-derived Weil-descent falls are a separate phenomenon |
| [SEC 2, version 2.0](https://www.secg.org/sec2-v2.pdf), Sections 2.4–2.6 and 3.1–3.7 | Official large-prime parameters, generators, ordinary binary models and field sizes | The larger-curve applications satisfy the existing model; public small-multiple witnesses avoid depending on a new group-order certification |
| [Existing ledger](../notes/index-calculus/RESEARCH_FALL_DEGREE_BOUNDS.md), Sections 0–2 | Definitions, first/last-fall distinction, prior bounds | This study uses explicit membership cardinality, not the ledger's Boolean variable count |
| [Current E12 report](../fall_degree_bounds_20261008/e12/completed-20261028/README.md) | Four retained labels and mechanical scoring limits | The mixed E12 result and independent closure-check gap remain unchanged |

The new argument's central steps are staged normalization of quadratic
families, staged extraction of a scalar resultant, and inversion in a finite
grid quotient. The growing-base extension additionally uses a triple-selector
partition of unity. The written proofs supply degree accounting; Lean checks
only the declarations listed in `formal/PROOF_MAP.json`.

Relevant sources were accessible. No claim that an unsuccessful search proves
novelty is made, and no experimental timing is used as a mathematical premise.
