#!/usr/bin/env python3
"""Bind the rendered PDF's Subject to the report's exact coverage marker."""

import argparse
from pathlib import Path
import re

from pypdf import PdfReader, PdfWriter, __version__ as pypdf_version


HERE = Path(__file__).resolve().parent
REPORT = HERE / "README.md"
PDF = HERE / "cover_transfer_assessment_20261013.pdf"

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--pdf-engine", choices=("tectonic", "xelatex"), default="tectonic")
args = parser.parse_args()

match = re.search(r'^subject: "(.*)"$', REPORT.read_text(), re.MULTILINE)
if match is None:
    raise SystemExit("report front matter has no subject")
subject = match.group(1)

reader = PdfReader(PDF)
writer = PdfWriter()
writer.clone_document_from_reader(reader)
writer.add_metadata(
    {
        "/Title": "Catalog-wide curve-cover transfer assessment",
        "/Subject": subject,
        "/Creator": f"Pandoc, {args.pdf_engine}, pypdf {pypdf_version}",
        "/CreationDate": "D:20261021000000Z",
        "/ModDate": "D:20261021000000Z",
    }
)
temp = PDF.with_suffix(".pdf.tmp")
with temp.open("wb") as stream:
    writer.write(stream)
if PdfReader(temp).metadata.subject != subject:
    temp.unlink()
    raise SystemExit("rendered PDF subject does not match report front matter")
temp.replace(PDF)
