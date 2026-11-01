# Finite-field reference knowledge base

The supplied book is **Gary L. Mullen and Daniel Panario (editors),
Handbook of Finite Fields**, Chapman and Hall/CRC, 2013, ISBN 9781439873786.
This directory connects textbook references to actual Lean statements,
their assumptions, and recorded proof checks.

**Current scope:** 5,850 numbered items inventoried; 58 Lean theorems mapped
to 35 source items. Many mappings cover only part of a book item. The
whole-book validation and formalization request remains open. The 49
conjectures retain an unchecked-current-status label; the book's date alone
does not establish their present status. Definitions are recorded as
definitions, with some consequences proved separately.

## Reference lookup

From this repository's root:

```sh
knowledge/finite-fields/kb search 'trace'
knowledge/finite-fields/kb search 'irreducible splitting'
knowledge/finite-fields/kb show FF.irreducible_splits_degree_extension
knowledge/finite-fields/kb cite FF.irreducible_splits_degree_extension
knowledge/finite-fields/kb show HFF2013:3.1.12
knowledge/finite-fields/kb search 'normal' --all
knowledge/finite-fields/kb stats
knowledge/finite-fields/kb check knowledge/finite-fields/source.txt
```

The default search returns rules whose recorded Lean proof checks still
match the files on disk. `--all` explicitly searches unreviewed inventory
entries as well, and does not certify their truth. Search is deterministic
lexical matching over titles, statements, assumptions and aliases; it does
not perform semantic theorem search or infer that a theorem applies.

`cite` produces a statement, source ID, declaration, assumptions, source
coordinates, and scope note. `show` also displays the recorded Lean type and
axioms. A source item can have several rules: citing one does not certify
the other subclaims in that item. Source alignment has been reviewed by the
implementing agent; independent fidelity review remains pending.

For example, `FF.irreducible_splits_degree_extension` cites the corrected
first sentence of Remark 2.1.118: an **irreducible** degree-n polynomial over
a finite field splits in its degree-n extension. Its Lean statement requires
the finite field, prime characteristic, positive n, irreducibility, and the
degree equality. It does not certify the rest of the paragraph.

## Use in proofs

The package root is `lean/`, with namespace `FiniteFieldHandbook`.
Add a local Lake dependency to that directory in a research project, keeping
the same toolchain and Mathlib pin, and import `FiniteFieldHandbook`:

```lean
import FiniteFieldHandbook

example (K : Type*) [Field K] [Fintype K] (a : K) (ha : a ≠ 0) :
    a ^ (Fintype.card K - 1) = 1 :=
  FiniteFieldHandbook.fermat K a ha
```

[`lean/Usage.lean`](lean/Usage.lean) checks this interface on three research
use cases. Lean checks the assumptions at the call site. A natural-language
similarity match is only a way to find the declaration.

The shared CryptoLean library currently pins the same Lean/Mathlib versions.
Keep these source-specific wrappers here while the book coverage is being
reviewed. General lemmas worth sharing can later move into CryptoLean with
their own reviewed PR, while these stable book IDs keep pointing to them.
No sibling repository is changed by this intake.

## Evidence and provenance

| File | Purpose |
| --- | --- |
| [source.json](source.json) | Bibliography and original text digest |
| [inventory.json](inventory.json) | Kind, exact byte/line span, digest and search terms for each numbered item |
| [rules.json](rules.json) | Original statement summaries, hypotheses, declarations and fidelity notes |
| [corrections.json](corrections.json) | 37 numbered-item editor corrections and one explicit characteristic clarification |
| [lean/FiniteFieldHandbook.lean](lean/FiniteFieldHandbook.lean) | Actual proofs over Mathlib objects |
| [receipts/axioms.log](receipts/axioms.log) | Printed theorem types and their transitive axioms |
| [receipts/audit.log](receipts/audit.log) | Audit of every named theorem in the module |
| [receipts/dependencies.json](receipts/dependencies.json) | Direct constants in elaborated theorem types and proof terms |
| [receipts/verification.json](receipts/verification.json) | File hashes binding the recorded checks to their inputs |
| [REPORT.md](REPORT.md) | Requirement status, findings, limits and continuation |

