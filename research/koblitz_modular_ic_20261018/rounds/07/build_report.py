"""Layout only: plot frozen ledger values and render report/proof-map prose."""
from pathlib import Path
import json
import re
import typst
import pymupdf

HERE = Path(__file__).resolve().parent
record = json.loads((HERE / "ledger.json").read_text())
first = record["relations"]["after_first"]
second = record["relations"]["after_second"]
bars = []
for y, label, rank, population in [
    (124, "Round 06", 4, "510 / 520"),
    (169, "First full-map norm", first["nonzero"][0]["rank"], "565 / 570"),
    (214, "Second full-map norm", second["nonzero"][0]["rank"], "675 / 678"),
]:
    bars.append(f'<text x="40" y="{y+18}" font-size="17">{label}</text>'
                f'<rect x="300" y="{y}" width="{rank*65}" height="27" fill="#3778ad"/>'
                f'<text x="{315+rank*65}" y="{y+20}" font-size="18">rank {rank} · {population} columns</text>')

svg = '''<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="840" viewBox="0 0 1100 840">
<rect width="1100" height="840" fill="#f8fafc"/>
<g font-family="DejaVu Sans,sans-serif" fill="#172438">
<text x="28" y="38" font-size="25" font-weight="bold">Koblitz modular carriers · round 07 · exact constructions</text>
<rect x="28" y="58" width="1044" height="236" rx="8" fill="#e3effa" stroke="#4777ba"/>
<text x="40" y="89" font-size="21" font-weight="bold">Nonzero-image relation rank · both signs · F11</text>
<text x="40" y="112" font-size="15">Separate populations for a=1 / a=0; no timing or uncertainty estimate applies</text>
''' + "".join(bars) + '''
<text x="40" y="276" font-size="16">Final all-column rank 7 / 768 · new column degrees 60 and 660</text>
<rect x="28" y="312" width="1044" height="166" rx="8" fill="#e1f1e8" stroke="#398460"/>
<text x="48" y="347" font-size="21" font-weight="bold">Odd reverse divisor → all 20 pullbacks in coordinates</text>
<text x="48" y="378" font-size="18">2,200 entries · positive / negative degree 6,600 · formal F2 descent</text>
<text x="48" y="408" font-size="18">Independent forward returns source · even divisor: 7,200 entries checked</text>
<text x="48" y="446" font-size="17">OPEN: reduced Jacobian arithmetic and higher 11-power reverse fixtures</text>
<rect x="28" y="496" width="1044" height="190" rx="8" fill="#eee8f7" stroke="#8770a8"/>
<text x="48" y="531" font-size="21" font-weight="bold">4 determinant-19 mixed quotient atlases · 1,444 kernel points</text>
<text x="48" y="562" font-size="18">Expanded [19] cofactor cover: degrees 361 / 360 and 540 / 540</text>
<text x="48" y="592" font-size="18">Exact curve identity · 722 kernel checks · 64 scalar comparisons</text>
<rect x="43" y="612" width="1014" height="49" rx="4" fill="#fff4d9" stroke="#b5822b"/>
<text x="56" y="642" font-size="18" font-weight="bold">OPEN: intrinsic mixed fivefold invariant image ideal and chart elimination</text>
<text x="28" y="725" font-size="19">17 named Lean results · 18 audited declarations · no native trust</text>
<text x="28" y="757" font-size="16">Source: ledger.json + PROOF_MAP.md · written global geometry and mask interpretation</text>
<text x="28" y="788" font-size="16">Same-agent construction, fresh replay and separate checks · no IC efficiency assessment</text>
<text x="28" y="817" font-size="14">Run: koblitz_modular_ic_20261018 / rounds / 07 · 2026-10-19 UTC</text>
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
