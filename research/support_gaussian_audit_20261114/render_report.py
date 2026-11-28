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
rows=[(96,'Literal source step',['Actual convergent 2D Gaussian < geometric expression','Equality is false; <= repair is proved for the printed k=3 step']),
      (50,'Actual sampling',['Uniform injective product expectation / descending factorial','Bounded by product of coordinate sums; no independence premise']),
      (4,'Actual walk decay',['Canonical centered phases + Jordan bound + PMF variance','Positive constant chosen from fixed weights before p/support/time'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.8),(130,yy+25,lines[0],7.8),(130,yy+9,lines[1],8.1)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'support-gaussian.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('Gaussian equality refuted; sampling and decay checked',t),
 p('<b>Source counterexample.</b> Lean proves that the actual convergent two-dimensional sum of exp(-a(m²+n²)), over natural indices, is strictly below 1/(1-exp(-a))² for every a&gt;0. The literal final equality in the author preprint PDF17 (original1994 proof199) is therefore false at allowed k=3. The printed real-power exponent and positive1.1epsilon/(2p) coefficient are retained in a direct affine equality negation. The valid upper-inequality repair is also proved. This does not refute qualified mixing. [pcc:unverified]'),d,Spacer(1,6),
 p('<b>Convergence and integral bound.</b> Gaussian and geometric summability are proved before factorizing the actual double sum. An antitone integral test and the Gaussian integral give the positive-index tail bound sqrt(pi/a)/2. The actual zero-excluded integer Gaussian is summable and its sum is at most sqrt(pi/a); every finite set avoiding0 inherits the bound. At a=0 the natural Gaussian is not summable, and a singleton directly refutes dropping positivity from the finite bound. No divergent totalized sum is used as a numerical Gaussian value. [pcc:unverified]'),
 p('<b>Without-replacement sampling.</b> For any finite X, dimension d at most |X| and nonnegative coordinate functions, Lean derives the actual uniform injective-sample product expectation as its actual finite sum divided by |X| descending-factorial d. The numerator is bounded by the product of coordinate sums using the actual embedding into all functions. No independence of the sampled coordinates is assumed. [pcc:unverified]'),
 p('<b>Fixed-weight actual walk bound.</b> The real part of the actual standard character equals the cosine of its canonical centered residue. Jordan’s global bound and actual normalized PMF phase variance give normSq(lambda)^t and the actual iid walk character moment at most exp(-8 omega² t times centeredRadius). A positive omega is derived from the fixed strictly positive label law before any prime modulus, support, frequency, time or start is selected. No mixing or spectral-average conclusion is a premise. [pcc:unverified]'),
 p('<b>Review and limits.</b> Hildebrand1994 full13-page review credited; selected proof197-199 rechecked. Author math/0401189v1 selected12-18/reference36 read; visual15-18, newly17-18. Not full37-page read; the separate k=2 issue is already author-acknowledged. Official pinned Mathlib integral, Jordan, infinite-sum and finite-product APIs checked. Correction attachment inaccessible; no novelty or global absence claim. Full expected-support mixing exponent and static rho/collision-penalty/limiting constants remain open. [pcc:unverified]'),
 p(f'<b>Checks.</b> Whole gate {g["declarations"]} declarations/0 failures/0 trusted native exceptions. All {a["files"]} source/config/build/provenance files retained in {a["unique_objects"]} lossless objects, including failed sparse-discovery gate; final five gate files and recursive pins. Lean4.35.0/Mathlib56a00585…, standard foundations/default settings; signed standing unverified. No holes/custom axioms/native evaluation/raised limits/global flags/waivers/native experiment/spend/upload. [pcc:unverified]')]

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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/support-gaussian-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
