"""Render the proved recursion and explicit remaining obligation; no experiment."""
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
d=Drawing(500,139);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="139" viewBox="0 0 500 139">']
def label(x,y,s,size=8):
 d.add(String(x,y,s,fontName='DV',fontSize=size,fillColor=colors.HexColor('#234b70')));svg.append(f'<text x="{x}" y="{139-y}" font-family="DejaVu Sans,sans-serif" font-size="{size}" fill="#234b70">{html.escape(s)}</text>')
def box(x,y,w,h):
 d.add(Rect(x,y,w,h,rx=4,fillColor=colors.HexColor('#edf3f8'),strokeColor=colors.HexColor('#8eb0cb')));svg.append(f'<rect x="{x}" y="{139-y-h}" width="{w}" height="{h}" rx="4" fill="#edf3f8" stroke="#8eb0cb"/>')
for x,lines in [(3,['Prescribed Fubini','Exact S3 transfer']),(173,['Actual recurrence','All-block induction']),(343,['Universal MV ring','Literal polynomial equality'])]:
 box(x,82,151,48);label(x+7,112,lines[0],8.1);label(x+7,94,lines[1],7.9)
for x in [157,327]:
 d.add(Line(x,106,x+12,106,strokeColor=colors.HexColor('#234b70')));svg.append(f'<path d="M{x},33 L{x+12},33" stroke="#234b70"/>')
box(3,20,238,44);label(10,47,'Actual full symmetry',8.2);label(10,30,'Elimination variable last: proved bridge',8)
box(255,20,239,44);label(262,47,'Injective independent-variable maps',8.2);label(262,30,'True factor degrees: 2^(m+1), 2^(n+1)',8)
label(10,4,'Every source N ≥ 4, 1 ≤ k ≤ N−3: N=(m+2)+(n+2), k=n+1.',8.2)
svg.append('</svg>');(o/'proof-structure.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text())
flow=[Paragraph('Semaev: every literal source-ordered split',t),p('<b>Proved.</b> For all fields and all m,n≥0, the actual source polynomials of arities m+3 and n+3, with elimination variable last, have natural degrees 2^(m+1) and 2^(n+1). Their resultant with these true dimensions equals the literal canonical polynomial of arity N=(m+2)+(n+2). Explicit Fin casts bind the variables. Every source index N≥4 and 1≤k≤N−3 is covered by k=n+1. Signed standing unverified. [pcc:unverified]'),d,Spacer(1,9),p('<b>Proof.</b> Independent generic leading coefficients and specialization prove fully prescribed Fubini. The actual S3 quadratic and proved even resultant sign give convolution transfer. Actual canonical recurrence, exact tuple/list bridges and full input symmetry prove the two-block induction. The target is the actual multivariate polynomial ring: universal evaluation and variable-block concatenation yield literal polynomial equality. This is not an inference from agreement at finite-field points. [pcc:unverified]'),p('<b>Degrees and source order.</b> Coefficient evaluation at injectively embedded independent variables is identified with MVrename and proved injective. Mapping therefore preserves the actual canonical degrees. A separate ring-hom extensionality proof identifies the source kernel with partial evaluation of canonicalMv; full symmetry moves its free variable from first to last. No degree, ordering or split premise replaces these bridges. [pcc:unverified]'),p('<b>Specialization.</b> The certificate also proves every two-block identity over any commutative domain under any coefficient map, allowing repeated coordinates and degree drops. Here prescribed dimensions are retained. The literal universal factors have true natural dimensions; a specialized factor can have a smaller natural degree. These are explicit distinct statements. [pcc:unverified]'),p('<b>Negative controls.</b> General resultant interchange with reset natural degrees can give −1 on the left and 2 on the right, while prescribed dimensions agree. Independent b=(0,0,0,1) breaks S4 symmetry, but Lean proves these invariants cannot come from a rational Weierstrass curve. Both controls are actual inequalities in the closed certificate. Neither refutes the qualified recursion. [pcc:unverified]'),p(f'<b>Checks.</b> Whole gate {g["declarations"]} declarations / 0 failures / 0 trusted native exceptions; Lean4.35.0 and Mathlib56a00585…. Complete declaration/import/config and artifact pins are in sources.json. Failed and successful source/log/exit records and the gate are retained losslessly. Only propext, Classical.choice and Quot.sound; no holes, custom axioms, native evaluator, raised limits, global elaboration flags or waivers. Exact PCC replay is separate; signed standing unverified. [pcc:unverified]'),p('<b>Sources and boundary.</b> Semaev2004 Theorem1 equation2/proof, originally characteristic not2/3, and Kosters–Yeo2015 Section2/p3 general invariant model. This confirms the qualified recursion; no novelty or source refutation is claimed. The formal invariant-polynomial identity holds in all characteristics for actual coefficient models, without a new group interpretation for singular models. Higher-arity absolute irreducibility remains open, as do the other obligations in the all-fourteen map. KB unavailable; bounded primary/repository/library review recorded before edits. No new native experiment, paid compute or agent upload. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595.276,841.89),leftMargin=47,rightMargin=47,topMargin=35,bottomMargin=35).build(flow)
doc=fitz.open(o/'report.pdf');layout={'pages':[],'visual_inspection':'pending'}
for i,page in enumerate(doc):
 bad=[]
 for block in page.get_text('dict')['blocks']:
  for line in block.get('lines',[]):
   for span in line['spans']:
    x0,y0,x1,y1=span['bbox']
    if x0<0 or y0<0 or x1>page.rect.width or y1>page.rect.height:bad.append(span['text'])
 layout['pages'].append({'page':i+1,'text_within_page':not bad,'out_of_bounds':bad});page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/semaev-general-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n');print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
