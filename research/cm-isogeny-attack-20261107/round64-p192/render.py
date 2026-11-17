"""Render the retained norm contract and primitive counts; no new trials."""
from pathlib import Path
import json,html,typst,pymupdf
r=Path(__file__).resolve().parent
flow=[['Prepared H,E0,E1 -> field Euclidean arithmetic','Monic square-free H; norm(E0+s E1) = A+s B, s^2=0'],['Unit E0 -> resultant and trace coefficient',"A=Res(H,E0); B=A*coeff_(b-1)((E1/E0)*H' mod H)"],['One common root t -> smaller resultant','A=0; B=E1(t)*Res(H/(Z-t),E0); empty norm is one'],['Two common roots -> zero first coefficient','No nilpotent inverse; chosen norm feeds block output, with dense and stream references']]
(r/'flow.json').write_text(json.dumps(flow,indent=2)+'\n');s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="320"><rect width="1000" height="320" fill="white"/>']
for i,(a,b) in enumerate(flow):
 y=4+79*i;s.append(f'<rect x="5" y="{y}" width="990" height="73" rx="6" fill="#e7f0ec" stroke="#49617a"/><text x="18" y="{y+27}" font-family="sans-serif" font-size="21">{html.escape(a)}</text><text x="18" y="{y+55}" font-family="sans-serif" font-size="16">{html.escape(b)}</text>')
s.append('</svg>');(r/'flow.svg').write_text(''.join(s))
d=json.loads((r/'plot-data.json').read_text());s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="350"><rect width="1000" height="350" fill="white"/><text x="15" y="25" font-family="sans-serif" font-size="22">Matched norm component: 22,764 identical polynomial inputs</text><rect x="15" y="42" width="18" height="16" fill="#42876a"/><text x="40" y="56" font-family="sans-serif" font-size="17">Field resultants</text><rect x="260" y="42" width="18" height="16" fill="#557b9e"/><text x="285" y="56" font-family="sans-serif" font-size="17">Dense norm</text>']
for i,(label,scale)in enumerate([('K multiplications',30000),('K inversions',600)]):
 y=83+113*i;s.append(f'<text x="15" y="{y+25}" font-family="sans-serif" font-size="19">{label}</text>')
 for j,(arm,color)in enumerate([('field','#42876a'),('dense','#557b9e')]):
  val=d[arm][i];w=val/scale;s.append(f'<rect x="235" y="{y+j*32}" width="{w}" height="27" fill="{color}"/><text x="{245+w}" y="{y+20+j*32}" font-family="sans-serif" font-size="17">{val:,}</text>')
s.append('<text x="15" y="318" font-family="sans-serif" font-size="15">Source: response.log. Separate scales; multiplications include inverse powers. No preparation or timing claim.</text></svg>');(r/'norm-counts.svg').write_text(''.join(s))
t=['#set page(paper:"a4",margin:16mm)','#set text(size:9.1pt)']
for para in(r/'report.md').read_text().split('\n\n'):
 if para.startswith('# '):t.extend(['#heading(level:1,'+json.dumps(para[2:])+')','#image("flow.svg",width:100%)'])
 elif para:
  t.extend(['#text('+json.dumps(para)+')','#parbreak()'])
  if para.startswith('Matched norm costs.'):t.append('#image("norm-counts.svg",width:100%)')
(r/'report.typ').write_text('\n'.join(t));typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'));doc=pymupdf.open(r/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=pymupdf.Matrix(1.3,1.3)).save(r/f'page-{i+1}.png')
print('pages',len(doc))
