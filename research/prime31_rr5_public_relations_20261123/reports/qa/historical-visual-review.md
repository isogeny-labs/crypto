# Rendered report inspection

- Artifact: reports/rr5-public-relations.pdf,8 US-letter pages.
- SHA256:07effda525c5678750fe45440a4a881fc8d2f1fdf82a4b7d647050a098dcadd0.
- Renderer: Pandoc markdown+tex_math_single_backslash to Typst,then Typst0.15.1
  through the layout-only Python binding. Source,style,Lua image sizing filter,
  SVG figures and build entry point are retained beside the PDF.
- Inspected all8 rendered pages at95dpi. The final render's pages1–5 are
  pixel-identical to the previously inspected pages; pages6–8 were inspected
  again after shortening the setup-scope sentence to remove a lone carried word.
- Equations,signs,table entries and both SVG figures are legible and within page
  bounds. Neither figure is clipped. Headers,page numbers and mathematical
  glyphs render correctly. The opening page is an overview with white space;
  later pages separate the stage diagram,scope table and retained outcome table.
- Figure1 reports completion/cap stages. Figure2 uses recorded input-term counts,
  with different caps and both inconclusive outcomes stated. Neither presents
  runtime,cost or performance improvement.
- This inspection concerns document layout. It does not upgrade the unsigned
  scientific evidence or qualify any unexecuted root extraction or comparison.

Reproduction: run reports/build-report.sh from a Python environment containing
Typst,then pdftoppm -r95 -png reports/rr5-public-relations.pdf OUTPUT_PREFIX.
