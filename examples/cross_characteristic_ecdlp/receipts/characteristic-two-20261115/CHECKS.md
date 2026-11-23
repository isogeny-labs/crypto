# Characteristic-two validation record

This addition addresses the user's request for an even-characteristic target.
The target is characteristic 2, with the concrete field `GaloisField 2 2`.
No source code, old verification receipt, old claim file or managed artifact
manifest from the F5-to-F7 example is replaced.

## Commands and raw outputs

Run from `examples/cross_characteristic_ecdlp`, with
`/home/agent/.elan/bin` on `PATH`:

```sh
lake env lean /workspace/scratch/BinaryAssemblyProbe.lean
lake build CrossCharacteristicTwo
lake build
lake env lean AxiomAudit.lean
```

The assembly source is retained as `assembly-probe.txt` and its empty output
as `binary-assembly-probe.log` (exit 0). The first two concrete builds failed:
`binary-build-01.log` records remaining characteristic-two numeral obligations;
`binary-build-02.log` records the documentation-comment placement error.
`binary-build-03.log` records the successful full build (exit 0).
The explicit audit output is `binary-axiom-audit.log` (exit 0): the printed
results depend only on `propext`, `Classical.choice` and `Quot.sound`.

After staging the new module for the gate's tracked-source inventory, run
from the repository root:

```sh
PATH=/home/agent/.elan/bin:$PATH USE_FRO_CACHE=1 \
  tools/lean/proof-gate.sh \
  --out examples/cross_characteristic_ecdlp/receipts/characteristic-two-20261115/gate \
  examples/cross_characteristic_ecdlp
```

Its unchanged checker, cache, build, module and transitive-axiom outputs are
retained under `gate/`. Dependency and source hashes are in `source-pins.json`.
The gate completed with exit 0: `PASS 50 thm 0 trusted`, zero failing theorems.
No native evaluation exception, custom axiom or resource-limit override is added.
These are local checks, not a signed independent PCC verdict. [pcc:unverified]

The new editable SVG was rendered to a 300 dpi PNG using the installed Node
`sharp` package, then embedded in `binary-report.html` and rendered with:

```sh
pandoc binary-report.html --from=html --pdf-engine=xelatex \
  -V 'mainfont=DejaVu Sans' -V papersize=a4 -V geometry:margin=17mm \
  -o binary-report.pdf
pdfinfo binary-report.pdf
pdftoppm -scale-to 1500 -png binary-report.pdf /workspace/scratch/binary-report
```

The renderer exited 0. `pdfinfo` reports one A4 page, and that page was opened
for visual inspection: all table labels, theorem names and sources are legible.
The original SVG, PDF and artifact-store mapping remain unchanged.

## Scope and concurrent work

The source curve is unchanged. Only matching order-three subgroups are
identified; no arbitrary-curve transfer algorithm or efficiency result is
claimed. No benchmark or performance graph needs a new value.

The existing PR independently advanced to merge commit
`5e2b86ebdd65b223eec365cdfa3cac7116196ac5` while this addition was prepared.
The example files and managed artifact manifest were unchanged by that merge.
A new conflict with newer main is confined to the shared daily journal in the
read-only merge-tree inspection. Request comment `6900616661` asks Copilot to
resolve that conflict; the journal is not edited while the request is pending.

Reviewing that pre-existing merge against both parents also found that it moved
the old example's ledger event from index 1087 to index 1088 and changed its
chain hash. The semantic event and original claim file remain present, and
the main ledger prefix is intact; the original source ledger hash prefix is
not intact. `inherited-merge-review.json` records exact parent and row hashes.
This addition does not repair, erase or endorse that historical rewrite.
The issue is reported for review rather than choosing a frozen history.

The journal review also records full normalized blocks that differ from each
parent; this is a byte-preservation finding, not an inference that every such
block's semantic content is missing. Restoring full parent-block multiplicities
is deferred to the requested resolver. The normal four-part journal entry is
retained here until the shared file is safe to edit:

### What was done (commands, files, features)

- Added the characteristic-two module, a second Lake default target, explicit
  audit, source-linked report, diagram, PDF and fresh validation receipts.
- Retained both failed builds and the final successful checks. Added the new
  declaration-bound claim without editing any existing claim or ledger row.

### Decisions made (and why)

- Use the nonsingular binary model and a genuine four-element field. Reuse
  the existing Mathlib group constructor and original source obligations.
- Report the inherited frozen-ledger discrepancy. Keep auto-merge paused until
  that review is resolved; no gate is bypassed and no history is chosen.

### Open threads (unfinished, pick up next time)

- Copilot request `6900616661`, complete parent-history preservation, required
  current-head hosted checks, independent PCC standing and merge.
- Append this entry to the canonical daily journal after its conflict is safe
  to resolve. Existing scheduled PR check-in remains responsible for follow-up.

### Errors (what went wrong, how it was handled)

- The two development errors and their fixes are retained above. No proof
  obligation was weakened and no resource limit was raised.
- The independent merge's source-ledger prefix mismatch is unresolved and is
  reported explicitly; this addition does not rewrite frozen evidence.
