"""Display already recorded formal values; no research calculation."""
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
b=ParagraphStyle('b',fontName='DV',fontSize=9,leading=12.5,spaceAfter=11);t=ParagraphStyle('t',fontName='DVB',fontSize=16,leading=21,spaceAfter=14)
def p(s):return Paragraph(s,b)
d=Drawing(500,135);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="135" viewBox="0 0 500 135">']
def label(x,y,s,size=9):
 d.add(String(x,y,s,fontName='DV',fontSize=size,fillColor=colors.HexColor('#234b70')));svg.append(f'<text x="{x}" y="{135-y}" font-family="DejaVu Sans,sans-serif" font-size="{size}" fill="#234b70">{html.escape(s)}</text>')
def bar(y,w,color):
 d.add(Rect(160,y,w,19,fillColor=colors.HexColor(color),strokeColor=None));svg.append(f'<rect x="160" y="{135-y-19}" width="{w}" height="19" fill="{color}"/>')
label(4,119,'All 512 equally likely ordered triples; n=8, four negation orbits',10)
label(4,85,'Actual expectation');bar(79,220,'#2c788e');label(390,85,'11/16')
label(4,48,'Printed lower bound');bar(42,240,'#bd6248');label(410,48,'12/16')
label(4,12,'Exact rational values; no statistical uncertainty. Total intersections:352.',8.4)
svg.append('</svg>');(o/'collision-bound.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());cid=json.loads((r/'identity-certificate.json').read_text())['id']
flow=[Paragraph('Thériault’s finite collision lower bound fails',t),p('<b>Result.</b> Lean disproves the lower bound in Theorem1 of Thériault (ASIACRYPT2003), printed p85, within the theorem’s own finite sampling model. At n=8,s=3 the actual expected intersections are 11/16, below the printed 2s²/(3n)=3/4. The hypotheses 3≤s&lt;n/2 hold. Signed standing unverified. [pcc:unverified]'),d,Spacer(1,12),p('<b>Actual model.</b> Pool=Fin4×Bool has eight elements. Negation flips the Boolean coordinate; Lean proves it is involutive with no fixed points, and that equal first coordinates mean equality up to negation. Samples are all functions Fin3→Pool, hence all512 independent ordered draws with replacement, each equally likely. Intersections are three minus the number of distinct negation orbits. The exact sum352 is checked by the Lean kernel. A triple in the same orbit contributes two intersections; three distinct orbits contribute zero. The printed upper bound passes. [pcc:unverified]'),p('<b>Source check.</b> Definition7/Proposition2 count repeated occurrences after the first, rather than every unordered collision pair. The model excludes self-negative large primes and pairs the rest by sign. The printed proof on p86 contains an invalid inequality after replacing s(s−1) with s². Separately, Lean proves the source’s displayed closed form at s=3 is 6/n−4/n², strictly below its claimed lower6/n for every rational n&gt;6. This algebraic check agrees with the actual finite counterexample. [pcc:unverified]'),p('<b>Conditional repair.</b> Assuming e₀=0 and the displayed recurrence eₛ₊₁=(1−2/n)eₛ+2s/n, Lean proves 2s(s−1)/(3n)≤eₛ≤s(s−1)/n for n≥2 and s≤n/2. For s≥3 this gives 4s²/(9n)≤eₛ≤s²/n. The recurrence therefore retains constant-factor quadratic bounds. Its generic identification with an actual iid expectation is a separate open bridge; the finite counterexample above assumes no recurrence. The overall algorithm and complexity Theorem3 are not refuted by this constant error. [pcc:unverified]'),p('<b>Attribution and evidence.</b> N.Thériault, <i>Index Calculus Attack for Hyperelliptic Curves of Small Genus</i>, LNCS2894(2003),75–92, DOI10.1007/978-3-540-40061-5_5. Publisher PDF216881B/18pages, SHA3048be18…eb6a33. Exact pp85–86 rendered and visually inspected; relevant definitions/algorithm and selected §§5.3–5.12 read. No full-paper/dependency read or absence-of-errata/novelty claim. Additive coverage preserves historical corpus/backlog labels. KB unavailable; primary/repository/Mathlib fallback retained. [pcc:unverified]'),p(f'<b>Validation and boundary.</b> Scalar supplement {cid}: official checker32+32points; corresponding Lean cubic identity proved. Whole gate {g["declarations"]}/0failures/0trusted; only propext/Classical.choice/Quot.sound. Lean4.35.0 and Mathlib56a00585… pinned. Complete closed binding, source/config/artifact pins and lossless unsuccessful/successful attempts retained. No holes, custom axioms, native evaluator, raised limits, global flags or waivers. Exact PCC replay and signed standing are separate. No native experiment, paid compute or upload. Actual Jacobian smoothness, walk sampling, subgroup distribution, rank/invertibility and full runtime formalization remain distinct obligations. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595.276,841.89),leftMargin=47,rightMargin=47,topMargin=35,bottomMargin=35).build(flow)
doc=fitz.open(o/'report.pdf');layout={'pages':[],'visual_inspection':'pending'}
for i,page in enumerate(doc):
 bad=[]
 for block in page.get_text('dict')['blocks']:
  for line in block.get('lines',[]):
   for span in line['spans']:
    x0,y0,x1,y1=span['bbox']
    if x0<0 or y0<0 or x1>page.rect.width or y1>page.rect.height:bad.append(span['text'])
 layout['pages'].append({'page':i+1,'text_within_page':not bad,'out_of_bounds':bad});page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/theriault-collision-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n');print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
