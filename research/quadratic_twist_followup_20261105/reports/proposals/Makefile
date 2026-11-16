all:
	dot -Tpdf routes.dot -o routes.pdf
	dot -Tsvg routes.dot -o routes.svg
	pdflatex -interaction=nonstopmode -halt-on-error report.tex
