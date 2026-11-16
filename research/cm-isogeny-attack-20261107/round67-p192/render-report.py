from pathlib import Path
import json
import typst,pymupdf
r=Path(__file__).resolve().parent;j=json.loads((r/'audit-summary.json').read_text());p=j['P192'];s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="270"><rect width="1000" height="270" fill="white"/><text x="15" y="25" font-family="sans-serif" font-size="22">P192: 24 prescribed responses, explicit K multiplication calls</text>']
for i,(label,key) in enumerate([('selected query','selected'),('numerator','numerator_formation_and_norm_counts'),('denominator','denominator_formation_and_norm_counts')]):
 y=48+63*i;s.append(f'<text x="15" y="{y+20}" font-family="sans-serif" font-size="17">{label}</text>')
 for n,(mode,color)in enumerate([('candidate','#42876a'),('expanded','#557b9e')]):
  v=p[mode][key][0];width=620*v/56637;s.append(f'<rect x="200" y="{y+n*25}" width="{width}" height="21" fill="{color}"/><text x="{width+210}" y="{y+16+n*25}" font-family="sans-serif" font-size="16">{v:,}</text>')
s.append('<text x="15" y="260" font-family="sans-serif" font-size="14">Green reduced; blue expanded. Shared scale. Setup separate. Source: audit-summary.json and raw P192 rows.</text></svg>');(r/'costs.svg').write_text(''.join(s))
(r/'flow.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="115"><rect width="1000" height="115" fill="white"/><g fill="#edf4f7" stroke="#557b9e"><rect x="5" y="10" width="225" height="75"/><rect x="270" y="10" width="280" height="75"/><rect x="590" y="10" width="400" height="75"/></g><g font-family="sans-serif" font-size="17"><text x="18" y="35">Public baby points G,Q</text><text x="18" y="63">cache beta,gamma,H_Q</text><text x="234" y="50">→</text><text x="282" y="35">Giant block: numerator norm</text><text x="282" y="63">divide by H_G(x)^2 H_Q(z)^2</text><text x="553" y="50">→</text><text x="602" y="35">center/tail product → B-only response</text><text x="602" y="63">calibration + seven shifts → decoder</text></g><text x="12" y="108" font-family="sans-serif" font-size="14">P192 executes prescribed 27-factor windows; the complete decoder panel runs on the small curves.</text></svg>''')
t=['#set page(paper:"a4",margin:16mm)','#set text(size:9.2pt)']
for para in (r/'report.md').read_text().split('\n\n'):
 if para.startswith('# '):t+=['#heading(level:1,'+json.dumps(para[2:])+')','#image("costs.svg",width:100%)','#image("flow.svg",width:100%)']
 elif para:t+=['#text('+json.dumps(para)+')','#parbreak()']
(r/'report.typ').write_text('\n'.join(t));typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'));doc=pymupdf.open(r/'report.pdf')
for i,page in enumerate(doc):page.get_pixmap().save(r/f'page-{i+1}.png')
print('pages',len(doc))
