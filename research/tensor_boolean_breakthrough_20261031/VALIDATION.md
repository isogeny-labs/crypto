# Validation receipt

Date: 2026-10-31  
Scope: experiment design and rendered report only; no affected experiment data

## Protocol and structured records

Command:

```text
python3 /workspace/crypto-autoresearcher/tools/experiment_protocol.py \
  validate research/tensor_boolean_breakthrough_20261031/protocol.draft.json
```

Result:

```text
OK: experiment.protocol/v1 research/tensor_boolean_breakthrough_20261031/protocol.draft.json (0 finding(s))
```

`jq empty` passed for both `protocol.draft.json` and `experiment-matrix.json`.

The protocol remains intentionally `draft`. Its power record has `passes: false`, and the binary, image, analysis, runtime, memory, final-cut-cap, final-ladder, protocol-hash, commit, and freeze fields remain unset. No production seeds were derived and no affected rows were generated.

## Figures and PDF

- Graphviz rendered `figures/experiment-funnel.dot` to SVG and PNG successfully.
- Pandoc with pdfLaTeX rendered `REPORT.md` to `report.pdf` successfully.
- `pdftotext` recovered the required Recommendation, first-test, SAT-falsifier, claim-boundary, and evidence sections.
- PDF metadata: 2 US-letter pages, 312,903 bytes.
- SHA-256 of `report.pdf`: `902cfa86ccb0c53c41cf5b49e1dedc91fca8eaaef0d1e8337104f680cddb7824`.
- Both PDF pages and the source PNG were opened. Text, table, labels, arrows, caption, and page boundaries were present without clipping. The wide graph is intended for page-width display and remains zoomable in the PDF.

## Repository checks

`git diff --check` passed. No Rust source, manifest, benchmark, experiment export, or result row changed, so no code test is claimed by this design-only package.

The required pre-work `conductor check` was attempted for the study and session-log scopes and returned exit 127 because `conductor` is absent. The repository's warn-and-allow path was used; no reservation was claimed.

## Evidence status

| Item | Status |
|---|---|
| Experiment portfolio and dependencies | Complete at design scope |
| E1 controlled protocol | Draft, schema-valid |
| A/A, positive, negative and sham controls | Specified |
| SAT/BDD triangulation | Experiment card specified; separate protocol not yet frozen |
| Canonical IR implementation | Not started |
| Runtime/power preflight | Not started |
| E1 measurements | None |
| Breakthrough or speedup claim | Not established |

