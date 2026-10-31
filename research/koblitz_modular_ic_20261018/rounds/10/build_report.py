"""Layout only: render frozen ledger values, report and proof map."""
from pathlib import Path
import json
import re
import typst
import pymupdf
HERE = Path(__file__).resolve().parent
record = json.loads((HERE / "ledger.json").read_text())
q = record["quotients"]
d = record["coupled_divisors"]
s = record["diagonal"]
rows = []
for i, fun in enumerate(d["functions"]):
    y = 390 + 42 * i
    cusp_width = 475 * fun["cuspidal_zero_degree"] / 3900
    residual_width = 475 * fun["residual_noncuspidal_zero_degree"] / 3900
    rows.append(f'<text x="44" y="{y+17}" font-size="16">{fun["function"]}</text>'
                f'<rect x="340" y="{y}" width="{cusp_width}" height="24" fill="#398460"/>'
                f'<rect x="{340+cusp_width}" y="{y}" width="{residual_width}" height="24" fill="#e7b54d"/>'
                f'<text x="835" y="{y+18}" font-size="16">{fun["cuspidal_zero_degree"]} + {fun["residual_noncuspidal_zero_degree"]}</text>')
svg = f'''<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="1000" viewBox="0 0 1100 1000">
<rect width="1100" height="1000" fill="#f8fafc"/>
<g font-family="DejaVu Sans,sans-serif" fill="#172438">
<text x="28" y="38" font-size="25" font-weight="bold">Koblitz modular carriers · round 10 · exact construction</text>
<text x="28" y="67" font-size="16">Four determinant-19 quotients · F2 polynomial models of dense opens · no IC cost measured</text>
<rect x="28" y="87" width="1044" height="190" rx="7" fill="#e3effa" stroke="#4777ba"/>
<text x="44" y="122" font-size="22" font-weight="bold">{q['total_F2_equations']:,} explicit F2 equations · degree at most 2</text>
<text x="44" y="156" font-size="18">Graphs 0 / 1: 424,080 each          Graphs 2 / 3: 415,800 each</text>
<text x="44" y="190" font-size="18">Shared basis tensor + all indexed blocks → lossless scalar polynomial systems</text>
<text x="44" y="223" font-size="18">Inverse witnesses → denominator saturation → closed affine open embeddings</text>
<text x="44" y="255" font-size="18" font-weight="bold">OPEN: global projective saturation, boundary gluing and proper quotient embedding</text>
<rect x="28" y="300" width="1044" height="465" rx="7" fill="#eaf4ee" stroke="#398460"/>
<text x="44" y="334" font-size="22" font-weight="bold">Complete pole degrees and cuspidal zero strata · all 480 cusps</text>
<rect x="44" y="352" width="18" height="16" fill="#398460"/><text x="70" y="366" font-size="16">cuspidal zeros</text>
<rect x="250" y="352" width="18" height="16" fill="#e7b54d"/><text x="276" y="366" font-size="16">unresolved noncuspidal zeros</text>
<text x="692" y="366" font-size="16">sum = pole degree</text>
{''.join(rows)}
<text x="44" y="746" font-size="17" font-weight="bold">x0*xinf complete: 22 closed cusp points · zero / pole degree 2,640 each</text>
<rect x="28" y="785" width="1044" height="126" rx="7" fill="#fff4d9" stroke="#b5822b"/>
<text x="44" y="817" font-size="21" font-weight="bold">Diagonal support located; multiplicities remain incomplete</text>
<text x="44" y="849" font-size="18">{s['matching_geometric_labels']} SS moduli labels in {s['closed_labels']} degree-10 orbits</text>
<text x="44" y="882" font-size="18">Lower multiplicity {s['multiplicity_lower_sum']} / required {s['required_residual_degree']} → gap {s['unresolved_multiplicity']} · antidiagonal open</text>
<text x="28" y="945" font-size="16">23 new Lean proofs · 146 project theorems audited · exact replay and archived failures</text>
<text x="28" y="975" font-size="15">Source: ledger.json + PROOF_MAP.md · native checks and written geometric bridges · 2026-10-19 UTC</text>
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
