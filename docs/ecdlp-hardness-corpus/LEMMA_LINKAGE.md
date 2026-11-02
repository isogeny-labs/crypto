# Lemma linkage and the attack graph

**Question.** "Fake degree is small, so by Lemma 4 the first fall degree is small,
so the attack is cheap." That inference can make an index-calculus or summation-polynomial
attack look cheaper than it is. Which estimates in this repository rest on it, and how do
we find every inference like it before it reaches a cost row?

**Answer (2026-11-02).** One estimate rests on Lemma 4 directly: Nagao's own
`O(n^(8w+1))` (ePrint 2015/984, Prop. 5). No code path and no repository cost row does.
The wider exposure is the **first-fall-degree assumption** (FFDA): the conditional
ECC2K-130 projection of `2^86` and the `icx` binary heuristic both take a first-fall
degree as the solving degree, and both are already labelled conditional. Two primary
sources read for this pass show why that label has to stay: Kosters–Yeo prove first fall
degree 2 for descended `S_3` at every `n` while the measured degree of regularity climbs
from 3 to at least 5, and Caminata–Gorla prove the sound bridge
`sd_σ(F) = max{last fall degree, max GB degree}` (Thm 3.1), which the first fall degree
does not appear in.

## What changed in the corpus

The corpus already recorded claims and typed edges. It could not record that a claim is
**false**, how far a claim is **formalized**, or **which files** lean on it. This pass
adds that layer:

| addition | meaning | enforced by |
|---|---|---|
| status `refuted`, evidence class `counterexample` | a published claim shown false | a refuted node needs an incoming `refutes` edge |
| relation `refutes` | counterexample → claim | its target must be `refuted`; its source must be established |
| `formal: {state, artifacts}` | `kernel_checked`, `lean_written_unchecked`, `native_exact`, `stated_open`, `none` | artifacts must exist; `kernel_checked` Lean may not contain `sorry` or `native_decide` |
| `cited_by: [{path, use}]` | `depends`, `reports`, `measures`, `mention` | paths must exist |
| `citation_patterns` | regexes that recognise a citation of the claim | `attack-graph.mjs --check` fails on any matching file not in `cited_by` |
| taint rule | nothing reached from a refuted claim along `conditions`, `enables` or `model_bridge` may be `proved_*`, `algorithm` or `measured` | `validate.mjs` |

The fragment [`fall-degree-lemmas.json`](fall-degree-lemmas.json) adds 14 claims, 18 edges,
8 sources and 3 research targets: the four degree definitions, Lemma 4 (refuted), the
witness, the Caminata–Gorla, Kosters–Yeo and HKYY theorems, Nagao's complexity claim,
the ECC2K-130 FFDA projection, and three open obligations.

Each gate was checked with a negative control: promote a dependent of Lemma 4 to
`proved_external` (taint error), drop one `cited_by` entry (lint error naming the file),
refute a non-refuted node (edge error), and point `formal` at a missing file (artifact
error). All four fail as intended, and the corpus as committed passes.

## The attack graph

`node attack-graph.mjs` writes [`ATTACK_GRAPH.md`](ATTACK_GRAPH.md),
[`attack-graph.json`](attack-graph.json) and [`attack-graph.svg`](attack-graph.svg) /
[`.pdf`](attack-graph.pdf). For every claim it computes the weakest status among
the claim and everything it rests on, the refuted ancestors, and the claims that rest on
it. The **frontier** is every claim that is not established (refuted, open, heuristic,
assumed, conditional, unreviewed). It is ranked by impact: claims resting on it
transitively, plus files citing it as `depends`. Each frontier claim gets one action:
prove or refute, measure without censoring, review and kernel-check, or discharge
conditions.

Top of the current frontier (55 claims):

| claim | status | impact | why it matters |
|---|---|---|---|
| `COND-ECC2K130-FFDA-PROJECTION` | conditional | 8 | eight files quote the `2^86` projection, all resting on FFDA |
| `HEUR-FIRST-FALL-DEGREE` | heuristic | 4 | four attack claims rest on it, Nagao's included; Kosters–Yeo obstruct it |
| `CLAIM-NAGAO-LEMMA4` | **refuted** | 2 | Nagao's Prop. 5 justification rests on it |
| `ATTACK-MOV-PAIRING` | conditional | 3 | standard-curve exclusions rest on it |
| `FACT-S3-ROOTS-ARE-GROUP-LAW` | unreviewed | 3 | carries the generic-table bridge to Shoup and CGK |

## Lemma 4, precisely

