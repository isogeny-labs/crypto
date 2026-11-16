"""Layout recorded formal statements and provenance; no research computation."""
from pathlib import Path
import html,json,fitz
from reportlab.platypus import SimpleDocTemplate,Paragraph,Spacer
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib import colors
from reportlab.graphics.shapes import Drawing,Rect,String
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
r=Path(__file__).resolve().parent;o=r/'reports';o.mkdir(exist_ok=True)
pdfmetrics.registerFont(TTFont('DV','/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf'))
pdfmetrics.registerFont(TTFont('DVB','/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf'))
pdfmetrics.registerFontFamily('DV',normal='DV',bold='DVB')
b=ParagraphStyle('b',fontName='DV',fontSize=8.7,leading=11.8,spaceAfter=10)
t=ParagraphStyle('t',fontName='DVB',fontSize=16,leading=20,spaceAfter=12)
def p(s):return Paragraph(s,b)
d=Drawing(500,142);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="142" viewBox="0 0 500 142">']
rows=[(96,'Bounded native audit',['62 named bindings in 65 immutable source plans','61 complete written headers; 1 issued prefix; 0 unresolved']),
      (50,'Concrete BK receipt',['Stored header176chars ends inside geomCurve (L:=L)','Official full written header913chars; original37-proof gate retained']),
      (4,'Evidence boundary',['Lean: genuine controls + delimiter-model noninjectivity','Native count is procedural; no false theorem/signed status inferred'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.8),(130,yy+25,lines[0],7.8),(130,yy+9,lines[1],8.1)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'statement-binding.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
archive_bytes=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('One issued Lean binding stops inside its signature',t),
 p('<b>Result.</b> The independent native Lean official-parser audit resolves62 named bindings in65 source plans added to this stacked feature:61 complete written signatures, one truncated issued binding, zero unresolved. Snapshot1f0eba8e31c5d77626b13c1ab932757056366739. Three native/catalog-only plans remain explicitly outside the theorem count. These are surface-header observations, not proofs of theorem truth or all historical claims. [pcc:unverified]'),d,Spacer(1,6),
 p('<b>Affected receipt.</b> PCC-20261110-99983d, BKActualCertificateAudit.bk_actual_case_audit_certificate, stores176characters ending at geomCurve (L, before a legal named-argument assignment. Stored hash f56bb53d… differs from full written-header913characters/hash1f432993…. The existing actual-object source, source pins and whole37-declaration Lean gate are preserved; the observation does not refute the BK theorem. A complete versioned wrapper is the next repair, with no old-claim/ledger/source rewrite. [pcc:unverified]'),
 p('<b>Independent parsing and controls.</b> The comparator uses official Lean4.35 command grammar, byte ranges, actual compiler parser context and qualified source-namespace selection; source and stored hashes are checked. Seven genuine declarations cover bars, named arguments, default proofs, inline by, comments, strings and a complete positive control. Actual frozen-extractor output is separately captured. Native comparison controls produce4prefix/1complete/2explicitunsupported normalization cases, expected exit1. No unsupported case is counted as a pass. [pcc:unverified]'),
 p('<b>Failed pass retained.</b> The first corpus pass gives58complete/1prefix/3unsupported. Source-declared Matrix notation context and a same-short-name namespace ambiguity are diagnosed; the second pass uses identical immutable inputs and finishes61/1/0. A public-equivalent-tree replay reproduces61/1/0. All63 candidate source hashes/pins match. The final bounded headers contain no unsupported comment/string policy cases. [pcc:unverified]'),
 p('<b>What Lean proves.</b> Kernel-checked semantic controls and a transparent List-Char delimiter model: concrete unequal strings have equal prefixes for three delimiter constants, so each model function is not injective. The model is not a formal refinement of Python regex, native auditor or SHA256. Native counts remain procedural evidence. Complete written headers also do not bind ambient variable telescopes, all definitions/dependencies or fully elaborated kernel types; canonical semantic binding remains a separate contract. [pcc:unverified]'),
 p(f'<b>Provenance and checks.</b> Whole model/control gate {g["declarations"]}/0 failures/0 trusted native exceptions; {a["files"]} build/runtime/input/provenance files preserved losslessly in {a["unique_objects"]} objects, including failed controls/first pass and final gate. Long-dictionary XZ raw-tree archive is{archive_bytes}bytes, below65536; full-signature index/raw written headers and immutable Git restore recipe retained. Frozen verifier SHA5e1a9703… unchanged. Lean4.35.0/Mathlib56a00585…, standard foundations/default settings. Signed standing unverified. No holes/custom axioms/native proof evaluation/unsafe initializer call/flags/limits/waivers/cryptographic experiment/spend/upload. Original fourteen-item gaps remain. [pcc:unverified]')]

SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595.276,841.89),leftMargin=47,rightMargin=47,topMargin=35,bottomMargin=35).build(flow)
doc=fitz.open(o/'report.pdf');layout={'pages':[],'visual_inspection':'pending'}
for i,page in enumerate(doc):
 bad=[]
 for block in page.get_text('dict')['blocks']:
  for line in block.get('lines',[]):
   for span in line['spans']:
    x0,y0,x1,y1=span['bbox']
    if x0<0 or y0<0 or x1>page.rect.width or y1>page.rect.height:bad.append(span['text'])
 layout['pages'].append({'page':i+1,'text_within_page':not bad,'out_of_bounds':bad})
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/statement-binding-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
