from pathlib import Path
import typst
r=Path(__file__).resolve().parent
typst.compile(str(r/"report.typ"),output=str(r/"report.pdf"))
