.PHONY: all clean
all: report.pdf degrees.svg

degrees.pdf degrees.svg &: plot.py coverage-data.json
	MPLCONFIGDIR=/tmp/twist-reduced-mpl XDG_CACHE_HOME=/tmp/twist-reduced-fonts python3 plot.py

report.pdf: report.tex degrees.pdf
	pdflatex -interaction=nonstopmode -halt-on-error -jobname=report-build report.tex
	gs -q -dBATCH -dNOPAUSE -sDEVICE=pdfwrite -dCompatibilityLevel=1.7 -dPDFSETTINGS=/ebook -dEmbedAllFonts=false -dSubsetFonts=true -sOutputFile=$@ report-build.pdf
	pdftotext report-build.pdf report-text-before.txt
	pdftotext $@ report-text-after.txt
	cmp report-text-before.txt report-text-after.txt

clean:
	rm -f report-build.* report-text-*.txt
