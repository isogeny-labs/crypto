"""Layout only: render frozen ledger values, report and proof map."""
from pathlib import Path
import json
import re
import typst
import pymupdf
HERE = Path(__file__).resolve().parent
record = json.loads((HERE / "ledger.json").read_text())
q = record["quotients"]
l = record["low_genus"]
m = record["relations"]
svg = f'''<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="780" viewBox="0 0 1100 780">
<defs><marker id="arrow" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto"><path d="M0 0 L8 4 L0 8" fill="none" stroke="#37577b" stroke-width="1.4"/></marker></defs>
<rect width="1100" height="780" fill="#f8fafc"/>
<g font-family="DejaVu Sans,sans-serif" fill="#172438">
<text x="28" y="37" font-size="25" font-weight="bold">Koblitz modular carriers · round 09 · exact construction</text>
<text x="28" y="64" font-size="16">Four determinant-19 quotients · K = F2^360 · no timing or IC efficiency assessment</text>
<rect x="28" y="86" width="474" height="167" rx="8" fill="#e3effa" stroke="#4777ba"/>
<text x="44" y="118" font-size="21" font-weight="bold">Dual base: B = D^(-transpose)</text>
<text x="44" y="150" font-size="18">beta(P) = (-P3, P0-P3, P1-P3, P2-P3, P4)</text>
<text x="44" y="181" font-size="18">Root action uses N = [[0,9],[1,t]], t = ±1</text>
<text x="44" y="214" font-size="16">z0' = sigma(z1) H0; z1' = sigma(z0)^9 sigma(z1)^t H1</text>
<rect x="558" y="86" width="514" height="167" rx="8" fill="#eaf4ee" stroke="#398460"/>
<text x="575" y="118" font-size="21" font-weight="bold">Explicit rational correction circuits</text>
<text x="575" y="151" font-size="18">{q['univariate_class_identities']} class identities · {q['exact_slice_identities']} exact slices · {q['chart_pair_checks']} chart squares</text>
<text x="575" y="183" font-size="18">Two 360-step cocycles per quotient, both roots return</text>
<text x="575" y="215" font-size="18">Global bridge: base-trivial iterate is constant deck action</text>
<path d="M507 167 L549 167" stroke="#37577b" stroke-width="2" marker-end="url(#arrow)"/>
<path d="M814 261 L814 286" stroke="#37577b" stroke-width="2" marker-end="url(#arrow)"/>
<rect x="28" y="297" width="1044" height="193" rx="8" fill="#eee8f7" stroke="#8770a8"/>
<text x="44" y="331" font-size="22" font-weight="bold">{q['invariant_rational_generators_per_graph']:,} invariant rational generating circuits per quotient</text>
<text x="44" y="365" font-size="20">I(f,k) = sum_j rho^j(b_k f)                 f = sum_k d_k I(f,k)</text>
<text x="44" y="399" font-size="18">{q['trace_duality_entries']:,} trace-duality entries · {q['separability_idempotent_sums']} idempotent sums · {q['sampled_invariant_values']:,} sampled values</text>
<text x="44" y="431" font-size="18">{q['coordinate_recoveries']} exact recoveries · {q['equivariance_checks']} equivariance controls · {record['new_named_lean_theorems']} named Lean results</text>
<text x="44" y="466" font-size="18" font-weight="bold">OPEN: F2 polynomial relations, saturation, closed embedding, Jacobian identification</text>
<rect x="28" y="516" width="500" height="167" rx="8" fill="#fff4d9" stroke="#b5822b"/>
<text x="44" y="550" font-size="21" font-weight="bold">Bounded lower-genus diagnostic</text>
<text x="44" y="582" font-size="18">{l['signed_controls']} signed diagonal-twist / reflection controls</text>
<text x="44" y="614" font-size="18">{l['stable_and_known_component_retained']} stable cases retaining the known A5 component</text>
<text x="44" y="648" font-size="17">Other factors and carrier families remain open</text>
<rect x="552" y="516" width="520" height="167" rx="8" fill="#fff4d9" stroke="#b5822b"/>
<text x="570" y="550" font-size="21" font-weight="bold">Coupled-function support diagnostic</text>
<text x="570" y="582" font-size="18">{m['coupled_functions']} functions × {m['known_columns_tested']} known columns: no zeros</text>
<text x="570" y="614" font-size="18">Outside support and poles uncomputed; no new rank</text>
<text x="570" y="648" font-size="17">Retained rank 12/1308; nonzero-image rank 11</text>
<text x="28" y="720" font-size="16">Source: round09 ledger.json + PROOF_MAP.md · exact finite checks and written global bridges</text>
<text x="28" y="748" font-size="16">Same-agent fresh replay and separate parser with shared primitives · 2026-10-19 UTC</text>
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
