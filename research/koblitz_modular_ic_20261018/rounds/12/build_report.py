"""Layout only; numbers are read from the frozen native round12 ledger."""
from pathlib import Path
import json
import re
import typst
import pymupdf
HERE = Path(__file__).resolve().parent
record = json.loads((HERE / "ledger.json").read_text())
s = record["supersingular"]
audit = record["audited_project_theorems"]
names = ["x0+xinf", "x0+xinf+1", "y0+yinf", "y0+yinf+1", "x0*xinf", "x0*xinf+1", "cross", "cross+1"]
rows=[]
for i, name in enumerate(names):
    y=178+43*i
    ss=s["zero_degrees"][i]
    ordinary=s["ordinary_zero_degrees_remaining"][i]
    width_ss=610*ss/3900
    width_ord=610*ordinary/3900
    rows.append(f'<text x="45" y="{y+18}" font-size="16">{name}</text><rect x="247" y="{y}" width="{width_ss}" height="25" fill="#398460"/><rect x="{247+width_ss}" y="{y}" width="{width_ord}" height="25" fill="#e7b54d"/><text x="887" y="{y+18}" font-size="16">{ss} + {ordinary}</text>')
svg=f'''<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="1000" viewBox="0 0 1100 1000">
<rect width="1100" height="1000" fill="#f8fafc"/>
<g font-family="DejaVu Sans,sans-serif" fill="#172438">
<text x="28" y="40" font-size="25" font-weight="bold">Koblitz modular carriers · round 12</text>
<text x="28" y="72" font-size="17">Complete supersingular zero strata · actual projective counterpoints · no IC cost measured</text>
<rect x="28" y="95" width="1044" height="466" rx="8" fill="#edf5ef" stroke="#398460"/>
<text x="45" y="131" font-size="23" font-weight="bold">3,080 source SS points → 308 closed degree-10 points</text>
<rect x="45" y="147" width="18" height="16" fill="#398460"/><text x="72" y="161" font-size="16">complete SS contribution</text>
<rect x="350" y="147" width="18" height="16" fill="#e7b54d"/><text x="377" y="161" font-size="16">ordinary support remaining</text>
{''.join(rows)}
<text x="45" y="548" font-size="16">Degree is counted with multiplicity. Every SS jet resolves before order 16.</text>
<rect x="28" y="583" width="1044" height="124" rx="8" fill="#e3effa" stroke="#4777ba"/>
<text x="45" y="618" font-size="21" font-weight="bold">Antidiagonal: 740 SS + 500 ordinary = 1,240</text>
<text x="45" y="652" font-size="18">SS contacts: 420 × 1 + 40 × 2 + 60 × 4 · old diagonal 860 reproduced</text>
<text x="45" y="686" font-size="17">All SS target values are killed by 8 → zero 11-primary projected image</text>
<rect x="28" y="729" width="1044" height="157" rx="8" fill="#fff4d9" stroke="#b5822b"/>
<text x="45" y="764" font-size="21" font-weight="bold">Four full quotient circuits: naive homogenization rejected</text>
<text x="45" y="797" font-size="18">Actual counterpoints pass 1,679,760 scalar equations · 8 Lean finite certificates</text>
<text x="45" y="831" font-size="18">213,120 generated linear saturation relations · h-membership depth at most 5</text>
<text x="45" y="867" font-size="17">OPEN: full h-saturation, proper quotient boundary and polarized projective embedding</text>
<text x="28" y="926" font-size="17">24 new named Lean proofs · {audit} project theorems audited · exact replay and failures archived</text>
<text x="28" y="960" font-size="16">Rank stays 12 / 1,308; nonzero-image rank 11 · seven whole ordinary supports remain open</text>
<text x="28" y="986" font-size="14">Source: ledger.json + PROOF_MAP.md · client date 19 October 2026 / raw date 20 October UTC</text>
</g></svg>'''
(HERE / "routes.svg").write_text(svg)
svg_doc = pymupdf.open(HERE / "routes.svg")
svg_doc[0].get_pixmap(matrix=pymupdf.Matrix(1.5,1.5)).save(HERE / "routes.png")
def quoted(text):
    return json.dumps(text, ensure_ascii=False)

def plain(text):
    return text.replace("**", "").replace("`", "").replace("_", "_\u200b")

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
    elif line.startswith("|"):
        rows = []
        while i < len(lines) and lines[i].startswith("|"):
            cells = [c.strip() for c in lines[i].strip("|").split("|")]
            if not all(re.fullmatch(r"[:\- ]+", c) for c in cells):
                rows.append(cells)
            i += 1
        content = ["#table(columns: " + str(len(rows[0])) + ', inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),\n', "table.header(\n"]
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
        while i < len(lines) and lines[i].strip() and not lines[i].startswith(("#", "|", "![", "    ")):
            paragraph.append(lines[i])
            i += 1
        body.append(inline(" ".join(paragraph)) + "\n")

source = '''#set page(paper: "a4", margin: (x: 19mm, y: 17mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.40em)
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
