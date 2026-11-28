# Native learning-bound report validation

trace_learning_audit.rs produced the fixed once-only native development output;
trace_learning_report.rs read that output and the proof-bound saved counters.
Typst0.15.1produced one A4page, Poppler rendered it and the separate interval
figure at150dpi. Inspected the full color page, final four-color report PNG
(63988B) and16-color figure PNG(18922B). All caps, labels, tables, formulas,
source references and unsigned/partial-scope captions are legible; no clipping,
overlap or missing glyph observed. Bounds are explicitly nonstatistical.

Exact values agree with audit-original.json and derived-comparison.json. The
formulas separate row multiplication, additions/negation, inverse calls,
expanded Fermat multiplications and contributor word ORs. Inverse alternatives
are not added twice. The original audit output and pre-call bindings are
unchanged and verify. No old Run51proof/result or pricing was modified.

Initial Typst compilation rejected formula asterisks as an unclosed markup
delimiter. Rendered those formulas as literal inline code, rebuilt the native
report generator and verified derived-comparison.json unchanged byte-for-byte.
This was a presentation failure; no controlled F4call was repeated. Source
formatter, both strict scoped Clippy checks, three arithmetic controls and
report/README prose lint pass. Current-root and legacy benchmark gates retain
their previously observed failures; no full-library pass is asserted.

This is a bounded recorded-input mathematical audit, not an elapsed benchmark.
No new target, paid runner, signed verdict, benchmark/leaderboard promotion,
artifact-store upload or rho-parity claim. Library budget clocks are internal;
no elapsed values are exported or compared.
