# ECDLP hardness claim corpus

This directory is a machine-validated graph of what supports elliptic-curve
discrete-log security: definitions, assumptions, reductions, algorithms, model lower
bounds, conditional attacks, heuristics, measurements, known obstructions, and open
research obligations.

It is deliberately not a flat knowledge base. Every claim states its model and scope,
and every edge says how one claim depends on or limits another.

## Current snapshot

- 115 claim nodes (one refuted)
- 163 typed dependency edges
- 58 source records
- 16 prioritized research targets
- 717 labeled results and 375 numbered sections in the Galbraith book inventory

Run `node validate.mjs` for the authoritative assembled counts; generated counts are in
`stats.tsv`.

## Start here

- [`REPORT.md`](REPORT.md): findings, theorem table, attack surface, and research plan
- [`LEMMA_LINKAGE.md`](LEMMA_LINKAGE.md): the proof-linkage layer (refutations,
  formalization state, repository citations, taint) and the Nagao Lemma 4 audit
- [`ATTACK_GRAPH.md`](ATTACK_GRAPH.md), [`attack-graph.svg`](attack-graph.svg): every
  unestablished claim ranked by what rests on it, with the action that would settle it
- [`fall-degree-lemmas.json`](fall-degree-lemmas.json): fall-degree definitions, Lemma 4,
  its counterexample, the Caminata–Gorla bridge, and the FFDA-conditioned estimates
- [`overview.svg`](overview.svg): seven-layer overview
- [`graph.svg`](graph.svg): full claim-level dependency graph
- [`corpus.json`](corpus.json): curated backbone and fragment manifest
- [`corpus-extension.json`](corpus-extension.json): deep foundations, index-calculus,
  xedni, protocol-boundary, and quantum-resource expansion
- [`galbraith-promotions.json`](galbraith-promotions.json): missing generalized-DLP,
  rigorous-walk, CDH-reduction, isogeny, and pairing dependencies promoted from the book
- [`galbraith-2018-inventory.json`](galbraith-2018-inventory.json): all distinct numbered
  definitions, theorems, lemmas, corollaries, conjectures, heuristics, and algorithms in
  the supplied 696-page PDF, with page provenance
- [`GALBRAITH_BOOK_AUDIT.md`](GALBRAITH_BOOK_AUDIT.md): chapter counts, extraction
  reconciliation, and book-result-to-graph promotion ledger
- [`schema.json`](schema.json): field and vocabulary contract
- [`claims.tsv`](claims.tsv): flat claim index
- [`sources.tsv`](sources.tsv): source status, location, and primary-text hashes
- [`LITERATURE_SEARCH.md`](LITERATURE_SEARCH.md): search procedure, coverage, and
  inaccessible-source ledger
- [`GENERIC_INTERFACE_AUDIT.md`](GENERIC_INTERFACE_AUDIT.md): first active research
  result, classifying every live decomposition oracle against generic interfaces

## Reading the graph

The core claim classes are:

| Class | Meaning |
|---|---|
| `model_lower_bound` | rigorous lower bound inside a declared oracle/model |
| `algorithmic_upper_bound` | an algorithm or resource upper estimate |
| `conditional_attack` | faster route only on a family or under additional input/heuristics |
| `heuristic` | random-walk, yield, rank, degree, or solver behavior not proved at scale |
| `measured_bound` | repository measurement on a declared finite domain |
| `assumption` | security statement that remains unproved in the unrestricted model |
| `open_obligation` | precise missing theorem, experiment, or model bridge |

Reduction direction is fixed in `corpus.json`: `A reduces_to B` means an algorithm for
`B` solves `A`. Thus `CDH reduces_to DLP`; DLP hardness alone does not imply CDH
hardness.

## Central conclusion

