from pathlib import Path
import json,typst,pymupdf
r=Path(__file__).resolve().parent;j=json.loads((r/'audit-summary.json').read_text());p=j['P192'];baseline=json.loads((r/'per-block-P192.json').read_text());old_norm=sum(b['numerator_norm_counts'][0]for row in baseline['rows']for b in row['blocks']);data=[('selected query',p['candidate']['selected'][0],p['per-block']['selected'][0]),('field norms',p['final_norm_counts'][0],old_norm),('extra jet merges',p['quotient_jet_merge_counts'][0],0)];(r/'plot-data.json').write_text(json.dumps(data,indent=2)+'\n');s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="270"><rect width="1000" height="270" fill="white"/><text x="15" y="25" font-family="sans-serif" font-size="22">P192: 24 prescribed responses, explicit K multiplication calls</text>']
for i,(label,a,b)in enumerate(data):
 y=48+63*i;s.append(f'<text x="15" y="{y+20}" font-family="sans-serif" font-size="17">{label}</text>')
 for k,(v,color)in enumerate([(a,'#42876a'),(b,'#557b9e')]):
  width=620*v/19449;s.append(f'<rect x="200" y="{y+k*25}" width="{width}" height="21" fill="{color}"/><text x="{width+210}" y="{y+16+k*25}" font-family="sans-serif" font-size="16">{v:,}</text>')
s.append('<text x="15" y="260" font-family="sans-serif" font-size="14">Green one norm; blue per-block. Shared scale. Constructors unchanged; all merge/reduction costs charged.</text></svg>');(r/'costs.svg').write_text(''.join(s))
(r/'flow.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="115"><rect width="1000" height="115" fill="white"/><g fill="#edf4f7" stroke="#557b9e"><rect x="5" y="10" width="230" height="75"/><rect x="275" y="10" width="300" height="75"/><rect x="615" y="10" width="380" height="75"/></g><g font-family="sans-serif" font-size="17"><text x="17" y="35">Cached numerator jets U_i</text><text x="17" y="63">nonzero denominators d_i</text><text x="239" y="50">→</text><text x="287" y="35">stream product mod H_G,s^2</text><text x="287" y="63">three products + two reductions</text><text x="579" y="50">→</text><text x="627" y="35">ONE field norm / product d_i</text><text x="627" y="63">combine scalar centers and tails</text></g><text x="12" y="108" font-family="sans-serif" font-size="14">Product proof includes zero jets and repeated roots; no numerator norm is inverted.</text></svg>''')
t=['#set page(paper:"a4",margin:16mm)','#set text(size:9.2pt)']
for para in(r/'report.md').read_text().split('\n\n'):
 if para.startswith('# '):t+=['#heading(level:1,'+json.dumps(para[2:])+')','#image("costs.svg",width:100%)','#image("flow.svg",width:100%)']
 elif para:t+=['#text('+json.dumps(para)+')','#parbreak()']
(r/'report.typ').write_text('\n'.join(t));typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'));doc=pymupdf.open(r/'report.pdf')
for i,page in enumerate(doc):page.get_pixmap().save(r/f'page-{i+1}.png')
print('pages',len(doc))
