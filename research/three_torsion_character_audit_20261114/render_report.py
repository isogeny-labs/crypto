"""Render recorded formal scopes and counts; no research computation."""
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
d=Drawing(500,127);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="127" viewBox="0 0 500 127">']
def label(x,y,s,size=8.6):
 d.add(String(x,y,s,fontName='DV',fontSize=size,fillColor=colors.HexColor('#234b70')));svg.append(f'<text x="{x}" y="{127-y}" font-family="DejaVu Sans,sans-serif" font-size="{size}" fill="#234b70">{html.escape(s)}</text>')
def bar(y,w,color):
 d.add(Rect(198,y,w,16,fillColor=colors.HexColor(color),strokeColor=None));svg.append(f'<rect x="198" y="{127-y-16}" width="{w}" height="16" fill="{color}"/>')
label(4,110,'Actual geometric three-torsion, identity included; kernel-checked counts',9.1)
label(4,80,'char≠3: nonsingular family');bar(75,225,'#2c788e');label(430,80,'9 points')
label(4,50,'char3: y²+xy+y=x³');bar(45,75,'#bd6248');label(280,50,'3 points')
label(4,15,'Origin-first basis → explicit cube-root character → actual det=q → descent.',8.1)
svg.append('</svg>');(o/'character-determinant.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());cid=json.loads((r/'identity-certificate.json').read_text())['id']
flow=[Paragraph('A cubic character proves actual Frobenius det=q',t),p('<b>Result.</b> Lean proves the actual Frobenius determinant formula at prime3 for every nonsingular model y²+a1xy+a3y=x³ over every finite field of characteristic other than three, including two. It derives the qualified BK containment criterion for every chosen finite extension and compatible embedding, with q≠1mod3. No determinant, rank, pairing or abstract rationality premise is supplied. Arbitrary-curve normalization remains separate. Signed standing unverified. [pcc:unverified]'),d,Spacer(1,10),p('<b>Actual character.</b> P=(0,0) is a nonzero actual point killed by three; a3≠0 follows from the discriminant. For an actual three-torsion point Q=(x,y) outside the origin line, define w(Q)=−(2y+a1x+a3)/(y+a1x+2a3). The literal division polynomial, curve equation and nonsingularity prove the denominator is nonzero and w²+w+1=0. Actual addition of P preserves the value and actual negation inverts it. Set the value to1 on O, P and−P; that is exactly its kernel. These statements hold without division by two. The normal form is not assumed to have exactly a cyclic full torsion group. [pcc:unverified]'),p('<b>Actual determinant and extension images.</b> Reuse the preceding general geometric rank/basis proof, then construct an actual basis with P first by an invertible F3 matrix and actual linear equivalence. The character is the second basis vector’s primitive cube root raised to the actual second coordinate. Actual point Frobenius fixes P and sends character values to their q-th powers. Primitive-root exponent injectivity forces the second diagonal entry to be qmod3; the first column is(1,0), so the determinant isq. Basis independence gives every basis. Existing actual chosen-field descent and the qualified matrix theorem then prove the source criterion in this family. [pcc:unverified]'),p('<b>Controls.</b> The genuine nonsingular characteristic3 model y²+xy+y=x³ has discriminant1, exactly three geometric three-torsion points and actual module rank1, using an explicit subgroup/OptionBool equivalence. All three already lie in every compatible extension image, while3never divides3^k−1 for positivek. Lean directly refutes the unrestricted inclusion paraphrase after deleting coprimality. Original BK asks for nine points in this case; that condition is false too, so its original theorem is not refuted. Separate singular controls distinguish an equation point from a nonsingular group point and show nonzeroa3does not alone force nonzero discriminant. [pcc:unverified]'),p('<b>Sources and remaining scope.</b> BK1998 Theorem1/proof/Remarks pp142–143, DOI10.1007/s001459900040; Dujella author-hosted section15.3,p494 coordinate/order3normal form; Elkies HarvardMath2232024 III.8 tangent functions and pairing-convention warning; Snowden Michigan2013Lecture2 Weil/determinant passage. Selected sections read, exact primary hashes and inspected p494 retained. Prior general rank/Frobenius/descent and F5/F7/F25 cases credited. General coordinate/group/Frobenius normalization, canonical Weil identification, other primes and full all-k p&lt;500census remain open. No full-book/paper read, novelty, qualified-source refutation or corpus promotion. [pcc:unverified]'),p(f'<b>Validation.</b> Whole gate{g["declarations"]}/0failures/0trusted; complete statement binding and recursive custom-source/config pins. All{a["files"]}build-provenance files roundtrip through{a["unique_objects"]}unique objects; archive64558B and five gate files retained. Scalar {cid}, four variables/degree4: universal Lean identity and official32+32-point check, same producer/host; it does not certify geometry. Pinned Lean4.35.0/Mathlib56a00585…, only propext/Classical.choice/Quot.sound. No holes/custom axioms/native evaluator/raised limits/new global flags/waivers. Exact PCC replay and signed standing are separate. KB/Conductor unavailable, primary/repository/library fallback. No native experiment, paid compute or agent upload. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595.276,841.89),leftMargin=47,rightMargin=47,topMargin=35,bottomMargin=35).build(flow)
doc=fitz.open(o/'report.pdf');layout={'pages':[],'visual_inspection':'pending'}
for i,page in enumerate(doc):
 bad=[]
 for block in page.get_text('dict')['blocks']:
  for line in block.get('lines',[]):
   for span in line['spans']:
    x0,y0,x1,y1=span['bbox']
    if x0<0 or y0<0 or x1>page.rect.width or y1>page.rect.height:bad.append(span['text'])
 layout['pages'].append({'page':i+1,'text_within_page':not bad,'out_of_bounds':bad});page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/three-torsion-character-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n');print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
