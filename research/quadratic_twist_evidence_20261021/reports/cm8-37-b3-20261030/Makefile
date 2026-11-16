all: report.pdf
frontier.pdf frontier.svg: plot.py plot-data.json
	OPENBLAS_NUM_THREADS=1 python3 plot.py
bridge.pdf: bridge.dot
	dot -Tpdf $< -o $@
bridge.svg: bridge.dot
	dot -Tsvg $< -o $@
report.pdf: report.tex frontier.pdf bridge.pdf
	pdflatex -interaction=nonstopmode -halt-on-error report.tex
