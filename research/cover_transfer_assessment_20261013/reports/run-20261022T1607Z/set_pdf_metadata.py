#!/usr/bin/env python3
"""Set the exact report coverage marker as the PDF subject."""

from pathlib import Path
import re

from pypdf import PdfReader, PdfWriter

HERE = Path(__file__).resolve().parent
REPORT = HERE / "REPORT.md"
PDF = HERE / "report.pdf"

match = re.search(r'^subject: "(.*)"$', REPORT.read_text(), re.MULTILINE)
if match is None:
    raise SystemExit("report front matter has no subject")
subject = match.group(1)

reader = PdfReader(PDF)
writer = PdfWriter()
writer.clone_document_from_reader(reader)
writer.add_metadata(
    {
        "/Title": "Expanded curve-cover catalog assessment",
        "/Subject": subject,
        "/Creator": "Tectonic and pypdf 6.0.0",
        "/CreationDate": "D:20261023000000Z",
        "/ModDate": "D:20261023000000Z",
    }
)
temp = PDF.with_suffix(".pdf.tmp")
with temp.open("wb") as stream:
    writer.write(stream)
if PdfReader(temp).metadata.subject != subject:
    temp.unlink()
    raise SystemExit("PDF subject does not match report front matter")
temp.replace(PDF)
