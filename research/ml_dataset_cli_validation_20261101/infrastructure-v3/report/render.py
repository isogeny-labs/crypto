from pathlib import Path
import json, html
p=Path(__file__).parent
rows=json.loads((p/'data.json').read_text())
colors={'verified':'#287b63','unsupported':'#8c8793','partial':'#d59736','budget_exhausted':'#b54343'}
s=['<svg xmlns="http://www.w3.org/2000/svg" width="780" height="300" viewBox="0 0 780 300"><rect width="780" height="300" fill="white"/>','<g font-family="sans-serif" font-size="14" fill="#222"><text x="12" y="22">Recorded solver outcomes: 46 local software cases</text>']
for i,r in enumerate(rows):
 y=52+i*37;s.append(f'<text x="12" y="{y+18}">{html.escape(r["panel"])}</text>');x=185
 for k,c in colors.items():
  n=r['outcomes'].get(k,0)
  if n:s.append(f'<rect x="{x}" y="{y}" width="{n*20}" height="25" fill="{c}"/><text x="{x+n*10}" y="{y+18}" text-anchor="middle" fill="white">{n}</text>');x+=n*20
 s.append(f'<text x="{x+6}" y="{y+18}">n={r["receipts"]}</text>')
x=12
for k,c in colors.items():
 s.append(f'<rect x="{x}" y="249" width="12" height="12" fill="{c}"/><text x="{x+17}" y="260">{k}</text>');x+=180
s.append('<text x="12" y="287">Counts, not acceptance rates. Fixed fixture; no statistical uncertainty estimated.</text></g></svg>')
(p/'outcomes.svg').write_text(''.join(s))
