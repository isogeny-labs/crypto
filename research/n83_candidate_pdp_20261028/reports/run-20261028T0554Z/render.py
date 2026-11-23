from pathlib import Path
import json
import typst
p=Path(__file__).resolve().parent
index=json.loads((p/'../../data/index.json').read_text())
parts=['<svg xmlns="http://www.w3.org/2000/svg" width="760" height="250" viewBox="0 0 760 250"><rect width="760" height="250" fill="white"/><g font-family="sans-serif" fill="#122c3c"><text x="15" y="25" font-size="18">Stored affine points — full enumeration</text>']
for i,r in enumerate(index['families']):
 y=65+i*56
 w=r['point_count']/150000000*440
 parts.append(f'<text x="15" y="{y+16}" font-size="15">{r["family"]}</text><rect x="110" y="{y}" width="{w}" height="24" fill="#247e9a"/><text x="{120+w}" y="{y+17}" font-size="14">{r["point_count"]:,}</text>')
parts.append('<text x="110" y="237" font-size="12">0</text><text x="500" y="237" font-size="12">150 million points</text></g></svg>')
(p/'counts.svg').write_text(''.join(parts))
typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'))
