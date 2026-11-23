"""Render the editable torsion mechanism and fixed-cell count graph as SVG."""

import json
from pathlib import Path
from xml.sax.saxutils import escape

HERE = Path(__file__).resolve().parent
data = json.loads((HERE / "counts.json").read_text())
cells = data["cells"]

mechanism = '''<svg xmlns="http://www.w3.org/2000/svg" width="900" height="235" viewBox="0 0 900 235">
<rect width="900" height="235" fill="#f8fafc"/>
<text x="30" y="38" font-family="sans-serif" font-size="22" font-weight="bold" fill="#102a43">Order-two first row</text>
<text x="30" y="79" font-family="sans-serif" font-size="17" fill="#102a43">T = (0, 1), 2T = O</text>
<text x="280" y="79" font-family="sans-serif" font-size="17" fill="#102a43">P and -P share x</text>
<text x="560" y="79" font-family="sans-serif" font-size="17" fill="#102a43">x(T + P) = 1/x(P)</text>
<path d="M195 72 H260 M465 72 H540" stroke="#2a6f97" stroke-width="3" marker-end="url(#arrow)"/>
<defs><marker id="arrow" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto"><path d="M0 0 L8 4 L0 8 Z" fill="#2a6f97"/></marker></defs>
<rect x="26" y="115" width="420" height="86" rx="8" fill="#e5f4ef" stroke="#21867a"/>
<text x="43" y="149" font-family="sans-serif" font-size="16" fill="#102a43">Keep first witness (T, P); batch-invert x(P).</text>
<text x="43" y="177" font-family="sans-serif" font-size="16" fill="#102a43">The signed twin yields the same table key.</text>
<rect x="473" y="115" width="400" height="86" rx="8" fill="#fff2de" stroke="#bc6c25"/>
<text x="490" y="149" font-family="sans-serif" font-size="16" fill="#102a43">Check all other collisions in the full group.</text>
<text x="490" y="177" font-family="sans-serif" font-size="16" fill="#102a43">Seed the single torsion column relation.</text>
</svg>'''
(HERE / "mechanism.svg").write_text(mechanism)

parts = ['<svg xmlns="http://www.w3.org/2000/svg" width="900" height="335" viewBox="0 0 900 335">',
         '<rect width="900" height="335" fill="#f8fafc"/>',
         '<text x="25" y="35" font-family="sans-serif" font-size="21" font-weight="bold" fill="#102a43">Fixed-cell collision candidates (one assertion per cell)</text>',
         '<text x="25" y="61" font-family="sans-serif" font-size="14" fill="#334e68">Original candidate count = retained full-group checks + verified torsion twins</text>']
scale = 2.6
for i, cell in enumerate(cells):
    y = 100 + i * 75
    kept = cell["retained_checks"] * scale
    twins = cell["torsion_twins"] * scale
    parts.append(f'<text x="25" y="{y + 20}" font-family="sans-serif" font-size="15" fill="#102a43">{escape(cell["curve"])} d={cell["dimension"]}</text>')
    parts.append(f'<rect x="245" y="{y}" width="{kept:.1f}" height="30" fill="#21867a"/>')
    parts.append(f'<rect x="{245 + kept:.1f}" y="{y}" width="{twins:.1f}" height="30" fill="#e59b36"/>')
    parts.append(f'<text x="{260 + kept + twins:.1f}" y="{y + 21}" font-family="sans-serif" font-size="15" fill="#102a43">{cell["original_candidates"]}</text>')
parts.extend(['<rect x="245" y="296" width="18" height="14" fill="#21867a"/>',
              '<text x="270" y="309" font-family="sans-serif" font-size="14" fill="#102a43">Retained full-group checks</text>',
              '<rect x="495" y="296" width="18" height="14" fill="#e59b36"/>',
              '<text x="520" y="309" font-family="sans-serif" font-size="14" fill="#102a43">Torsion twins represented by one seed</text>',
              '</svg>'])
(HERE / "counts.svg").write_text("\n".join(parts))
