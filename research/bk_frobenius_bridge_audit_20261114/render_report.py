"""Render recorded proof scopes and literal counts; no research computation."""
from pathlib import Path
import json,fitz,html
from reportlab.platypus import SimpleDocTemplate,Paragraph,Spacer
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib import colors
from reportlab.graphics.shapes import Drawing,Rect,String,Line
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
r=Path(__file__).resolve().parent;o=r/'reports';o.mkdir(exist_ok=True)
pdfmetrics.registerFont(TTFont('DV','/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf'));pdfmetrics.registerFont(TTFont('DVB','/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf'));pdfmetrics.registerFontFamily('DV',normal='DV',bold='DVB')
b=ParagraphStyle('b',fontName='DV',fontSize=8.7,leading=11.8,spaceAfter=10);t=ParagraphStyle('t',fontName='DVB',fontSize=16,leading=20,spaceAfter=12)
def p(s):return Paragraph(s,b)
d=Drawing(500,128);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="128" viewBox="0 0 500 128">']
def label(x,y,s,size=8.6):
 d.add(String(x,y,s,fontName='DV',fontSize=size,fillColor=colors.HexColor('#234b70')));svg.append(f'<text x="{x}" y="{128-y}" font-family="DejaVu Sans,sans-serif" font-size="{size}" fill="#234b70">{html.escape(s)}</text>')
def bar(y,w,color):
 d.add(Rect(185,y,w,16,fillColor=colors.HexColor(color),strokeColor=None));svg.append(f'<rect x="185" y="{128-y-16}" width="{w}" height="16" fill="{color}"/>')
label(4,111,'Actual E/F2: y²+y=x³+1; kernel-checked point cardinalities',9.2)
label(4,81,'Rational points E(F2)');bar(76,75,'#447c9b');label(267,81,'3 points')
label(4,51,'Geometric E[3]');bar(46,225,'#2c788e');label(417,51,'9 points')
label(4,16,'Actual Frobenius → actual F3 matrix → powers ↔ actual extension image.',8.2)
svg.append('</svg>');(o/'frobenius-bridge.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());cid=json.loads((r/'identity-certificate.json').read_text())['id']
flow=[Paragraph('Actual Frobenius, basis matrices and descent',t),p('<b>Result.</b> Lean connects actual geometric three-torsion to an actual Frobenius matrix and to containment in every chosen finite extension. The basis is constructed from the preceding general rank proof for every nonsingular Weierstrass model in characteristic other than three, including two. The generic determinant formula remains an explicit premise; a concrete characteristic-two model proves it separately. Signed standing unverified. [pcc:unverified]'),d,Spacer(1,10),p('<b>General actual-object bridge.</b> Restrict the actual coordinate q-Frobenius to the subgroup killed by three, then use its induced ZMod3 module to obtain a linear map. Its actual basis matrix acts on the actual coordinate representation. Matrix powers are the identity exactly when the corresponding iterated point Frobenius fixes every torsion point. The determinant is nonzero and independent of the chosen basis. Actual rational point-count divisibility supplies a nonzero fixed vector through Cauchy and injective point inclusion. Existing finite-field descent then identifies matrix power identity with actual point-map image containment in every compatible field of size q^k. With the displayed hypotheses det=q and q≠1 mod3, the earlier linear theorem supplies the qualified BK criterion. No rank, fixed-vector or abstract rationality predicate is assumed. [pcc:unverified]'),p('<b>Binary positive and negative controls.</b> The displayed F2 curve has three rational points and nine geometric three-torsion points. Its actual three-division quartic forces x^4=x; its equation forces y^4=y. Frobenius squared is therefore the identity. Frobenius itself is not the identity: actual descent would otherwise inject nine torsion points into three rational points. Kernel-checked classification of the81two-by-two F3 matrices, with a nonzero fixed vector, proves its determinant is2. The source criterion follows for every chosen field of size2^k and compatible embedding, with no determinant premise. This is not a new generic determinant theorem or a new finite-field-existence construction. The existing F7 curve y²=x³+3x+2 also yields a constructed actual matrix different from identity although7≡1mod3, testing the excluded boundary. [pcc:unverified]'),p('<b>Prior work and source.</b> Original Balasubramanian–Koblitz, J.Cryptology11(1998),141–145, DOI10.1007/s001459900040: Theorem1/proof/Remarks pp142–143 re-read. Prime/r∣#E/r∤q−1 conditions are retained. Existing actual F5/F7 all-k cases, concrete F25 Frobenius basis, short-only three-torsion count and generic chosen-field descent are credited. This packet integrates those components with the new general-model basis. General determinantq/canonical Weil pairing, other odd primes≥5 and the full all-k p&lt;500census remain open. The qualified source theorem is not refuted; no full-paper/book read or novelty claim. [pcc:unverified]'),p(f'<b>Validation and failed attempts.</b> Whole gate {g["declarations"]}/0failures/0trusted; exact full declaration binding and recursive custom-dependency/config pins. All135build-provenance files roundtrip through41unique objects, preserving rejected identities, exact coefficient corrections and one interrupted build without an exit. Scalar supplement {cid}: two variables/degree6, universal CommRing Lean proof and official32+32-point check, same host/producer. It does not prove geometry. Pinned Lean4.35.0/Mathlib56a00585…, only propext/Classical.choice/Quot.sound; no holes/custom axioms/native evaluator/raised limits/new global flags/waivers. Exact PCC replay and signed standing are separate. KB/Conductor unavailable; primary/repository/library fallback. No new native experiment, paid compute or agent upload. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595.276,841.89),leftMargin=47,rightMargin=47,topMargin=35,bottomMargin=35).build(flow)
doc=fitz.open(o/'report.pdf');layout={'pages':[],'visual_inspection':'pending'}
for i,page in enumerate(doc):
 bad=[]
 for block in page.get_text('dict')['blocks']:
  for line in block.get('lines',[]):
   for span in line['spans']:
    x0,y0,x1,y1=span['bbox']
    if x0<0 or y0<0 or x1>page.rect.width or y1>page.rect.height:bad.append(span['text'])
 layout['pages'].append({'page':i+1,'text_within_page':not bad,'out_of_bounds':bad});page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/bk-frobenius-bridge-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n');print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
