"""Render existing source report and SVG; never imports the materializer."""
from pathlib import Path
import textwrap
import fitz
p=Path(__file__).resolve().parent
doc=fitz.open();page=doc.new_page(width=595,height=842);y=42
for paragraph in (p/'REPORT.md').read_text().split('\n\n'):
    heading=paragraph.startswith('#')
    text=paragraph.lstrip('# ').replace('`','')
    lines=[]
    for line in text.splitlines():lines.extend(textwrap.wrap(line,width=92 if not heading else 75))
    height=len(lines)*(15.6 if heading else 13)+18
    if y+height>790:page=doc.new_page(width=595,height=842);y=42
    page.insert_text((40,y),'\n'.join(lines),fontsize=10 if not heading else 12,lineheight=1.3)
    y+=height
page=doc.new_page(width=595,height=842)
svg=fitz.open(stream=(p/'pipeline.svg').read_bytes(),filetype='svg');diagram=fitz.open('pdf',svg.convert_to_pdf());page.show_pdf_page(fitz.Rect(25,45,570,240),diagram,0)
page.insert_text((40,280),'Source-fidelity review and historical receipt readback.',fontsize=11)
doc.save(p/'report.pdf')
for i,page in enumerate(doc): page.get_pixmap().save('/var/tmp/dispatch-review-page-%s.png'%i)
