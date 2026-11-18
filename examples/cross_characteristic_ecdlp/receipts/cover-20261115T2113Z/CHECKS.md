# Concrete cubic cover to a four-element target

The requested deliverable is a concrete cover and a weaker target over a
smaller field. The source field has 65,537 elements; the target field has four.
The transported subgroup has order seven on both sides. The target is solved
by seven-element enumeration. No GHS vulnerability or large-order reduction is
asserted. Publication claims `PCC-20261115-1148a8` and `PCC-20261115-97a477`
are unsigned. [pcc:unverified]

## Requirement and proof boundary

| Requirement | Evidence | Scope |
| --- | --- | --- |
| Actual smaller characteristic-two field | `CoverDecodeExample.small_card`, `target_smaller` | Explicit `Subfield` with four elements |
| Concrete cubic covers | `sourceCoverAlgebra`, `targetCoverAlgebra`, basis and equation lemmas | Finite free affine cover algebras of rank three |
| Fixed common integral lift | `CoverIntegralLift.concrete_good_specializations` | Two ring maps, reduced models and nonzero discriminants |
| Preserve and reflect the scalar | `CoverDecodeExample.example_dlog`, `scalar_mod`, `seven_row_table` | Actual elliptic-point groups, all seven residues |
| Pullback, genus, Jacobian specialization and norm | `COVER-CONSTRUCTION.md` | Written geometric proof; not a fully formal Jacobian functor |
| Weaker target | Target subgroup has seven elements | Enumeration; no GHS solver |
| Reviewable delivery | PR 4308, report source, HTML, PDF, SVG | Prior evidence retained |

The integral model is over `Z[z]/(z²+z+1)`, with parameter `2+65537z`.
Its source specialization receives values in `F_(65537²)`; the reduced model
and seven marked points descend to the original `F_65537`. Its other
specialization is in the actual four-element field. The cover does not by
itself change characteristic: the marked integral sections provide that
bridge. [pcc:unverified]

## Lean validation

With `/home/agent/.elan/bin` on `PATH`, from the example directory:

```sh
lake env lean /workspace/scratch/CoverFieldProbe.lean
lake build CoverDecodeExample
lake build CoverIntegralLift
```

`assembly-probe.txt` retains the field/cardinality/base-change probe. Probe
attempts 01–06 and cover builds 01–05 retain failed development checks;
cover build 06 succeeds. The first 137-theorem gate and its explicit audit
are frozen in `gate/`, `cover-proof-gate.log`, `cover-axiom-audit.log`.
The companion builds 01–04 retain their errors, resolved by existing
`AdjoinRoot`, polynomial-degree, characteristic and model-map laws. Build 05
succeeds with one tactic-style linter warning. No limit or trust exception
was added. [pcc:unverified]

After staging all sources for the tracked inventory, from the repository root:

```sh
PATH=/home/agent/.elan/bin:$PATH USE_FRO_CACHE=1 \
  tools/lean/proof-gate.sh \
  --out examples/cross_characteristic_ecdlp/receipts/cover-20261115T2113Z/gate-integral \
  examples/cross_characteristic_ecdlp
```

The unchanged gate exits zero: **156 theorems, zero failing, zero trusted
native exceptions**. Its receipts are under `gate-integral/`; the raw
summary is `cover-integral-proof-gate.log`. Then run, from the project:

```sh
lake env lean AxiomAudit.lean
```

The expanded explicit audit is retained in `cover-integral-axiom-audit.log`.
Only `propext`, `Classical.choice`, and `Quot.sound` are allowed by the gate.
One Lake process ran at a time. Source/dependency/checker hashes are in
`source-pins.json`. These are local kernel checks, separate from independent
signed PCC standing. [pcc:unverified]

## Native identity records

```sh
g++ -std=c++17 -O2 -Wall -Wextra -Werror \
  examples/cross_characteristic_ecdlp/identity_receipts.cpp -lcrypto \
  -o /workspace/scratch/cover-identity-issuer
/workspace/scratch/cover-identity-issuer /workspace/scratch/cover-identity-replay \
  examples/cross_characteristic_ecdlp/CoverDecodeExample.lean
```

The original records remain in `identities/`. IDs are
`IDC1hd9c51d3b1a10865f`, `IDC1h7b0661d4e033557f`,
`IDC1hb3b3ac5e66594860`, and `IDC1h2bc056bd8eacea35`.
The producer issuer accepts 32 derived points per identity and refuses all
four perturbed controls. The exact polynomial identities are separately
checked in Lean. JSON statement IDs and proof hashes are metadata checks.
Rust was unavailable for the repository's independent Rust replay; no such
replay or signed certificate acceptance is claimed. [pcc:unverified]

## Report rendering

The editable SVG is rendered to PNG with Node `sharp` at density 300.
The first PDF attempt failed because Pandoc's default Markdown reader did
not recognize single-backslash display math; its log is retained. The
successful command is:

