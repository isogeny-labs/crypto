"""Document layout only: render the recorded report and SVGs with Typst."""
from pathlib import Path
import json
import typst
study=Path(__file__).resolve().parent.parent
source=study/'ROOT_INTEGRATION_REPORT_167.md'
lines=['#set page(paper: "a4", margin: 1.4cm)','#set text(font: "DejaVu Sans", size: 9pt)','#set par(leading: 0.45em, spacing: 0.8em)','#set heading(numbering: none)']
for paragraph in source.read_text().split('\n\n'):
    paragraph=paragraph.strip()
    if not paragraph:continue
    if paragraph.startswith('!['):
        caption,asset=paragraph[2:].split('](',1);asset=asset[:-1]
        lines.extend(['#image('+json.dumps(asset)+', width: 100%)','#text(size: 8pt, fill: rgb("#466078"), '+json.dumps(caption)+')','#parbreak()'])
    elif paragraph.startswith('#'):
        prefix,body=paragraph.split(' ',1);lines.append('#heading(level: '+str(len(prefix))+', '+json.dumps(body)+')')
    else:
        for part in paragraph.split('\n- '):
            lines.extend(['#text('+json.dumps(part.removeprefix('- ').replace('`',''))+')','#parbreak()'])
(study/'root-integration-report-167.typ').write_text('\n'.join(lines)+'\n')
typst.compile(str(study/'root-integration-report-167.typ'),output=str(study/'root-integration-report-167.pdf'),root=str(study))
