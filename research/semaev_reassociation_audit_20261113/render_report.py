"""Render recorded proof scope and evidence, without research computation."""
from pathlib import Path
import json,fitz
from reportlab.platypus import SimpleDocTemplate,Paragraph,Spacer,Table,TableStyle
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib import colors
from reportlab.graphics.shapes import Drawing,Rect,Line,String
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
r=Path(__file__).resolve().parent;o=r/'reports';o.mkdir(exist_ok=True)
pdfmetrics.registerFont(TTFont('DV','/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf'));pdfmetrics.registerFont(TTFont('DVB','/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf'));pdfmetrics.registerFontFamily('DV',normal='DV',bold='DVB')
body=ParagraphStyle('b',fontName='DV',fontSize=8.8,leading=11.8,spaceAfter=7);title=ParagraphStyle('t',fontName='DVB',fontSize=16,leading=19,spaceAfter=11)
def p(s):return Paragraph(s,body)
blue=colors.HexColor('#234b70');fig=Drawing(500,130)
boxes=[(0,77,'Actual S3 and fixed sizes','Bidegrees2; exact source identity'),(263,77,'Ordered two-step reassociation','Generic variables; all degree drops'),(263,0,'Literal canonical polynomial','Generic evaluation map = identity'),(0,0,'Source-order S4 block','Actual-curve cross-pair symmetry')]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="130" viewBox="0 0 500 130">']
for x,y,a,b in boxes:
 fig.add(Rect(x,y,237,48,rx=5,fillColor=colors.HexColor('#edf5fa'),strokeColor=blue));svg.append(f'<rect x="{x}" y="{130-y-48}" width="237" height="48" rx="5" fill="#edf5fa" stroke="#234b70"/>')
 for k,t in enumerate([a,b]):
  fig.add(String(x+118.5,y+29-k*15,t,fontName='DV',fontSize=8.6,textAnchor='middle',fillColor=blue));svg.append(f'<text x="{x+118.5}" y="{130-y-29+k*15}" text-anchor="middle" font-family="DejaVu Sans,sans-serif" font-size="8.6" fill="#234b70">{t}</text>')
for x1,y1,x2,y2 in[(237,101,262,101),(381,77,381,49),(263,24,238,24)]:
 fig.add(Line(x1,y1,x2,y2,strokeColor=blue));svg.append(f'<path d="M{x1},{130-y1} L{x2},{130-y2}" stroke="#234b70"/>')
svg.append('</svg>');(o/'proof-flow.svg').write_text('\n'.join(svg)+'\n')
gate=json.loads((r/'evidence/gate.json').read_text())
flow=[Paragraph('Actual S3 recursion: source-order reassociation',title),
 p('<b>Proved component.</b> Two actual recursive S3 resultant steps equal the source-ordered balanced expression with the existing S4 polynomial. Equality holds in the formal polynomial ring at every canonical stage. Signed standing: unverified. [pcc:unverified]'),
 p('For deg P≤D, put A(T)=Res<sub>U</sub>(P(U),S3(z,T,U); D,2). All sizes after the semicolon are declared Sylvester sizes and remain fixed under specialization. [pcc:unverified]'),
 p('<b>Res<sub>T</sub>(A(T),S3(x,y,T); 2D,2)<br/>= Res<sub>U</sub>(P(U),S4(z,x,y,U); D,4).</b> [pcc:unverified]'),
 p('The source-order theorem uses the actual b-invariants of a Weierstrass curve over a commutative integral domain. It includes zero P, repeated coordinates and every characteristic; nonsingularity is not needed for this polynomial identity. Actual recursivePolynomial W(n+2) uses D=2^(n+1). [pcc:unverified]'),
 fig,Spacer(1,9),
 p('<b>Proof.</b> Literal S3 instantiates the fixed-size interchange theorem. Independent x/y variables followed by coefficient evaluation cover x=y. An independent leading coefficient specialized to0 covers every retained D≥deg P. The generic evaluation homomorphism is proved the identity on the ring of three independent variables, giving literal polynomial equality. Actual-curve S4 middle transposition and cyclic reordering then match the source argument order. [pcc:unverified]'),
 p('<b>Negative control.</b> Arbitrary independent b-coefficients do not have the required S4 symmetry. For (b2,b4,b6,b8)=(0,0,0,1) over ℚ, exchanging the middle two inputs of (0,1,2,3) changes the exact resultant. These coefficients violate the Weierstrass b-relation; this is not a qualified source counterexample. [pcc:unverified]'),
 p('<b>Scope.</b> The canonical split with an S4 block (k=2, arity≥5) is normalized. General splits k≥3, higher full permutation symmetry and higher absolute irreducibility remain open. The all-fourteen audit retains its other original obligations. No new native experiment or performance claim. [pcc:unverified]'),
 p(f'<b>Evidence.</b> Final whole Lean gate {gate["declarations"]}/0failures/0trusted; initial51gate retained separately. Lean4.35.0 / Mathlib56a00585…. Source/attempt/gate pins, exact declaration and proof map are in sources.json and evidence/. Standard foundations only; no custom axioms, native evaluation, raised limits or waivers. [pcc:unverified]'),
 p('<b>Sources.</b> Semaev2004 Theorem1/p2; Kosters–Yeo2015 arXiv1503.08001v3 Section2/p3 general S3/recursion, text and PDF formula inspected; actual canonical/S4/evaluation source modules, Mathlib resultant maps and b-invariants. KB unavailable; bounded fallback, no novelty claim. Paid compute and agent uploads:0. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595.276,841.89),leftMargin=47,rightMargin=47,topMargin=36,bottomMargin=36).build(flow)
doc=fitz.open(o/'report.pdf');layout={'pages':[],'visual_inspection':'pending'}
for i,page in enumerate(doc):
 bad=[]
 for block in page.get_text('dict')['blocks']:
  for line in block.get('lines',[]):
   for span in line['spans']:
    x0,y0,x1,y1=span['bbox']
    if x0<0 or y0<0 or x1>page.rect.width or y1>page.rect.height:bad.append(span['text'])
 layout['pages'].append({'page':i+1,'text_within_page':not bad,'out_of_bounds':bad});page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/semaev-reassociation-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n');print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
