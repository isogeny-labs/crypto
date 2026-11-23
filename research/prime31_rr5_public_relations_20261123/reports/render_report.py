"""Layout-only rendering of the already written report; no research computation.

Run pandoc first (see build-report.sh). Requires the typst Python wheel.
"""
from pathlib import Path
import os
import typst

study = Path(__file__).resolve().parent.parent
os.chdir(study)
try:
    typst.compile(
        (study / "reports/rr5-public-relations.typ").read_bytes(),
        root=str(study),
        output=str(study / "reports/rr5-public-relations.pdf"),
    )
except typst.TypstError as error:
    print(error.diagnostic)
    raise
