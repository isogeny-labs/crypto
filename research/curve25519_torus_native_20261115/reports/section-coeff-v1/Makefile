.PHONY: check
# Inspect the qualified outputs. Rebuilding them creates a new report version.
check:
	sha256sum --check REPORT_SHA256.txt
	pdfinfo report.pdf | sed -n '/Pages:/p'
	! rg 'Overfull|LaTeX Error' renderer-final.log
