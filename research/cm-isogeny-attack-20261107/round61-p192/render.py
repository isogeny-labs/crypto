"""Render retained exact counts and construction; no new research computation."""
from pathlib import Path
import json,html,typst,pymupdf
r=Path(__file__).resolve().parent
flow=[['Point action -> six scalar images','T([z]G)=[1/(1-z)]G from the transverse norm; S(W)=G-W'],['Canonical point -> retained transporter','Least encoded image and normalized H; generic canonicalization charges two changing-target norms'],['Known affine move -> closed matrix tracker','M <- H A M; f_M(k)=(ak+b)/(ck+d). Rational degree stays one.'],['Nontrivial collision -> checked roots','Cross-multiply M1 and M2; degree <= 2. Reject proportional matrices; verify roots against Q.']]
(r/'flow.json').write_text(json.dumps(flow,indent=2)+'\n')
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="320"><rect width="1000" height="320" fill="white"/>']
for i,(a,b) in enumerate(flow):
 y=4+79*i;s.append(f'<rect x="5" y="{y}" width="990" height="73" rx="6" fill="#e7f0ec" stroke="#49617a"/><text x="18" y="{y+27}" font-family="sans-serif" font-size="21">{html.escape(a)}</text><text x="18" y="{y+55}" font-family="sans-serif" font-size="16">{html.escape(b)}</text>')
s.append('</svg>');(r/'flow.svg').write_text(''.join(s))
rows=json.loads((r/'plot-data.json').read_text())['rows'];series=[('known_orbit_value','Known value','#557b9e'),('order3_stabilizer','Order-three stabilizer','#aa8553'),('nontrivial_collision','Nontrivial collision','#42876a')]
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="360"><rect width="1000" height="360" fill="white"/><text x="15" y="25" font-family="sans-serif" font-size="22">Recovery path for each declared target (63 total)</text>']
for i,(_,label,color) in enumerate(series):
 x=15+325*i;s.append(f'<rect x="{x}" y="41" width="18" height="16" fill="{color}"/><text x="{x+25}" y="55" font-family="sans-serif" font-size="16">{label}</text>')
for i,row in enumerate(rows):
 y=78+47*i;s.append(f'<text x="15" y="{y+23}" font-family="sans-serif" font-size="18">F{row["p"]}, r={row["r"]}</text>');x=220
 for key,label,color in series:
  v=row['reasons'].get(key,0);w=33*v
  if v:s.append(f'<rect x="{x}" y="{y}" width="{w}" height="33" fill="{color}"/><text x="{x+w/2}" y="{y+23}" text-anchor="middle" font-family="sans-serif" font-size="18" fill="white">{v}</text>')
  x+=w
s.append('<text x="15" y="333" font-family="sans-serif" font-size="15">Unit: targets. Sources: orbits.json, residue-control.json, large-residue.json. Exact cases; no rates.</text></svg>');(r/'recoveries.svg').write_text(''.join(s))
t=['#set page(paper:"a4",margin:16mm)','#set text(size:9.1pt)']
for para in (r/'report.md').read_text().split('\n\n'):
 if para.startswith('# '):t.extend(['#heading(level:1,'+json.dumps(para[2:])+')','#image("flow.svg",width:100%)'])
 elif para:
  t.extend(['#text('+json.dumps(para)+')','#parbreak()'])
  if para.startswith('Recovery paths,'):t.append('#image("recoveries.svg",width:100%)')
(r/'report.typ').write_text('\n'.join(t));typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'));doc=pymupdf.open(r/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=pymupdf.Matrix(1.3,1.3)).save(r/f'page-{i+1}.png')
print('pages',len(doc))
