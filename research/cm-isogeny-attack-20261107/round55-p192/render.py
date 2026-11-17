"""Document rendering only; consumes saved native records and runs no research code."""
from pathlib import Path
import json,typst,pymupdf
r=Path(__file__).resolve().parent;c=json.loads((r/'native-verification.json').read_text())
flow=[['Explicit graph kernel and quadratic shift','Saved small-field kernel; dense coefficient construction charged'],['Regularize trace; evaluate at P=-U','N(-U) / (4*y(U)^2*f_r_prime(x(U))^2) = x(R0+psi(U))'],['Lift and select the rational point','Marked action psi(U)=[x]U; require 2R0 nonrational'],['Repeat action, then implicit BSGS','d-1 actions produce [x^d]G; charge both search stages']]
(r/'flow.json').write_text(json.dumps({'nodes':flow,'scope':'first three stages checked on r7; P192 normalization only'},indent=2)+'\n')
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="320"><rect width="1000" height="320" fill="white"/>']
for i,(a,b) in enumerate(flow):
 y=3+i*79;s.append(f'<rect x="10" y="{y}" width="980" height="72" rx="5" fill="#e7f0ec" stroke="#49617a"/><text x="25" y="{y+27}" font-family="sans-serif" font-size="22">{a}</text><text x="25" y="{y+54}" font-family="sans-serif" font-size="18">{b}</text>')
s.append('</svg>');(r/'flow.svg').write_text(''.join(s))
rows=[['Direct mixed traces',c['direct_trace_checks']],['Marked actions',c['marked_actions']],['Auxiliary recoveries',c['auxiliary_recoveries']],['Wrong auxiliaries rejected',c['wrong_auxiliary_rejections']]]
(r/'plot-data.json').write_text(json.dumps({'source':'native-verification.json','rows':rows,'unit':'exact arithmetic checks','targets':6,'uncertainty':'none; finite deterministic panel','not_a_runtime_comparison':True},indent=2)+'\n')
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="250"><rect width="1000" height="250" fill="white"/><text x="15" y="27" font-family="sans-serif" font-size="21">Round55 native control checks (six targets, r=7)</text>']
for i,(label,n) in enumerate(rows):
 y=45+40*i;s.append(f'<text x="15" y="{y+24}" font-family="sans-serif" font-size="19">{label}</text><rect x="310" y="{y}" width="{n*15}" height="30" fill="#496f8a"/><text x="{320+n*15}" y="{y+24}" font-family="sans-serif" font-size="20">{n}</text>')
s.append('<text x="15" y="232" font-family="sans-serif" font-size="17">Unit: exact check count. Source: native-verification.json. No timing inference.</text></svg>');(r/'counts.svg').write_text(''.join(s))
t=['#set page(paper:"a4",margin:16mm)','#set text(size:9.5pt)']
for para in (r/'report.md').read_text().split('\n\n'):
 if para.startswith('# '):t+=['#heading(level:1,'+json.dumps(para[2:])+')','#image("flow.svg",width:100%)']
 else:
  t+=['#text('+json.dumps(para)+')','#parbreak()']
  if para.startswith('Native exact checks.'):t+=['#image("counts.svg",width:100%)']
(r/'report.typ').write_text('\n'.join(t));typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'));doc=pymupdf.open(r/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap().save(r/f'page-{i+1}.png')
print('pages',len(doc))
