"""Render checked statements/provenance; no research computation."""
from pathlib import Path
import html,json,fitz
from reportlab.platypus import SimpleDocTemplate,Paragraph,Spacer
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib import colors
from reportlab.graphics.shapes import Drawing,Rect,String
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
r=Path(__file__).resolve().parent;o=r/'reports';o.mkdir(exist_ok=True)
pdfmetrics.registerFont(TTFont('DV','/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf'))
pdfmetrics.registerFont(TTFont('DVB','/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf'))
pdfmetrics.registerFontFamily('DV',normal='DV',bold='DVB')
b=ParagraphStyle('b',fontName='DV',fontSize=9,leading=12.1,spaceAfter=10)
t=ParagraphStyle('t',fontName='DVB',fontSize=17,leading=22,spaceAfter=14)
def p(s):return Paragraph(s,b)
d=Drawing(500,120);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="120" viewBox="0 0 500 120">']
rows=[(81,'Actual field','All roots adjoin = simple primitive-root extension','Finiteness derived; actual Frobenius order = actual field degree'),
      (42,'Rationality','Finite extension L: all roots lie in L iff r divides #L−1','Actual #L=q^finrank; no elliptic torsion identification assumed'),
      (3,'Guard control','Characteristic p: actual pth-root field is the base field','Actual degree1 versus modular order0; r1/r2 also degree1')]
for yy,title,a,c in rows:
 d.add(Rect(0,yy,500,34,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')))
 svg.append(f'<rect x="0" y="{120-yy-34}" width="500" height="34" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+17,title,8.1),(104,yy+21,a,7.6),(104,yy+7,c,7.6)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{120-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'cyclotomic.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Cyclotomic degree and rationality: actual fields',t),
 p('<b>General finite-field theorem proved.</b> For every finite field K with q elements and every natural r nonzero in K, the actual field obtained by adjoining all r-th roots of unity in AlgebraicClosure(K) has degree equal to the multiplicative order of q modulo r. Its actual degree divides n exactly when r divides q^n−1. Prime r is not required. This proves a field-theoretic bridge used by items2/3/4. [pcc:unverified]'),d,Spacer(1,9),
 p('<b>Construction and proof.</b> The field is IntermediateField.adjoin K {z | z^r=1}. A primitive root is derived from the characteristic guard; every root is its power, so this actual adjoin is a simple algebraic extension. Integrality, finite dimension and the finite carrier are derived. Actual q-Frobenius to the nth power fixes every generator exactly when r divides q^n−1; the primitive root proves the converse. Equality of actual Frobenius order with actual Module.finrank gives the degree formula. No modular-order/field-degree identification, supplied primitive root or assumed rank is a premise. [pcc:unverified]'),
 p('<b>All-root rationality in actual extensions.</b> For every actual finite intermediate field L of AlgebraicClosure(K), X^#L−X splits over L and its roots in the closure are exactly the elements of L. This proves root-field containment in L iff r divides #L−1. The actual module cardinality theorem gives #L=q^finrank(K,L), yielding all-root rationality iff r divides q^finrank(K,L)−1. Finiteness of L is explicit; the extension degree is actual. [pcc:unverified]'),
 p('<b>Skeptical controls.</b> For every finite field K of prime characteristic p, injectivity of actual Frobenius proves every p-th root of unity equals1. The actual p-th-root adjoin is the base field and has degree1, whereas the order of q modulo p is0 and p does not divide q−1. Both guard-free degree/order and containment/divisibility identities fail on these same actual fields. For every field, the actual first- and second-root fields have degree1. This refutes omission of the characteristic guard; the correctly qualified cyclotomic statement survives. [pcc:unverified]'),
 p('<b>Boundary.</b> This is the cyclotomic roots field, not a proof that it equals the full elliptic E[r] field. The general canonical Weil/Tate and elliptic-torsion bridge, nonprime supersingular at-most6 classification, isogeny/Lang/Cartier/Jacobian/divisor/MOV–Frey–Rück comparisons and other fourteen-item obligations remain. No canonical corpus promotion or novelty claim. [pcc:unverified]'),
 p(f'<b>Checks and retained failures.</b> Whole gate {g["declarations"]}/0 failures/0 trusted native; only propext/Classical.choice/Quot.sound. Complete {binding["full_characters"]}-character declaration SHA {binding["full_decl_sha256"]}; independent native1complete/0prefix/0unresolved. Eight snapshots retain three failed attempts: {a["files"]} files/{a["unique_objects"]} objects/{size}-byte losslessXZ, plus five gate files. Final08 and gate logs fully searched: no warnings/errors. Lean4.35.0/Mathlib56a00585…/defaults. No holes/custom axioms/native proof evaluation/raised limits/flags/waivers/frozen edits/cryptoPython/new native crypto/spend/upload. Header processing procedural; signed standing unverified. [pcc:unverified]'),
 p('<b>Sources checked.</b> FST2010 DOI10.1007/s00145-009-9048-z, primary723726bytes/57pages/SHA2acd937b…: exact printed230/PDF7 Definition2.1/Remark2.2 and printed237/PDF14 were fully text-read/visually inspected in the preceding packet, not all57pages. Prior prime-field CyclotomicDegree, GaloisGenerator/FiniteQuotients, CharTwoGaloisControl and SourceFSTBoundary arguments generalized and credited; originals byte-checked and retained. Pinned official primitive-root/adjoin/Frobenius/order/cardinality APIs read. KB/Conductor127, prior/primary/official fallback; no absence claim. Primary PDF/text/images local only. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/finite-cyclotomic-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
