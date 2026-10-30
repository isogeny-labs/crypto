.PHONY: all check clean
all: report.pdf lifting.svg bridge.svg

lifting.pdf lifting.svg &: plot.py coverage-data.json
	MPLCONFIGDIR=/tmp/twist-signed-cache-mpl XDG_CACHE_HOME=/tmp/twist-signed-cache-fonts python3 plot.py

bridge.pdf: bridge.dot
	dot -Tpdf $< -o $@
bridge.svg: bridge.dot
	dot -Tsvg $< -o $@

report.pdf: report.tex lifting.pdf bridge.pdf
	pdflatex -interaction=nonstopmode -halt-on-error -jobname=report-build report.tex
	gs -q -dBATCH -dNOPAUSE -sDEVICE=pdfwrite -dCompatibilityLevel=1.7 -dPDFSETTINGS=/ebook -dEmbedAllFonts=false -dSubsetFonts=true -sOutputFile=$@ report-build.pdf
	pdftotext report-build.pdf report-text-before.txt
	pdftotext $@ report-text-after.txt
	cmp report-text-before.txt report-text-after.txt

check: all
	! rg -n 'Overfull \\hbox|undefined references|Citation.*undefined' report-build.log

clean:
	rm -f report-build.* report-text-*.txt
