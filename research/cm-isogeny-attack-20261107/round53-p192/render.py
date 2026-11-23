from pathlib import Path
import json,typst,pymupdf
r=Path(__file__).parent
flow=[['Rational graph plus orthogonal p-eigenspace','All inputs are public curve and point data'],['Complete Miller and Frobenius constraints','P-192:23678 variables,95695 quartic equations'],['Actual extension-field kernel replay','Six targets,49 kernel points each; quotient evaluation remains open']]
(r/'flow.json').write_text(json.dumps(flow,indent=2)+'\n')
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="255"><rect width="1000" height="255" fill="white"/>']
for i,(a,b) in enumerate(flow):
 y=4+i*84;s.append(f'<rect x="10" y="{y}" width="980" height="74" rx="5" fill="#e7f0ec" stroke="#49617a"/><text x="25" y="{y+28}" font-family="sans-serif" font-size="21">{a}</text><text x="25" y="{y+56}" font-family="sans-serif" font-size="18">{b}</text>')
s.append('</svg>');(r/'flow.svg').write_text(''.join(s))
data=[{'label':'Initial equation count','dimension':117523},{'label':'Reduced equation count','dimension':95695}];(r/'plot-data.json').write_text(json.dumps(data,indent=2)+'\n')
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="190"><rect width="1000" height="190" fill="white"/>']
for i,z in enumerate(data):
 y=15+65*i;s.append(f'<text x="20" y="{y+26}" font-family="sans-serif" font-size="21">{z["label"]}</text><rect x="355" y="{y}" width="{z["dimension"]*.0039}" height="38" fill="#356f99"/><text x="{365+z["dimension"]*.0039}" y="{y+26}" font-family="sans-serif" font-size="21">{z["dimension"]}</text>')
s.append('<text x="20" y="175" font-family="sans-serif" font-size="18">Exact P-192 constraint counts from retained streams; two circuit substitutions; no timing data.</text></svg>');(r/'counts.svg').write_text(''.join(s))
t=['#set page(paper:"a4",margin:16mm)','#set text(size:9.5pt)']
for para in (r/'report.md').read_text().split('\n\n'):
 if para.startswith('# '):t+=['#heading(level:1,'+json.dumps(para[2:])+')','#image("flow.svg",width:100%)']
 else:
  t+=['#text('+json.dumps(para)+')','#parbreak()']
  if para.startswith('Positive power chain.'):t+=['#image("counts.svg",width:100%)']
(r/'report.typ').write_text('\n'.join(t));typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'));doc=pymupdf.open(r/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap().save(r/f'page-{i+1}.png')
print('pages',len(doc))
