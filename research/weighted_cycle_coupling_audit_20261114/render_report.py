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
rows=[(96,'Actual arbitrary weights',['Normalized label PMF; product law = two iid binds','Equal-mass label-pair swap preserves both margins']),
      (50,'Exact scaled drift',['Normalized difference; V = k(p − k), V ≤ p²/4','Expected drop 2 w_a w_b until agreement']),
      (4,'Qualified source result',['Event distance ≤ p²/(8 w_a w_b t)','All supports; γ depends only on fixed weights and ε'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.8),(130,yy+25,lines[0],7.8),(130,yy+9,lines[1],8.1)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'weighted-cycle-bound.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('General weights: uniform quadratic mixing',t),
 p('<b>Result.</b> Lean proves the qualified uniform-support Theorem2 of Hildebrand1994: for every fixed finite k≥2 and strictly positive normalized weight vector, a positive γ depending only on weights and ε works over every prime modulus and every distinct support. At all integer times t≥γp², every event differs from uniform by less than ε. The constant is chosen before the modulus/support. [pcc:unverified]'),d,Spacer(1,6),
 p('<b>Actual probability and coupling.</b> A normalized NNReal vector constructs an actual Mathlib PMF, bound afresh for each increment. The actual product-label PMF equals two independent binds. A concrete transposition on the two selected ordered pairs preserves product mass and both marginal expectations. After agreement, increments are shared. The difference divided by the selected increment difference moves±1 with probability w_a·w_b each. Actual modular potential drift, coalescence, finite expectation identities, event inequalities and uniform stationarity are proved, not premises. [pcc:unverified]'),
 p('<b>Explicit bound and scope.</b> Two positive weights at distinct increments suffice for p²/(8w_aw_bt); γ=1/(4w_aw_bε) gives distance≤ε/2&lt;ε. The source-shaped wrapper retains all positive weights and support injectivity. The product coupling gives p²/(2t) at two equal halfweights; the prior antithetic packet retains its sharper p²/(4t). This component extends scope, with no sharper-constant claim. [pcc:unverified]'),
 p('<b>Direct prime-deletion counter.</b> Over Z/4, actual distinct increments0/2 with both weights1/2 stay in the actual two-element event{0,2}. Event probability1 versus uniform1/2 holds at every time. Lean directly rules out any positive quadratic time constant giving every event distance&lt;1/4. Sourcep192 already warns about composite-modulus subgroup confinement; no novelty claim. [pcc:unverified]'),
 p('<b>Primary review.</b> <link href="https://link.springer.com/content/pdf/10.1007/BF01199265.pdf" color="blue">Hildebrand1994, PTRF100,191–203</link>: all13 published pages/proofs/discussion/references read and all13 rendered pages visually inspected; Theorem2p193/proof199–200, definitions/warningp192. Established coupling/potential method credited to <link href="https://pages.uoregon.edu/dlevin/MARKOV/markovmixing.pdf" color="blue">Levin–Peres, contributions Wilmer</link>, selected Proposition4.7/Theorem5.4/Corollary5.5/cycle5.3.2/Example10.27 context. Full book not read. Paper event normalization used; no identification with differently normalized Mathlib tvDist. [pcc:unverified]'),
 p(f'<b>Checks and remainder.</b> Wholegate{g["declarations"]}/0failures/0trusted; all{a["files"]} build/processing/provenance files retained in{a["unique_objects"]}objects, including every failed state and compiler-panic log. Lean4.35.0/Mathlib56a00585…, standard foundations/default limits. Signed standing unverified. Stronger random-support Theorem1 exponent, static rho and collision constants remain open. KB/Conductor unavailable, bounded fallback before code. No holes/custom axioms/native evaluation/raised limits/global flags/waivers/native experiment/paid compute/upload. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/weighted-cycle-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
