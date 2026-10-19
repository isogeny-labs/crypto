"""Layout only: render frozen round06 results, prose and proof map."""
from pathlib import Path
import json
import re
import typst
import pymupdf
HERE = Path(__file__).resolve().parent
record = json.loads((HERE / "ledger.json").read_text())
svg = '''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="650" viewBox="0 0 1000 650">
<rect width="1000" height="650" fill="#f8fafc"/>
<g font-family="DejaVu Sans,sans-serif" fill="#172438">
<text x="28" y="38" font-size="25" font-weight="bold">Koblitz carriers · round 06 · evaluated constructions</text>
<rect x="28" y="58" width="944" height="142" rx="8" fill="#deebff" stroke="#4777ba"/>
<text x="48" y="88" font-size="21" font-weight="bold">44 degree-210 columns → zero images for both signs</text>
<text x="48" y="118" font-size="17">616 nonzero geometric tuples per sign → relative 14-traces all zero</text>
<text x="48" y="147" font-size="17">8 fibers × 7 choices · each of 12 target sums = order-2 cusp T</text>
<text x="48" y="177" font-size="17">Evaluated rank 5 / 588 · nonzero-image ranks stay 4 / 510 and 4 / 520</text>
<rect x="28" y="218" width="944" height="127" rx="8" fill="#eee8f7" stroke="#8770a8"/>
<text x="48" y="250" font-size="21" font-weight="bold">10 / 10 determinant-5 graphs → explicit quotients</text>
<text x="48" y="280" font-size="17">Three split E^5 models · effective F2 descent · principal polarizations</text>
<text x="48" y="310" font-size="17">250 kernel points checked · determinant-19 quotients remain OPEN</text>
<rect x="28" y="363" width="944" height="157" rx="8" fill="#e1f1e8" stroke="#398460"/>
<text x="48" y="395" font-size="21" font-weight="bold">Even reverse divisor → all 20 fibers in coordinates</text>
<text x="48" y="425" font-size="17">Positive seed: 660 split points · negative seed: 60 degree-11 algebras</text>
<text x="48" y="455" font-size="17">7,200 entries · positive / negative degree 6,600 · F2 descent checked</text>
<text x="48" y="486" font-size="17">Forward check uses Gram · pointwise norms / Jacobian reduction OPEN</text>
<text x="28" y="558" font-size="18">14 new named Lean results · ring / descent / group / matrix proofs</text>
<text x="28" y="588" font-size="16">Source: ledger.json + PROOF_MAP.md · global geometric bridges written separately</text>
<text x="28" y="620" font-size="16">Same-agent fresh replay and separate checks · no IC speed measurement</text>
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
