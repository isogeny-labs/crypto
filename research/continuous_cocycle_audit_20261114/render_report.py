"""Layout recorded formal statements and provenance; no research computation."""
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
b=ParagraphStyle('b',fontName='DV',fontSize=8.7,leading=11.8,spaceAfter=10)
t=ParagraphStyle('t',fontName='DVB',fontSize=16,leading=20,spaceAfter=12)
def p(s):return Paragraph(s,b)
d=Drawing(500,142);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="142" viewBox="0 0 500 142">']
rows=[(96,'Actual source group',['Absolute Gal(F2bar/F2), with compact Krull topology','Exact q-power Frobenius: x ↦ x²']),
      (50,'Actual coefficient group',['Discrete integers, trivial continuous action','Every continuous 1-cocycle vanishes']),
      (4,'Actual evaluation',['Coinvariants are integers; integer class 1 is nonzero','Evaluation misses class 1, even on any class quotient'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.8),(130,yy+25,lines[0],7.8),(130,yy+9,lines[1],8.1)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'continuous-cocycle-obstruction.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('Continuous integer cocycles: the broad evaluation formula fails',t),
 p('<b>Result.</b> Lean disproves the evaluation-surjectivity required by the broad arbitrary-discrete-module wording of Bruin2011 equation2.1. The example uses the actual Krull absolute Galois group of F2, actual square Frobenius, and discrete integers with trivial continuous action. Every continuous one-cocycle is zero; the actual coinvariant quotient contains the nonzero integer class1, which evaluation cannot reach. The finite-module Lemma2.1/application is retained. [pcc:unverified]'),d,Spacer(1,6),
 p('<b>Proof and faithful domains.</b> A continuous homomorphism from a compact group has finite image in a discrete group. Image elements have finite order, so a torsion-free target forces all values to be identity. Lean constructs the actual Frobenius AlgEquiv and proves its x² formula, verifies trivial additive-action identity/composition laws and joint continuity, and proves every continuous integer-valued function satisfying the standard cocycle identity vanishes. Bundling as continuous homomorphisms excludes no other cocycles. [pcc:unverified]'),
 p('<b>The claimed map cannot be repaired by quotienting.</b> The actual image of identity minus identity consists only of zero. Lean proves the integer class1 remains nonidentity in the actual coinvariant quotient and that evaluation is nonsurjective at every actual Galois element, specifically at square Frobenius. It directly rules out any type H with a surjective cocycle-representative map and a bijection H→coinvariants agreeing with evaluation. No homemade H1 object or supplied cohomology value is used. [pcc:unverified]'),
 p('<b>Controls and notation.</b> Integer1 is not killed by any positive natural number. The identity continuous homomorphism on discrete integers has nonidentity value at1: continuity alone does not force vanishing after removing compactness. Code uses Multiplicativeℤ for the quotient API; its multiplicative identity denotes additive integer0. The missing class represents additive integer1. [pcc:unverified]'),
 p('<b>Primary review and limit.</b> <link href="https://www.numdam.org/article/JTNB_2011__23_2_323_0.pdf" color="blue">Bruin2011, JTNB23,323–328, published325/PDF4, equation2.1</link>: the preceding module/action paragraph quantifies arbitrary discrete groups with continuous action. The finite restriction enters the subsequent Lemma2.1. Prior full published-note reading/allseven-page inspection credited. Selected <link href="https://pub.math.leidenuniv.nl/~bruinpj/tate-pairing.pdf" color="blue">author preprint page3/equation1</link> context read/rendered/fullyinspected, with matching wording. Bounded correction/qualification search and author links checked; no novelty/global errata-absence claim. No cited Serre theorem is marked read or refuted. [pcc:unverified]'),
 p(f'<b>Validation and remainder.</b> Wholegate{g["declarations"]}/0failures/0trusted native exceptions; all{a["files"]} build/processing/provenance files losslessly retained in{a["unique_objects"]}objects, five gate files and recursive source/config pins. Lean4.35.0/Mathlib56a00585…, standard foundations/default limits. Exact local replay differs from signed standing. Canonical continuous H1 is not reconstructed; corrected finite/torsion H1, Lang/Cartier/canonical-divisor/qualified pairing and other fourteen-item gaps remain open. OriginalFR1994 unread. KB/Conductor unavailable, bounded fallback recorded. No corpus promotion, holes/custom axioms/native evaluation/raised limits/global flags/waivers/native experiment/paid compute/upload. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/continuous-cocycle-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
