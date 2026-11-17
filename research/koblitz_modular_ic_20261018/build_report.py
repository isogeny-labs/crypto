"""Document layout only: renders recorded values and Markdown; no research calculations."""
from pathlib import Path
import json
import re
import typst
import pymupdf

HERE = Path(__file__).resolve().parent
data = json.loads((HERE / "results.json").read_text())
baseline = data["baseline_131"]["genus_split_cusps"]
signed = next(r for r in data["best"] if r["n"] == 131 and r["character_order"] == 262)
# Scaling recorded values to drawing coordinates is layout, not a cost model.
width = 740 * data["signed_131_genus_ratio"]
svg = f'''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="720" viewBox="0 0 1000 720">
<rect width="1000" height="720" fill="#f8fafc"/>
<g font-family="DejaVu Sans,sans-serif" fill="#172438">
<text x="36" y="43" font-size="25" font-weight="bold">Koblitz modular carriers: what the evidence supplies</text>
<text x="36" y="74" font-size="16">n = 131, a = 0 · structural results · no measured IC cost</text>
<rect x="36" y="104" width="928" height="90" rx="9" fill="#deebff" stroke="#4777ba"/>
<text x="56" y="136" font-size="19">Signed character modulo 787 → modular Jacobian factor</text>
<text x="56" y="169" font-size="17">Exact Frobenius match · desired dimension 130 · maps not evaluated</text>
<text x="36" y="237" font-size="21" font-weight="bold">Carrier genus — derived, not timed</text>
<text x="36" y="271" font-size="16">Historical recipe: {baseline:,}</text>
<rect x="36" y="284" width="740" height="31" fill="#8799ae"/>
<text x="36" y="345" font-size="16">Signed variant: {signed['genus']:,}</text>
<rect x="36" y="358" width="{width:.2f}" height="31" fill="#286fb8"/>
<text x="36" y="421" font-size="16">Recorded ratio 0.7443181823 · historical IC model window: genus 130–300</text>
<path d="M 500 442 L 500 468" stroke="#61748e" stroke-width="3"/>
<rect x="36" y="476" width="448" height="104" rx="8" fill="#fff0d6" stroke="#bd8b39"/>
<text x="54" y="506" font-size="18" font-weight="bold">Executable IC bridge: open</text>
<text x="54" y="535" font-size="15">Subgroup maps, relations, descent, rank,</text>
<text x="54" y="559" font-size="15">and complete matched costs are missing.</text>
<rect x="516" y="476" width="448" height="104" rx="8" fill="#e7edf4" stroke="#61748e"/>
<text x="534" y="506" font-size="18" font-weight="bold">Degree-131 cyclic covers</text>
<text x="534" y="535" font-size="15">Written bound: genus ≥ 394 for any base.</text>
<text x="534" y="559" font-size="15">Numerical implication checked in Lean.</text>
<text x="36" y="621" font-size="18" font-weight="bold">Still open: noncyclic low-genus carriers and other relation algorithms</text>
<text x="36" y="656" font-size="14">Source: results.json, cyclic-window.json, REPORT.md · 2026-10-18</text>
<text x="36" y="681" font-size="14">The geometry is a written argument, not a fully formalized or independently reviewed proof.</text>
</g></svg>'''
(HERE / "routes.svg").write_text(svg)

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

lines = (HERE / "REPORT.md").read_text().splitlines()
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
        for row in rows:
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
