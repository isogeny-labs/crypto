"""Render recorded formal statements and domain controls only."""
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
b=ParagraphStyle('b',fontName='DV',fontSize=8.8,leading=12.1,spaceAfter=10);t=ParagraphStyle('t',fontName='DVB',fontSize=16,leading=20,spaceAfter=13)
def p(s):return Paragraph(s,b)
d=Drawing(500,123);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="123" viewBox="0 0 500 123">']
def label(x,y,s,size=8.2):
 d.add(String(x,y,s,fontName='DV',fontSize=size,fillColor=colors.HexColor('#234b70')));svg.append(f'<text x="{x}" y="{123-y}" font-family="DejaVu Sans,sans-serif" font-size="{size}" fill="#234b70">{html.escape(s)}</text>')
def box(x,y,w,h):
 d.add(Rect(x,y,w,h,rx=4,fillColor=colors.HexColor('#edf3f8'),strokeColor=colors.HexColor('#8eb0cb')));svg.append(f'<rect x="{x}" y="{123-y-h}" width="{w}" height="{h}" rx="4" fill="#edf3f8" stroke="#8eb0cb"/>')
label(5,111,'Actual E/F29: y²=x³+1, #E=30, P=(4,6), order(P)=5',9.4)
for x,lines in[(4,['β=5, α=0','Nonzero mod30','Zero mod5: no log info']),(173,['β=2, α=3','Not invertible mod30','Invertible mod5: log1']),(342,['β=1, α=29','Unit mod30','Ambient recovery gives1'])]:
 box(x,36,153,61)
 for y,line in zip([81,63,45],lines):label(x+7,y,line)
label(5,12,'Generic repair: reduce the actual relation modulo the exact prime subgroup order.',8.4)
svg.append('</svg>');(o/'modulus-controls.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());cid=json.loads((r/'identity-certificate.json').read_text())['id']
flow=[Paragraph('Subgroup extraction needs the correct modulus',t),p('<b>Result.</b> Lean refutes the unqualified algebraic rule that a nonzero ambient coefficient suffices to extract a discrete logarithm. It proves the corrected rule for every additive commutative group and exact prime-order point, gives unbounded finite cofactor examples, and checks actual elliptic point-group controls. Signed standing unverified. [pcc:unverified]'),d,Spacer(1,10),p('<b>Source condition.</b> Thériault2003 §§2.1/2.4/2.5 assumes a prime-order D₁ in practice but keeps coefficients and linear algebra modulo the full Jacobian order. Printed p78 then tests only β=0 before extracting −α/β; §5.10 repeats the ambient modulus. A prime subgroup need not have prime ambient order. The exact printed page was rendered and visually inspected. This packet audits that algebraic inference; it does not claim an executed full higher-genus algorithm counterexample. [pcc:unverified]'),p('<b>Generic repair.</b> For any actual additive group, P of exact prime order r and relation aP+b(xP)=0, the order criterion gives r∣a+bx. In the actual field ZMod r, b≠0 therefore implies x=−a/b. Division is in the subgroup modulus. For actual ambient ZMod(2r), P=2 has order r and the ambient cardinality is2r. The coefficient β=r is nonzero modulo2r, nonunit, zero modulo r, and gives the same relation for every candidate x. Euclid supplies examples of unbounded prime order; P≠0 checks genuine distinct candidate logs. [pcc:unverified]'),p('<b>Actual curve controls.</b> Lean defines the nonsingular curve y²=x³+1 over F29, constructs P=(4,6), proves its exact order5 and counts every curve point via an explicit point/code equivalence, obtaining30. β5 is nonzero modulo30 and yields0P+5P=0, but raw inverse multiplication returns0 rather than the log1 of P. A universal ambient-extraction implication is directly negated. The useful relation3P+2P=0 recovers1 modulo5 although2 is not invertible modulo30. Ambient β1 is a unit and recovers1 as a positive control. No composite division-field instance is fabricated. [pcc:unverified]'),p('<b>Attribution and scope.</b> N.Thériault, <i>Index Calculus Attack for Hyperelliptic Curves of Small Genus</i>, LNCS2894(2003),75–92, DOI10.1007/978-3-540-40061-5_5. Relevant original passages, prior immutable source review/curve proof and pinned Mathlib group-order/modular/point-count APIs read. Full genus≥3 Jacobian/Picard identification and a realized algorithm relation trace remain open. No full-paper read, full-runtime refutation or failure-probability refutation is claimed; the latter also needs a coefficient distribution. Historical corpus/backlog labels remain unchanged. [pcc:unverified]'),p(f'<b>Assurance.</b> Whole gate {g["declarations"]}/0failures/0trusted; complete source binding/custom source/config/artifact pins, failed/successful attempts, SVG/PDF retained. Scalar supplement {cid} has a generic symbolic Lean commutative-ring proof and official32+32-point check; it does not prove curve order or extraction. Only propext/Classical.choice/Quot.sound; no holes/custom axioms/native evaluator/raised limits/global flags/waivers. Lean4.35.0/Mathlib56a00585… pinned. Exact PCC replay and signed standing are separate. KB unavailable, bounded source/formal fallback; no native experiment, paid compute or upload. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595.276,841.89),leftMargin=47,rightMargin=47,topMargin=35,bottomMargin=35).build(flow)
doc=fitz.open(o/'report.pdf');layout={'pages':[],'visual_inspection':'pending'}
for i,page in enumerate(doc):
 bad=[]
 for block in page.get_text('dict')['blocks']:
  for line in block.get('lines',[]):
   for span in line['spans']:
    x0,y0,x1,y1=span['bbox']
    if x0<0 or y0<0 or x1>page.rect.width or y1>page.rect.height:bad.append(span['text'])
 layout['pages'].append({'page':i+1,'text_within_page':not bad,'out_of_bounds':bad});page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/subgroup-extraction-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n');print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
