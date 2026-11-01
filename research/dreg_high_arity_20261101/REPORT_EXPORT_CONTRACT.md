# Publication contract for the high-arity closure study

Status: **contract registered; publisher implementation and consistency tests
pass; frozen binary hashes remain pending**.
`examples/dreg_high_arity_report.rs` must still be source- and binary-hashed
and included in the frozen publication identity before any registered draw is
generated. Until the remaining freeze gates pass, the only
scientific analysis output available is `dreg_high_arity.summary/v1`; no
`experiment.export/v1`, report, figure, Typst source, or PDF from a registered
run is claimed to exist.

## Frozen command and trust boundary

The dedicated native report binary has this interface:

```text
dreg_high_arity_report \
  --summary SUMMARY \
  --protocol PROTOCOL \
  --out-dir PUBLICATION_DIR \
  [--check]
```

The publisher refuses a non-frozen protocol or unavailable/mismatched frozen
protocol, source, or binary-manifest provenance. Invalid resume-import custody
is instead a retained campaign outcome: with sound frozen provenance it must
publish as invalid/no-conclusion so the custody failure stays visible. It
validates the analyzer schema, exactly 19 subprocess rows, five packages, and
three registered cells. It may accept only the frozen analyzer's immutable
`SUMMARY`; it cannot edit or reclassify it. It atomically creates
`PUBLICATION_DIR` with these files only after all are complete and internally
consistent:

```text
PUBLICATION_DIR/experiment.export.json
PUBLICATION_DIR/REPORT.md
PUBLICATION_DIR/closure-outcomes.svg
PUBLICATION_DIR/report.typ
PUBLICATION_DIR/report.pdf
```

`--check` regenerates the five publication files and byte-compares them with
the existing directory. Let `H` be the lowercase SHA-256 of `summary.json`.
The runner chooses
`PUBLICATION_DIR=reports/run-DREG-HIGH-ARITY-<first 16 hex digits of H>`,
matching exporter ID `DREG-HIGH-ARITY-<first 16 hex digits of H>`, and retains
`SUMMARY` separately as `summary.json`.

The publisher is a serialization and rendering layer. It may not reclassify a
subprocess, gate, cell, or global result. `summary.json` remains the sole
scientific decision source. Report generation must succeed for valid,
unmeasured, and invalid campaigns alike; it cannot omit censored or
contradictory evidence.

The report binary emits `report.pdf` itself from the same deterministic report
model used for Markdown, SVG, and Typst. No unpinned external Typst executable
participates in publication. The PDF SHA-256 is retained; PDF rendering cannot
modify the JSON export or scientific classification.

## `experiment.export/v1` mapping

The export must validate against the canonical schema in
`crypto-autoresearcher/schemas/experiment-export.v1.json` and pass
`experiment_protocol.py check-export` with full file verification.

- `protocol` records the frozen `protocol.json` path, canonical hash,
  protocol ID, and version.
- `source` records repository `crypto`, the common frozen instrument commit,
  and `dirty: false`. Generation, closure, analysis, and publication may run
  on different pushed descendant commits as immutable draws and receipts are
  added; their validated execution-ref/commit history remains verbatim in
  `backend_record` and the linked attempt/generation metadata rather than being
  collapsed into one fictitious live execution commit.
- `backend` is `ephemeral`. The schema-level aggregate `host` fields are null,
  because the 19 jobs may use distinct workers and the runner produces no
  single aggregate-host artifact. Every per-attempt host record remains hashed
  and linked through `files` and the relevant package artifacts.
- `arms` contains the five protocol arms with their frozen closure binary,
  source commit, and image digest.
- `runs` contains exactly five composite rows: identity replay, six-control
  package, and the three four-UNSAT cell packages. Each uses instance ID
  `0` (the canonical single instance derived from `sample_size.instances = 1`),
  `replicate: 0`, `warmup: false`, and seed `20261101`.
  Its artifacts link every constituent subprocess, attempt, draw, generation
  sidecar, checkpoint manifest, v3 resume-import record with its
  copy/immutable/external-anchor manifests, final pointer, and analyzer row.
- A composite run records `attempts = max(1, constituent closure attempts)`:
  the schema requires at least one attempt, and the synthetic composite
  analysis row itself is the one attempt when an earlier invalid gate prevented
  every constituent from launching. The exact, possibly zero constituent count
  is retained as `measurements.counters.closure_attempts`; it is never presented
  as executed closure work. `resumed` is the sum of constituent attempts whose
  analyzer-validated metadata records `checkpoint_restore.restored = true`;
  each such field binds the restored generation and embedded committed
  checkpoint manifest, whose unique entries must cover exactly every ordinary
  generation file including `catalog.json` and whose `spec_sha256` must equal
  that catalog's system identity. The analyzer also requires a
  claimed-success output's `checkpoint_segments` to be exactly one for fresh
  metadata and at least two for restored metadata. Checkpoint segments, retry
  histories, and resume-import
  manifests remain linked evidence. `wall_valid` is always false because time
  is not a registered metric and any composite, retried, or
  checkpoint-restored duration is inadmissible.
