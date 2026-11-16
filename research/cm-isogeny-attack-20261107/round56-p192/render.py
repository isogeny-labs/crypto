"""Document rendering only; consumes saved exact records."""
from pathlib import Path
import json,typst,pymupdf
r=Path(__file__).resolve().parent
c=json.loads((r/'certificate.json').read_text());old=json.loads((r.parent/'round55-p192/certificate.json').read_text())
flow=[['Public G,Q -> cyclic graph trace','No nonrational torsion basis; r graph pairs'],['Two normalizations -> same marked point action','Cyclic kernel denominator, or compact (X^p-X)^2 with factor4*y(U)^2'],['Balanced rational merge -> coefficient construction','Five polynomial products per node; O(M(r)log r) after graph/weights'],['P-192 Frobenius normalizer -> compressed map','Actual quadratic-field map checked; paired weighted trace remains next']]
(r/'flow.json').write_text(json.dumps(flow,indent=2)+'\n')
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="320"><rect width="1000" height="320" fill="white"/>']
for i,(a,b) in enumerate(flow):
 y=3+i*79;s.append(f'<rect x="10" y="{y}" width="980" height="72" rx="5" fill="#e7f0ec" stroke="#49617a"/><text x="25" y="{y+27}" font-family="sans-serif" font-size="21">{a}</text><text x="25" y="{y+54}" font-family="sans-serif" font-size="17">{b}</text>')
s.append('</svg>');(r/'flow.svg').write_text(''.join(s))
data={'labels':['Kernel polynomial','A numerator','B numerator (maximum)'],'full':[len(old['psi'])-1,max(t['A_degree'] for t in old['targets']),max(t['B_degree'] for t in old['targets'])],'cyclic':[len(c['controls'][0]['kernel_polynomial'])-1,max(t['A_degree'] for t in c['controls'][0]['targets']),max(t['B_degree'] for t in c['controls'][0]['targets'])],'unit':'polynomial degree','targets':6,'r':7,'sources':['../round55-p192/certificate.json','certificate.json'],'uncertainty':'none; exact complete finite panel'}
(r/'plot-data.json').write_text(json.dumps(data,indent=2)+'\n')
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="250"><rect width="1000" height="250" fill="white"/><text x="15" y="25" font-family="sans-serif" font-size="20">Polynomial degrees on the six F11 targets (r=7)</text><text x="580" y="25" font-family="sans-serif" font-size="17">Grey: full trace; blue: cyclic trace</text>']
for i,label in enumerate(data['labels']):
 y=42+56*i;s.append(f'<text x="15" y="{y+24}" font-family="sans-serif" font-size="18">{label}</text>')
 for j,key in enumerate(['full','cyclic']):
  n=data[key][i];yy=y+j*22;color='#7b858b' if j==0 else '#376e8a';s.append(f'<rect x="300" y="{yy}" width="{12*n}" height="19" fill="{color}"/><text x="{310+12*n}" y="{yy+16}" font-family="sans-serif" font-size="17">{n}</text>')
s.append('<text x="15" y="231" font-family="sans-serif" font-size="16">Unit: polynomial degree; all six targets. Native records, rounds55-56. No timing inference.</text></svg>');(r/'degrees.svg').write_text(''.join(s))
t=['#set page(paper:"a4",margin:16mm)','#set text(size:9.3pt)']
for para in (r/'report.md').read_text().split('\n\n'):
 if para.startswith('# '):t+=['#heading(level:1,'+json.dumps(para[2:])+')','#image("flow.svg",width:100%)']
 else:
  t+=['#text('+json.dumps(para)+')','#parbreak()']
  if para.startswith('Exact controls.'):t+=['#image("degrees.svg",width:100%)']
(r/'report.typ').write_text('\n'.join(t));typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'));doc=pymupdf.open(r/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap().save(r/f'page-{i+1}.png')
print('pages',len(doc))
