"""Render archived qualification values only; no fixture execution."""
from pathlib import Path
import csv,html,json,textwrap
import fitz
p=Path(__file__).resolve().parent
r=json.loads((p/'result.json').read_text())
with (p/'metrics.csv').open('w',newline='') as f:
 w=csv.writer(f);w.writerow(['case','assertions_passed','storage_consistency','reported_status','committed_records','inline_records','unreferenced_tail_bytes','rejected'])
 for c in r['cases']:
  d=c['details'];w.writerow([c['case'],c['passed'],d.get('consistency',''),d.get('reported_status',''),d.get('committed_records',''),d.get('inline_records',''),d.get('unreferenced_tail_bytes',''),d.get('rejected','')])
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="920" height="640" viewBox="0 0 920 640">','<rect width="920" height="640" fill="white"/>','<text x="24" y="30" font-size="21" font-family="sans-serif">Acknowledged cleanup records in retained receipts</text>']
for i,c in enumerate(r['cases']):
 y=70+i*31;d=c['details'];n=d.get('committed_records')
 svg.append(f'<text x="24" y="{y+15}" font-size="13" font-family="sans-serif">{html.escape(c["case"])}</text>')
 if n is not None:
  svg.append(f'<rect x="420" y="{y}" width="{n*150}" height="20" fill="#376d96"/><text x="{430+n*150}" y="{y+15}" font-size="13" font-family="sans-serif">{n}</text>')
 else:svg.append(f'<text x="420" y="{y+15}" font-size="13" font-family="sans-serif">Rejected malformed receipt (count not admitted)</text>')
svg.extend(['<text x="24" y="570" font-size="13" font-family="sans-serif">Units: records. 15 deterministic cases; no sampling uncertainty or comparative timing estimate.</text>','<text x="24" y="594" font-size="13" font-family="sans-serif">Run: pencil-materializer-qualification-20261109. Source: result.json; failures remain failed/incomplete.</text>','<text x="24" y="618" font-size="13" font-family="sans-serif">A passing assertion means the expected storage behavior was observed, not successful materialization.</text>','</svg>'])
(p/'records.svg').write_text('\n'.join(svg)+'\n')
doc=fitz.open();page=doc.new_page(width=595,height=842);y=42
for para in (p/'REPORT.md').read_text().split('\n\n'):
 heading=para.startswith('#');text=para.lstrip('# ').replace('`','');lines=[]
 for line in text.splitlines():lines.extend(textwrap.wrap(line,width=90 if not heading else 72))
 height=len(lines)*(15 if heading else 13)+17
 if y+height>790:page=doc.new_page(width=595,height=842);y=42
 page.insert_text((40,y),'\n'.join(lines),fontsize=12 if heading else 10,lineheight=1.3);y+=height
page=doc.new_page(width=842,height=650);s=fitz.open(stream=(p/'records.svg').read_bytes(),filetype='svg');d=fitz.open('pdf',s.convert_to_pdf());page.show_pdf_page(fitz.Rect(20,20,822,630),d,0)
doc.save(p/'report.pdf')
for i,page in enumerate(doc):page.get_pixmap(matrix=fitz.Matrix(1,1)).save('/var/tmp/pencil-qualification-report-page-'+str(i)+'.png')
print('pages',len(doc))
