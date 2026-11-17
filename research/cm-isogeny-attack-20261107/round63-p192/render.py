"""Render the retained proof interface and exact primitive counts; no trials."""
from pathlib import Path
import json,html,typst,pymupdf
r=Path(__file__).resolve().parent
flow=[['Public G,Q -> signed baby / giant tables','b+n addition calls and b+1+n point slots per generator; exact final-block clipping'],['Plus/minus factors -> first spectral jet','(L-J+s)(L+J+s) = (L^2-J^2) + s*(2L); preserve relative y signs'],['Baby coordinates -> norm algebra','H(Z), beta=x(jQ), gamma=y(jG)y(jQ); norm(E0+s E1)/norm(D^2)'],['Singular determinant -> calibrated point action','Unit-only pivots retain B at zero constant; one calibration and seven shifts decode']]
(r/'flow.json').write_text(json.dumps(flow,indent=2)+'\n');s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="320"><rect width="1000" height="320" fill="white"/>']
for i,(a,b) in enumerate(flow):
 y=4+79*i;s.append(f'<rect x="5" y="{y}" width="990" height="73" rx="6" fill="#e7f0ec" stroke="#49617a"/><text x="18" y="{y+27}" font-family="sans-serif" font-size="21">{html.escape(a)}</text><text x="18" y="{y+55}" font-family="sans-serif" font-size="16">{html.escape(b)}</text>')
s.append('</svg>');(r/'flow.svg').write_text(''.join(s))
rows=json.loads((r/'plot-data.json').read_text())['panels'];s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="350"><rect width="1000" height="350" fill="white"/><text x="15" y="25" font-family="sans-serif" font-size="22">Group-add calls per coefficient query (identity and doubling included)</text><rect x="15" y="40" width="18" height="16" fill="#42876a"/><text x="40" y="54" font-family="sans-serif" font-size="17">Signed blocks</text><rect x="240" y="40" width="18" height="16" fill="#557b9e"/><text x="265" y="54" font-family="sans-serif" font-size="17">Complete-graph reference</text>']
for i,row in enumerate(rows):
 y=74+74*i;s.append(f'<text x="15" y="{y+23}" font-family="sans-serif" font-size="18">F{row["p"]},r={row["r"]}</text>')
 for j,(key,color) in enumerate([('query_add_calls','#42876a'),('reference_query_add_calls','#557b9e')]):
  val=row[key];w=val*1.65;s.append(f'<rect x="210" y="{y+j*28}" width="{w}" height="24" fill="{color}"/><text x="{220+w}" y="{y+18+j*28}" font-family="sans-serif" font-size="17">{val}</text>')
s.append('<text x="15" y="327" font-family="sans-serif" font-size="15">Source: response.log. Per query; setup, field operations, decoding and references separately retained.</text></svg>');(r/'point-calls.svg').write_text(''.join(s))
t=['#set page(paper:"a4",margin:16mm)','#set text(size:9.1pt)']
for para in (r/'report.md').read_text().split('\n\n'):
 if para.startswith('# '):t.extend(['#heading(level:1,'+json.dumps(para[2:])+')','#image("flow.svg",width:100%)'])
 elif para:
  t.extend(['#text('+json.dumps(para)+')','#parbreak()'])
  if para.startswith('Exact point-call costs.'):t.append('#image("point-calls.svg",width:100%)')
(r/'report.typ').write_text('\n'.join(t));typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'));doc=pymupdf.open(r/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=pymupdf.Matrix(1.3,1.3)).save(r/f'page-{i+1}.png')
print('pages',len(doc))
