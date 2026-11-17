#!/bin/bash
# Render README.md → report.typ → report.pdf (Pandoc + Typst from the scratch venv),
# with the generated tables and figures of RUN_DIR spliced in.  Layout only.
set -euo pipefail
cd "$(dirname "$0")"
RUN=${1:-runs/run-20261018T0857Z}
PY=${PY:-/tmp/claude-0/-home-user/6228277d-9258-5be8-8b7e-87e18d717e78/scratchpad/venv/bin/python}
$PY make_report.py "$RUN" > "$RUN/tables.md"
$PY - "$RUN" <<'PYEOF'
import sys, re
from pathlib import Path
run = Path(sys.argv[1])
src = Path("README.md").read_text()
tables = (run / "tables.md").read_text()
headline = (run / "HEADLINE.md").read_text() if (run / "HEADLINE.md").exists() else ""
reading = (run / "READING.md").read_text() if (run / "READING.md").exists() else ""
out = src.replace("<!-- HEADLINE -->", headline).replace("<!-- TABLES -->", "## Results\n\n" + tables).replace("<!-- READING -->", reading)
Path("REPORT.md").write_text(out)
PYEOF
pandoc REPORT.md -o report.typ
sed -i '1i #set page(paper: "a4", margin: 1.4cm)\n#set text(size: 8pt)\n#set table(stroke: 0.4pt)\n#show table: set text(size: 5.2pt)\n#show image: it => block(width: 100%, it)' report.typ
$PY -c "import typst; typst.compile('report.typ', output='report.pdf')"
$PY - <<'PYEOF'
from pypdf import PdfReader
r = PdfReader("report.pdf")
print("report.pdf pages:", len(r.pages))
PYEOF
