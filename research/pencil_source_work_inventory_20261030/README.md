# Pencil source work inventory

[Two-page PDF](report.pdf) · [Editable workflow SVG](workflow.svg) · [Standard JSON](inventory.json) · [Unchanged full review](review.md)

Static source inspection only. Bounds are conditional on 16 eligible walks, and vector-header bytes assume the intended 64-bit ABI. They exclude payload/allocator overhead and are not RSS. Symbolic graph width and measured bottleneck remain unknown. All cost ratios are null; the full 1,260-target goal remains incomplete and historical failed runs remain retained.

Source paths/hashes are repository-root relative in source-hashes.sha256. The earlier rejection proof JSON is copied unchanged as retained evidence, not newly proved or executed. The original private review is byte-identical to review.md. The source read for the early-rejection README was the tracked HEAD blob; its digest is retained separately.

Native rendering: `rsvg-convert -f pdf -o workflow-figure.pdf workflow.svg`, then `pdflatex -interaction=nonstopmode -halt-on-error -no-shell-escape report.tex` twice. LaTeX explicitly sets PDF version1.7. Both final PDF pages were rasterized using pdftoppm and visually inspected, without clipping. A case-insensitive final render log scan found no warning, overfull box or error. Editable sources and raw render logs are retained.

Conductor T-941. No solver, replay, new timing run, research build, Git commit or push. Parent owns delivery and shared session logging.
