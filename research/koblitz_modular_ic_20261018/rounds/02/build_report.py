"""Document layout only. Reads recorded quantities; does not run research arithmetic."""
from pathlib import Path
import json
import re
import typst
import pymupdf
HERE = Path(__file__).resolve().parent
record = json.loads((HERE / "ledger.json").read_text())
mr = record["mr2b"]
svg = f'''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="760" viewBox="0 0 1000 760">
<rect width="1000" height="760" fill="#f8fafc"/>
<g font-family="DejaVu Sans,sans-serif" fill="#172438">
<text x="30" y="40" font-size="24" font-weight="bold">Koblitz carrier experiments · round 02</text>
<text x="30" y="70" font-size="16">Exact finite experiments and partial formal proofs · no timing measurements</text>
<rect x="30" y="95" width="940" height="76" rx="8" fill="#deebff" stroke="#4777ba"/>
<text x="50" y="123" font-size="19" font-weight="bold">E1 / F8, order-7 subgroup → Klein quartic / F2, genus 3</text>
<text x="50" y="153" font-size="16">56 degree-three quotients · 42 nonzero traced transfers · 14 zero maps</text>
<path d="M500 172 L500 194" stroke="#4777ba" stroke-width="3"/>
<rect x="30" y="200" width="940" height="104" rx="8" fill="#e1f1e8" stroke="#398460"/>
<text x="50" y="230" font-size="19" font-weight="bold">Geometric line norms → factor-base logs → target recovery</text>
<text x="50" y="259" font-size="17">F512: {mr['relations']:,} relations · {mr['columns']} columns · rank {mr['rank']} modulo 7 + anchor</text>
<text x="50" y="286" font-size="16">{mr['recovered']} / {mr['fixtures']} fixtures recovered · separate row, rank and divisor-certificate checker</text>
<rect x="30" y="330" width="455" height="136" rx="8" fill="#fff0d6" stroke="#bd8b39"/>
<text x="50" y="360" font-size="19" font-weight="bold">Low-genus A5 searches</text>
<text x="50" y="391" font-size="16">Genus 5: {record['genus5_models']:,} explicit models</text>
<text x="50" y="419" font-size="16">Genus 6: {record['genus6_forms_and_sections']:,} forms / sections</text>
<text x="50" y="447" font-size="16">Zero polynomial matches in these families</text>
<rect x="515" y="330" width="455" height="136" rx="8" fill="#eee8f7" stroke="#8770a8"/>
<text x="535" y="360" font-size="19" font-weight="bold">Gluing and higher-n relations</text>
<text x="535" y="391" font-size="16">Candidate primes 5, 11, 19 · 8 Hom spaces</text>
<text x="535" y="419" font-size="16">n=131 recurrence: rank 261 / 262 anchored</text>
<text x="535" y="447" font-size="16">Polarizations and cross-orbit relations open</text>
<text x="30" y="503" font-size="18" font-weight="bold">Genus-six family populations screened (forms, not curve classes)</text>
<text x="30" y="536" font-size="15">Balanced trigonal</text>
<rect x="260" y="518" width="555" height="22" fill="#4777ba"/>
<text x="830" y="536" font-size="15">1,048,575</text>
<text x="30" y="568" font-size="15">Unbalanced trigonal</text>
<rect x="260" y="550" width="555" height="22" fill="#4777ba"/>
<text x="830" y="568" font-size="15">1,048,575</text>
<text x="30" y="600" font-size="15">Five del Pezzo twists</text>
<rect x="260" y="582" width="173.437" height="22" fill="#4777ba"/>
<text x="830" y="600" font-size="15">327,675</text>
<rect x="30" y="627" width="940" height="68" rx="8" fill="#e7edf4" stroke="#61748e"/>
<text x="50" y="655" font-size="17">13 Lean structural lemmas checked; geometric bridges remain partial.</text>
<text x="50" y="680" font-size="16">Next: two del Pezzo twists · integral polarized gluing · evaluated n=5 map / relations</text>
<text x="30" y="730" font-size="14">Sources: ledger.json, PROOF_MAP.md and runs.tar.xz · user assesses IC implications</text>
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
