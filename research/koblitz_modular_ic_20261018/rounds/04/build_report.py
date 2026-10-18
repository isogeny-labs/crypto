"""Layout only. Plot frozen ledger values; render report and proof map."""
from pathlib import Path
import json
import re
import typst
import pymupdf
HERE = Path(__file__).resolve().parent
record = json.loads((HERE / "ledger.json").read_text())
r = record["relations"]
q = record["quotients"]
t = record["transport"]
svg = f'''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="845" viewBox="0 0 1000 845">
<rect width="1000" height="845" fill="#f8fafc"/>
<g font-family="DejaVu Sans,sans-serif" fill="#172438">
<text x="30" y="40" font-size="24" font-weight="bold">Koblitz constructions · round 04</text>
<text x="30" y="69" font-size="16">Actual quotient and modular-divisor evaluations · no timing measurements</text>
<rect x="30" y="94" width="940" height="112" rx="8" fill="#eee8f7" stroke="#8770a8"/>
<text x="50" y="124" font-size="19" font-weight="bold">A5(E_a) × E_a → Res(E_a) over F2 · q(a,Q)=a−diag(Q)</text>
<text x="50" y="153" font-size="16">Natural determinant-5 quotient: equations, rational inverse, 25-point kernels</text>
<text x="50" y="180" font-size="16">14 integral quotient modules · both determinant-19 forms have global minimum 3</text>
<text x="50" y="199" font-size="14">Other individual graph equations and polarized decomposition remain OPEN</text>
<rect x="30" y="230" width="940" height="113" rx="8" fill="#deebff" stroke="#4777ba"/>
<text x="50" y="261" font-size="19" font-weight="bold">C = X_H(5929), genus 2,841 → A5(E_a)(F2), order 11</text>
<text x="50" y="290" font-size="16">{t['supported_sources']} supported F64 base inputs · twelve actual Hecke targets each · T11=[4]</text>
<text x="50" y="318" font-size="16">Paired bases + canonical Frobenius: {r['geometric_points']:,} geometric points → {r['columns']} closed columns</text>
<text x="50" y="336" font-size="14">Finite forward divisor projection; global geometric bridges / reverse transfer remain partial</text>
<text x="30" y="380" font-size="18" font-weight="bold">Nonzero projected columns (same population: 544 modular closed points)</text>
<text x="30" y="417" font-size="16">E1: y²+xy=x³+x²+1</text>
<rect x="300" y="397" width="544" height="27" fill="#e2e8f0"/><rect x="300" y="397" width="{r['nonzero_images_by_sign'][0]}" height="27" fill="#398460"/>
<text x="865" y="417" font-size="16">{r['nonzero_images_by_sign'][0]}/544</text>
<text x="30" y="458" font-size="16">E0: y²+xy=x³+1</text>
<rect x="300" y="438" width="544" height="27" fill="#e2e8f0"/><rect x="300" y="438" width="{r['nonzero_images_by_sign'][1]}" height="27" fill="#398460"/>
<text x="865" y="458" font-size="16">{r['nonzero_images_by_sign'][1]}/544</text>
<text x="30" y="495" font-size="15">Closed degrees: 20 × degree 6; 524 × degree 30. Both signs use F32 coordinate descent.</text>
<rect x="30" y="524" width="940" height="110" rx="8" fill="#fff0d6" stroke="#bd8b39"/>
<text x="50" y="554" font-size="19" font-weight="bold">{r['principal_rows']} full-fiber principal modular rows · rank {r['rank_mod11']} / 544 modulo 11</text>
<text x="50" y="583" font-size="16">Four closed base fibers force rank ≤ 4; {r['missing_rank']} additional ranks to unique scale</text>
<text x="50" y="612" font-size="16">Level function x(P)+1: zero degree 9,240; none represented → row REJECTED</text>
<rect x="30" y="659" width="940" height="73" rx="8" fill="#e1f1e8" stroke="#398460"/>
<text x="50" y="688" font-size="17">Separate checker: 1,088 point images, 102 rows, BigInt rank, kernels and Hecke sums</text>
<text x="50" y="716" font-size="16">11 new named Lean theorems · kernel + axiom audit PASS</text>
<text x="30" y="771" font-size="15">Next: nonnatural quotient equations · reverse map / saturation · missing interior divisors</text>
<text x="30" y="801" font-size="14">Sources: ledger.json, PROOF_MAP.md, runs.tar.xz. Full scheme/Picard formalization partial.</text>
<text x="30" y="827" font-size="14">No individual descent or scalar recovery attempted; user assesses IC implications.</text>
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
