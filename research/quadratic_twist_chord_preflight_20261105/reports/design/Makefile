all:
	dot -Tpdf branches.dot -o branches.pdf
	dot -Tsvg branches.dot -o branches.svg
	pdflatex -interaction=nonstopmode -halt-on-error -jobname=report-uncompressed report.tex
	gs -q -dNOPAUSE -dBATCH -sDEVICE=pdfwrite -dCompatibilityLevel=1.5 -dPDFSETTINGS=/prepress -dCompressFonts=true -dSubsetFonts=true -sOutputFile=report.pdf report-uncompressed.pdf

.PHONY: all
