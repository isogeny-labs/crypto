#!/usr/bin/env python3
"""Render the evidence report with compact, readable tables via Pandoc/Typst."""

import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parent


def main():
    with tempfile.TemporaryDirectory(prefix="ghs-report-") as directory:
        source = Path(directory) / "report.typ"
        subprocess.run(
            ["pandoc", "-s", "-t", "typst", str(ROOT / "REPORT.md"),
             "-o", str(source), "-V", "papersize=us-letter"],
            check=True,
        )
        document = source.read_text()
        replacements = {
            "inset: 6pt": "inset: 2pt",
            "margin: (x: 1.25in, y: 1.25in)":
                "margin: (x: 0.52in, y: 0.65in)",
            "fontsize: 11pt": "fontsize: 9pt",
            "#let horizontalRule":
                "#show table: set text(size: 7pt)\n\n#let horizontalRule",
            "== GF(8)/GF(512) extension":
                "#pagebreak()\n\n== GF(8)/GF(512) extension",
        }
        for original, replacement in replacements.items():
            if document.count(original) != 1:
                raise ValueError(f"unexpected Pandoc Typst output: {original}")
            document = document.replace(original, replacement)
        source.write_text(document)
        subprocess.run(["typst", "compile", str(source),
                        str(ROOT / "REPORT.pdf")], check=True)


if __name__ == "__main__":
    main()
