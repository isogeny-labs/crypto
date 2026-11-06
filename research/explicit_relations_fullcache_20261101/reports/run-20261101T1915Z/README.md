# Second large full-cache comparison report

Raw results were committed and pushed as fd8c4bd2051f before rendering.
Both pages of report.pdf were rendered and visually inspected. The figure is
the unchanged output of the frozen analyze-v3.rs, using all three recorded
cold complete-recovery clocks. The table and derived ratios read raw CSV data.

Results: ../../results-v3/export.json and ../../results-v3/analysis-replay.json.
Protocol: ../../protocol-v3.json. All three certificates agree and replay;
source/host/artifact hashes and deletion/budget receipts are retained there.
The previous interrupted run and refused launch are reported, not overwritten.

The benchmark PDF and SVG copies are byte-identical. Its PNG is a derived view
of the same SVG. presentation-custody.json binds all presentation files.
