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
rows=[(96,'Actual independent laws',['Fair Boolean PMF, fresh bind at each step','Both concrete coupling marginals equal the walk']),
      (50,'Exact potential drift',['Difference in Z/n; V = k(n − k), V ≤ n²/4','Coalescence + expected drop 1 until agreement']),
      (4,'Finite-time conclusion',['Every-event distance to uniform ≤ n²/(4t)','Any two distinct equalweight increments over Fp'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.8),(130,yy+25,lines[0],7.8),(130,yy+9,lines[1],8.1)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'cycle-coupling-bound.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('Independent cyclic walks: exact coupling and mixing',t),
 p('<b>Result.</b> Lean proves that the actual independent half-stay/half-add-one walk on every cycle Z/n, n≥2, has every-event distance to uniform at most n²/(4t) for t&gt;0. For every prime p and any two distinct equalweight increments, the same bound holds with n=p. No supplied coupling, stationarity or mixing premise; no enumeration cutoff. [pcc:unverified]'),d,Spacer(1,6),
 p('<b>Actual laws and proof.</b> The normalized fair Boolean Mathlib PMF is mapped and bound afresh at each step. Its real finite expectation equals the explicit averaging operator. The concrete coupling moves only one particle before agreement, and moves both together afterward. Lean checks both marginal expectations, modular neighbor values, potential V=k(n−k), exact expected drop1 before agreement, coalescence and V≤n²/4. Telescoping bounds disagreement; pointwise event inequalities and actual finite translation stationarity yield the uniform event bound. [pcc:unverified]'),
 p('<b>Prime two-support transfer.</b> The exact expectation identity uses time-dependent affine coordinates x+t·a+(b−a)·z. For distinct a,b in Fp this is an actual permutation, preserving event cardinality and uniform comparison. A fixed transition-kernel conjugacy is not assumed. This proves a quantitative k=2/equalweight case of the qualified mixing claim. [pcc:unverified]'),
 p('<b>Direct negative controls.</b> On the two-cycle, time0 gives singleton distance1/2 and the displayed zero-denominator expression evaluates to0, so the zero-time extension is false. The actual deterministic add-one PMF has singleton event distance1/2 at every time; Lean directly negates eventual distance≤1/4. Strictly positive weights in the primary theorem remain intact. [pcc:unverified]'),
 p('<b>Credited primary method and scope.</b> <link href="https://link.springer.com/content/pdf/10.1007/BF01199265.pdf" color="blue">Hildebrand1994, Theorems1–2 and proof2</link>: selected191–193/199–200, independent increments, random distinct supports versus uniform quadratic bound. <link href="https://pages.uoregon.edu/dlevin/MARKOV/markovmixing.pdf" color="blue">Levin–Peres, contributions Wilmer, second edition</link>: selected Proposition4.7, Theorem5.4/Corollary5.5, cycle section5.3.2 and Example10.27 context; printed50/62/63/64 visually inspected. Established coupling/potential method credited, not a full-book read or novelty claim. Paper event normalization is used; no identification with Mathlib’s differently normalized tvDist is asserted. [pcc:unverified]'),
 p(f'<b>Checks and remainder.</b> Wholegate{g["declarations"]}/0failures/0trusted; all{a["files"]} build/processing/provenance files retained losslessly in{a["unique_objects"]}objects, fivegate files and recursive source/config pins. Lean4.35.0/Mathlib56a00585…, standard foundations/default limits. Signed standing remains unverified. General unequal weights, k≥3/random-support asymptotics, static rho mixing and collision constants remain open. KB/Conductor unavailable; bounded fallback recorded before code. No holes/custom axioms/native evaluation/raised limits/global flags/waivers/native experiment/paid compute/upload. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/cycle-coupling-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
