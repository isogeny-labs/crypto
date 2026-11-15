"""Render exact retained counts and transport formulas; no research trials."""
from pathlib import Path
import json,html,typst,pymupdf
r=Path(__file__).resolve().parent
flow=[['One reusable calibration + seven first spectral coefficients','Bcal = B(P,P); y_i = Bcal*y(P)/B(P+iG,P+iG+U). No C or c_Q input.'],['Cubic candidates -> unique signed point','At most three x roots; two distinct candidates cannot agree at seven shifts (pole degree six).'],['Horizontal transport retains its derivative correction',"C'/B' = F(C/B); B' = H B / F'(C/B). Require 2f(P) nonrational."],['Actual P192 transfer -> supplied maps and explicit inverse','47 maps and closed duals; V = [degree^(-1) mod r] dual(f)(V_destination).']]
(r/'flow.json').write_text(json.dumps(flow,indent=2)+'\n');s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="320"><rect width="1000" height="320" fill="white"/>']
for i,(a,b) in enumerate(flow):
 y=4+79*i;s.append(f'<rect x="5" y="{y}" width="990" height="73" rx="6" fill="#e7f0ec" stroke="#49617a"/><text x="18" y="{y+27}" font-family="sans-serif" font-size="21">{html.escape(a)}</text><text x="18" y="{y+55}" font-family="sans-serif" font-size="16">{html.escape(b)}</text>')
s.append('</svg>');(r/'flow.svg').write_text(''.join(s))
rows=json.loads((r/'plot-data.json').read_text())['rows'];s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="400"><rect width="1000" height="400" fill="white"/><text x="15" y="27" font-family="sans-serif" font-size="22">Exact checks by named obligation (not rates)</text>']
for i,row in enumerate(rows):
 y=50+48*i;v=row['count'];w=v*1.35;s.append(f'<text x="15" y="{y+23}" font-family="sans-serif" font-size="17">{html.escape(row["name"])}</text><rect x="410" y="{y}" width="{w}" height="33" fill="#557b9e"/><text x="{420+w}" y="{y+23}" font-family="sans-serif" font-size="18">{v}</text>')
s.append('<text x="15" y="374" font-family="sans-serif" font-size="15">Sources: B-only.json, transport.json, P192-transport.json. Unit: exact checks; deterministic declared cases.</text></svg>');(r/'checks.svg').write_text(''.join(s))
t=['#set page(paper:"a4",margin:16mm)','#set text(size:9.1pt)']
for para in (r/'report.md').read_text().split('\n\n'):
 if para.startswith('# '):t.extend(['#heading(level:1,'+json.dumps(para[2:])+')','#image("flow.svg",width:100%)'])
 elif para:
  t.extend(['#text('+json.dumps(para)+')','#parbreak()'])
  if para.startswith('P-192 map transport.'):t.append('#image("checks.svg",width:100%)')
(r/'report.typ').write_text('\n'.join(t));typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'));doc=pymupdf.open(r/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=pymupdf.Matrix(1.3,1.3)).save(r/f'page-{i+1}.png')
print('pages',len(doc))
