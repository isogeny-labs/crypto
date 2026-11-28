"""Display recorded formal cardinalities and scopes, without a research computation."""
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
b=ParagraphStyle('b',fontName='DV',fontSize=8.7,leading=11.8,spaceAfter=10);t=ParagraphStyle('t',fontName='DVB',fontSize=16,leading=20,spaceAfter=12)
def p(s):return Paragraph(s,b)
d=Drawing(500,133);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="133" viewBox="0 0 500 133">']
def label(x,y,s,size=8.6):
 d.add(String(x,y,s,fontName='DV',fontSize=size,fillColor=colors.HexColor('#234b70')));svg.append(f'<text x="{x}" y="{133-y}" font-family="DejaVu Sans,sans-serif" font-size="{size}" fill="#234b70">{html.escape(s)}</text>')
def bar(y,w,color):
 d.add(Rect(190,y,w,18,fillColor=colors.HexColor(color),strokeColor=None));svg.append(f'<rect x="190" y="{133-y-18}" width="{w}" height="18" fill="{color}"/>')
label(4,117,'Actual geometric E[3] over algebraic closures; identity included',9.4)
label(4,84,'char2: y²+y=x³+1');bar(79,225,'#2c788e');label(427,84,'9 points')
label(4,47,'char3: y²=x³−x');bar(42,25,'#bd6248');label(226,47,'1 point')
label(4,12,'Generic char≠3: four roots × two ordinates + identity =9; exact-order3 points=8.',8)
svg.append('</svg>');(o/'geometric-torsion.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());cid=json.loads((r/'identity-certificate.json').read_text())['id']
flow=[Paragraph('Actual geometric three-torsion has rank two',t),p('<b>Result.</b> Lean proves that every nonsingular Weierstrass model over an algebraically closed field of characteristic other than three has exactly nine points killed by three, eight points of exact order three, and an actual two-element basis over F3. Characteristic two is included. The rank and basis are derived from the actual curve group and count. Signed standing unverified. [pcc:unverified]'),d,Spacer(1,10),p('<b>Point and polynomial bridge.</b> A universal coordinate identity relates the literal Mathlib three-division polynomial Ψ₃, the curve equation and the two partial derivatives. On a nonsingular point whose abscissa is a Ψ₃ root, the y-derivative cannot vanish. Actual duplication and group cancellation then prove 3P=0 exactly when Ψ₃(x)=0, in every characteristic. No rank assumption or characteristic-two division is used. [pcc:unverified]'),p('<b>Actual geometric count.</b> For every abscissa, evaluating the curve polynomial gives a monic ordinate polynomial of degree two, hence an ordinate exists over the algebraic closure. At a Ψ₃ root, nonsingularity gives a nonzero ordinate derivative. The identity Ψ₃′=3·Ψ₂Sq then proves Ψ₃ separable when three is nonzero. There are four distinct abscissas and exactly two distinct ordinates above each. An explicit equivalence identifies the actual subgroup killed by three with these eight coordinate pairs plus the identity. Its finiteness is proved before cardinality and rank are used. [pcc:unverified]'),p('<b>Actual module and controls.</b> The proved three-annihilation equips the actual subgroup with Mathlib’s ZMod3 module structure. Its cardinality9 forces finrank2 and produces an actual Fin2 basis; the exact-order subtype has cardinality8. Every scalar extension through an actual field homomorphism into an algebraically closed field has the same count. A concrete algebraic closure of F2 with y²+y=x³+1 checks cardinality9/rank2/basis and nonzero discriminant. A genuine nonsingular y²=x³−x over the algebraic closure of F3 has only the identity in its three-torsion. This tests the excluded characteristic. The independent short-curve derivation is retained as a proof cross-check with shared author, library and kernel. [pcc:unverified]'),p('<b>Primary source and remaining bridge.</b> Balasubramanian–Koblitz, <i>The Improbability That an Elliptic Curve Has Subexponential Discrete Log Problem under the Menezes–Okamoto–Vanstone Algorithm</i>, J.Cryptology11(1998),141–145, DOI10.1007/s001459900040. Original Theorem1/proof/Remarks pp142–143 re-read and visually inspected; primary73656B/SHAac837b8e…c987499. Its prime/r∣#E/r∤q−1 qualifications and the corpus’s coprimality condition are preserved. This supplies its geometric rank input at prime3. Other primes, actual Frobenius action/matrix/determinant, fixed-field descent, canonical pairings and the full source theorem remain open. Nine torsion points include identity; the source’s order shorthand is not treated as a terminology counterexample. No full-paper or Silverman-book read is claimed. [pcc:unverified]'),p(f'<b>Validation.</b> Whole gate {g["declarations"]}/0failures/0trusted; pinned Lean4.35.0/Mathlib56a00585…. Twelve custom modules and three configs, complete binding, failed/successful attempts, source coverage and inspected PDF/SVG are retained. Scalar supplement {cid}, seven variables/degree5, has the actual universal Lean coordinate proof and official32+32-point check; it does not replace the geometry proof. Only propext/Classical.choice/Quot.sound, no holes/custom axioms/native evaluator/raised limits/global flags/waivers. Exact PCC replay/signed standing are separate. KB/Conductor unavailable, explicit primary/formal fallback. Qualified BK/canonical corpus labels unchanged; no novelty, native experiment, paid compute or upload. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595.276,841.89),leftMargin=47,rightMargin=47,topMargin=35,bottomMargin=35).build(flow)
doc=fitz.open(o/'report.pdf');layout={'pages':[],'visual_inspection':'pending'}
for i,page in enumerate(doc):
 bad=[]
 for block in page.get_text('dict')['blocks']:
  for line in block.get('lines',[]):
   for span in line['spans']:
    x0,y0,x1,y1=span['bbox']
    if x0<0 or y0<0 or x1>page.rect.width or y1>page.rect.height:bad.append(span['text'])
 layout['pages'].append({'page':i+1,'text_within_page':not bad,'out_of_bounds':bad});page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/three-torsion-rank-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n');print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
