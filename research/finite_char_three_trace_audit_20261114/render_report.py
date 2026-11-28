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
rows=[(96,'Every finite char3',['Source gcd(trace,q)≠1 ↔ 3 divides trace ↔ #E≡1 mod3','↔ b2=0 ↔ j=0 ↔ geometric three-torsion count 1']),
      (50,'Actual proof route',['Complete square; finite cube root; paired twist fibers','Ordinary: one partner has a 3-point; a=0: neither does']),
      (4,'Explicit controls',['Ordinary F3 pair: #E=5/3, rational 3-torsion=1/3','Source-SS F3 pair: #E=7/1, traces −3/+3'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.8),(130,yy+25,lines[0],7.8),(130,yy+9,lines[1],8.1)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'finite-char3-trace.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('Every finite characteristic-three field: trace and geometric torsion',t),
 p('<b>Result.</b> For every finite characteristic-three field and nonsingular general Weierstrass model, Lean proves the exact source gcd(q+1−#E,q)≠1 predicate iff three divides the actual trace iff actual point count is one modulo three iff b2=0 iff j=0 iff actual geometric three-torsion count is one iff its ZMod3 rank is zero. The source predicate is invariant under finite field extensions. No field-size/degree bound or Hasse/trace-square/Endrank premise. [pcc:unverified]'),d,Spacer(1,6),
 p('<b>Ordinary branch.</b> For y²=x³+a·x²+b·x+c with a≠0, finite cube-root surjectivity supplies an actual third-division-polynomial abscissa. Complementary square-root fibers supply an ordinate on the curve or its nonsquare-scaled partner. One partner therefore has an actual nonzero point killed by three. Actual Cauchy/Lagrange and twist-source invariance exclude the source predicate. No rational-torsion-existence or non-isomorphism premise. [pcc:unverified]'),
 p('<b>Special branch and general models.</b> If a=0, the actual division polynomial excludes nonzero three-torsion on both partners. Cauchy makes both point counts nonzero modulo three; their actual sum2(q+1), with q=3ⁿ and n&gt;0, forces each count to be one modulo three. A complete-square unit-u1 shift preserves discriminant, actual additive point group/cardinality and b2, and its x² coefficient equals original b2. It transfers the result to every general model. Prior actual group/geometric j/module proofs are credited. [pcc:unverified]'),
 p('<b>Controls.</b> The credited ordinary F3 curve y²=x³+2x²+1 has j=1, five actual points and rational three-torsion count one. Its d=2 partner has three points, rational three-torsion count three and P=(1,1) of exact order three; both source predicates are false. Lean negates the rational replacement criterion. The credited source-SS F3 curve y²=x³+2x+1 and its partner have seven/one points and traces−3/+3. Lean directly refutes the universal trace-zero strengthening. The source p≥5 prime-field clause is retained. [pcc:unverified]'),
 p('<b>Primary review.</b> <link href="https://link.springer.com/content/pdf/10.1007/s00145-009-9048-z.pdf" color="blue">FST2010 selectedpp230/237</link>: exact trace predicate/qualifications. <link href="https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch9.pdf" color="blue">Galbraith Ex9.5.4 count context</link>, preceding inspectedPDF7 and actual cubic-with-x² proof credited; the false non-isomorphism exercise is not a premise. <link href="https://www.jmilne.org/math/Books/EC2.pdf" color="blue">Milne Remark3.16/contextpp98–99/p75notes</link>: geometric p-rank/Endrank distinction. Official finite Frobenius/cardinality/coprime-power and preceding actual division-polynomial/Cauchy/geometric j proofs reviewed. KB/Conductor unavailable, bounded fallback before code. No fullread/corpus promotion/novelty. [pcc:unverified]'),
 p(f'<b>Validation and scope.</b> Whole gate {g["declarations"]} declarations, zero failures, zero trusted native exceptions. All {a["files"]} build/provenance files retained losslessly in {a["unique_objects"]} objects; five gate files, complete statement and recursive source/config pins. Lean4.35.0/Mathlib56a00585…, standard foundations only. Exact local PCC replay differs from signed standing. General p≥5 trace/geometric identifications, Endrank equivalences, Hasse/embedding/full-torsion-field bounds and remaining original fourteen-item gaps stay open. No holes/custom axioms/native evaluator/raised limits/new global flags/waivers/native experiment/paid compute/agent upload. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/finite-char-three-trace-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
