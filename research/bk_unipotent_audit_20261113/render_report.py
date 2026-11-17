"""Render proved symbolic criteria; no native research computation."""
from pathlib import Path
import json,fitz
from reportlab.platypus import SimpleDocTemplate,Paragraph,Spacer
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib import colors
from reportlab.graphics.shapes import Drawing,Rect,String
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
r=Path(__file__).resolve().parent;o=r/'reports';o.mkdir(exist_ok=True)
pdfmetrics.registerFont(TTFont('DV','/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf'));pdfmetrics.registerFont(TTFont('DVB','/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf'));pdfmetrics.registerFontFamily('DV',normal='DV',bold='DVB')
b=ParagraphStyle('b',fontName='DV',fontSize=8.8,leading=11.8,spaceAfter=7);t=ParagraphStyle('t',fontName='DVB',fontSize=16,leading=19,spaceAfter=11)
def p(s):return Paragraph(s,b)
d=Drawing(500,102);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="102" viewBox="0 0 500 102">']
def label(x,y,txt,size=7.5):
 d.add(String(x,y,txt,fontName='DV',fontSize=size,fillColor=colors.HexColor('#234b70')));svg.append(f'<text x="{x}" y="{102-y}" font-family="DejaVu Sans,sans-serif" font-size="{size}" fill="#234b70">{txt}</text>')
label(0,89,'Positive extension degree k');label(0,56,'3 divides 7^k−1');label(0,25,'All geometric 3-torsion rational')
for k in range(1,13):
 x=189+25*(k-1);label(x+4,89,str(k))
 for y,yes in [(47,True),(16,k%3==0)]:
  fill='#e1f1e6' if yes else '#fbe6df';ink='#20633d' if yes else '#9d3428';txt='yes' if yes else 'no'
  d.add(Rect(x,y,23,22,rx=2,fillColor=colors.HexColor(fill),strokeColor=colors.white));svg.append(f'<rect x="{x}" y="{102-y-22}" width="23" height="22" rx="2" fill="{fill}"/>');d.add(String(x+11.5,y+8,txt,fontName='DV',fontSize=7,textAnchor='middle',fillColor=colors.HexColor(ink)));svg.append(f'<text x="{x+11.5}" y="{102-y-8}" text-anchor="middle" font-family="DejaVu Sans,sans-serif" font-size="7" fill="{ink}">{txt}</text>')
svg.append('</svg>');(o/'extension-criterion.svg').write_text('\n'.join(svg)+'\n');g=json.loads((r/'evidence/gate.json').read_text())
flow=[Paragraph('BK boundary: actual torsion field versus roots of unity',t),
 p('<b>Actual result.</b> For E/F7: y²=x³+3x+2, Lean proves Δ≠0, #E(F7)=9 and #E[3](F7bar)=9. For every positive k and every compatible embedding of a field of size 7^k, all geometric 3-torsion is rational exactly when 3 divides k. Yet 3 divides 7^k−1 for every k; the cyclotomic embedding degree is 1. Signed standing: unverified. [pcc:unverified]'),
 d,Spacer(1,9),
 p('<b>Figure.</b> These are corollaries of the all-k Lean theorem, not sampled measurements. The entire geometric torsion first becomes rational in degree 3; the rows disagree for infinitely many degrees, including every 3n+1. [pcc:unverified]'),
 p('<b>Coordinate proof.</b> The actual point/S3 equivalence factors the literal division quartic as 3(x−5)(x³+5x²+3x+2). The rational branch has y=±3. On the cubic branch, y=±3x, x^7=6x²+6x+3, and (6x²+6x+3)²=x+1. Explicit small polynomial factors and (7:F7[X])=0 prove the identities by checked ring arithmetic. They give actual point Frobenius cubed equal to the identity on all 3-torsion. [pcc:unverified]'),
 p('<b>All extensions.</b> The existing actual nonfixed geometric point forces period 3. Modular decomposition of every k then gives fixedness iff 3 divides k. The existing proved finite-field descent connects those actual Frobenius iterates to actual point-map images, for every chosen field and compatible embedding. GaloisField7k cardinality and algebraic-closure embeddings are constructed. The nine-point geometric count follows from the existing arbitrary short-curve theorem after checking the actual curve and discriminant. No torsion basis, determinant, Weil pairing or rationality conclusion is assumed. [pcc:unverified]'),
 p('<b>Source correction.</b> Original BK1998 Theorem1, pp142–143, explicitly excludes r dividing q−1. Its Remarks already warn that r² dividing #E does not repair deleting this hypothesis, and give a different F19 example. Our actual F7 certificate confirms that excluded-boundary mechanism. It disproves the unrestricted corpus reading; no qualified source theorem or novelty claim is asserted. [pcc:unverified]'),
 p(f'<b>Checks.</b> Whole-project gate {g["declarations"]}/0failures/0trusted. Lean4.35.0 / Mathlib56a00585…. Only standard foundations. No holes, custom axioms, native evaluator, raised limits, global elaboration flags or waivers. Failed source/log/exit records are retained. Exact complete main type and all dependency/artifact hashes are in sources.json; see proof-map.json and evidence/. [pcc:unverified]'),
 p('<b>Scope.</b> No native experiment, paid compute or agent upload. KB unavailable; original primary paper, existing actual F7/Frobenius/cardinality modules and pinned Mathlib reviewed. Generic all-prime rank/determinant/Weil bridges and the complete all-extension census remain open in the fourteen-item map. This directory does not assert a separately computed Frobenius matrix. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595.276,841.89),leftMargin=47,rightMargin=47,topMargin=37,bottomMargin=37).build(flow)
doc=fitz.open(o/'report.pdf');layout={'pages':[],'visual_inspection':'pending'}
for i,page in enumerate(doc):
 bad=[]
 for block in page.get_text('dict')['blocks']:
  for line in block.get('lines',[]):
   for span in line['spans']:
    x0,y0,x1,y1=span['bbox']
    if x0<0 or y0<0 or x1>page.rect.width or y1>page.rect.height:bad.append(span['text'])
 layout['pages'].append({'page':i+1,'text_within_page':not bad,'out_of_bounds':bad});page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/bk-unipotent-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n');print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
