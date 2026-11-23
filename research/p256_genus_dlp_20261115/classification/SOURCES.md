# Source audit

Primary texts read on 2026-11-15. External documents stay outside the repository;
these SHA-256 digests identify the downloaded bytes.

| Source | Scope read | SHA-256 |
|---|---|---|
| Smith, arXiv:0806.2995 PDF | Sections 3–7, pair kernels, trigonal maps, rationality, canonical quartic | `44b809946008b6c5d1e9dcadb54b6103fa46ab56a13c0ebeadf9a191b2763140` |
| Stacks Project, curves.tex | Curves/function fields, Riemann–Hurwitz, smooth inseparable factorization | `115fb62d9803e8deaa9c40de4cfe9f953b20b6463f8c67633329d4e7ea0a68c3` |
| Stacks Project, pione.tex | Fundamental-group theorem and finite etale covers | `947055995cd971d8c9886cbec72e90d6ec352b693eae35588e9522b74f80cdbc` |

URLs are in FRAMEWORK.md. Exact Algebraic Curves source labels read:

- `theorem-curves-rational-maps`
- `section-riemann-hurewitz`
- `lemma-purely-inseparable-smooth`
- `lemma-purely-inseparable-smooth-genus`
- `proposition-unwind-morphism-smooth`

The fundamental-group source label is `theorem-fundamental-group`.
Source text hashes identify
what was read; they are not independent verification of the new derivations.

The repository sources reviewed are the three Smith modules, the higher-genus
reference, and the prior P-256 native implementation. No novelty claim is made.
The source hashes used for execution are recorded separately in sources.sha256.
The KB is unavailable (no MCP tools, wrapper exit 127); this is a targeted
primary-source and repository review, not an exhaustive literature survey.

## Formalization map

- All-cover representation and Frobenius factorization: imported primary
  theory from the Stacks Project, with exact labels in FRAMEWORK.md.
- Hyperelliptic rational-function representation: written proof using the
  hyperelliptic involution, no genus-zero map to E, and unique halving in the
  odd-order group E(Fp). Concrete Lean curve/Picard definitions are not supplied.
- Quadratic-cover classification: written line-bundle/square-class argument;
  its specialization to P-256 uses odd rational group order. No new compiled
  formal proof is claimed.
- Smith degeneracy: exact prerequisite outputs plus written polynomial-degree
  arguments. `lean/DegenerateRows.lean` states a row-decomposition lemma only.
  It is source-only because Lean/Lake/Mathlib are unavailable locally.
- Native correctness: new code compared with unchanged existing word-size
  Smith implementation and its published positive control, within declared
  input bounds. Both remain producer checks without signed execution standing.

No abstract group lemma is presented as a formal construction of the concrete
curves, divisor groups, normalizations or Smith correspondence.
