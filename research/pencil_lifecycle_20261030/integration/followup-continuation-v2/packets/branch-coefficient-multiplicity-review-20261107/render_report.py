from pathlib import Path
import json, hashlib, textwrap
import fitz
p=Path(__file__).resolve().parent
lines=(p/'REVIEW.md').read_text().splitlines()
svg='''<svg xmlns="http://www.w3.org/2000/svg" width="1040" height="300" viewBox="0 0 1040 300"><rect width="1040" height="300" fill="#f7fafc"/><g font-family="sans-serif" fill="#17324d"><text x="25" y="32" font-size="21">Coefficient-data multiplicity: source composition</text>'''
for x,y,w,title,detail in [(25,65,300,'Actual coefficientData','hb + valid1 + valid2'),(365,65,300,'Source norm identity','n^2 - u1*u2*U = b^2*f'),(705,65,310,'Generic multiplicity theorem','hroot + simple root + hsupport')]:
 svg+=f'<rect x="{x}" y="{y}" width="{w}" height="90" rx="9" fill="#e2eaf3"/><text x="{x+15}" y="{y+30}" font-size="17">{title}</text><text x="{x+15}" y="{y+60}" font-size="15">{detail}</text>'
svg+='<path d="M325 110H355 M665 110H695" stroke="#17324d" stroke-width="3"/><text x="25" y="208" font-size="18">Separable quintic + branch root supplies simple-root condition.</text><text x="25" y="243" font-size="18">Conclusion: u1,u2,U nonzero; multiplicities at a sum to 1.</text><text x="25" y="278" font-size="17" fill="#8b4513">Source fidelity supported; no kernel compilation or axiom readback.</text></g></svg>'
(p/'pipeline.svg').write_text(svg)
doc=fitz.open(); page=None;y=0
for line in lines:
 if page is None or y>730:
  page=doc.new_page(width=595,height=842);y=45
 size=14 if line.startswith('# ') else 11 if line.startswith('## ') else 10
 text=line.lstrip('# ').replace('`','')
 for wrapped in textwrap.wrap(text,width=98 if size==10 else 80) or ['']:
  if y>780: page=doc.new_page(width=595,height=842);y=45
  page.insert_text((36,y),wrapped,fontsize=size);y+=size+4
 y+=4
page=doc.new_page(width=595,height=842)
image=fitz.open(stream=svg.encode(),filetype='svg'); pdf=fitz.open('pdf',image.convert_to_pdf());page.show_pdf_page(fitz.Rect(20,50,575,220),pdf,0)
page.insert_text((25,270),'Diagram of proof dependencies; no quantitative measurements are plotted.',fontsize=10)
doc.save(p/'report.pdf')
(p/'pdf-inspection.json').write_text(json.dumps({'pages':len(doc),'text_nonempty_all_pages':all(bool(x.get_text().strip()) for x in doc),'renderer':'PyMuPDF '+fitz.VersionBind,'raster_visual_inspection':'pending'},indent=2)+'\n')
doc[0].get_pixmap(matrix=fitz.Matrix(1,1)).save(p/'report-preview.png')
print('Rendered',len(doc),'pages')
