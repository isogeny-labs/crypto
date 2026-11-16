"""Render the checked mathematical argument; no research computation."""
from pathlib import Path
import json,fitz
from reportlab.platypus import SimpleDocTemplate,Paragraph,Spacer,Table,TableStyle
from reportlab.lib.styles import getSampleStyleSheet,ParagraphStyle
from reportlab.lib import colors
from reportlab.graphics.shapes import Drawing,Rect,Line,String,Polygon
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
r=Path(__file__).resolve().parent;o=r/'reports';o.mkdir(exist_ok=True)
pdfmetrics.registerFont(TTFont('DejaVuSans','/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf'));pdfmetrics.registerFont(TTFont('DejaVuSans-Bold','/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf'));pdfmetrics.registerFontFamily('DejaVuSans',normal='DejaVuSans',bold='DejaVuSans-Bold')
styles=getSampleStyleSheet();styles.add(ParagraphStyle(name='BodyProof',fontName='DejaVuSans',fontSize=8.5,leading=11.2,spaceAfter=6));styles.add(ParagraphStyle(name='TitleProof',fontName='DejaVuSans-Bold',fontSize=15,leading=18,spaceAfter=9))
def p(t):return Paragraph(t,styles['BodyProof'])
blue=colors.HexColor('#234b70');flow=Drawing(500,136)
boxes=[(1,83,['Literal S3 polynomial','exact existing-S3 evaluation']),
 (173,83,['Discriminant16f(x)f(y)','odd rational-function degree']),
 (345,83,['No root, degree two','irreducible in K(x)(y)[z]']),
 (345,4,['Actual coefficient embedding','proved primitive; Gauss lemma']),
 (173,4,['Actual three-variable ring','explicit algebra equivalences']),
 (1,4,['Every field scalar extension','absolute irreducibility'])]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="136" viewBox="0 0 500 136">','<defs><marker id="arrow" markerWidth="7" markerHeight="7" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6" fill="#234b70"/></marker></defs>']
for x,y,labels in boxes:
 flow.add(Rect(x,y,154,46,rx=5,ry=5,fillColor=colors.HexColor('#edf5fa'),strokeColor=blue));svg.append(f'<rect x="{x}" y="{136-y-46}" width="154" height="46" rx="5" fill="#edf5fa" stroke="#234b70"/>')
 for n,t in enumerate(labels):
  flow.add(String(x+77,y+28-n*15,t,textAnchor='middle',fontName='DejaVuSans',fontSize=8.5,fillColor=blue));svg.append(f'<text x="{x+77}" y="{136-y-28+n*15}" text-anchor="middle" font-family="DejaVu Sans,sans-serif" font-size="8.5" fill="#234b70">{t}</text>')
for x1,y1,x2,y2 in[(155,106,171,106),(327,106,343,106),(422,82,422,52),(344,27,329,27),(172,27,157,27)]:
 flow.add(Line(x1,y1,x2,y2,strokeColor=blue))
 if y1==y2:
  dx=1 if x2>x1 else -1;flow.add(Polygon([x2,y2,x2-5*dx,y2+3,x2-5*dx,y2-3],fillColor=blue,strokeColor=blue))
 else:flow.add(Polygon([x2,y2,x2-3,y2+5,x2+3,y2+5],fillColor=blue,strokeColor=blue))
 svg.append(f'<path d="M{x1},{136-y1} L{x2},{136-y2}" stroke="#234b70" marker-end="url(#arrow)"/>')
