"""Render the recorded mathematical argument and pins; no research computation."""
from pathlib import Path
import json,fitz
from reportlab.platypus import SimpleDocTemplate,Paragraph,Spacer
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib import colors
from reportlab.graphics.shapes import Drawing,Rect,Line,String
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
r=Path(__file__).resolve().parent;o=r/'reports';o.mkdir(exist_ok=True)
pdfmetrics.registerFont(TTFont('DV','/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf'));pdfmetrics.registerFont(TTFont('DVB','/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf'));pdfmetrics.registerFontFamily('DV',normal='DV',bold='DVB')
b=ParagraphStyle('b',fontName='DV',fontSize=8.8,leading=11.8,spaceAfter=7);t=ParagraphStyle('t',fontName='DVB',fontSize=16,leading=19,spaceAfter=11)
def p(s):return Paragraph(s,b)
blue=colors.HexColor('#234b70');d=Drawing(500,134)
boxes=[(0,82,'Original canonical polynomial','Actual InputRing/tower equivalence'),(173,82,'All adjacent swaps','S3/S4 bases + actual recurrence'),(346,82,'Actual variable renamings','Scalar and coordinate maps proved'),(346,0,'All finite permutations','Mathlib adjacent-generator closure'),(173,0,'Actual MV IsSymmetric','Full arity m=n+3≥3'),(0,0,'Pullback to original source','Every input; coefficient field fixed')]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="134" viewBox="0 0 500 134">']
for x,y,a,c in boxes:
 d.add(Rect(x,y,154,48,rx=5,fillColor=colors.HexColor('#edf5fa'),strokeColor=blue));svg.append(f'<rect x="{x}" y="{134-y-48}" width="154" height="48" rx="5" fill="#edf5fa" stroke="#234b70"/>')
 for k,txt in enumerate([a,c]):
  d.add(String(x+77,y+29-k*15,txt,fontName='DV',fontSize=7.4,textAnchor='middle',fillColor=blue));svg.append(f'<text x="{x+77}" y="{134-y-29+k*15}" text-anchor="middle" font-family="DejaVu Sans,sans-serif" font-size="7.4" fill="#234b70">{txt}</text>')
for x1,y1,x2,y2 in[(154,106,172,106),(327,106,345,106),(423,82,423,49),(346,24,328,24),(173,24,155,24)]:
 d.add(Line(x1,y1,x2,y2,strokeColor=blue));svg.append(f'<path d="M{x1},{134-y1} L{x2},{134-y2}" stroke="#234b70"/>')
svg.append('</svg>');(o/'proof-flow.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text())
flow=[Paragraph('Full symmetry of the actual canonical polynomials',t),
 p('<b>Proved component.</b> For every field K, actual Weierstrass curve W and stage n, the existing recursivePolynomial W n is invariant under every permutation of its m=n+3 inputs. Its literal multivariate image is IsSymmetric. Signed standing: unverified. [pcc:unverified]'),
 p('<b>Actual action.</b> The pullback permutation sends inputVariable(i) to inputVariable(σ(i)), fixes every C(coefficientEmbedding n a), and identifies outer coordinate0 with X. Explicit ring equivalences connect this action and the multivariate polynomial to the original canonical source. No symmetry or recurrence is assumed. [pcc:unverified]'),
 d,Spacer(1,10),
 p('<b>Induction.</b> S3 gives the arity3 base. Actual-curve S4 middle symmetry gives the arity4 case. The proved source-ordered S4 block handles the newest-three boundary swap at later stages. Older swaps pass through the actual resultant coefficient maps and the prior stage’s invariance. This proves every adjacent swap at every stage. [pcc:unverified]'),
 p('<b>Global bridge.</b> An explicit multivariate/tower equivalence maps every scalar and variable correctly. Each tower adjacent action is proved equal to the corresponding Fin-index variable renaming. Mathlib mclosure_swap_castSucc_succ says these swaps generate the finite symmetric group. The resulting IsSymmetric theorem is pulled back to the actual original polynomial. [pcc:unverified]'),
 p('<b>Scope and control.</b> No nonsingularity or characteristic restriction is required for this polynomial identity. The inherited rational independent-b S4 counterexample b=(0,0,0,1), inputs(0,1,2,3), refutes an unrestricted coefficient extension; the parent Lean proof rules out a rational curve with those b-coefficients. No qualified source theorem is refuted. General splits k≥3 and higher absolute irreducibility remain open. [pcc:unverified]'),
 p(f'<b>Checks.</b> Whole-project gate {g["declarations"]}/0failures/0trusted. Lean4.35.0 / Mathlib56a00585…. Only standard foundations; no holes, custom axioms, native evaluation, raised limits, global elaboration flags or waivers. Tower is a transparent abbreviation for actual polynomial rings. See sources.json, proof-map.json and evidence/ for exact bindings, complete custom source pins, failed attempts and whole-gate records. [pcc:unverified]'),
 p('<b>Sources.</b> Original Semaev2004 Theorem1/p2; Kosters–Yeo2015 Section2/p3 general formula and recursion; actual canonical/transport/S4 modules and the immutable source-order packet; Mathlib finite permutation closure and MV rename/equivalence APIs. KB unavailable; bounded fallback, no novelty claim. No native experiment, paid compute or agent upload. The all-fourteen audit retains its remaining obligations. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595.276,841.89),leftMargin=47,rightMargin=47,topMargin=37,bottomMargin=37).build(flow)
doc=fitz.open(o/'report.pdf');layout={'pages':[],'visual_inspection':'pending'}
for i,page in enumerate(doc):
 bad=[]
 for block in page.get_text('dict')['blocks']:
  for line in block.get('lines',[]):
   for span in line['spans']:
    x0,y0,x1,y1=span['bbox']
    if x0<0 or y0<0 or x1>page.rect.width or y1>page.rect.height:bad.append(span['text'])
 layout['pages'].append({'page':i+1,'text_within_page':not bad,'out_of_bounds':bad});page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/semaev-symmetry-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n');print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
