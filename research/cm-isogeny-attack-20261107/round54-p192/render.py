from pathlib import Path
import json,typst,pymupdf
r=Path(__file__).parent;c=json.loads((r/'certificate.json').read_text())
flow=[['Mixed trace -> exact marked translation stabilizer','Add multiplication-map coordinates to generate the quotient field'],['Quadratic twist shift -> finite rational-input chart','Actual P-192 shift checked; P-192 trace not evaluated'],['Six targets x eight fixed shifts -> exact finite controls','All collisions retained; common n=6 separates49 conjugates']]
(r/'flow.json').write_text(json.dumps(flow,indent=2)+'\n')
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="255"><rect width="1000" height="255" fill="white"/>']
for i,(a,b) in enumerate(flow):
 y=4+i*84;s.append(f'<rect x="10" y="{y}" width="980" height="74" rx="5" fill="#e7f0ec" stroke="#49617a"/><text x="25" y="{y+28}" font-family="sans-serif" font-size="21">{a}</text><text x="25" y="{y+56}" font-family="sans-serif" font-size="18">{b}</text>')
s.append('</svg>');(r/'flow.svg').write_text(''.join(s))
data=[[a['distinct_conjugates'] for a in t['attempts']] for t in c['targets']];(r/'plot-data.json').write_text(json.dumps({'values':data,'unit':'distinct finite conjugate values','target_order':'certificate order','selection':'n=6 chosen after full fixed panel'},indent=2)+'\n')
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="330"><rect width="1000" height="330" fill="white"/><text x="20" y="25" font-family="sans-serif" font-size="20">Distinct conjugates: six target kernels, all eight fixed shifts</text>']
for j in range(8):s.append(f'<text x="{160+j*95}" y="54" font-family="sans-serif" font-size="17">n={j+1}</text>')
for i,row in enumerate(data):
 y=65+i*35;s.append(f'<text x="15" y="{y+23}" font-family="sans-serif" font-size="17">Target {i+1}</text>')
 for j,v in enumerate(row):
  color='#b8d8c4' if v==49 else '#edc8a3';s.append(f'<rect x="{140+j*95}" y="{y}" width="90" height="32" fill="{color}"/><text x="{170+j*95}" y="{y+23}" font-family="sans-serif" font-size="18">{v}</text>')
s.append('<text x="15" y="307" font-family="sans-serif" font-size="17">Exact finite-field counts; all outcomes retained. n=6 separates all49 in every row.</text></svg>');(r/'counts.svg').write_text(''.join(s))
t=['#set page(paper:"a4",margin:16mm)','#set text(size:9.5pt)']
for para in (r/'report.md').read_text().split('\n\n'):
 if para.startswith('# '):t+=['#heading(level:1,'+json.dumps(para[2:])+')','#image("flow.svg",width:100%)']
 else:
  t+=['#text('+json.dumps(para)+')','#parbreak()']
  if para.startswith('Exact small-field evaluator.'):t+=['#image("counts.svg",width:100%)']
(r/'report.typ').write_text('\n'.join(t));typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'));doc=pymupdf.open(r/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap().save(r/f'page-{i+1}.png')
print('pages',len(doc))
