"""Layout only: render the frozen round05 ledger, report and proof map."""
from pathlib import Path
import json
import re
import typst
import pymupdf
HERE = Path(__file__).resolve().parent
record = json.loads((HERE / "ledger.json").read_text())
r = record["reverse"]
q = record["quotients"]
m = record["interior"]
svg = f'''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="680" viewBox="0 0 1000 680">
<rect width="1000" height="680" fill="#f8fafc"/>
<g font-family="DejaVu Sans,sans-serif" fill="#172438">
<text x="30" y="37" font-size="25" font-weight="bold">Koblitz carriers · round 05 · exact constructions</text>
<rect x="30" y="60" width="940" height="105" rx="8" fill="#deebff" stroke="#4777ba"/>
<text x="50" y="90" font-size="20" font-weight="bold">A5 → Jac(C) → A5 = [{r['composition_scalar']}] · scalar mod 11 = 1</text>
<text x="50" y="121" font-size="17">Integral polarized adjoint · H = I + ones · written 11-primary splitting</text>
<text x="50" y="149" font-size="16">Reverse pullback formula: positive and negative degree ≤ {r['positive_reverse_degree_bound']}</text>
<rect x="30" y="185" width="940" height="88" rx="8" fill="#eee8f7" stroke="#8770a8"/>
<text x="50" y="216" font-size="19" font-weight="bold">{q['equivalent_forms']} / {q['forms_tested']} determinant-5 forms → canonical quotient equations</text>
<text x="50" y="246" font-size="16">Frozen first graphs: {q['frozen_graph_pm_identity_matches']} / {q['frozen_graphs_tested']} natural matches; determinant-19 equations OPEN</text>
<rect x="30" y="293" width="940" height="105" rx="8" fill="#e1f1e8" stroke="#398460"/>
<text x="50" y="324" font-size="19" font-weight="bold">X(P)+1 · complete written divisor of degree {m['ordinary_zero_degree']:,}</text>
<text x="50" y="354" font-size="17">44 ordinary closed zeros × degree 210; 3,080 supersingular poles × order 3</text>
<text x="50" y="382" font-size="16">Pole images killed by 8 and 11 → zero · new zero images UNEVALUATED</text>
<text x="30" y="433" font-size="19" font-weight="bold">Projected relation rank mod 11 · different column populations</text>
<text x="30" y="468" font-size="16">Evaluated · 544 columns</text>
<rect x="350" y="447" width="330" height="29" fill="#e2e8f0"/>
<rect x="350" y="447" width="{m['evaluated_matrix_rank']*55}" height="29" fill="#4777ba"/>
<text x="700" y="468" font-size="17">{m['evaluated_matrix_rank']}</text>
<text x="30" y="512" font-size="16">Symbolic · 588 columns</text>
<rect x="350" y="491" width="330" height="29" fill="#e2e8f0"/>
<rect x="350" y="491" width="{m['rank_after']*55}" height="29" fill="#bd8b39"/>
<text x="700" y="512" font-size="17">{m['rank_after']}</text>
<text x="30" y="557" font-size="16">18 new named Lean theorems · exact matrices / inverse pairings / torsion implications</text>
<text x="30" y="588" font-size="16">Geometry written separately · new44 coordinates / reverse divisor reduction OPEN</text>
<text x="30" y="624" font-size="15">Source: round05 ledger.json + PROOF_MAP.md · same-agent native replay and separate checks</text>
<text x="30" y="654" font-size="15">No timing measurements or IC speed verdict · no n=131 transfer claimed</text>
</g></svg>'''
(HERE / "routes.svg").write_text(svg)
svg_doc = pymupdf.open(HERE / "routes.svg")
svg_doc[0].get_pixmap(matrix=pymupdf.Matrix(1.5, 1.5)).save(HERE / "routes.png")
def quoted(text):
    return json.dumps(text, ensure_ascii=False)

def plain(text):
    return text.replace("**", "").replace("`", "")

def inline(text):
    pieces = []
    offset = 0
    for match in re.finditer(r"\[([^\]]+)\]\(([^)]+)\)", text):
        pieces.append("#text(" + quoted(plain(text[offset:match.start()])) + ")")
        pieces.append("#link(" + quoted(match[2]) + ")[#text(" + quoted(plain(match[1])) + ")]")
        offset = match.end()
    pieces.append("#text(" + quoted(plain(text[offset:])) + ")")
    return "".join(pieces)

lines = (HERE / "REPORT.md").read_text().splitlines() + ["", "# Formal proof map and written arguments", ""] + (HERE / "PROOF_MAP.md").read_text().splitlines()[1:]
body = []
i = 0
while i < len(lines):
    line = lines[i]
    if not line.strip():
        i += 1
        continue
    if line.startswith("!["):
        body.append('#image("routes.svg", width: 100%)\n')
        i += 1
    elif line.startswith("#"):
        heading = re.match(r"(#+)\s+(.*)", line)
        body.append(f"#heading(level: {len(heading[1])})[{inline(heading[2])}]\n")
        i += 1
    elif line.startswith("```"):
        code = []
        i += 1
        while i < len(lines) and not lines[i].startswith("```"):
            code.append(lines[i])
            i += 1
        body.append("#block(width: 100%, fill: rgb(\"f1f5f9\"), inset: 7pt)[#set text(size: 7.5pt)\n#raw(" + quoted("\n".join(code)) + ", block: true)]\n")
        i += 1
    elif line.startswith("|"):
        rows = []
        while i < len(lines) and lines[i].startswith("|"):
            cells = [c.strip() for c in lines[i].strip("|").split("|")]
            if not all(re.fullmatch(r"[:\- ]+", c) for c in cells):
                rows.append(cells)
            i += 1
        columns = len(rows[0])
        content = ["#table(columns: " + str(columns) + ', inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),\n']
        content.append("table.header(\n")
        content.extend("[" + inline(cell) + "],\n" for cell in rows[0])
        content.append("),\n")
        for row in rows[1:]:
            content.extend("[" + inline(cell) + "],\n" for cell in row)
        content.append(")\n")
        body.append("".join(content))
    elif line.startswith("    "):
        code = []
        while i < len(lines) and lines[i].startswith("    "):
            code.append(lines[i][4:])
            i += 1
        body.append("#block(inset: 5pt)[#set text(size: 8pt)\n#raw(" + quoted("\n".join(code)) + ", block: true)]\n")
    else:
        paragraph = [line]
        i += 1
        while i < len(lines) and lines[i].strip() and not lines[i].startswith(("#", "|", "```", "![", "    ")):
            paragraph.append(lines[i])
            i += 1
        body.append(inline(" ".join(paragraph)) + "\n")

source = '''#set page(paper: "a4", margin: (x: 19mm, y: 17mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.55em)
#set heading(numbering: none)
#show link: set text(fill: rgb("245c91"))
''' + "\n".join(body)
(HERE / "report.typ").write_text(source)
typst.compile(str(HERE / "report.typ"), output=str(HERE / "report.pdf"), root=str(HERE.parent.parent))
doc = pymupdf.open(HERE / "report.pdf")
review = HERE / "pdf-review"
review.mkdir(exist_ok=True)
for page in doc:
    page.get_pixmap(matrix=pymupdf.Matrix(1, 1)).save(review / f"page-{page.number+1:02}.png")
(HERE / "pdf-text.txt").write_text("\n\n".join(page.get_text() for page in doc))
print(f"Rendered {len(doc)} pages; inspect pdf-review/page-*.png")
