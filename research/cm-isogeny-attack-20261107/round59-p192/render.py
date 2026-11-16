"""Derived document rendering only; consumes saved native evidence."""
from pathlib import Path
import json,typst,pymupdf,html
r=Path(__file__).resolve().parent;c=json.loads((r/'carriers.json').read_text())
flow=[['Nonzero signed carrier F(X,Y) + Z H(X,Y)','Weights 1,1,3; global subgroup coverage or finite-chart total weight'],['Translate intersections -> elliptic subgroup','Explicit incidence inequality, adjunction and a free prime-order quotient'],['CM norm -> bounded public point equation','Projection degrees <=2d; [a]Q=[u]G and u^2=ab'],['Point hash -> scalar u/a, or degree exclusion','Verify the scalar point; retain compressed high-degree circuits as a separate interface']]
(r/'flow.json').write_text(json.dumps(flow,indent=2)+'\n');s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="330"><rect width="1000" height="330" fill="white"/>']
for i,(a,b) in enumerate(flow):
 a,b=html.escape(a),html.escape(b);y=5+81*i;s.append(f'<rect x="8" y="{y}" width="984" height="73" rx="5" fill="#e7f0ec" stroke="#49617a"/><text x="22" y="{y+27}" font-family="sans-serif" font-size="20">{a}</text><text x="22" y="{y+55}" font-family="sans-serif" font-size="16">{b}</text>')
s.append('</svg>');(r/'flow.svg').write_text(''.join(s))
rows=[{'target':t['name'],'matches':t['match_count']} for t in c['bounded_point_search']['targets']]
(r/'plot-data.json').write_text(json.dumps({'rows':rows,'unit':'exact bounded point-relation matches','a_max':4096,'abs_u_max':4096,'source':'carriers.json','uncertainty':'none; exhaustive box and independent point-generation replay'},indent=2)+'\n')
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="360"><rect width="1000" height="360" fill="white"/><text x="15" y="25" font-family="sans-serif" font-size="21">Actual P-192 public-point search: a, |u| &lt;= 4096</text>']
for i,t in enumerate(rows):
 y=44+39*i;w=t['matches']/8
 s.append(f'<text x="15" y="{y+23}" font-family="sans-serif" font-size="18">{t["target"]}</text><rect x="260" y="{y}" width="{max(w,1)}" height="29" fill="#4f8b75"/><text x="{270+w}" y="{y+22}" font-family="sans-serif" font-size="17">{t["matches"]}</text>')
s.append('<text x="15" y="343" font-family="sans-serif" font-size="15">Counts of [a]Q=[u]G. Source: carriers.json; seven frozen fixtures; no runtime inference.</text></svg>');(r/'matches.svg').write_text(''.join(s))
bounds=[{'target':'public_Q','d':c['degree_conditions']['small']['d']},{'target':'mu_G','d':c['degree_conditions']['large']['d']},{'target':'relation_lambda_G','d':c['degree_conditions']['large']['d']}]
(r/'degree-plot-data.json').write_text(json.dumps({'bounds':bounds,'unit':'excluded total global signed weight','source':'carriers.json + carrier-theorem.md','standing':'derived, unsigned; not a circuit/subset bound'},indent=2)+'\n')
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="230"><rect width="1000" height="230" fill="white"/><text x="15" y="26" font-family="sans-serif" font-size="21">Derived global carrier bounds: log2(weight)</text>']
for i,t in enumerate(bounds):
 y=48+44*i;bits=int(t['d']).bit_length()-1;w=bits*11
 s.append(f'<text x="15" y="{y+23}" font-family="sans-serif" font-size="18">{t["target"]}</text><rect x="260" y="{y}" width="{w}" height="30" fill="#557b9e"/><text x="{270+w}" y="{y+23}" font-family="sans-serif" font-size="17">2^{bits}</text>')
s.append('<text x="15" y="212" font-family="sans-serif" font-size="15">Written geometric criterion + exact scalar certificates; global equations and total finite-chart weight.</text></svg>');(r/'bounds.svg').write_text(''.join(s))
t=['#set page(paper:"a4",margin:16mm)','#set text(size:9.1pt)']
for para in (r/'report.md').read_text().split('\n\n'):
 if para.startswith('# '):t.extend(['#heading(level:1,'+json.dumps(para[2:])+')','#image("flow.svg",width:100%)'])
 elif para:
  t.extend(['#text('+json.dumps(para)+')','#parbreak()'])
  if para.startswith('Actual P-192 bounded search.'):t.append('#image("matches.svg",width:100%)')
  if para.startswith('Larger labeled-fixture bounds.'):t.append('#image("bounds.svg",width:100%)')
(r/'report.typ').write_text('\n'.join(t));typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'));doc=pymupdf.open(r/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=pymupdf.Matrix(1.3,1.3)).save(r/f'page-{i+1}.png')
print('pages',len(doc))
