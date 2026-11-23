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
rows=[(96,'Exact finite core',['Actual uniform conditional injective support law','Probability ≤ (W+1)^d / (p−1).descFactorial d']),
      (50,'Derived asymptotics',['d≥2: W→∞ and actual denominator / p^d → 1','Exact W^d = ((ε/2)/2^d)·p^(d−1); correction → 1']),
      (4,'Corrected result',['Uniform in cell shifts, a₀ and nonzero index j','For all sufficiently large primes: probability ≤ 1.1·(ε/2)/(2^d·p)'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.8),(130,yy+25,lines[0],7.8),(130,yy+9,lines[1],8.1)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'corrected-box-asymptotic.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('Hildebrand: corrected box asymptotic proved',t),
 p('<b>Result.</b> Lean proves the corrected author-preprint Lemma2 for k=d+1≥3. For every fixed d≥2 and ε>0, there exists a threshold such that every sufficiently large prime has actual uniform conditional box probability ≤1.1·(ε/2)/(2^d·p). The threshold is independent of all integer cell shifts, first support point a₀ and nonzero Fourier index j. [pcc:unverified]'),d,Spacer(1,6),
 p('<b>Actual source and literal formulas.</b> The sampler covers all injective (d+1)-point field supports with fixed first point. Its actual cardinality is (p−1).descFactorial d; its mapped PMF equals the uniform law on the full conditional space. Source coordinates use the actual canonical centered representative of (a₀−aᵢ)·j. Lean binds explicit identities with the printed real-power endpoints mᵢ·(ε/2)^(1/d)·p^((d−1)/d)/(2p) and the corresponding mᵢ+1 upper endpoint. [pcc:unverified]'),
 p('<b>Limits are derived.</b> Actual factorial normalization equals the product of factors1−i/p and tends1. The unnormalized width W=(ε/2)^(1/d)·p^((d−1)/d)/2 tends to infinity when d≥2 and satisfies the exact dth-power identity. Hence (1+1/W)^d divided by the normalized denominator tends1. An exact finite-ratio identity and eventual strict bound11/10 yield the printed1.1 constant. No asymptotic, probability, cardinality or mixing premise is supplied. [pcc:unverified]'),
 p('<b>Known k=2 failure retained.</b> Lean proves W(1,ε,p)=ε/4 and directly negates width divergence. The preceding actual k2 cell counter remains in the main certificate: ε=1/m₂=4/a₁=0/j=1 gives probability1/(p−1)>11/(40p) for every prime p≥3, with direct negation of any eventual prime threshold. The later author preprint acknowledges the older proof failure and treats k=2 separately; no novelty claim is made. Qualified mixing theorems are retained. [pcc:unverified]'),
 p('<b>Sources and remainder.</b> <link href="https://arxiv.org/abs/math/0401189" color="blue">Author preprint math/0401189v1</link>: selected section4.2/pages13–16/reference14p36 read, visual15–16 only; corrected Lemma2 explicitly requires k≥3. <link href="https://link.springer.com/content/pdf/10.1007/BF01199265.pdf" color="blue">Hildebrand1994</link>: full13-page original read/inspection credited; proof197 rechecked. Official Mathlib source APIs and preceding finite-counting proof read. The stronger expected random-support mixing exponent still needs the spectral argument. [pcc:unverified]'),
 p(f'<b>Checks.</b> Whole gate: {g["declarations"]} declarations, zero failures, zero trusted native exceptions. All {a["files"]} build/processing/provenance files retained losslessly in {a["unique_objects"]} objects, five gate files and recursive source/config pins. Lean4.35.0/Mathlib56a00585…, standard foundations/default limits. Signed standing unverified; KB/Conductor fallback recorded. No holes/custom axioms/native evaluation/raised limits/global flags/waivers/native experiment/paid compute/upload. [pcc:unverified]')]

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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/hildebrand-box-asymptotic-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
