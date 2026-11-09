from pathlib import Path
import json,typst,pymupdf
r=Path(__file__).resolve().parent
a=json.loads((r/"denominators.json").read_text())
def w(n,v):(r/n).write_text(json.dumps(v,indent=2)+"\n")
rows=a['panels'];s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="270"><rect width="1000" height="270" fill="white"/><text x="15" y="25" font-size="22" font-family="sans-serif">Matched denominator K multiplications: Horner / resultant</text>']
for i,p in enumerate(rows):
 y=48+i*63;s.append(f'<text x="15" y="{y+20}" font-size="18" font-family="sans-serif">r={p["r"]}</text>')
 for j,(k,c)in enumerate([('horner_counts','#42876a'),('resultant_counts','#557b9e')]):
  v=p[k][0];width=650*v/max(1,p['resultant_counts'][0],p['horner_counts'][0]);s.append(f'<rect x="140" y="{y+j*25}" width="{width}" height="22" fill="{c}"/><text x="{150+width}" y="{y+17+j*25}" font-size="16" font-family="sans-serif">{v:,}</text>')
s.append('<text x="15" y="258" font-size="14" font-family="sans-serif">Source: denominators.json. Separate row scales; green Horner, blue resultant. No timing claim.</text></svg>');(r/'counts.svg').write_text(''.join(s));w('plot-data.json',rows)
t=['#set page(paper:"a4",margin:16mm)','#set text(size:9.2pt)']
for p in(r/'report.md').read_text().split('\n\n'):
 if p.startswith('# '):t+=['#heading(level:1,'+json.dumps(p[2:])+')','#image("counts.svg",width:100%)']
 elif p:t+=['#text('+json.dumps(p)+')','#parbreak()']
(r/'report.typ').write_text('\n'.join(t));typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'));doc=pymupdf.open(r/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap().save(r/f'page-{i+1}.png')
