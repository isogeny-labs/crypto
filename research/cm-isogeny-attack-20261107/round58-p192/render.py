"""Derived document rendering only; reads retained exact native outputs."""
from pathlib import Path
import json,typst,pymupdf
r=Path(__file__).resolve().parent;c=json.loads((r/'poles.json').read_text());full=json.loads((r/'P192-poles.json').read_text())
flow=[['Signed block n/d at P=-U','Preserve x(jG), x(jQ), and the relative y sign'],['Six-coefficient norm det M(d+s*n0)','Keep e^0,e^1,e^2 and s*e^0,s*e^1,s*e^2; invert only units'],['Pole coefficient = [s]det / [e^2]det','First derivative of translated x suffices; origin chart handled separately'],['Divide by 4*y(U)^2, lift and subtract R0','Unique rational point; replay against the declared public point action']]
(r/'flow.json').write_text(json.dumps(flow,indent=2)+'\n');svg=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="330"><rect width="1000" height="330" fill="white"/>']
for i,(a,b) in enumerate(flow):
 y=5+81*i;svg.append(f'<rect x="8" y="{y}" width="984" height="73" rx="5" fill="#e7f0ec" stroke="#49617a"/><text x="22" y="{y+27}" font-family="sans-serif" font-size="21">{a}</text><text x="22" y="{y+55}" font-family="sans-serif" font-size="16">{b}</text>')
svg.append('</svg>');(r/'flow.svg').write_text(''.join(svg))
rows=[{'label':f'F{p["p"]}, r={p["r"]}','actions':p['actions'],'block_poles':p['genuine_pole_blocks'],'tail_poles':p['explicit_tail_poles'],'source':'poles.json'} for p in c['controls']]
rows.append({'label':'P-192 selected blocks','actions':full['pole_blocks'],'block_poles':full['pole_blocks'],'tail_poles':None,'source':'P192-poles.json'})
(r/'plot-data.json').write_text(json.dumps({'unit':'exact counts per frozen panel','uncertainty':'none; exact identities, no sampling inference','rows':rows},indent=2)+'\n')
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="290"><rect width="1000" height="290" fill="white"/><text x="15" y="28" font-family="sans-serif" font-size="21">Native marked actions: exact frozen-panel counts</text><text x="15" y="52" font-family="sans-serif" font-size="16">456 complete toy actions; 32 selected full-width P-192 block actions</text>']
for i,row in enumerate(rows):
 y=73+44*i;w=row['actions']*1.65
 svg.append(f'<text x="15" y="{y+23}" font-family="sans-serif" font-size="18">{row["label"]}</text><rect x="280" y="{y}" width="{w}" height="31" fill="#4f8b75"/><text x="{290+w}" y="{y+23}" font-family="sans-serif" font-size="18">{row["actions"]}</text>')
svg.append('<text x="15" y="276" font-family="sans-serif" font-size="15">Sources: poles.json and P192-poles.json. Counts denote checked actions, not runtime.</text></svg>');(r/'counts.svg').write_text(''.join(svg))
t=['#set page(paper:"a4",margin:16mm)','#set text(size:9.1pt)']
for para in (r/'report.md').read_text().split('\n\n'):
 if para.startswith('# '):t.extend(['#heading(level:1,'+json.dumps(para[2:])+')','#image("flow.svg",width:100%)'])
 elif para:
  t.extend(['#text('+json.dumps(para)+')','#parbreak()'])
  if para.startswith('Frozen complete panel.'):t.append('#image("counts.svg",width:100%)')
(r/'report.typ').write_text('\n'.join(t));typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'));doc=pymupdf.open(r/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=pymupdf.Matrix(1.3,1.3)).save(r/f'page-{i+1}.png')
print('pages',len(doc))
