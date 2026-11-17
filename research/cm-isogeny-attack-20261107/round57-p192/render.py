"""Document rendering only; consumes the saved native records."""
from pathlib import Path
import json,typst,pymupdf
r=Path(__file__).resolve().parent;c=json.loads((r/'P192-degrees.json').read_text())
flow=[['Signed paired coordinates -> quadratic block','Keep x(jG), x(jQ), and y(jG)*y(jQ)'],['Baby algebra -> resultant norm and trace coefficient','h(T), A(T), B(T); denominator inversion modulo h'],['Degree-dependent contraction -> explicit cost','Charge encoding, polynomial degree, point inputs and pole precision'],['P-192 audit -> bounded encoding exclusions','Known small-scalar controls pass; three declared target/CM fixtures fail caps8/16']]
(r/'flow.json').write_text(json.dumps(flow,indent=2)+'\n')
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="320"><rect width="1000" height="320" fill="white"/>']
for i,(a,b) in enumerate(flow):
 y=3+i*79;s.append(f'<rect x="10" y="{y}" width="980" height="72" rx="5" fill="#e7f0ec" stroke="#49617a"/><text x="25" y="{y+27}" font-family="sans-serif" font-size="21">{a}</text><text x="25" y="{y+54}" font-family="sans-serif" font-size="17">{b}</text>')
s.append('</svg>');(r/'flow.svg').write_text(''.join(s))
rows=[]
for t in c['targets']:
 row={'target':t['name'],'caps':[]}
 for cap in [8,16]:
  systems=[s for s in t['systems'] if s['degree_cap']==cap];row['caps'].append({'cap':cap,'ranks':[s['rank'] for s in systems],'columns':systems[0]['columns'],'both_fit':all(s['status']=='rational_pair_found' for s in systems)})
 rows.append(row)
(r/'plot-data.json').write_text(json.dumps({'rows':rows,'source':'P192-degrees.json','unit':'matrix rank / column count','coordinates':['x(jQ)','y(jG)y(jQ)'],'nodes':'first24 or40 positive odd indices;8 heldout','uncertainty':'none; exact deterministic panel'},indent=2)+'\n')
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="350"><rect width="1000" height="350" fill="white"/><text x="15" y="25" font-family="sans-serif" font-size="20">P-192 bounded rational encoding audit (both paired functions)</text><text x="340" y="54" font-family="sans-serif" font-size="18">Degree cap8:18 columns</text><text x="660" y="54" font-family="sans-serif" font-size="18">Degree cap16:34 columns</text>']
for i,row in enumerate(rows):
 y=65+34*i;s.append(f'<text x="15" y="{y+23}" font-family="sans-serif" font-size="18">{row["target"]}</text>')
 for j,cap in enumerate(row['caps']):
  color='#c4ddcf' if cap['both_fit'] else '#efcfaa';label='fit' if cap['both_fit'] else 'excluded';a,b=cap['ranks'];n=cap['columns'];s.append(f'<rect x="{310+j*330}" y="{y}" width="320" height="30" fill="{color}"/><text x="{325+j*330}" y="{y+22}" font-family="sans-serif" font-size="17">{label}: ranks {a}/{n}, {b}/{n}</text>')
s.append('<text x="15" y="333" font-family="sans-serif" font-size="16">Full rank excludes the capped rational pair. Positive fits also pass heldout/global-map checks.</text></svg>');(r/'degrees.svg').write_text(''.join(s))
t=['#set page(paper:"a4",margin:16mm)','#set text(size:9.2pt)']
for para in (r/'report.md').read_text().split('\n\n'):
 if para.startswith('# '):t+=['#heading(level:1,'+json.dumps(para[2:])+')','#image("flow.svg",width:100%)']
 else:
  t+=['#text('+json.dumps(para)+')','#parbreak()']
  if para.startswith('Audit outcomes'):t+=['#image("degrees.svg",width:100%)']
(r/'report.typ').write_text('\n'.join(t));typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'));doc=pymupdf.open(r/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap().save(r/f'page-{i+1}.png')
print('pages',len(doc))
