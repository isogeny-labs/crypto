# Larger-field validation record

Request: “Try bigger field.” The chosen fields have cardinalities 65537 and
`2^256`. The new subgroup order is 7. The target coefficient is outside `F_2`,
but its coefficients and selected points descend to `F_4`; increasing ambient
field size does not turn this into a large-subgroup ECDLP instance.

## Requirements and evidence

| Requirement | Evidence | Status |
| --- | --- | --- |
| Larger prime and binary fields | `source_card`, `target_card` | Local kernel check |
| Actual characteristic-two field | `GaloisField 2 256` | Existing Mathlib field/characteristic instances |
| New concrete subgroup | `P_order`, `R_order`, both equal 7 | Local kernel check |
| Coefficient outside the prime subfield | `omega_not_from_F2` | Local kernel check |
| Preserve and reflect the scalar | `transfer_dlog`, `example_dlog` | Local kernel check |
| Scalar residue for chosen challenge | `source_scalar_mod`, `target_scalar_mod` | Local kernel check, `3 ≡ k (mod 7)` |
| PR and source-linked report | New diagram, HTML and PDF; PR 4308 | Source ready; merge still has inherited blockers |

The Lean declaration prefix is `CrossCharacteristicLarger`. Claim
`PCC-20261115-53afbe` is unsigned; local validation is separate from independent
PCC standing. [pcc:unverified]

## Exact validation commands and attempts

With `/home/agent/.elan/bin` on `PATH`, from the example directory:

```sh
lake env lean /workspace/scratch/LargerAssemblyProbe.lean
lake build CrossCharacteristicLarger
```

The final assembly source is retained as `assembly-probe.txt`.
`larger-assembly-01.log` records the initial missing prime-number tactic
extension. Importing Mathlib's `Tactic.NormNum.Prime` resolves that obligation;
`larger-assembly-02.log` is the empty successful output, exit 0.

`larger-build-01.log` through `larger-build-04.log` preserve unsuccessful
development attempts. They expose literal branch conditions, characteristic-two
casts and field divisions. The fixes use existing named laws, including
`CharP.cast_eq_mod`, `div_eq_iff` and `div_self`; noncomputable inverses are not
evaluated by `decide`. `larger-build-05.log` records the successful build, exit 0.
No order hypothesis was weakened and no resource limit was raised.

After staging the source for the tracked-module inventory, from the root:

```sh
PATH=/home/agent/.elan/bin:$PATH USE_FRO_CACHE=1 \
  tools/lean/proof-gate.sh \
  --out examples/cross_characteristic_ecdlp/receipts/larger-20261115T2025Z/gate \
  examples/cross_characteristic_ecdlp
```

The unchanged gate exits 0: **PASS, 94 theorems, 0 failing, 0 trusted native
exceptions** across all three modules. `larger-proof-gate.log` and `gate/`
retain the raw cache, build, source and transitive-axiom outputs. Then, from
the example directory:

```sh
lake env lean AxiomAudit.lean
```

`larger-axiom-audit.log` records exit 0 and only `propext`, `Classical.choice`,
`Quot.sound`. Only one Lake process runs at a time. [pcc:unverified]

## Rendering

The SVG is editable source. Node `sharp` renders it to a PNG at 300 dpi.
The HTML embeds that PNG and is rendered with:

```sh
pandoc larger-report.html --from=html --pdf-engine=xelatex \
  -V 'mainfont=DejaVu Sans' -V papersize=a4 -V geometry:margin=17mm \
  -o larger-report.pdf
pdfinfo larger-report.pdf
pdftoppm -scale-to 1500 -png larger-report.pdf /workspace/scratch/larger-report
```

Sources and exact hashes are in `source-pins.json`. Original reports, managed
artifact entries, claim records and verification receipts are preserved.
No benchmark, performance-panel or curve-graph result changes.

The final renderer exits 0; `pdfinfo` reports one A4 page. That page was opened
and inspected after the final diagram edit: the field labels, coordinates,
subgroup size, scope statement, theorem names and source links are legible.

## Concurrent histories and journal entry

The authoring parent is `41a9816f5b72800944e025e1e77cd9c7a8fdd782`.
Fetched main is `5616d4c8825034ed82275d73cc86939a1a5a8391`. The current
addition appends one new ledger event; it preserves all bytes of its parent
ledger. Verification against `HEAD` succeeds at 1091 entries. Comparison with
incoming main fails at index 1088 because the histories diverged; the tool's
`BROKEN` label includes that prefix check, not just intrinsic chain validity.
Both raw checks and exit statuses are retained. [pcc:unverified]

Independent prior merge `5e2b86eb` also has the already reported source-parent
ledger rehash, documented in the preceding characteristic-two receipt.
No attempt is made to choose a frozen history or rewrite those events.
Copilot request `6900616661` remains pending for the shared journal. That file
is not edited while the resolver is pending; auto-merge remains paused for
the history review. The normal session entry is retained here for later append:

### What was done (commands, files, features)

- Added the larger-field module, default build target, axiom audit entries,
  proof map, new report and diagram, and versioned validation receipts.
- Checked the primitive-root witness, actual affine-point chains, exact
  order-seven obligations and scalar endpoint using existing Mathlib laws.
- Retained unsuccessful attempts and the successful 94-theorem gate.

### Decisions made (and why)

- Increase both ambient field sizes and subgroup order, while explicitly
  distinguishing `2^256` field elements from 7 subgroup elements and retaining
  the four-element subfield limitation. No efficient-map claim is made.
- Publish the example without rewriting inherited journal or ledger evidence.

### Open threads (unfinished, pick up next time)

- Shared-journal resolution, frozen-history review, required current-head
  hosted checks and merge. Existing scheduled PR monitoring continues.
- Append this complete entry to the canonical journal once safe; retain all
  parent blocks and multiplicities. Independent signed PCC standing is pending.

### Errors (what went wrong, how it was handled)

- The assembly and four development failures are retained above. Existing
  theorems and explicit arithmetic rewrites resolve them without trust or
  resource-limit changes.
- The incoming-main ledger prefix comparison fails and remains a reported
  history blocker. No old claim, ledger row or unrelated benchmark is changed.
