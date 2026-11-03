.PHONY: all check clean
all: report.pdf comparisons.svg bridge.svg
comparisons.pdf comparisons.svg bridge.pdf bridge.svg &: plot.py graph-data.json
	MPLCONFIGDIR=/tmp/twist-cm8-mpl XDG_CACHE_HOME=/tmp/twist-cm8-fonts python3 plot.py
report.pdf: report.tex comparisons.pdf bridge.pdf
	pdflatex -interaction=nonstopmode -halt-on-error -jobname=report-build report.tex
	gs -q -dBATCH -dNOPAUSE -sDEVICE=pdfwrite -dCompatibilityLevel=1.7 -dPDFSETTINGS=/ebook -dEmbedAllFonts=false -dSubsetFonts=true -sOutputFile=$@ report-build.pdf
	pdftotext report-build.pdf report-text-before.txt
	pdftotext $@ report-text-after.txt
	cmp report-text-before.txt report-text-after.txt
check: all
	! rg -n 'Overfull \\hbox|undefined references|Citation.*undefined' report-build.log
clean:
	rm -f report-build.* report-text-*.txt
