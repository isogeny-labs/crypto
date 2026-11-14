"""Lay out recorded formal scopes; no mathematical computation."""
from pathlib import Path
import html,json,fitz
from reportlab.platypus import SimpleDocTemplate,Paragraph,Spacer
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib import colors
from reportlab.graphics.shapes import Drawing,Rect,String,Line
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
r=Path(__file__).resolve().parent;o=r/'reports';o.mkdir(exist_ok=True)
pdfmetrics.registerFont(TTFont('DV','/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf'))
pdfmetrics.registerFont(TTFont('DVB','/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf'))
pdfmetrics.registerFontFamily('DV',normal='DV',bold='DVB')
b=ParagraphStyle('b',fontName='DV',fontSize=8.8,leading=12,spaceAfter=11)
t=ParagraphStyle('t',fontName='DVB',fontSize=16,leading=20,spaceAfter=12)
def p(s):return Paragraph(s,b)
d=Drawing(500,116);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="116" viewBox="0 0 500 116">']
for x,label,detail in [(0,'Actual rational 3-point','Cauchy from 3 | #E(Fq)'),(173,'Actual normal form','Tangent shift + shear'),(346,'Chosen extension','Inclusion iff 3 | q^k−1')]:
 d.add(Rect(x,45,154,57,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')))
 svg.append(f'<rect x="{x}" y="14" width="154" height="57" fill="#eaf2f6" stroke="#2c788e"/>')
 for yy,tx,sz in [(80,label,9),(60,detail,8)]:
  d.add(String(x+7,yy,tx,fontName='DV',fontSize=sz));svg.append(f'<text x="{x+7}" y="{116-yy}" font-size="{sz}" font-family="DejaVu Sans,sans-serif">{html.escape(tx)}</text>')
for x in [155,328]:
 d.add(Line(x,74,x+16,74,strokeColor=colors.HexColor('#2c788e')));svg.append(f'<path d="M{x} 42h16" stroke="#2c788e"/>')
txt='Actual AddEquiv + field-map naturality + all-n torsion + original nine-point bridge'
d.add(String(2,18,txt,fontName='DV',fontSize=8.2));svg.append(f'<text x="2" y="98" font-size="8.2" font-family="DejaVu Sans,sans-serif">{txt}</text>');svg.append('</svg>')
(o/'normalization-descent.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('The qualified BK criterion at three, for all models',t),
 p('<b>Result.</b> Every nonsingular general Weierstrass model over every finite field, including characteristic two. With gcd(3,q)=1, 3 dividing the actual rational point-group cardinality and 3 not dividing q−1, all geometric points killed by three lie in a chosen compatible extension of cardinality q^k exactly when 3 divides q^k−1. Equivalently, that extension has exactly nine killed-three points, including the identity. No normalization, determinant, rank, pairing or abstract rationality premise. The qualified source survives this check at three. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>Actual normalization.</b> Additive Cauchy supplies an actual nonzero rational point P killed by three. Translate P to the origin and shear by its tangent slope. The curve equation proves a6=0, the slope proves a4=0 and the literal three-division polynomial proves a2=0. Actual nonsingularity proves the tangent denominator nonzero. The discriminant is preserved. No division by two is used. Coprimality supplies characteristic other than three through the additive order of 1. [pcc:unverified]'),
 p('<b>Actual group and descent bridges.</b> The explicit coordinate map and inverse form an additive equivalence of actual Mathlib point groups. The proof handles identities, inverse pairs, secants and tangents. This equivalence commutes with actual field maps, compatible tower inclusions and point Frobenius. For every integer n it transports killed-point cardinalities and torsion containment in the chosen extension. Apply the preceding normal-form cubic-character criterion. The actual injective map on killed-three subgroups is surjective exactly when all geometric torsion descends; the preceding geometric nine-point theorem makes this equivalent to nine extension points. [pcc:unverified]'),
 p('<b>Skeptical controls.</b> The credited actual F7 curve has 3² dividing its rational point count and q=1mod3 but a geometric three-torsion point not fixed by Frobenius. The genuine ordinary characteristic-three curve y²+xy+y=x³ has nonzero discriminant and only three geometric three-torsion points. Deleting coprimality refutes the unrestricted inclusion paraphrase there; the original nine-point condition is false too. These controls preserve the qualified theorem rather than refuting it. [pcc:unverified]'),
 p('<b>Primary and prior sources.</b> <link href="https://link.springer.com/content/pdf/10.1007/s001459900040.pdf" color="blue">BK1998 Theorem 1/proof/Remarks, pp142–143</link>, DOI10.1007/s001459900040; <link href="https://web.math.pmf.unizg.hr/~duje/pdf/section15-3.pdf" color="blue">Dujella section15.3, p494</link> coordinate/order-three normal form, discussed over Q. Pinned Mathlib variable-change/affine-point APIs, prior short scaling, actual F5/F7/F25 cases, general rank, Frobenius/descent and normal-form character proofs credited. Selected sections read; no full-book/paper read, novelty or corpus promotion. KB/Conductor unavailable; bounded fallback recorded. [pcc:unverified]'),
 p(f'<b>Validation and remaining scope.</b> Whole gate {g["declarations"]} declarations, zero failures/trusted native exceptions. Complete statement binding and recursive custom-source/config pins. All {a["files"]} build-provenance files retained losslessly in {a["unique_objects"]} objects; five gate records retained. Pinned Lean4.35.0/Mathlib56a00585…, only propext/Classical.choice/Quot.sound. Exact local PCC replay and signed standing remain separate. Other primes ≥5, canonical Weil identification, full all-k p&lt;500 census and remaining fourteen-item obligations are open. No native experiment, paid compute, upload, holes/custom axioms/native evaluator/raised limits/new global flags/waivers. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595.276,841.89),leftMargin=47,rightMargin=47,topMargin=35,bottomMargin=35).build(flow)
doc=fitz.open(o/'report.pdf');layout={'pages':[],'visual_inspection':'pending'}
for i,page in enumerate(doc):
 bad=[]
 for block in page.get_text('dict')['blocks']:
  for line in block.get('lines',[]):
   for span in line['spans']:
    x0,y0,x1,y1=span['bbox']
    if x0<0 or y0<0 or x1>page.rect.width or y1>page.rect.height:bad.append(span['text'])
 layout['pages'].append({'page':i+1,'text_within_page':not bad,'out_of_bounds':bad})
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/bk-three-normalization-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
