#!/usr/bin/env python3
"""Presentation compilation; install pinned typst from report-tools.json."""
from pathlib import Path
import typst
HERE=Path(__file__).resolve().parent
typst.compile(str(HERE/'reports/report.typ'),output=str(HERE/'reports/report.pdf'))
