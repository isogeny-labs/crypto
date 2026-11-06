"""Document layout only: render recorded quantities, prose and proof map."""
from pathlib import Path
import json
import re
import typst
import pymupdf
HERE = Path(__file__).resolve().parent
record = json.loads((HERE / "ledger.json").read_text())
p = record["polarizations"]
n = record["n5"]
svg = f'''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="820" viewBox="0 0 1000 820">
<rect width="1000" height="820" fill="#f8fafc"/>
<g font-family="DejaVu Sans,sans-serif" fill="#172438">
<text x="30" y="40" font-size="24" font-weight="bold">Koblitz carriers · round 03 evidence</text>
<text x="30" y="70" font-size="16">Exact finite experiments · partial formal proofs · no timing measurements</text>
<rect x="30" y="95" width="940" height="100" rx="8" fill="#fff0d6" stroke="#bd8b39"/>
<text x="50" y="126" font-size="19" font-weight="bold">Full A5 factor over F2 · conditional small-genus exclusion</text>
<text x="50" y="155" font-size="16">{record['dp3']['forms_tested']:,} remaining del Pezzo sections: no matches</text>
<text x="50" y="181" font-size="16">Genus 4 count obstructions + published genus 5 / 6 censuses exclude genus ≤ 6*</text>
<rect x="30" y="219" width="940" height="125" rx="8" fill="#eee8f7" stroke="#8770a8"/>
<text x="50" y="249" font-size="19" font-weight="bold">Integral polarized gluing over F2 · finite kernels verified</text>
<text x="50" y="279" font-size="16">{p['forms_tested']:,} forms → {p['positive_forms']} positive → 24 retained sign-specific candidates</text>
<text x="50" y="305" font-size="16">Determinant 5: 10 graph cases / 60 anti-isometries; determinant 19: 4 / 80</text>
<text x="50" y="331" font-size="16">Quotient equations / Jacobian realization OPEN; short-vector search bounded</text>
<rect x="30" y="368" width="940" height="94" rx="8" fill="#deebff" stroke="#4777ba"/>
<text x="50" y="399" font-size="19" font-weight="bold">Same-field n=5 control · E_a / F32 ← degree 3 ← C_a / F32</text>
<text x="50" y="429" font-size="16">C_a: v² + u²v = u⁷ + a u⁴ + u; quotient (u,v) ↦ (u³,uv+1)</text>
<text x="50" y="452" font-size="15">All order-11 subgroup points transferred; no F2 modular-Jacobian evaluator claimed</text>
<text x="30" y="495" font-size="18" font-weight="bold">a=1 relation collection: matrix rank modulo 11 on 21 columns</text>
<text x="30" y="525" font-size="15">Family</text><text x="260" y="525" font-size="15">Rank (0 to 20; one scale remains)</text><text x="840" y="525" font-size="15">Rows</text>
<text x="30" y="555" font-size="15">Degree ≤ 3</text><rect x="260" y="538" width="260" height="22" fill="#bd8b39"/><text x="535" y="555" font-size="15">10</text><text x="840" y="555" font-size="15">10</text>
<text x="30" y="587" font-size="15">+ vertical functions</text><rect x="260" y="570" width="416" height="22" fill="#bd8b39"/><text x="691" y="587" font-size="15">16</text><text x="840" y="587" font-size="15">21</text>
<text x="30" y="619" font-size="15">+ monic quartics</text><rect x="260" y="602" width="416" height="22" fill="#bd8b39"/><text x="691" y="619" font-size="15">16</text><text x="840" y="619" font-size="15">21</text>
<text x="30" y="651" font-size="15">+ Hermite quartics</text><rect x="260" y="634" width="520" height="22" fill="#398460"/><text x="795" y="651" font-size="15">{n[1]['rank']}</text><text x="840" y="651" font-size="15">{n[1]['relations']}</text>
<rect x="30" y="678" width="940" height="65" rx="8" fill="#e1f1e8" stroke="#398460"/>
<text x="50" y="704" font-size="17">Final: a=0 rank {n[0]['rank']}/{n[0]['columns']}; a=1 rank {n[1]['rank']}/{n[1]['columns']} → 22/22 fixtures verified</text>
<text x="50" y="730" font-size="16">1,851 principal rows checked separately; 13 new named Lean lemmas audited</text>
<text x="30" y="774" font-size="14">*Assumes published census correctness/completeness; only full A5 Jacobian carriers over F2.</text>
<text x="30" y="799" font-size="14">Sources: ledger.json, REPORT.md, PROOF_MAP.md, runs.tar.xz · user assesses IC implications.</text>
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
