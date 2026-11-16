"""Render retained status-map.json only; no experiment or repricing."""
from pathlib import Path
import json, hashlib, html, textwrap
import fitz
P=Path(__file__).resolve().parent
x=json.loads((P/'status-map.json').read_text())
labels=[('Factor-base / graph','Five fixed arms x 1,260 targets','Further graph policies: prospective'),('Weighted genus-2 law','Two arms x 1,260 targets','Global class / dispatcher joins remain'),('Point-driven pencil','543 historical solved keys; C1 separate','Full matched 1,260-target work remains'),('Kummer / labels','N19: 1,260 verified cases','Natural exceptional closure remains')]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="1050" height="590" viewBox="0 0 1050 590">','<rect width="1050" height="590" fill="#f7f9fc"/>','<g font-family="sans-serif" fill="#17263b">','<text x="30" y="42" font-size="25">Four followup evidence map</text>','<text x="30" y="72" font-size="16">Historical records only - no new replay, repricing, or experimental metrics</text>']
for i,(a,b,c) in enumerate(labels):
 y=100+i*108
 svg += [f'<rect x="30" y="{y}" width="990" height="92" rx="8" fill="white" stroke="#b9c7d7"/>',f'<text x="48" y="{y+27}" font-size="20">{html.escape(a)}</text>',f'<text x="48" y="{y+53}" font-size="17">{html.escape(b)}</text>',f'<text x="48" y="{y+77}" font-size="15" fill="#455870">{html.escape(c)}</text>']
svg += ['<text x="30" y="563" font-size="14">Counts belong to distinct retained arms; pencil and C1 are not added. Source: status-map.json.</text>','</g></svg>']
(P/'evidence-map.svg').write_text('\n'.join(svg)+'\n')
doc=fitz.open()
def page(title):
 p=doc.new_page(width=595,height=842)
 p.insert_text((35,38),title,fontsize=17,color=(.08,.16,.26))
 p.insert_text((35,815),'Source-only historical reconciliation | no replay or repricing | '+str(len(doc)),fontsize=8)
 return p
p=page('Four followup evidence map')
s=fitz.open(stream=(P/'evidence-map.svg').read_bytes(),filetype='svg'); q=fitz.open('pdf',s.convert_to_pdf());p.show_pdf_page(fitz.Rect(25,65,570,380),q,0)
t='This report locates retained evidence for the four approved followups. It does not rerun measurements, reprice rows, or issue a new scientific verdict. The original 1,260-target obligation is preserved.\n\nThe source map retains the recorded numerical results, failure counts and missing components. Artifact hashes copied from the earlier inventory are historical receipts, not fresh artifact readbacks.\n\nHistorical audit commit: '+x['historical_audit_commit']+'\n\nDetailed deliverables follow on each lead page. PDF links reference the recorded source paths. Machine-readable companions: status-map.json and status-map.csv. Current campaign accounting remains in the coordinator ledger; historical deadlines do not renew authorization.'
assert p.insert_textbox(fitz.Rect(35,405,560,780),t,fontsize=11,lineheight=1.35)>=0
for row in x['rows']:
 p=page(row['lane']); y=66
 for heading,key in [('Recorded component','achieved'),('Remaining component','remaining')]:
  p.insert_text((35,y),heading,fontsize=12,color=(.08,.2,.35));y+=17
  lines=textwrap.wrap(row[key],width=87); h=len(lines)*13+8
  assert p.insert_textbox(fitz.Rect(35,y,560,y+h),'\n'.join(lines),fontsize=10,lineheight=1.2)>=0;y+=h+10
 p.insert_text((35,y),'Existing detailed artifacts (historical locators)',fontsize=12);y+=20
 for artifact in row['deliverables']:
  path=artifact.get('path',artifact.get('path_suffix')); lines=textwrap.wrap(path,width=107,break_long_words=True,break_on_hyphens=False);h=len(lines)*10+3
  assert p.insert_textbox(fitz.Rect(35,y,560,y+h),'\n'.join(lines),fontsize=8,lineheight=1.12)>=0
  p.insert_link({'kind':fitz.LINK_URI,'from':fitz.Rect(35,y,560,y+h),'uri':path if path.startswith('/') else 'https://github.com/isogeny-labs/crypto/tree/'+x['historical_audit_commit']+'/research/'+path})
  y+=h+5
 assert y<795,y
 p.insert_text((35,790),'Evidence level: retained records inspected, not replayed in this pass.',fontsize=8)
doc.save(P/'report.pdf',garbage=4,deflate=True)
preview=Path('/var/tmp/four-followup-status-preview-20261109');preview.mkdir(exist_ok=True)
for i,p in enumerate(doc):p.get_pixmap(matrix=fitz.Matrix(1,1)).save(preview/f'page-{i+1}.png')
(P/'render-receipt.json').write_text(json.dumps({'renderer':'PyMuPDF','pages':len(doc),'source_sha256':hashlib.sha256((P/'status-map.json').read_bytes()).hexdigest(),'preview_directory':str(preview),'new_experiments':False},indent=2)+'\n')
print('rendered',len(doc),'pages')
