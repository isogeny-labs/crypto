"""Render the existing proposal and diagram; no experimental calculations."""
from pathlib import Path
import json,typst
study=Path(__file__).resolve().parent.parent
text=(study/'HYPOTHESIS_PACKED_PIPELINE_182.md').read_text()
lines=['#set page(paper: "a4", margin: 1.2cm)','#set text(font: "DejaVu Sans", size: 8.5pt)','#set par(leading: 0.35em, spacing: 0.65em)','#set heading(numbering: none)']
for para in text.split('\n\n'):
 para=para.strip()
 if not para:continue
 if para.startswith('!['):
  cap,asset=para[2:].split('](',1);lines+=['#image('+json.dumps(asset[:-1])+', width: 100%)','#text(size: 8pt, '+json.dumps(cap)+')','#parbreak()']
 elif para.startswith('#'):
  pre,body=para.split(' ',1);lines.append('#heading(level: '+str(len(pre))+', '+json.dumps(body)+')')
 else:lines+=['#text('+json.dumps(para.replace('`',''))+')','#parbreak()']
(study/'packed-proposal-182.typ').write_text('\n'.join(lines)+'\n')
typst.compile(str(study/'packed-proposal-182.typ'),output=str(study/'packed-proposal-182.pdf'),root=str(study))