svg.append('</svg>');(o/'proof-flow.svg').write_text('\n'.join(svg)+'\n')
gate=json.loads((r/'evidence/gate.json').read_text());n=gate['declarations']
story=[Paragraph('S3: symmetry and absolute irreducibility',styles['TitleProof']),p(f'Item1 | 13 November 2026 | Whole gate {n}/0 failures/0 trusted | [pcc:unverified]'),
p('For every field K with2≠0 and arbitrary coefficients a,b, Lean proves the literal short-Weierstrass S3 polynomial is irreducible after every field homomorphism K→L. This includes algebraic closure. It also proves actual permutation symmetry and exact evaluation as the existing SemaevS3.S3 definition.'),flow,Spacer(1,7),
p('<b>Exact source and symmetry.</b> S3=(x−y)²z²−2((x+y)(xy+a)+2b)z+(xy−a)²−4b(x+y). In the elementary symmetric polynomials it equals e2²−4e1e3−2ae2−4be1+a². The exact identity and Mathlib rename_esymm prove invariance under every variable permutation.'),
p('<b>Function-field argument.</b> Put f(T)=T³+aT+b. The exact quadratic discriminant is16f(x)f(y). In the actual nested rational-function field K(x)(y), f(x) is a nonzero coefficient and the y-degree of this discriminant is three. Every nonzero rational-function square has even integer degree, including rational functions with poles. Thus the quadratic has no root; its degree is two, so it is irreducible.'),
p('<b>Actual polynomial bridge.</b> The coefficient map K[x,y]→K(x)(y) is proved injective. The prime diagonal y−x does not divide the middle coefficient: evaluating y=x gives the nonzero polynomial −4f(x). Every common divisor of the leading diagonal-square and middle coefficient is a unit, proving primitiveness. Gauss’s injective-map theorem transfers irreducibility to K[x][y][z]. Explicit algebra equivalences then reach the actual MvPolynomial(Fin3) expression. The proof applies again after any field scalar extension.'),
p('<b>Characteristic2 control.</b> For the literal short formula over F2 at a=b=0, Lean proves S3=((x+y)z+xy)² and proves its factor is not a unit using evaluation at the origin. Hence this input is directly not irreducible. This is a restriction control, not a refutation of the qualified source theorem or the distinct ordinary characteristic2 elliptic model.'),
p('<b>Proof map.</b> Namespace SemaevIrreducibilityAudit: OddDegree.lean proves odd_degree_not_square; S3FunctionField.lean proves the actual discriminant/no-root/degree2 result; PrimitiveBridge.lean proves the coefficient embedding, primitive condition and nested irreducibility; ActualPolynomial.lean proves the actual MV/scalar-extension/symmetry/control certificate actual_S3_absolute_irreducibility_with_control.'),
p('<b>Scope and assurance.</b> This closes the n=3 algebraic symmetry/absolute-irreducibility gap. Higher arity, arbitrary-split normalization and other original14 obligations remain open. The source’s short elliptic model assumes characteristic other than2/3 and nonsingularity; this algebraic statement needs only2≠0. Lean4.35.0 / Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679; standard propext, Classical.choice, Quot.sound only. No holes, custom axioms, native_decide, raised limits or waived checks. Local checks and independently signed standing are distinct. No new native experiment, paid compute or agent storage upload.'),
p('<b>Primary source.</b> Semaev, <i>Summation polynomials and the discrete logarithm problem on elliptic curves</i>, author manuscript5February2004, Theorem1 and page3; PDF SHA256 prefix1cab0ae67599, full URL/hash/read scope in evidence/source-review.json. The page3 overbar covers the whole function field K(X3), as visually checked; text extraction loses it. KB unavailable (MCP absent/wrapper127); bounded primary/repository/Mathlib fallback. No novelty claim. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595,842),leftMargin=47,rightMargin=48,topMargin=30,bottomMargin=30).build(story)
d=fitz.open(o/'report.pdf');pages=[]
for i,page in enumerate(d):
 bad=[list(b[:4])for b in page.get_text('blocks')if b[0]<0 or b[1]<0 or b[2]>page.rect.width or b[3]>page.rect.height];assert not bad,(i,bad)
 page.get_pixmap(matrix=fitz.Matrix(1.2,1.2)).save(f'/workspace/scratch/semaev-irred-report-{i+1}.png');pages.append({'page':i+1,'text_blocks_in_bounds':True})
(o/'layout.json').write_text(json.dumps({'pages':pages,'visual_inspection':'pending'},indent=2)+'\n');print(json.dumps({'pages':len(d),'bytes':(o/'report.pdf').stat().st_size}))