The supplied original is retained locally as ignored `source.txt`. To inspect
a span, use `kb show HFF2013:2.1.118 --source PATH`. The source digest must
match. A different edition or corrected OCR extraction needs a new source
version and explicit coordinate migration. The catalogue commits coordinates,
digests and derived terms, rather than a copy of the complete book.

Spans extend to the next recognized numbered item, so they can include
intervening headings and prose. Bracketed numbers are reference candidates,
not reviewed bibliography links. Neither the span nor its keyword list is
an assertion that every mathematical claim in that region was extracted.

The full `inventory.json` is local and ignored under the artifact-store
policy. Publication is currently blocked: the available credentials returned
403 for the artifact tool's access check and AccessDenied for an immutable
object upload. [PUBLICATION.json](PUBLICATION.json) records its identity and
the failed attempts; it is explicitly not an artifact manifest. A fresh
checkout can regenerate the exact catalogue from the original supplied text
with `knowledge/finite-fields/kb ingest PATH`. Once access is restored,
upload through the artifact tool, download and hash the complete object,
then commit its real manifest. The graph SVG and PDF are committed; the PNG
is a local preview.

## Verification

```sh
knowledge/finite-fields/verify.sh knowledge/finite-fields/source.txt
```

This runs the locked standalone Rust tests, builds the Lake library, audits
its transitive axioms with the repository auditor, prints every theorem's
type and axioms, exports dependencies, checks consumer examples, and records
hashes. If the original text is supplied, it also regenerates the inventory
in memory and checks it exactly. A fresh checkout can verify the formal
library without the original text by omitting the final argument.

The pin is Lean **4.35.0**, Mathlib
`cfc55fc58714a59dc06e6e79edc6d1d95140d3c2`, matching the existing shared
CryptoLean pin. On a cold checkout use `lake update` in `lean/`, check that
the lockfile stays unchanged, and fetch the imported Mathlib modules' cache
before running verification. Build dependencies stay under ignored `.lake/`.
The repository-wide Lean proof gate also discovers this Lake project once
it is tracked; it runs on the workflow's existing main/scheduled triggers.

The accepted transitive foundations are `propext`, `Classical.choice` and
`Quot.sound`. No book assertion is introduced as a custom axiom. The local
toolchain does not include `lean4checker`; a fresh secondary checker run is
still unavailable. Kernel compilation and the axiom audit have run.

Receipts attest to the recorded local commands and detect stale files;
they are not signed remote attestations. Anyone able to edit all source and
receipts can forge a record, so an independent reviewer should rerun the
verification command. The proof receipt does not establish that an English
translation matches its source; the fidelity notes and review serve that
separate obligation.

## Extend coverage

1. Read a source span, its surrounding assumptions, editor corrections and
   cited primary reference. Resolve OCR ambiguity before translating.
2. Split multi-part items into explicit claims. Record whether each is a
   definition, established result, conjecture, algorithm contract, heuristic,
   numerical table or historical statement.
3. Find the corresponding result in the pinned Mathlib source. Reuse it when
   faithful; otherwise implement the missing definitions and proofs.
4. Add the actual Lean theorem, a `rules.json` entry and explicit checks in
   `PrintAxioms.lean`. Distinguish full equivalence, partial coverage,
   consequences of definitions and corrected statements.
5. Run verification and review statement fidelity. Preserve prior source
   versions and failed proof attempts. Resolve conjectures through a separate
   reviewed evidence path; this tool intentionally refuses to promote them
   merely through catalogue annotations.

The remaining whole-book obligations and concrete next batch are recorded
in [the problem record](../../problems/finite-field-handbook-formalization.md).
