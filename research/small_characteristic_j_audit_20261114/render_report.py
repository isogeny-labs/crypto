"""Render recorded formal counts/scopes; no research computation."""
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
b=ParagraphStyle('b',fontName='DV',fontSize=8.8,leading=12,spaceAfter=11)
t=ParagraphStyle('t',fontName='DVB',fontSize=16,leading=20,spaceAfter=12)
def p(s):return Paragraph(s,b)
d=Drawing(500,108);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="108" viewBox="0 0 500 108">']
def label(x,y,s,size=8.7):
 d.add(String(x,y,s,fontName='DV',fontSize=size));svg.append(f'<text x="{x}" y="{108-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(s)}</text>')
label(0,91,'Same F3 curve: y²=x³+2x²+1, actual Δ=1 / j=1 / rational point count=5',8.6)
for y,w,txt,n in [(60,60,'Rational subgroup killed by three','1'),(27,180,'Geometric subgroup killed by three','3')]:
 label(0,y,txt);d.add(Rect(242,y-4,w,15,fillColor=colors.HexColor('#2c788e'),strokeColor=None));label(250+w,y,n)
 svg.append(f'<rect x="242" y="{108-y-11}" width="{w}" height="15" fill="#2c788e"/>')
svg.append('</svg>');(o/'rational-geometric-control.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('Actual geometric p-torsion and j=0, in chars two and three',t),
 p('<b>Result.</b> Every nonsingular general Weierstrass model over every algebraically closed field of characteristic p=2 or3: actual subgroup killed by p has count1 if j=0, otherwise p; the actual induced ZModp module has rank0 if j=0, otherwise1. Equivalently, j=0 iff every actual killed-p point is the identity. Every algebraically closed scalar extension has the count determined by the base Mathlib j-invariant. Actual IsElliptic means unit discriminant, equivalent to nonzero discriminant over a field. No rank, basis, normal-form, trace or pairing premise. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>Actual proof bridges.</b> In characteristic three, the literal division polynomial evaluates to b2*x³+b8. If b2=0, the actual discriminant/invariant relation forces b8≠0, so only the identity is killed by three. If b2≠0, construct an actual cube root and ordinate, hence a nonzero nonsingular point killed by three; the previous actual one-or-three classification forces three. Combine Mathlib’s existing j=0 iff b2=0. In characteristic two reuse the earlier actual subgroup classification and official j=0 iff a1=0. Construct the actual two-torsion module; finite counts determine module ranks. Official map_j and injective field maps prove scalar-extension criteria. [pcc:unverified]'),
 p('<b>Lean negative control.</b> The actual F3 curve above has five rational points, so its rational killed-three subgroup is trivial, despite j=1. Its geometric killed-three subgroup has three points. Kernel-checked point enumeration and an actual group-order argument establish these facts. Lean negates the concrete iff and the universal statement obtained by dropping algebraic closure. This tests a deleted hypothesis and does not refute the qualified geometric criterion. [pcc:unverified]'),
 p('<b>Primary source and qualifications.</b> <link href="https://www.jmilne.org/math/Books/EC2.pdf" color="blue">Milne, Elliptic Curves</link>, selected Theorem3.12/context/Remark3.16, printedpp98–99, and notes p75; PDFpages104/80 rendered and inspected. The remark describes algebraically closed p-power torsion ranks, while p75 describes endomorphism-rank terminology. No equivalence is assumed here; only p itself in characteristics2/3 is handled. Primary book URL/hash retained locally, no raw book republished or full-book reading claimed. Prior actual torsion proofs and official Mathlib coefficient identities credited. KB/Conductor unavailable; bounded fallback recorded. [pcc:unverified]'),
 p('<b>Scope kept open.</b> The original prime-field node already requires p≥5, so characteristic-three examples cannot refute it. Existing conditional trace-zero arithmetic is credited. The FST trace-based definition, endomorphism-rank equivalence, Hasse/trace geometry, general embedding degree, other-prime geometry/full all-k p&lt;500 census and remaining fourteen-item obligations remain open. [pcc:unverified]'),
 p(f'<b>Validation.</b> Whole gate{g["declarations"]}/0failures/0trusted native exceptions; complete declaration binding/recursive source-config pins. All{a["files"]}attempt-provenance files retained losslessly in{a["unique_objects"]}objects; five gate records and failed source/config/log/exit snapshots. Pinned Lean4.35.0/Mathlib56a00585…, only standard foundations. Exact local PCC replay is separate from signed standing. No novelty, qualified-source refutation, corpus promotion, native experiment, paid compute, upload, holes/custom axioms/native evaluator/raised limits/new global flags/waivers. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/small-characteristic-j-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
