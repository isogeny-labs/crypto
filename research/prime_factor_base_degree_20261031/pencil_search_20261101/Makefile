REPORT_PYTHON ?= /tmp/n83-report-venv/bin/python
.PHONY: all check
all: reports/report.pdf
reports/report.pdf: reports/report.typ reports/construction.svg
	$(REPORT_PYTHON) -c "import typst; typst.compile('reports/report.typ', output='reports/report.pdf')"
check: reports/report.pdf
	pdfinfo reports/report.pdf
	$(REPORT_PYTHON) -c "import xml.etree.ElementTree as E; E.parse('reports/construction.svg')"
