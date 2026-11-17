#!/bin/sh
set -eu
cd "$(dirname "$0")"
/home/ec2-user/primefield-ecdlp-research-20261028/.venv/bin/python -c 'import typst; typst.compile("report.typ",output="report.pdf")'
