"""Layout only: render frozen ledger values, report and proof map."""
from pathlib import Path
import json
import re
import typst
import pymupdf
HERE = Path(__file__).resolve().parent
record = json.loads((HERE / "ledger.json").read_text())
bars = []
for y, label, rank, count in [
    (124, "Round 07", 6, "675 / 678"),
    (169, "Five-line family M10", record["relations"]["M10"]["nonzero"][0]["rank"], "1115 / 1110"),
    (214, "Add fixed line infinity", record["relations"]["M11"]["nonzero"][0]["rank"], "1165 / 1160"),
]:
    bars.append(f'<text x="42" y="{y+18}" font-size="17">{label}</text>'
                f'<rect x="300" y="{y}" width="{rank*33}" height="27" fill="#3778ad"/>'
                f'<text x="{315+rank*33}" y="{y+20}" font-size="18">rank {rank} · {count} columns</text>')
charts = ''.join(f'<rect x="{43+165*k}" y="419" width="145" height="43" rx="4" fill="#d9ece2" stroke="#398460"/><text x="{67+165*k}" y="446" font-size="17">chart k = {k}</text>' for k in range(6))
svg = '''<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="790" viewBox="0 0 1100 790">
<rect width="1100" height="790" fill="#f8fafc"/>
<g font-family="DejaVu Sans,sans-serif" fill="#172438">
<text x="28" y="38" font-size="25" font-weight="bold">Koblitz modular carriers · round 08 · exact constructions</text>
<rect x="28" y="58" width="1044" height="236" rx="8" fill="#e3effa" stroke="#4777ba"/>
<text x="42" y="89" font-size="21" font-weight="bold">Nonzero-image relation rank · both signs · F11</text>
<text x="42" y="112" font-size="15">Exact finite ranks; a=1 / a=0 populations shown; no timing or IC assessment</text>
''' + ''.join(bars) + '''
<text x="42" y="276" font-size="17">Final evaluated rank 12 / 1308 · 540 additional columns after round 07</text>
<rect x="28" y="312" width="1044" height="214" rx="8" fill="#eaf4ee" stroke="#398460"/>
<text x="43" y="347" font-size="21" font-weight="bold">Four determinant-19 quotients → intrinsic Kummer equations</text>
<text x="43" y="377" font-size="18">Two equations per chart: z_i^19 = product_j (A_ij(x_j) + B_ij(x_j)y_j)</text>
<text x="43" y="404" font-size="17">40 polynomial Miller functions · degrees A=9, B=8 · splitting field F2^360</text>
''' + charts + '''
<text x="43" y="491" font-size="17">Five factors exclude at most five charts → six cover; 200 exact transition identities</text>
<rect x="28" y="546" width="1044" height="108" rx="8" fill="#eee8f7" stroke="#8770a8"/>
<text x="43" y="580" font-size="20" font-weight="bold">16 named Lean results · 17 audited declarations · no native trust</text>
<text x="43" y="611" font-size="17">720 valuations · 640 Miller values · 100 curve identities · 400 point translations</text>
<rect x="28" y="674" width="1044" height="53" rx="5" fill="#fff4d9" stroke="#b5822b"/>
<text x="43" y="707" font-size="18" font-weight="bold">OPEN: effective F2 coordinates, projective embedding, Jacobian identification</text>
<text x="28" y="757" font-size="15">Source: ledger.json + PROOF_MAP.md · same-agent fresh replay and separate checks · 2026-10-19 UTC</text>
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