- **Statement.** `d_F ≤ d'_F` over `F_2`.
- **Refuted under the equality reading, both placements.** Witness A `{X²Y, XY+X}` and
  witness B `{X₁X₂+X₃, X₁X₃+X₂}` have `d'_F = 2 < d_F = 3`
  ([`papers/ic-negative-results`](../../papers/ic-negative-results/README.md), S5).
- **Not refuted under the printed `≥` reading with the field equations inside.** There
  witness A gives True = Fake = 2
  ([`fall-degree.tex`](../../papers/ecdlp-boundaries-20261018/sections/fall-degree.tex)).
  The corpus records the reading as a parameter of the definition nodes.
- **Formalized.** [`Lemma4Witness.lean`](../../papers/ic-negative-results/lean/IcNegative/Lemma4Witness.lean)
  states witness A in core Lean and checks every admissible multiplier tuple with
  `decide +kernel`. It is **written, not yet kernel-checked**: no Lean toolchain on the
  authoring host, so the repository proof gate is the first check. A Python mirror of the
  same functions agrees on every statement and on the positive control `X, X+1`. The
  two encoding bridges (mask completeness, reduction as a ring homomorphism) are argued
  in the module docstring, not proved.
- **What survives.** Per instance, a fake-degree certificate whose products already
  have unreduced degree `≤ d'_F` still certifies a true fall. Nagao's Propositions 2
  and 3 exhibit such certificates. Whether Prop. 5 survives is `GAP-NAGAO-PROP5-ROUTE`.

## Open obligations to prove or refute

| target | obligation | first step |
|---|---|---|
| `RT-LEMMA4-FORMAL` | witness A through the proof gate; then the bridges and witness B | read the gate result on this PR |
| `RT-LEMMA4-REVERSE` | the candidate repair `d'_F ≤ d_F` with field equations **outside** (0 counterexamples in the 7875-pair census; the inside form already fails in 224 pairs) | wider frozen census (4 variables, degree 3, 3 generators) |
| `RT-FALL-DEGREE-BRIDGE` | every degree-based cost goes through a measured solving degree or a proved `max{last fall, GB degree}` bound | `BinaryIcHeuristic::log2_cost` in `src/cryptanalysis/matmul_exponent.rs` |
| `GAP-NAGAO-PROP5-ROUTE` | does Prop. 5 hold without Lemma 4? | PDF-level transcription of Defs. 5–6, Lemma 4, Prop. 5 |

## Sources outside the corpus that should be linked next

These graphs and registries hold lemmas with dependencies, but are not yet in this corpus:

- `cryptanalysis:experiments/ic-assumption-graph/` (DOT assumption graph, 2026-10-06; it
  already calls FFDA "refuted as stated").
- `research/guarded_fall_degree_20261028/formal/` and `research/fall_degree_bounds_20261008/lean/`
  (Lean fall-degree developments with their own exact-pair claim files).
- `papers/ic-negative-results/lean/OPEN_TARGETS.md` (O1, O2, O4 as `stated_open` nodes).
- `crypto-autoresearcher:knowledge/findings/KN-FIND-*` (81 files here cite them; the
  sibling repository is not checked out in this session).
- About 60 other Lean projects under `research/` and `papers/`. Each proved theorem that
  a corpus claim relies on should get a `formal` record. Today 3 of 115 claims have one.

## Literature read for this pass

- Kosters, Yeo, *Notes on summation polynomials*, arXiv:1503.08001 (sha256 `b5ea5e4c…`).
- Huang, Kosters, Yang, Yeo, *On the last fall degree of zero-dimensional Weil descent
  systems*, arXiv:1505.02532 (sha256 `cffdbb88…`).
- Caminata, Gorla, *Solving degree, last fall degree, and related invariants*,
  arXiv:2112.05579 (sha256 `9e9323db…`).
- Nagao, ePrint 2015/984: abstract page only; the PDF was behind a bot check from
  this host (`metadata_only`).

## Rebuild

```sh
node docs/ecdlp-hardness-corpus/validate.mjs
node docs/ecdlp-hardness-corpus/render.mjs
node docs/ecdlp-hardness-corpus/attack-graph.mjs --check
dot -Tsvg docs/ecdlp-hardness-corpus/attack-graph.dot -o docs/ecdlp-hardness-corpus/attack-graph.svg
dot -Tpdf docs/ecdlp-hardness-corpus/attack-graph.dot -o docs/ecdlp-hardness-corpus/attack-graph.pdf
```

`LEMMA_LINKAGE.pdf` is this file plus the attack-graph figure, converted by pandoc to
standalone HTML and printed to A4 with headless Chromium (the authoring host had no TeX).

CI runs the first three in `.github/workflows/claim-corpus.yml`.
