"""Render proved polynomial identities and source boundaries; no experiment."""
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
b=ParagraphStyle('b',fontName='DV',fontSize=8.8,leading=11.8,spaceAfter=8);t=ParagraphStyle('t',fontName='DVB',fontSize=16,leading=19,spaceAfter=11)
def p(s):return Paragraph(s,b)
d=Drawing(500,132);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="132" viewBox="0 0 500 132">']
def label(x,y,s,size=8):
 d.add(String(x,y,s,fontName='DV',fontSize=size,fillColor=colors.HexColor('#234b70')));svg.append(f'<text x="{x}" y="{132-y}" font-family="DejaVu Sans,sans-serif" font-size="{size}" fill="#234b70">{html.escape(s)}</text>')
for x,title,line in [(3,'Literal binary S3','(xy+xz+yz)²+xyz+b'),(173,'Actual quadratic root','u=(x+y)²z/(xy)'),(343,'Artin–Schreier root','v²+v=Y+c+s/Y')]:
 d.add(Rect(x,59,151,59,rx=4,fillColor=colors.HexColor('#edf3f8'),strokeColor=colors.HexColor('#8eb0cb')));svg.append(f'<rect x="{x}" y="14" width="151" height="59" rx="4" fill="#edf3f8" stroke="#8eb0cb"/>');label(x+7,98,title,8.2);label(x+7,77,line,8)
for x in[157,327]:
 d.add(Line(x,86,x+12,86,strokeColor=colors.HexColor('#234b70')));svg.append(f'<path d="M{x},46 L{x+12},46" stroke="#234b70"/>')
label(10,37,'Proved primitive/MV bridges');label(245,37,'deg∞ RHS=1; every v²+v excluded',8.2);label(10,14,'Algebraic-closure coefficient map removes the square-root premise.',8)
svg.append('</svg>');(o/'proof-flow.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text())
flow=[Paragraph('Ordinary binary S3: actual absolute irreducibility',t),p('<b>Result.</b> The literal ordinary polynomial (xy+xz+yz)²+xyz+b is symmetric and irreducible over every characteristic-two field, and remains irreducible after every field scalar extension, for every b. Its evaluation is exactly the existing ordinary S3formula; on every nonsingular ordinary model y²+xy=x³+a₂x²+b over an algebraically closed field, vanishing is equivalent to an actual Mathlib three-point zero-sum. Signed standing unverified. [pcc:unverified]'),d,Spacer(1,9),p('<b>Figure.</b> Actual polynomial and rational-function maps are proved identities. After adjoining s²=b, a root would yield v²+v=Y+c+s/Y, whose actual rational-function degree is1. For positive degree v, the left side has even degree2deg(v); for nonpositive degree v it has degree at most0. Both contradict1. No Artin–Schreier obstruction or irreducibility is assumed. [pcc:unverified]'),p('<b>Literal and scalar bridges.</b> The actual polynomial is primitive: its leading coefficient is the diagonal square, while the middle coefficient xy is not divisible by that irreducible diagonal. The proved injective coefficient map into the algebraic-closure rational-function field and Gauss bridge descend irreducibility. Explicit MV/nested algebra equivalences, exact evaluation and scalar-map identities bind the result to the source polynomial, including all field extensions. The final theorem has no square-root or finite-enumeration premise. [pcc:unverified]'),p('<b>Negative control and skepticism.</b> The distinct short-Weierstrass expression at F2,a=b=0 is a nonunit square and directly not irreducible. Lean proves it is unequal to the ordinary binary polynomial by evaluation. The control distinguishes models; it does not refute the qualified ordinary source. Algebraic irreducibility allows b=0, while the actual elliptic-group equivalence explicitly retains nonsingularity and ordinary model conditions, including repeated-coordinate/two-torsion cases. [pcc:unverified]'),p(f'<b>Checks.</b> Whole gate{g["declarations"]}/0failures/0trusted. Lean4.35.0/Mathlib56a00585…. Complete closed actual_binary_S3_certificate, custom import graph, configs and declaration hash are pinned in sources.json; proof-map.json identifies the intermediate bridges. Failed/successful source/log/exit records are retained losslessly. Only propext/Classical.choice/Quot.sound; no holes, custom axioms, native evaluator, raised limits, global elaboration flags or waivers. Exact PCC receipt is separate; signed standing unverified. [pcc:unverified]'),p('<b>Sources.</b> Semaev2015section2/arXiv1504.01175v1, actual PDFpage3 formula visually inspected. Kosters–Yeo2015section2general b-invariant formula/Proposition2.1 and4.2ordinary hypotheses. Original Semaev2004Theorem1is characteristic not2/3. Existing actual BinaryS3group equivalence and short-model square control, pinned Mathlib degree/algebraic closure/Gauss/MV APIs reviewed. KB unavailable, bounded primary/repository/library fallback; no novelty claim. [pcc:unverified]'),p('<b>Remaining scope.</b> General higher-arity absolute irreducibility and recursive splits k≥3remain open. This packet confirms the ordinary three-variable formula and does not claim those broader theorems. No native experiment, paid compute or agent upload. All fourteen obligations remain in the current requirement map. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595.276,841.89),leftMargin=47,rightMargin=47,topMargin=37,bottomMargin=37).build(flow)
doc=fitz.open(o/'report.pdf');layout={'pages':[],'visual_inspection':'pending'}
for i,page in enumerate(doc):
 bad=[]
 for block in page.get_text('dict')['blocks']:
  for line in block.get('lines',[]):
   for span in line['spans']:
    x0,y0,x1,y1=span['bbox']
    if x0<0 or y0<0 or x1>page.rect.width or y1>page.rect.height:bad.append(span['text'])
 layout['pages'].append({'page':i+1,'text_within_page':not bad,'out_of_bounds':bad});page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/binary-s3-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n');print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
