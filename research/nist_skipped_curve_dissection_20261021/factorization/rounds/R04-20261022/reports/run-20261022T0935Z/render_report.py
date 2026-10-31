"""Document layout only: render the accompanying Markdown report with Typst."""
import json
import posixpath
import re
from pathlib import Path

import typst

here = Path(__file__).resolve().parent
repo_path = here.relative_to(next(p for p in here.parents if (p / "AGENTS.md").exists()))
repo = "https://github.com/aburan28/crypto/blob/codex/nist-skipped-curve-jobs/"

def quoted(value):
    return json.dumps(value, ensure_ascii=False)

def inline(value):
    value = value.replace("**", "").replace("`", "")
    output, end = [], 0
    for match in re.finditer(r"\[([^\]]+)\]\(([^)]+)\)", value):
        output.append("#text(" + quoted(value[end:match.start()]) + ")")
        label, url = match.groups()
        if not url.startswith("https://"):
            url = repo + posixpath.normpath(str(repo_path) + "/" + url)
        output.append("#link(" + quoted(url) + ")[#text(" + quoted(label) + ")]")
        end = match.end()
    output.append("#text(" + quoted(value[end:]) + ")")
    return "".join(output)

lines = (here / "REPORT.md").read_text().splitlines()
out = ['#set page(paper: "a4", margin: 17mm, footer: align(right)[#context counter(page).display()])',
       '#set text(font: "DejaVu Sans", size: 8.5pt)',
       '#set par(leading: 0.5em)', '#set heading(numbering: none)',
       '#set table(stroke: 0.3pt + luma(80%), inset: 4pt)']
paragraph = []

def flush():
    if paragraph:
        out.append(inline(" ".join(paragraph)) + "\n")
        paragraph.clear()

i = 0
while i < len(lines):
    line = lines[i]
    if line.startswith("#"):
        flush()
        level = len(line) - len(line.lstrip("#"))
        out.append(f"#heading(level: {level})[" + inline(line.lstrip("# ")) + "]\n")
    elif line.startswith("!["):
        flush()
        path = re.search(r"\]\(([^)]+)\)", line).group(1)
        out.append("#image(" + quoted(path) + ", width: 100%)\n")
    elif line.startswith("```"):
        flush()
        code = []
        i += 1
        while i < len(lines) and not lines[i].startswith("```"):
            code.append(lines[i])
            i += 1
        out.append('#text(size: 5.5pt)[#raw(' + quoted("\n".join(code)) + ')]\n')
    elif line.startswith("|"):
        flush()
        rows = []
        while i < len(lines) and lines[i].startswith("|"):
            cells = [cell.strip() for cell in lines[i].strip("|").split("|")]
            if not all(re.fullmatch(r"[:\-]+", cell) for cell in cells):
                rows.append(cells)
            i += 1
        i -= 1
        out.append("#text(size: 8pt)[#table(columns: " + str(len(rows[0])) + ",\n" +
                   ",\n".join("[" + inline(cell) + "]" for row in rows for cell in row) + ")]\n")
    elif not line.strip():
        flush()
    else:
        paragraph.append(line)
    i += 1
flush()
(here / "report.typ").write_text("\n".join(out))
typst.compile(str(here / "report.typ"), output=str(here / "report.pdf"))
