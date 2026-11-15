# Worked Lean example: cross-characteristic subgroup transfer

The curve `y² = x³ + 1` over `F_5` and over `F_7` has the order-three
generators `P = (0,1)` and `R = (0,1)`. This example uses Mathlib's existing
`addEquivOfAddOrderOfEq` constructor to obtain the subgroup isomorphism
`Φ([k]P) = [k]R`. [pcc:unverified]

| Scalar modulo 3 | Source over `F_5` | Target over `F_7` |
| --- | --- | --- |
| 0 | `O` | `O` |
| 1 | `P = (0,1)` | `R = (0,1)` |
| 2 | `Q = (0,4)` | `S = (0,6)` |

The module checks the complete table and its exhaustion of the source subgroup.
For every integer `k`, `S = [k]R ↔ Q = [k]P ↔ 2 ≡ k (mod 3)`.
It also checks that no unital field map `F_5 → F_7` exists. [pcc:unverified]

![Complete subgroup transfer](transfer.svg)

## Read and reproduce

- [Lean source](CrossCharacteristicExample.lean): literal curve and point
  definitions, instance obligations, constructed map and scalar endpoints.
- [Proof map and inheritance table](PROOF-MAP.md): exact hypotheses,
  Mathlib declarations, assembly probe and primary sources.
- [Report](report.html), [PDF](report.pdf) and editable [diagram](transfer.svg).
- [Gate receipts](receipts/) and [focused build output](build.log).

Lean is pinned to 4.24.0; mathlib is pinned to v4.24.0 and the exact
resolved revisions in `lake-manifest.json`. No new dependency is added to
the repository's Rust workspace.

```sh
cd examples/cross_characteristic_ecdlp
lake exe cache get
lake build
lake env lean AxiomAudit.lean
```

From the repository root, check every theorem with the existing gate:

```sh
tools/lean/proof-gate.sh examples/cross_characteristic_ecdlp
```

Mathlib's official alternate cache can be selected with `USE_FRO_CACHE=1`
if its default cache is unavailable. The raw receipts record the service
used in this environment. The gate audits transitive axioms, rather than
inferring proof status from a text scan.

The repository's artifact workflow manages `transfer.png` through the local
`.artifacts.json`; the editable SVG and report PDF remain in Git. Before
regenerating the report, hydrate the PNG from the repository root:

```sh
python3 tools/artifacts/artifacts.py pull examples/cross_characteristic_ecdlp
```

Then, from this example directory, render with Pandoc and XeLaTeX (the PNG is
a 300 dpi render of `transfer.svg`):

```sh
pandoc report.html --from=html --pdf-engine=xelatex \
  -V mainfont="DejaVu Sans" -V papersize=a4 -V geometry:margin=17mm \
  -o report.pdf
```

## Scope

This is an instance of an abstract cyclic-group isomorphism. The Mathlib
constructor uses classical choice of scalar representatives; it does not
supply an efficient evaluator for general encoded curve points. The
transfer is not an algebraic isogeny or a coefficient base change.

The existing problem record
[`prime-field-chained-transfer-no-advantage.md`](../../problems/prime-field-chained-transfer-no-advantage.md)
retains its separate efficient-map obligation. No benchmark, cost estimate,
curve-search result or performance-dashboard row changes here. Existing
performance and curve-graph panels remain unchanged for that reason.

Sources checked (KB unavailable): the runtime has no KB tools and
`scripts/research-kb status` exits 127 for an absent executable. The fallback
review used the existing problem record and the pinned Mathlib primary
sources listed in the proof map. No novelty claim is made.
