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
rows=[(96,'Actual source law',['Uniform over every ordered distinct support; point masses proved','First point + relative injective tuple: genuine expectation identity']),
      (50,'Finite bound',['Actual character Gaussian average + halfL1 Fourier bound','(E distance)^4 <= (pi / (8 omega²))^d p² / (4 t^d)']),
      (4,'Source theorem',['Fixed positive weights; prime p; d=k-1 >=1; every start','E distance < epsilon at floor(gamma p^(2/d)), gamma,N before p'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.8),(130,yy+25,lines[0],7.8),(130,yy+9,lines[1],8.1)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'random-support-average.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('Expected random-support mixing: qualified theorem proved',t),
 p('<b>Result.</b> Lean proves Hildebrand1994Theorem1 for every fixed k=d+1 at least2 and fixed strictly positive normalized label PMF. For every epsilon&gt;0 it derives gamma&gt;0 and N before choosing the prime modulus or support. At exact time floor(gamma p^(2/d)), every prime p at leastN and every starting point has actual expected halfL1 distance from uniform less than epsilon. Expectation is over all ordered distinct supports. [pcc:unverified]'),d,Spacer(1,6),
 p('<b>Actual probability bridge.</b> The support law is the genuine uniform PMF on embeddings Fin(d+1) into Fp, with point mass1/(p descending-factorial(d+1)). Lean constructs its bijection with (first point, relative injective tuple), and proves the actual expectation decomposes into uniform first point and uniform relative sample. It never assumes independence of the distinct support coordinates. The walk law is the actual recursively defined iid increment PMF. [pcc:unverified]'),
 p('<b>Derived finite average.</b> Nonzero frequency maps nonzero relative choices injectively into nonzero centered integers. The Gaussian integral bound and injective product estimate give the actual averaged character moment. Every complex additive character is indexed by the standard frequencies; no nontrivial character is omitted. Actual Fourier halfL1² bound and PMF weighted Cauchy give the finite expected/mean-square distance bound. The denominator limit derives a threshold with falling factorial at least p^d/2, yielding the fourth-power estimate shown above. No spectral-average or mixing conclusion is a premise. [pcc:unverified]'),
 p('<b>Exact floor and constants.</b> Choose a positive lower weight omega from the fixed label PMF, K=pi/(8 omega²), A=K^d, R=A/epsilon^4+1 and gamma=2R. Lean proves source time t at least R p^(2/d), hence t^d at least R p². Therefore (E distance)^4 is at most A/(4R), strictly below epsilon^4. The real-power exponent, floor and order of quantifiers are retained, including k=2. [pcc:unverified]'),
 p('<b>Skeptical control.</b> With the pure-label zero-weight PMF the actual walk is a point mass. On every prime, averaging over all supports still gives distance at least1/2 at every time and start. Lean directly negates existence of any gamma/N giving less than1/4 on sufficiently large primes. This deletes strict positivity; it does not refute the qualified source. Prior literal k2 counting/Gaussian equality errors retain their correction credit and valid repairs. [pcc:unverified]'),
 p(f'<b>Review and checks.</b> Original1994 full13-page read/inspection credited; exact Theorem1 on192/proof197-199 rechecked. Later author preprint selected12-18/reference36 only, visual15-18; no full37-page or novelty claim. Official pinned Mathlib character, finite Cauchy, floor/rpow and prior actual PMF proofs reviewed. Whole gate {g["declarations"]}/0 failures/0 trusted native; {a["files"]} provenance files in {a["unique_objects"]} lossless objects, first gate/dry-prefix retained, five final gate files/recursive pins. Lean4.35.0/Mathlib56a00585…, standard foundations/default settings. Signed standing unverified. Static rho/collision penalty/limiting constants and other fourteen-item gaps remain open. No holes/custom axioms/native evaluation/raised limits/global flags/waivers/native experiment/spend/upload. [pcc:unverified]')]

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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/random-support-average-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
