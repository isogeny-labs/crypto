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
rows=[(96,'Actual sampler',['Uniform injective d-tuples; d ≤ p−1','Full conditional supports: cardinality (p−1).descFactorial d']),
      (50,'Exact finite bound',['Nonzero j: coordinate count ≤ p·(U−L)+1','Probability ≤ product of counts / falling factorial']),
      (4,'Negative control',['Index j=0, d=1, L=U=0: probability 1','For p≥3, this exceeds deleted-index bound 1/(p−1)'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.8),(130,yy+25,lines[0],7.8),(130,yy+9,lines[1],8.1)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'injective-box-bound.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('Hildebrand: exact finite box counting',t),
 p('<b>Result.</b> Lean proves the finite counting core for genuine uniform sampling without replacement. The source box probability is at most the product of coordinate bounds p·(U−L)+1 divided by (p−1).descFactorial d, for every prime p, d≤p−1 and nonzero Fourier index j. The actual probability law, cardinality and event are bound in the main theorem. [pcc:unverified]'),d,Spacer(1,6),
 p('<b>Sampler and normalization.</b> Actual injective d-tuples from a finite population X have cardinality (card X).descFactorial d. Lean derives actual uniform point masses, the exact event/cardinality ratio and a coordinate-product upper bound by injection into the full coordinate product. No independence premise is assumed. If d exceeds the population, Lean proves that no PMF on the sample space exists. [pcc:unverified]'),
 p('<b>Full source space and geometry.</b> An explicit equivalence identifies injective tuples of nonzero prime-field elements with all injective support maps with fixed first point a₀. The mapped sampler equals the actual uniform law on this full conditional space. Its source expression (a₀−aᵢ)·j is carried through the equivalence. For j nonzero, transformed canonical centered residues form an injective integer image. Actual finite integer-interval counting gives width+1; the lattice correction is retained. [pcc:unverified]'),
 p('<b>Deleted-index negative control.</b> For every prime p≥3, j=0 makes each transformed centered coordinate zero. A zero-width coordinate box has p−1 choices, exceeding its width-plus-one bound1. For d=1, its actual full-source event probability equals1 and is strictly greater than1/(p−1). Lean directly negates the probability inequality with the nonzero-index hypothesis deleted. This does not refute the qualified lemma. [pcc:unverified]'),
 p('<b>Source review and remaining work.</b> <link href="https://link.springer.com/content/pdf/10.1007/BF01199265.pdf" color="blue">Hildebrand1994, proof p197</link> supplies the exact product(width+1) numerator and falling-factorial denominator; full13-page original read and inspection credited. <link href="https://arxiv.org/abs/math/0401189" color="blue">Author preprint math/0401189v1</link>, selected section4.2/pages13–16/reference14p36, restricts the later asymptotic lemma to k≥3; visual15–16 only. The 1.1 asymptotic still needs width-growth and denominator estimates; the stronger expected random-support mixing exponent remains open. [pcc:unverified]'),
 p(f'<b>Checks.</b> Whole-project gate: {g["declarations"]} declarations, zero failures, zero trusted native exceptions. All {a["files"]} build/processing/provenance files retained losslessly in {a["unique_objects"]} objects, plus five gate files and recursive source/config pins. Lean4.35.0/Mathlib56a00585…, standard foundations/default limits. Signed standing unverified. KB/Conductor unavailable; bounded fallback recorded before code. No holes/custom axioms/native evaluation/raised limits/global flags/waivers/native experiment/paid compute/upload. [pcc:unverified]')]

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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/injective-box-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