- `measurements.counted.closure_class` carries the exact package category;
  complete rows may additionally carry registered exact counters. Resource-
  censored and invalid constituents stay explicit in counters/notes and are
  never converted to zero.
- `outcome_class` is `completed` only for a scientifically classified package
  whose shared gates passed; otherwise it is `not_completed`. A valid exact
  package maps to `status: succeeded` and `verification.status: verified`; an
  unmeasured package maps to `status: interrupted` and
  `verification.status: not_run`; invalid evidence maps to `status: failed`
  and `verification.status: error`. The exact detailed reason remains in the
  linked summary and observation-only notes.
- `flow.planned` is five and the remaining flow counts are computed from the
  five composite rows. The underlying 19 subprocess histories remain in
  `files` and linked artifacts; they are never presented as a sixth package or
  silently dropped.
- `files` recursively hashes every ordinary scientific-evidence file under
  the summary's `draw_dir` and `output_root`, excluding the entire derived
  `reports/` namespace to avoid self-reference and to keep later sibling
  reviews from changing the export. It additionally requires and binds the
  summary, frozen protocol, frozen analyzer source, and the required
  `sources.sha256` / `binary.sha256` manifests, then adds `REPORT.md`,
  `closure-outcomes.svg`, `report.typ`, and `report.pdf`.
- `backend_record` retains all 19 analyzer subprocess rows and their attempt
  histories verbatim; the five export runs do not collapse that evidence.
- `analysis.script` is the frozen analyzer source and `analysis.outputs` binds
  `summary.json` plus the four report artifacts. The export also binds the
  summary SHA-256, frozen protocol SHA-256, analyzer source SHA-256, and report
  hashes. `experiment.export.json` is intentionally absent from its own
  `files` and `analysis.outputs` arrays.
- `reviews_dir` is `../reviews`, a sibling of the hash-named publication
  directory. Review and inspection records therefore remain outside the
  strict five-file atomic bundle and outside raw-evidence traversal.

Immediately before publication, the runner executes the frozen analyzer again
over the retained draws and output tree and requires its complete summary to
be byte-identical to `SUMMARY`. The publisher then independently validates the
row-to-cell-to-package-to-global grammar. A coherent edit of the summary alone
therefore cannot manufacture a scoped counterexample.

`created`, `export_id`, and report-directory naming are deterministic functions
of the frozen protocol and summary. Regenerating the same immutable evidence
must reproduce every publication byte; no wall-clock presentation timestamp is
introduced during rendering.

## Markdown, SVG, Typst, and PDF

`REPORT.md` contains the headline and scope, source links to the frozen
protocol, native summary, export, and diagram, all five package
classifications, all 19 subprocess classifications, every retained
retry/resume/resource record, per-cell counts, and the scoped next action for
each cell. The linked frozen protocol/export carry the registered question,
exact instrument identities, and draw-custody hashes; the narrative does not
duplicate those fields. It states that only an exact cell-level
universal-degree-four claim can be rejected and that resource limits never
close a lead.

`closure-outcomes.svg` is a quantitative grid with one labeled mark for each
of the twelve UNSAT draws, six SAT controls, and identity replay. Shape and
text encode `closure_le_4`, `above_4`, `passed`, `unmeasured`, or `invalid`, so
the figure remains interpretable without color. It includes no wall-time axis.

`report.typ` embeds the SVG and carries the status/scope, five package rows,
all 19 subprocess classifications, retry/resource history, and interpretation
boundary; it does not claim to duplicate every Markdown link or per-cell next
action. The
native-emitted PDF is text-extracted to check the protocol ID, all five package
categories, and the count of 19 subprocesses, then visually inspected. The
inspection record and independent `experiment.review/v1` are filed beside the
report; neither may alter the export.

## Required implementation tests

Before freeze, tests must prove:

1. all five package rows and all 19 subprocess histories are retained;
2. a resource-unmeasured shared control leaves candidate evidence visible but
   the package `not_completed`;
3. malformed claimed-success evidence and deterministic instrument failure
   export as invalid/failed, not as censoring;
4. checkpoint/retry evidence sets `wall_valid: false`, binds every import and
   restored-generation manifest, and computes `resumed` only from validated
   native `checkpoint_restore` fields;
5. changing any immutable input changes or invalidates the corresponding file
   entry; and
6. invalid resume custody emits only an invalid/no-conclusion bundle while
   unavailable frozen provenance is refused; and
7. schema validation, full file verification, deterministic Typst-source and
   native-PDF text checks, the external `../reviews` pointer, and the SVG
   accessibility checks pass.
