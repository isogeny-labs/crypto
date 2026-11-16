# Assessment validation — 2026-10-30

This is documentation/source inspection, not a new experimental result.

- Source snapshot: crypto fc9af16fa1ce0654ee9df3ce63101c487ed215fb;
  publication base advanced to b7f830121417997d8863189f435bcb66f6466ca2
  without changes to the inspected native crate, reference-arm specification,
  prime solver interface or leaderboard. The problem note is intentionally
  extended by this PR; source hashes describe its earlier reviewed content.
- sources.json parses; all15 pinned local source digests verified against Git.
  Current atlas dispositions total152. Anthropic MainResults source bytes
  reconstruct Git blob1c62a550fa1e4cbec65bb7112f92ad67589afddc at the pinned
  revision. These are provenance checks, not theorem checks.
- Render command: /tmp/hashed-ecdl-report-venv/bin/python
  research/hashed_ecdl_leads_20261030/render_report.py. Exit0. Python only
  lays out recorded Markdown/diagram, consistent with research-visuals.
  Graphviz generates the editable-source SVG; Typst0.15.0 produces the PDF.
- pdfinfo reports5 A4 pages,92720bytes, unencrypted, PDF1.7. Rendered every
  final page with pdftoppm -scale-to1200 -png and inspected all five: diagram,
  repeated table headers, source identifiers and text are legible, without
  clipped labels. Page rasterizations stay in scratch, not in the repository.
- No library, example, benchmark or research implementation changed. No broad
  library/test/benchmark panel rerun; historical source-specific results are
  not presented as current validation. No new identity is asserted; the
  bridge is an audit map rather than an identity certificate or formal proof.
- Unchanged artifacts policy and whitespace are checked against the complete
  staged tree and final commit before publication. Current-head required CI
  and actual reviews/protection still govern any merge.
- No scientific experiment, paid resource, image build, challenge solve,
  checkpoint promotion, ledger state change or performance claim occurred.
  Report PDFs remain in Git under existing policy; no remote upload is claimed.

Errors retained: conductor unavailable127 with documented warn/allow fallback;
tracked sparse paths materialized before reading; guessed Lean URL404 corrected
from publisher/catalog links; initial duplicate-operation patch rejected before
application; first PDF layout revised to repeat headers and avoid an almost
empty overflow page, then final five pages re-inspected. An irrelevant cross-
repository Git-object diff attempted lazy lookup and yielded no assessment
result; it completed with no changes and its output was not used. No test,
proof, gate, dependency or frozen evidence was changed to handle these issues.

Initial whitespace check found a generated final blank line; renderer now trims
only trailing whitespace. Initial artifacts check used defaults because the
tracked sparse .artifacts/config.json was not materialized. Materialized that
unchanged file, which carries the existing PDF keep policy, and reran the
complete staged-tree checks. No gate/config change or store upload was used.
The complete-tree whitespace read also initially treated a PDF as text while
tracked root .gitattributes was sparse. Materialized the unchanged attributes;
its existing binary policy applies. Both staged and complete-tree whitespace
then exit0; diagnostic output is empty. No attributes or policy edit was made.
