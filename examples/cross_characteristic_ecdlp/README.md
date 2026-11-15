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

## Transfer into characteristic two

An even-characteristic field has characteristic 2; its finite cardinality is
`2^m`. The additional module [CrossCharacteristicTwo.lean](CrossCharacteristicTwo.lean)
uses the nonsingular model `y² + y = x³` over any characteristic-two field `K`.
It specializes to `F_4 = GaloisField 2 2`, with its cardinality checked in Lean.
The source curve and points over `F_5` are imported unchanged. [pcc:unverified]

| Scalar modulo 3 | Source: `y² = x³ + 1` over `F_5` | Target: `y² + y = x³` over `F_4` |
| --- | --- | --- |
| 0 | `O` | `O` |
| 1 | `P = (0,1)` | `R = (0,0)` |
| 2 | `Q = (0,4)` | `S = (0,1)` |

Both generators have order 3. For every integer `k`, the module checks
`S = [k]R ↔ Q = [k]P ↔ 2 ≡ k (mod 3)`. The binary points already lie in
the prime subfield and remain points over the genuine four-element field.
`ZMod 4` is not used as a field. [pcc:unverified]

![Order-three transfer into characteristic two](binary-transfer.svg)

Read the [binary report](binary-report.html), its [PDF](binary-report.pdf),
and the [versioned binary receipts](receipts/characteristic-two-20261115/).
The declaration-bound claim is `PCC-20261115-6a7830`; its local kernel check
is separate from the repository's unsigned PCC status. [pcc:unverified]

For a different source and binary target, the cyclic-subgroup constructor
requires equal generator orders. A chosen target curve need not contain a
subgroup of the required order. This example supplies those obligations for
order 3; it supplies no general efficient odd-to-binary point evaluator.
The earlier short equation must not simply be reduced modulo 2: it is singular
there, which is why the binary target has a different Weierstrass model.

![Complete subgroup transfer](transfer.svg)

## Read and reproduce

- [Lean source](CrossCharacteristicExample.lean): literal curve and point
  definitions, instance obligations, constructed map and scalar endpoints.
- [Binary Lean source](CrossCharacteristicTwo.lean): characteristic-two model,
  order-three obligations, field-cardinality check and scalar endpoint.
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

The repository's artifact workflow manages PNGs through the local
`.artifacts.json`; editable SVGs and report PDFs remain in Git. Before
regenerating a report, hydrate the PNGs from the repository root:

```sh
python3 tools/artifacts/artifacts.py pull examples/cross_characteristic_ecdlp
```

Then, from this example directory, render with Pandoc and XeLaTeX (the PNG is
a 300 dpi render of `transfer.svg`):

```sh
pandoc report.html --from=html --pdf-engine=xelatex \
  -V mainfont="DejaVu Sans" -V papersize=a4 -V geometry:margin=17mm \
  -o report.pdf

pandoc binary-report.html --from=html --pdf-engine=xelatex \
  -V mainfont="DejaVu Sans" -V papersize=a4 -V geometry:margin=17mm \
  -o binary-report.pdf
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
