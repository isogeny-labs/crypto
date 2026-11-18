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
rows=[(96,'Derived Parseval',['Actual finite complex character transform','Sum of squared Fourier norms = |G|·sum of squared norms']),
      (50,'Actual probability',['Half-L1 = maximum event discrepancy; attaining event proved','Half-L1² ≤ nontrivial-character energy / 4']),
      (4,'Sharp control',['ZMod2 pointmass/uniform: fullL1=1, halfL1=1/2, energy=1','Quarter bound is equality for halfL1; false for fullL1'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.8),(130,yy+25,lines[0],7.8),(130,yy+9,lines[1],8.1)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'finite-fourier-upper-bound.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('Finite Fourier upper bound: normalization checked',t),
 p('<b>Result.</b> Lean proves the finite-abelian Upper Bound Lemma for actual PMFs and a genuine uniform law: half-L1 distance squared is at most one quarter of the nontrivial-character energy. Finite complex Plancherel is derived from actual character orthogonality, then combined with squared Cauchy–Schwarz and actual probability normalization. No spectral-energy or mixing bound is supplied as a premise. [pcc:unverified]'),d,Spacer(1,6),
 p('<b>Actual source distance and uniform law.</b> FullL1 sums absolute differences of actual point probabilities; source distance is halfL1. Lean proves its range[0,1], every-event bound and an explicit positive-difference event attaining it, so it equals maximum event discrepancy. Actual uniform point masses are1/|G|, event probabilities |A|/|G| and character coefficients1/0 at trivial/nontrivial modes. Probability normalization removes the trivial mode. [pcc:unverified]'),
 p('<b>Actual independent walk.</b> The preceding actual PMF character-moment law gives halfL1(walk,uniform)² ≤ one quarter of the sum over nontrivial characters of normSq(Σᵢqᵢψ(aᵢ))^t, for every finite abelian group, time and start. The same right side bounds every event’s squared discrepancy from |A|/|G|. No supplied convolution-power or spectral formula is used. [pcc:unverified]'),
 p('<b>Normalization counterexample and sharpness.</b> Actual ZMod2 pointmass0 versus uniform has fullL1=1, halfL1=1/2 and nontrivial energy1. Lean directly refutes using the quarter factor with fullL1, proves equality with halfL1 and shows any universal halfL1 constant on this group must be at least1/4. Original1981 uses fullL1; author1987 and Hildebrand use halfL1. Their qualified statements are retained. [pcc:unverified]'),
 p('<b>Primary review.</b> <link href="https://purl.stanford.edu/ps020md6873" color="blue">Diaconis author report269, April1987</link>: selected chapter3B printed29–33/PDF35–39 OCR read; visual29/32/33/contents, primary hash checked; not the1988 book pagination or full271-page read. Published Diaconis–Shahshahani1981: selected normalization/convolution/Plancherel and full Lemma14 proof168 read; visual159/161/162/168, not full21-page read. Hildebrand1994 full13-page review credited and195 rechecked; later preprint selected4.1/4.2 only. Raw primary PDFs/scans/OCR remain local, metadata retained. [pcc:unverified]'),
 p(f'<b>Checks and remainder.</b> Whole gate: {g["declarations"]} declarations/0 failures/0 trusted native exceptions. All {a["files"]} build/provenance files retained losslessly in {a["unique_objects"]} objects, including first gate and dry-binding correction; final five gate files and recursive pins. Lean4.35.0/Mathlib56a00585…, standard foundations/default limits. Signed standing unverified. Nonabelian matrix/measure-tvDist interfaces and random-support spectral averaging remain open. No holes/custom axioms/native evaluation/raised limits/global flags/waivers/native experiment/spend/upload. [pcc:unverified]')]

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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/finite-fourier-upper-bound-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
