.PHONY: report
report:
	dot -Tsvg trace-fiber.dot -o trace-fiber.svg
	dot -Tpdf trace-fiber.dot -o trace-fiber.pdf
	@set -e; tmp=$$(mktemp -d); pdflatex -halt-on-error -interaction=nonstopmode -output-directory=$$tmp report.tex; cp $$tmp/report.pdf report.pdf; echo "Renderer logs retained at $$tmp"
