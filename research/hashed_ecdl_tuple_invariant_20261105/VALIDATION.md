# Document/source checks — 2026-11-05

PCC-20261105-c0901e is a **statement**, standing **unverified**. Every source
observation and local check here is a producer assertion; these checks do not
constitute a signed verdict or a mathematical proof. [pcc:unverified]

## Scope and check commands

- `python3 tools/pcc/pcc.py standing PCC-20261105-c0901e --json`:exit2,
  standing unverified, no verdicts. Claim-file SHA256
  `f1dd653274520a23410c908d75e7fa635d04a215d6d5f5caf1097f9d1de0b864`.
- `python3 tools/pcc/pcc.py verify PCC-20261105-c0901e`:exit2,
  `not_covered`: statement claims have no proof kind and can never be verified.
- `python3 tools/pcc/pcc.py ledger verify --base origin/main`:exit0,
  100-entry chain at the publication base f96a6d475896e06065eda55ccecf5f0132692423.
  A later main advance requires its own prefix check. [pcc:unverified]
- `python3 tools/pcc/pcc.py lint-prose research/hashed_ecdl_tuple_invariant_20261105/README.md`:
  exit0, zero findings. Final PR prose is checked separately before publication.
- Metadata self-check: compare every sources.json byte count/SHA256/Git blob
  with `git show <crypto_revision>:<path>`, and each audit-contract.json
  anchor with the exact recorded line. Provenance checking only; no native
  counting/curve code is executed. [pcc:unverified]
- `/tmp/hashed-ecdl-tuple-report-venv/bin/python research/hashed_ecdl_tuple_invariant_20261105/render_report.py`:
  exit0 using Typst0.15.0 and Graphviz. Python performs document layout only.
- `pdfinfo`: three A4 pages,70430bytes, PDF1.7, unencrypted. Rasterized with
  `pdftoppm -scale-to1300 -png`; all final pages inspected for readable graph,
  tables, state formulas and source identifiers. Editable Graphviz/Typst and
  the renderer are retained beside the PDF. [pcc:unverified]
- `python3 tools/benchrec.py gate`: final exit0,42 existing records and
  current progress. This validates inherited record metadata; no benchmark
  or scientific run was launched, and no record/progress file changed.
  [pcc:unverified]

Complete staged-tree/final-commit artifacts and whitespace checks are required
before pushing. Their local outcomes remain separate from current-head hosted
checks and required reviews. No broad library suite, new native regression
panel, formal theorem build or dependency/kernel/axiom audit is claimed.

## Source/literature refresh

Read both old assessment packets and the relevant source functions. Compared
the target contraction blobs against the Nov4 pin; they are unchanged.
Read OpenAI Appendix D.1/D.4.5-D.4.7 from the cached PDF whose Git blob is
0503566e95bc587a4076573a45132d97531b8ba6. Publisher path metadata still names
42907c9e999cc00d6abe7a50bfc1023250edd1b9 as last change. [pcc:unverified]

The current atlas receipt is [ATLAS-MATCH.txt](ATLAS-MATCH.txt). It uses a
read-only export pinned to8fcfa58fee923c48f96ecf4126ffe4b1ea5596e7, with the
existing matching tool and its source dependencies. The dedicated lead was
read directly. No full delta review/stamp or blanket novelty clearance.

## Errors retained

- Executor initially unavailable during startup; no command success inferred.
- Conductor exited127. `docs/CONDUCTOR.md` documents warn-and-allow when the
  executable is missing. No reservation or active-task ownership is claimed.
- `lean` and `lake` absent from PATH, with no local elan binary directory.
  No formal build, theorem declaration, identity certificate or new trust
  exception was introduced. The missing Rust/usize/packing refinement remains
  explicit rather than being replaced by a finite control.
- Initial atlas API repository guess404 and search URL400. Corrected to the
  existing sibling crypto-autoresearcher atlas. Its old local cached match was
  read but is not the current atlas receipt. Current export first lacked
  build_frontier_map, then fast_yaml/orchestration imports (exit1); materialized
  those unchanged pinned modules and ran the same tool successfully. The
  earlier archive command completed before interruption was attempted; it
  failed in the missing-module import, not in a scientific execution.
- Initial PDF70191bytes had a diagram edge from initialization straight to
  advance. Corrected the figure to show the initial zero-tuple contribution,
  rebuilt70430byte PDF and inspected every page again. The README already
  included the zero tuple; runtime behavior was never changed.
- Benchmark gate initially found75 missing sparse input references across42
  inherited records. Materialized62 exact tracked paths/sidecars and reran
  the unchanged gate, which returned0. No upload, record edit or waiver.
  [pcc:unverified]

Canonical leaderboard/progress/curve/performance graphs have no new measurement
or route from this source audit and remain unchanged. Earlier report PDFs,
manifests and runtime sources are preserved. General audit status stays open.

Publication panel: complete staged-tree artifacts, staged whitespace and
base/tree whitespace returned0; all14 pinned source records and nine line
anchors matched. Source pin remainsf96a6d after main advanced to a4dbbd282d;
its PCC ledger and AGENTS bytes are unchanged. The final PR head and hosted
checks must be read independently. Prior reports/runtime files are outside
this scoped diff. [pcc:unverified]