```sh
pandoc cover-report.md --from=markdown+tex_math_single_backslash \
  --pdf-engine=xelatex -V 'mainfont=DejaVu Sans' \
  -V papersize=a4 -V geometry:margin=17mm -o cover-report.pdf
pandoc cover-report.md --from=markdown+tex_math_single_backslash \
  --standalone --mathjax -o cover-report.html
pdfinfo cover-report.pdf
pdftoppm -scale-to 1500 -png cover-report.pdf /workspace/scratch/cover-report
```

The PDF has two A4 pages; both were opened and inspected after the successful
render. Equations, field labels, subgroup order, source links, claim IDs and
the distinction between written geometry and Lean checks are legible.
The three existing artifact-store entries and migrated PNG bytes are
unchanged. The new PNG is left to the existing artifact workflow.
No benchmark or performance panel changes. [pcc:unverified]

PR-body prose lint has zero findings. The first report lint found two
unqualified words in the proof map; its original output is retained and
the corrected report lint has zero findings. Source diff inspection reports
one existing blank line at the new cover module's end; that pinned source
is kept byte-identical to its identity proof pointers. Raw failed-build logs
also retain their original terminal whitespace. No whitespace-clean claim
is made for those frozen receipts. [pcc:unverified]

## Concurrent histories and publication

The original authoring parent was `5668d68ccd8f1f1758db7f1e8c1cc1b865ce7a02`.
The remote then advanced through independent automatic merges `5dddc7d7` and
`c40b3982`, while this construction was unpublished. All existing example
files and the proof-gate script are byte-identical across that advance.
The ledger prefixes are not: both automatic merges differ from the authoring
parent starting at zero-based row 1088. Exact hashes, row counts and full
journal-block comparisons are retained in `remote-merge-review.json` and
`journal-parent-review.json`. Full-block differences require review; a heading
alone is not treated as a unique record. [pcc:unverified]

The unpublished construction was committed locally as `60905249a` and kept on
`archive/cover-unpublished-20261115`. Its original claim and full ledger bytes
survive as `unpublished-claim-c0a3d8.json` and `unpublished-ledger.snapshot`.
Only the example file changes were applied to the already-published remote
head `c40b39826f0b8378d939b624e78a68ae870208a1`. Two new declaration-bound
claims were issued there using the unchanged PCC tool. No incompatible
frozen histories were merged, selected, or rechained by this addition.
`publication-provenance.json` records the sequence. [pcc:unverified]

The pre-synchronization ledger check against c40b3982 passes at 1097
entries, with the parent bytes as an exact prefix. The comparison against
fetched `origin/main` fails at zero-based row 1092; raw outputs are retained.
The earlier frozen-history issue in independent merge `5e2b86eb` also remains
unresolved. Auto-merge remains paused; no gate is bypassed. [pcc:unverified]

The final remote synchronization is recorded separately in
`final-publication-resync.json`. While validation ran, the published branch
advanced to `14b075f4`; its existing example and checker inputs are unchanged.
The complete unpublished registration history was archived at `ce6235287`
and in `unpublished-ledger-c40b3982.snapshot`. The two publication claim files
stay byte-identical, and the existing PCC `append_entry` function emits fresh
registration events after the current published prefix. This is not a merger
of incompatible frozen ledgers. The final parent/main checks are retained in
`final-ledger-parent.log` and `final-ledger-main.log`. No additional Lean rerun
is claimed after this byte-identical synchronization. [pcc:unverified]

Copilot request 6900616661 is not duplicated. The independently produced
merge is reviewed as unverified, and the conflicted journal is not edited.
Required current-head `gate`, `size`, `artifacts`, review state and mergeability
must be inspected after publication. The existing monitoring task is updated
for about four hours later, with unchanged checks reset by this new activity.

## Deferred canonical session entry — concrete cover task

### What was done (commands, files, features)

- Constructed two explicit cubic covers and a fixed integral lift ending in
  the actual four-element field, with a complete seven-row scalar table.
- Added Lean instance obligations and endpoints, written geometric proof,
  formalization map, native identity records and inspected two-page report.
- The unchanged local proof gate passes 156 theorems with no failures or
  trusted native exceptions. Publication claims remain unsigned.
  [pcc:unverified]

### Decisions made (and why)

- Use the requested weaker-target option: seven-element enumeration, without
  asserting GHS applicability. State both source and target subgroup orders.
- Keep genus, Picard maps and specialization explicitly in the written proof;
  do not substitute an arbitrary group for a Jacobian.
- Archive unpublished ledger evidence before applying source-only changes to
  the independently advanced branch; append only fresh publication events.

### Open threads (unfinished, pick up next time)

- Historical ledger-prefix review, full journal reconciliation, current-head
  required checks, review and merge remain pending.
- Append this and the earlier deferred entries to the canonical journal when
  safe, preserving all full blocks and multiplicities. The genuine full Lean
  Jacobian formalization remains outside this deliverable's checked boundary.

### Errors (what went wrong, how it was handled)

- Retained the probe, proof and renderer failures; resolved them using existing
  library laws and the correct Markdown math reader.
- Incoming-main ledger comparison fails; no historical row or unrelated
  benchmark evidence was altered to repair it. Root Cargo validation remains
  unavailable in this sparse Lean-only environment; it is not claimed as a pass.