For well-screened named classical curves, ECDLP hardness is still an assumption.
Shoup and CGK rigorously rule out broad *generic* algorithm/preprocessing classes at
sub-birthday cost, but do not rule out coordinate algebra, polynomial systems,
pairings, special-curve transfers, or augmented cheap-automorphism interfaces.

The active attack surface is therefore not “break Shoup.” It is to construct and cost a
representation-specific algorithm that escapes the generic interface while satisfying
every end-to-end obligation: yield, solver degree, relation rank, linear algebra,
descent, memory, setup, and verification.

## Index calculus versus xedni calculus

The corpus treats these as different branches:

- **Index calculus** uses a factor base, relation decomposition, sparse linear algebra,
  and individual logarithm/descent. Semaev/Gaudry/Diem/PKM work belongs here.
- **Xedni calculus** (“index” reversed) lifts finite-field points to a curve over the
  rationals and searches for a Mordell–Weil dependence. JKSST gives a conditional
  canonical-height obstruction. A kernel-checked audit proves its finite counting,
  scalar-extraction, event-probability, cofactor, and conditional waiting-time pieces,
  while exposing the selected-lift and adaptive-trial bridges still needed for an
  end-to-end theorem. A separate frozen function-field family has exact lift density
  `Theta(p^-3)`.

## Rebuild and validate

From the repository root:

```sh
node docs/ecdlp-hardness-corpus/validate.mjs
node docs/ecdlp-hardness-corpus/render.mjs
node docs/ecdlp-hardness-corpus/attack-graph.mjs --check
dot -Tsvg docs/ecdlp-hardness-corpus/attack-graph.dot \
  -o docs/ecdlp-hardness-corpus/attack-graph.svg
dot -Tpdf docs/ecdlp-hardness-corpus/attack-graph.dot \
  -o docs/ecdlp-hardness-corpus/attack-graph.pdf
dot -Tsvg docs/ecdlp-hardness-corpus/overview.dot \
  -o docs/ecdlp-hardness-corpus/overview.svg
dot -Tpdf docs/ecdlp-hardness-corpus/overview.dot \
  -o docs/ecdlp-hardness-corpus/overview.pdf
dot -Tsvg docs/ecdlp-hardness-corpus/graph.dot \
  -o docs/ecdlp-hardness-corpus/graph.svg
(cd docs/ecdlp-hardness-corpus && \
  pandoc REPORT.md -o REPORT.pdf --pdf-engine=xelatex)
```

To reproduce the book inventory, pass the matching PDF (SHA-256 is checked):

```sh
node docs/ecdlp-hardness-corpus/extract-galbraith-inventory.mjs /path/to/book.pdf
```

`render.mjs` deterministically regenerates the DOT and TSV artifacts. Graphviz renders
the SVGs. The subshell for `REPORT.pdf` is intentional: `REPORT.md` embeds
`overview.pdf` by a path relative to the corpus directory.

## Evidence discipline

- `primary_read` means the primary text was read and its SHA-256 is recorded.
- `metadata_only` means identity/citation was checked but the text was unavailable.
- `secondary_only` means scope was cross-checked through a survey/repository audit.
- `repository_read` means the committed artifact was inspected; internal proofs remain
  internal unless independently reviewed.
- `ecbench.bound/v1` records are measured upper bounds on methods, never mathematical
  ECDLP lower bounds.

The validator requires a primary-read source for every node marked
`proved_external`, checks repository source paths, rejects dangling edges, and validates
research-target dependencies. It also enforces the proof-linkage layer: a `refuted`
claim needs a `refutes` edge from an established node, nothing resting on a refuted
claim may be marked proved, algorithm or measured, and `formal` and `cited_by` paths
must exist. `attack-graph.mjs --check` fails when a file matches a claim's
`citation_patterns` without being listed in its `cited_by`. CI:
`.github/workflows/claim-corpus.yml`. It also checks the fixed book digest, all inventory counts,
page ranges and label uniqueness, the complete Algorithm 1--29 sequence, and every book
locator cited by a promoted claim.
