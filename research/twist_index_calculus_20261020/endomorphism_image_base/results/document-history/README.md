# Document inspection history

- `boundary-before-scope-clarification.md` and `.pdf` preserve the document
  referred to by the earlier inspection receipt in the sealed raw archive.
- `boundary-svg-import-failure.pdf` is a failed document rendering: Typst
  misplaced the vector diagram. It is retained as a failure, not a finished
  report or scientific measurement. The SVG itself renders correctly in
  MuPDF and its matching PNG is unchanged.
- The current `../../BOUNDARY.pdf` includes the scope clarification, embeds
  the matching PNG, and has both pages inspected. Its current digest is in
  `../document-inspection.json`.

These changes affect documentation and rendering only. The sealed archive,
timed source, numerical CSVs and native replay remain byte-identical.
