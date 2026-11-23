"""Render proved actual torsion geometry and its source boundary; no experiment."""
from pathlib import Path
import json,fitz,html
from reportlab.platypus import SimpleDocTemplate,Paragraph,Spacer
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib import colors
from reportlab.graphics.shapes import Drawing,Rect,String
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
r=Path(__file__).resolve().parent;o=r/'reports';o.mkdir(exist_ok=True)
pdfmetrics.registerFont(TTFont('DV','/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf'));pdfmetrics.registerFont(TTFont('DVB','/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf'));pdfmetrics.registerFontFamily('DV',normal='DV',bold='DVB')
b=ParagraphStyle('b',fontName='DV',fontSize=8.6,leading=11.5,spaceAfter=8);t=ParagraphStyle('t',fontName='DVB',fontSize=15.5,leading=19,spaceAfter=11)
def p(s):return Paragraph(s,b)
d=Drawing(500,126);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="126" viewBox="0 0 500 126">']
def label(x,y,s,size=8):
 d.add(String(x,y,s,fontName='DV',fontSize=size,fillColor=colors.HexColor('#234b70')));svg.append(f'<text x="{x}" y="{126-y}" font-family="DejaVu Sans,sans-serif" font-size="{size}" fill="#234b70">{html.escape(s)}</text>')
def box(x,y,w,h):
 d.add(Rect(x,y,w,h,rx=4,fillColor=colors.HexColor('#edf3f8'),strokeColor=colors.HexColor('#8eb0cb')));svg.append(f'<rect x="{x}" y="{126-y-h}" width="{w}" height="{h}" rx="4" fill="#edf3f8" stroke="#8eb0cb"/>')
label(10,113,'Actual geometric 2-torsion over algebraically closed fields',9)
for x,lines in [(3,['2 ≠ 0','4 points; explicit binary basis']),(173,['Characteristic2, a₁=0','1 point: identity only']),(343,['Characteristic2, a₁≠0','2 points: unique nonzero'])]:
 box(x,58,151,42);label(x+6,83,lines[0],8.2);label(x+6,66,lines[1],7.6)
box(3,8,491,37);label(11,31,'Actual E/F2: y²+xy=x³+1, #E=4; geometric E[2] has2 points, all rational.',8.3);label(11,15,'2 ∤ 2^k−1 (k>0). Qualified corpus node retains gcd(r,q)=1: not refuted.',8.1)
svg.append('</svg>');(o/'rank-boundary.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text())
flow=[Paragraph('Actual 2-torsion: rank and characteristic boundary',t),p('<b>Proved geometry.</b> For every nonsingular general Weierstrass model over a field with 2≠0, the actual Mathlib subgroup killed by2 is finite and equivalent to identity plus distinct roots of the actual cubic. Its cardinality is 1+#roots≤4. Over every algebraically closed field and after every algebraically closed scalar extension it has4points, with an explicit binary basis: two nonzero distinct actual points A/B, full generation0/A/B/A+B, and independence of actual F2coefficient values under the group’s scalar action. No rank/basis premise is supplied. Signed standing unverified. [pcc:unverified]'),d,Spacer(1,8),p('<b>Proof.</b> Actual point negation and the checked coordinate/root construction give the full point-set equivalence. The cubic has discriminant16Δ, so algebraic closedness and Δ≠0 give three distinct roots. Finite group/cardinality reasoning with actual multiplicationby2 constructs the basis. Scalar-map injectivity transports 2≠0 and Δ≠0; no extra target-characteristic assumption replaces this step. [pcc:unverified]'),p('<b>Characteristic2.</b> Actual point negation gives 2P=0 iff a₁x+a₃=0 for affine points. When a₁=0 and Δ≠0, a₃≠0, so only identity remains. When a₁≠0, a nonzero killed point has x=−a₃/a₁ and is unique; algebraic closedness supplies its ordinate through the actual curve equation. The geometric cardinality is1or2. Concrete y²+xy=x³+1 and y²+y=x³ both haveΔ1 and cardinalities2/1 over every characteristic2field. [pcc:unverified]'),p('<b>Actual omitted-condition control.</b> For the first curve over F2, an injective coordinate bound5, three distinct actual points and actual order2divisibility force #E=4. Its actual AlgebraicClosure(F2) has two killed-by2points, both images of basepoints under the actual coordinatewise extension. Thus prime2divides #E and its square divides #E, while2∤2−1. Actual binary fields have card2^k and2∤2^k−1 for every positivek. A direct concrete point-inclusion/cyclotomic equivalence negation is proved. [pcc:unverified]'),p('<b>Source check.</b> The full THM-XT-BK-WEIL-SUFFICES node explicitly retains gcd(r,q)=1 and r∤q−1. Original BK1998Theorem1/proof pp142–143 states a full l²-point condition; its prime-to-characteristic rank step is invoked after cyclotomic divisibility forces l∤q. The characteristic2control deletes coprimality only. Neither the qualified node nor the qualified paper is refuted. Exact node/hash and read scope are retained; no corpus status is rewritten. [pcc:unverified]'),p(f'<b>Checks.</b> Whole gate {g["declarations"]} declarations / 0 failures / 0 trusted native exceptions; Lean4.35.0, Mathlib56a00585…. Complete closed declaration, custom sources/configs and artifact hashes are pinned in sources.json. Failed/successful source/log/exit records and full gate are losslessly archived. Only propext/Classical.choice/Quot.sound; no holes, custom axioms, native evaluator, raised limits, new global elaboration flags or waivers. Exact PCC replay is separate; signed standing unverified. [pcc:unverified]'),p('<b>Remaining scope.</b> Generic geometry at r2 is proved. Generic odd-prime rank/Frobenius determinant/representation/descent, full supersingular geometric/Hasse classification and canonical pairings remain open. No new group-scheme formalization or general Frobenius representation is claimed. Exact existing Mathlib proofs and primary BK were read; unavailable external Silverman is not marked read. KB unavailable, bounded fallback recorded. No novelty, native experiment, paid compute or agent upload. Other all-fourteen obligations remain explicit. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595.276,841.89),leftMargin=47,rightMargin=47,topMargin=35,bottomMargin=35).build(flow)
doc=fitz.open(o/'report.pdf');layout={'pages':[],'visual_inspection':'pending'}
for i,page in enumerate(doc):
 bad=[]
 for block in page.get_text('dict')['blocks']:
  for line in block.get('lines',[]):
   for span in line['spans']:
    x0,y0,x1,y1=span['bbox']
    if x0<0 or y0<0 or x1>page.rect.width or y1>page.rect.height:bad.append(span['text'])
 layout['pages'].append({'page':i+1,'text_within_page':not bad,'out_of_bounds':bad});page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/two-torsion-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n');print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
