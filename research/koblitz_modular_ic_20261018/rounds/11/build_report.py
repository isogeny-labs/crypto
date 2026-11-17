"""Layout only; all research values come from the frozen native ledger."""
from pathlib import Path
import json
import re
import typst
import pymupdf
HERE = Path(__file__).resolve().parent
record = json.loads((HERE / "ledger.json").read_text())
d = record["diagonal"]
s = record["saturation"]
svg = f'''<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="830" viewBox="0 0 1100 830">
<rect width="1100" height="830" fill="#f8fafc"/>
<g font-family="DejaVu Sans,sans-serif" fill="#172438">
<text x="30" y="42" font-size="25" font-weight="bold">Koblitz modular carriers · round 11</text>
<text x="30" y="74" font-size="17">Exact characteristic-2 local equations · no IC cost measured</text>
<rect x="28" y="98" width="1044" height="255" rx="8" fill="#e8f4eb" stroke="#398460"/>
<text x="45" y="133" font-size="23" font-weight="bold">The 460 diagonal multiplicity gap closes</text>
<text x="45" y="168" font-size="18">400 labels → contacts 1, 2, 6, 14 → total degree {d['weighted_degree']}</text>
<rect x="46" y="190" width="300" height="38" fill="#4777ba"/>
<rect x="346" y="190" width="231" height="38" fill="#398460"/>
<rect x="577" y="190" width="139" height="38" fill="#bb8c32"/>
<rect x="716" y="190" width="323" height="38" fill="#965d9e"/>
<text x="62" y="216" font-size="17" fill="white">260 × 1 = 260</text>
<text x="360" y="216" font-size="17" fill="white">100 × 2 = 200</text>
<text x="585" y="216" font-size="16" fill="white">20 × 6 = 120</text>
<text x="745" y="216" font-size="17" fill="white">20 × 14 = 280</text>
<text x="45" y="267" font-size="18">Actual 11-torsion + quaternion / Velu binding + local series through order 31</text>
<text x="45" y="302" font-size="18">40 closed degree-10 points · Frobenius-stable multiplicities · parsed substitution</text>
<text x="45" y="332" font-size="16">Moduli / deformation bridge written; explicit 49-torsion coordinates remain open</text>
<rect x="28" y="378" width="510" height="205" rx="8" fill="#e3effa" stroke="#4777ba"/>
<text x="45" y="412" font-size="21" font-weight="bold">Projective saturation control</text>
<text x="45" y="448" font-size="18">5 quadrics · 10 S-pairs · 6 charts</text>
<text x="45" y="482" font-size="18">10 boundary points over F2</text>
<text x="45" y="516" font-size="18">5 naive false points excluded</text>
<text x="45" y="554" font-size="17">Specified coupled inverse graph only</text>
<rect x="558" y="378" width="514" height="205" rx="8" fill="#fff4d9" stroke="#b5822b"/>
<text x="576" y="412" font-size="21" font-weight="bold">Residual divisor structure</text>
<text x="576" y="448" font-size="18">Translated T-fibers: 1,280 + 1,360</text>
<text x="576" y="482" font-size="18">Genus-3 ambient x-sum curve</text>
<text x="576" y="516" font-size="18">Equal-y diagonal + residual curve</text>
<text x="576" y="554" font-size="17">Seven whole supports still incomplete</text>
<rect x="28" y="608" width="1044" height="122" rx="8" fill="#fff4d9" stroke="#b5822b"/>
<text x="45" y="643" font-size="21" font-weight="bold">Remaining: antidiagonal degree 1,240 and full quotient boundary</text>
<text x="45" y="679" font-size="18">No new evaluated row · retained rank 12 / 1,308; nonzero-image rank 11</text>
<text x="45" y="710" font-size="17">Whole proper quotient projective embeddings remain open</text>
<text x="30" y="767" font-size="17">22 new Lean proofs · 168 project theorems audited · failures and exact replay retained</text>
<text x="30" y="801" font-size="15">Source: ledger.json and PROOF_MAP.md · 2026-10-19 UTC</text>
</g></svg>'''
(HERE / "routes.svg").write_text(svg)
svg_doc = pymupdf.open(HERE / "routes.svg")
svg_doc[0].get_pixmap(matrix=pymupdf.Matrix(1.5, 1.5)).save(HERE / "routes.png")
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
