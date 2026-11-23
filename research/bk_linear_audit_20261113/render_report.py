"""Render proved identities and the explicit geometric gap; no experiment."""
from pathlib import Path
import json,fitz,html
from reportlab.platypus import SimpleDocTemplate,Paragraph,Spacer
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib import colors
from reportlab.graphics.shapes import Drawing,Rect,Line,String
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
r=Path(__file__).resolve().parent;o=r/'reports';o.mkdir(exist_ok=True)
pdfmetrics.registerFont(TTFont('DV','/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf'));pdfmetrics.registerFont(TTFont('DVB','/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf'));pdfmetrics.registerFontFamily('DV',normal='DV',bold='DVB')
b=ParagraphStyle('b',fontName='DV',fontSize=8.7,leading=11.7,spaceAfter=8);t=ParagraphStyle('t',fontName='DVB',fontSize=16,leading=19,spaceAfter=11)
def p(s):return Paragraph(s,b)
d=Drawing(500,165);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="165" viewBox="0 0 500 165">']
def label(x,y,s,size=8):
 d.add(String(x,y,s,fontName='DV',fontSize=size,fillColor=colors.HexColor('#234b70')));svg.append(f'<text x="{x}" y="{165-y}" font-family="DejaVu Sans,sans-serif" font-size="{size}" fill="#234b70">{html.escape(s)}</text>')
def box(x,y,w,h):
 d.add(Rect(x,y,w,h,rx=4,fillColor=colors.HexColor('#edf3f8'),strokeColor=colors.HexColor('#8eb0cb')));svg.append(f'<rect x="{x}" y="{165-y-h}" width="{w}" height="{h}" rx="4" fill="#edf3f8" stroke="#8eb0cb"/>')
box(3,122,491,34);label(12,134,'Actual r | #E(Fq)  →  actual rational P ≠ 0, order(P)=r  (Cauchy)',9)
d.add(Line(249,120,249,113,strokeColor=colors.HexColor('#a05436'),strokeDashArray=[3,3]));d.add(Line(249,98,249,94,strokeColor=colors.HexColor('#a05436'),strokeDashArray=[3,3]));svg.append('<path d="M249,45 L249,52 M249,67 L249,71" stroke="#a05436" stroke-dasharray="3,3"/>');label(10,102,'Open bridge: geometric rank2 / representation / Frobenius determinant / descent',7.8)
for x,lines in[(3,['Actual A ∈ Mat₂(K)','Av=v≠0, det A=q≠1']),(173,['Derived charpoly','(X−1)(X−q)']),(343,['All k: power criterion','Aᵏ=I  ↔  qᵏ=1'])]:
 box(x,43,151,49);label(x+7,74,lines[0],8.1);label(x+7,56,lines[1],8)
for x in[157,327]:
 d.add(Line(x,67,x+12,67,strokeColor=colors.HexColor('#234b70')));svg.append(f'<path d="M{x},98 L{x+12},98" stroke="#234b70"/>')
label(9,17,'Excluded q=1: Jᵏ=[[1,k],[0,1]]; over Z/r, Jᵏ=I iff r divides k.',8.4)
svg.append('</svg>');(o/'proof-boundary.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text())
flow=[Paragraph('Qualified BK: linear criterion and exact exception',t),p('<b>Proved linear step.</b> For every 2×2 matrix A over a field, a nonzero fixed vector and det A=q≠1 imply Aᵏ=I iff qᵏ=1 for every k. Actual kernel/determinant identities force trace A=1+q and charpoly A=(X−1)(X−q). Coprime linear factors, polynomial evaluation and Cayley–Hamilton prove the criterion. No trace or diagonalization premise is supplied. Signed standing unverified. [pcc:unverified]'),d,Spacer(1,8),p('<b>Actual rational input.</b> Mathlib curve points inject into Option(K×K), hence are finite over every finite field. For every prime r dividing the actual point count, additive Cauchy supplies an actual nonzero rational point of exact order r, killed by r. The certificate retains the actual curve type and group operations. No asserted group cardinality, Hasse bound or torsion basis replaces this construction. [pcc:unverified]'),p('<b>Exact excluded boundary.</b> J(a)=[[1,a],[0,1]] has determinant1 and fixes the first vector. Lean proves J(1)ᵏ=J(k) over every commutative ring. Over Z/n, this is I exactly when n divides k. For every prime r, the fixed vector is nonzero and 1ᵏ=1 always, while Jᵏ=I only at multiples of r. A direct universal negation deletes q≠1 while retaining determinant and nonzero fixed-vector hypotheses. These are actual matrix identities, not a computation of the F7 curve’s Frobenius matrix. [pcc:unverified]'),p('<b>Geometric boundary.</b> Actual rational-point input and generic matrix theorem are separate conjuncts. Generic E[r] rank2 in the prime-to-characteristic case, the actual Frobenius determinant q, transporting rational torsion into that representation and torsion-field descent remain open. The characteristic-order rank case is separate. The dashed diagram edge is an unproved bridge, not a claimed implication. This proves the source’s rational and linear steps, not the full generic elliptic BK theorem. [pcc:unverified]'),p(f'<b>Checks.</b> Whole gate {g["declarations"]} declarations / 0 failures / 0 trusted native exceptions; Lean4.35.0, Mathlib56a00585…. Closed qualified_BK_linear_certificate, complete custom imports/configs and declaration hash are pinned in sources.json. proof-map.json identifies each bridge; failed and successful source/log/exit records are losslessly retained. Only standard propext/Classical.choice/Quot.sound; no holes, custom axioms, native evaluator, raised limits, global elaboration flags or waivers. Exact PCC receipt is separate; signed standing unverified. [pcc:unverified]'),p('<b>Sources.</b> Original Balasubramanian–Koblitz1998 Theorem1/proof and Remarks, pp142–143, DOI10.1007/s001459900040. The paper explicitly excludes r dividing q−1 and already describes the Jordan boundary. Pinned Mathlib matrix kernel/charpoly/Cayley–Hamilton, polynomial coprime/evaluation, actual curve points and additive Cauchy APIs were read. KB unavailable; bounded primary/repository/library fallback. No qualified-source refutation or novelty claim. [pcc:unverified]'),p('<b>Other obligations.</b> Actual F7 all-extension counterexamples remain in their separate packet, with no unproved identification with J. All fourteen requirements and remaining formalizations stay in the current map. No native experiment, paid compute or agent upload. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595.276,841.89),leftMargin=47,rightMargin=47,topMargin=37,bottomMargin=37).build(flow)
doc=fitz.open(o/'report.pdf');layout={'pages':[],'visual_inspection':'pending'}
for i,page in enumerate(doc):
 bad=[]
 for block in page.get_text('dict')['blocks']:
  for line in block.get('lines',[]):
   for span in line['spans']:
    x0,y0,x1,y1=span['bbox']
    if x0<0 or y0<0 or x1>page.rect.width or y1>page.rect.height:bad.append(span['text'])
 layout['pages'].append({'page':i+1,'text_within_page':not bad,'out_of_bounds':bad});page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/bk-linear-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n');print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
