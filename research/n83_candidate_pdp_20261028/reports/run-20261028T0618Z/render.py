from pathlib import Path
import json
import typst
p=Path(__file__).resolve().parent
rows=json.loads((p/'../../evidence/pdp/paired-summary.json').read_text())['rows']
rows=[r for r in rows if r['m']==2 and r['family']!='prime']
s=['<svg xmlns="http://www.w3.org/2000/svg" width="760" height="320" viewBox="0 0 760 320"><rect width="760" height="320" fill="white"/><g font-family="sans-serif" fill="#122c3c"><text x="10" y="24" font-size="18">Independent linear constraints in V2 — m=2</text><text x="200" y="46" font-size="12" fill="#247e9a">Baseline</text><text x="310" y="46" font-size="12" fill="#c16f2b">Spike40</text>']
for i,r in enumerate(rows):
 y=66+i*56
 s.append(f'<text x="8" y="{y+23}" font-size="12">{r["family"]} {r["target_kind"]}</text>')
 for j,(arm,color) in enumerate([('baseline','#247e9a'),('spike40','#c16f2b')]):
  v=r[arm]['linear_rank'];w=v*8
  s.append(f'<rect x="195" y="{y+j*21}" width="{w}" height="17" fill="{color}"/><text x="{201+w}" y="{y+j*21+13}" font-size="12">{v}</text>')
s.append('<text x="195" y="309" font-size="12">0</text><text x="573" y="309" font-size="12">54 constraints</text></g></svg>')
(p/'linear-ranks.svg').write_text(''.join(s))
typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'))
