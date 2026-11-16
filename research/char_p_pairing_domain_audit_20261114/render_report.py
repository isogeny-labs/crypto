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
rows=[(96,'Any char-p field',['p^k-th unit roots: identity only','Finite field: units / p^k-th powers has cardinality 1']),
      (50,'Actual curve domains',['Finite E/nE and E[n] have equal cardinality','Ordinary char2: 2 / 2 / 1; paired ordinary char3: 3 / 3 / 1']),
      (4,'Scope controls',['Prime nonidentity root implies coprimality','Imperfect F2(X): X gives a nonidentity square class'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.8),(130,yy+25,lines[0],7.8),(130,yy+9,lines[1],8.1)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'char-p-pairing-domains.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('Characteristic-order pairings: actual domains and failed strengthenings',t),
 p('<b>Result.</b> For every prime p, characteristic-p field and natural k, the only p^k-th unit root is 1. Over every finite such field, the unit-power map is surjective and the actual quotient by its image has cardinality one. A left-bilinear map from a p^k-killed additive group into any characteristic-p field’s units is identically one. A nonzero left element precludes nondegeneracy. [pcc:unverified]'),d,Spacer(1,6),
 p('<b>Actual domains.</b> Lean proves for every finite additive commutative group that the quotient by n-multiples has the same cardinality as the actual n-kernel, then specializes to Mathlib curve points. Left domain E[n], right domain E/nE and target K*/(K*)^n are actual kernels/ranges/quotients. Frobenius injectivity, finite surjectivity and the subgroup-index theorem supply the proofs, with no candidate pairing or nondegeneracy premise. [pcc:unverified]'),
 p('<b>Unbounded curve families.</b> Every finite characteristic-two field and nonsingular general model has left/right2-kernel/cokernel cardinality1 if a1=0 and2 otherwise; target cardinality1. Every ordinary model excludes unit-target candidates over any characteristic-two field and candidates on the proper Tate domains. The actual y²+xy=x³+1 model has point(0,1) of exact order2 over every such field. In finite characteristic3, a constructed nonzero3-point on one ordinary cubic or its nonsquare partner gives actual domain sizes3/3/1. The credited F3 partner has P=(1,1) of exact order3. Prior actual models, geometric torsion, twist construction and group proofs are credited. [pcc:unverified]'),
 p('<b>Source guard.</b> <link href="https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch26.pdf" color="blue">Galbraith author chapter26, Definition26.2.2/Theorem26.2.3 and Exercise26.3.1/Theorem26.3.3</link>, printed574–577/PDF2–5 read and visually inspected: coprimality and Tate root/domain hypotheses retained. Lean proves a prime-order nonidentity root already forces coprimality with field cardinality. Direct universal negations concern the unit claim after deleting coprimality and the separate unqualified Tate claim omitting both coprimality and root hypotheses. They do not refute the retained-root-qualified source or perfect group-scheme pairings. [pcc:unverified]'),
 p('<b>Imperfect-field negative control.</b> Over F2(X), the actual X-unit has a nonidentity class modulo squares; target cardinality is not1 and the square map is not surjective. The prior integer-degree nonsquare proof is credited. Unit square-roots of1 remain trivial. Lean directly refutes deleting finiteness from the universal trivial-target claim. [pcc:unverified]'),
 p(f'<b>Validation and remainder.</b> Wholegate{g["declarations"]}/0failures/0trusted native exceptions; all{a["files"]} build/provenance files retained losslessly in{a["unique_objects"]}objects, five gate files, complete main statement and recursive source/config pins. Lean4.35.0/Mathlib56a00585…, standard foundations/default limits. Exact local PCC replay differs from signed standing. KB/Conductor unavailable; bounded primary/prior/official fallback recorded. Canonical divisor/valuation Weil/Tate/MOV/FR, general End/Tate-module and other fourteen-item gaps remain open. No fullread/novelty/corpus promotion, holes/custom axioms/native evaluation/raised limits/global flags/waivers/native experiment/paid compute/upload. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/char-p-pairing-domain-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
