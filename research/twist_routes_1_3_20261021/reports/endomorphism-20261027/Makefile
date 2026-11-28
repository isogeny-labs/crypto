RESULTS.tex: RESULTS.md ../../execution-v8/support/break-report-hashes.mjs
	sed 's/(CONNECTION.svg)/(CONNECTION.pdf)/g' RESULTS.md | pandoc -s -f markdown -t latex -V geometry:margin=18mm -V fontsize=10pt -V mainfont='DejaVu Serif' -V monofont='DejaVu Sans Mono' -V colorlinks=false -o RESULTS.tex
	node ../../execution-v8/support/break-report-hashes.mjs RESULTS.tex

RESULTS.pdf: RESULTS.tex CONNECTION.pdf
	xelatex -interaction=nonstopmode -halt-on-error RESULTS.tex > render-make-v1.stdout 2> render-make-v1.stderr
	xelatex -interaction=nonstopmode -halt-on-error RESULTS.tex > render-make-v2.stdout 2> render-make-v2.stderr

.PHONY: check
check: RESULTS.pdf
	! rg 'Overfull|Missing character|undefined references|LaTeX Error' RESULTS.log
	test -s RESULTS.pdf
