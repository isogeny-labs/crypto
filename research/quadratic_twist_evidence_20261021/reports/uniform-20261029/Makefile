.PHONY: all
all:
	MPLCONFIGDIR=/tmp/twist-uniform-matplotlib python3 plot.py
	dot -Tsvg bridge.dot -o bridge.svg
	dot -Tpdf bridge.dot -o bridge.pdf
	dot -Tsvg attempt-01/bridge.dot -o attempt-01/bridge.svg
	dot -Tpdf attempt-01/bridge.dot -o attempt-01/bridge.pdf
	pdflatex -interaction=nonstopmode -halt-on-error report.tex
	pdflatex -interaction=nonstopmode -halt-on-error report.tex
	cd attempt-01 && pdflatex -interaction=nonstopmode -halt-on-error report.tex
	cd attempt-02 && pdflatex -interaction=nonstopmode -halt-on-error report.tex
