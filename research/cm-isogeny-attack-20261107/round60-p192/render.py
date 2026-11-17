"""Document rendering only; reads the retained native records."""
from pathlib import Path
import json,html,typst,pymupdf
r=Path(__file__).resolve().parent;c=json.loads((r/'norms.json').read_text());p=json.loads((r/'P192-blocks.json').read_text())
flow=[['Cross-difference norm -> short normalized base','N_Q / c_Q = x((1-pi)P)-x((1-pi)R); the base is identical for Q and -Q'],['Square-zero first variations -> graph-dependent coefficients','One zero factor at R=P+U; retain B and C, without dividing by that factor'],['Ratio and rational lift -> inverse-affine point action','C/B = x(P+[(1-k)^-1]U); subtract P and choose the rational lift'],['Reusable first call -> ordinary marked action','bG=M_Q(G), Q2=G-bG, output U-M_Q2(U); charge both constructors']]
(r/'flow.json').write_text(json.dumps(flow,indent=2)+'\n');s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="330"><rect width="1000" height="330" fill="white"/>']
for i,(a,b) in enumerate(flow):
 a,b=html.escape(a),html.escape(b);y=5+81*i;s.append(f'<rect x="8" y="{y}" width="984" height="73" rx="5" fill="#e7f0ec" stroke="#49617a"/><text x="22" y="{y+27}" font-family="sans-serif" font-size="20">{a}</text><text x="22" y="{y+55}" font-family="sans-serif" font-size="16">{b}</text>')
s.append('</svg>');(r/'flow.svg').write_text(''.join(s))
rows=[{'p':v['p'],'r':v['r'],'inverse':v['inverse_affine_actions'],'ordinary':v['ordinary_actions'],'wrong_graph':v['wrong_second_graph_rejections']} for v in c['controls']]
(r/'plot-data.json').write_text(json.dumps({'rows':rows,'unit':'exact checked point actions per frozen panel','source':'norms.json','P192_selected_blocks':p['selected_blocks'],'uncertainty':'none; deterministic exact checks; no rates or timing'},indent=2)+'\n')
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="320"><rect width="1000" height="320" fill="white"/><text x="15" y="26" font-family="sans-serif" font-size="21">Complete controls: inverse-affine and ordinary point actions</text><rect x="15" y="40" width="18" height="16" fill="#4f8b75"/><text x="40" y="54" font-family="sans-serif" font-size="16">Inverse-affine</text><rect x="230" y="40" width="18" height="16" fill="#557b9e"/><text x="255" y="54" font-family="sans-serif" font-size="16">Ordinary bridge</text>']
for i,row in enumerate(rows):
 y=72+70*i;s.append(f'<text x="15" y="{y+25}" font-family="sans-serif" font-size="18">F{row["p"]}, r={row["r"]}</text>')
 for j,(key,color) in enumerate([('inverse','#4f8b75'),('ordinary','#557b9e')]):
  val=row[key];w=val*1.5;s.append(f'<rect x="220" y="{y+j*29}" width="{w}" height="25" fill="{color}"/><text x="{230+w}" y="{y+19+j*29}" font-family="sans-serif" font-size="17">{val}</text>')
s.append('<text x="15" y="306" font-family="sans-serif" font-size="15">Source: norms.json. Separately: 24 selected P-192 blocks, not a full P-192 norm.</text></svg>');(r/'actions.svg').write_text(''.join(s))
t=['#set page(paper:"a4",margin:16mm)','#set text(size:9.1pt)']
for para in (r/'report.md').read_text().split('\n\n'):
 if para.startswith('# '):t.extend(['#heading(level:1,'+json.dumps(para[2:])+')','#image("flow.svg",width:100%)'])
 elif para:
  t.extend(['#text('+json.dumps(para)+')','#parbreak()'])
  if para.startswith('Exact complete controls.'):t.append('#image("actions.svg",width:100%)')
(r/'report.typ').write_text('\n'.join(t));typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'));doc=pymupdf.open(r/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=pymupdf.Matrix(1.3,1.3)).save(r/f'page-{i+1}.png')
print('pages',len(doc))
