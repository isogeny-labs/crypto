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
rows=[(96,'Literal original cell',['k=2, ε=1, m₂=4, a₁=0, j=1; prime p≥3','Canonical centered residue / p lies in [1/p, 5/(4p)]']),
      (50,'Actual probability',['Full uniform nonzero choices; cell is a₂=−1','Probability 1/(p−1) > printed bound 11/(40p)']),
      (4,'Scope and repair',['No sufficiently-large-prime threshold fixes this cell','Author acknowledged k=2 issue; valid mixing retained'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.8),(130,yy+25,lines[0],7.8),(130,yy+9,lines[1],8.1)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'hildebrand-cell-counter.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('Hildebrand: the acknowledged k=2 cell boundary',t),
 p('<b>Result.</b> Lean refutes the original printed Lemma4 at fixed k=2, ε=1, m₂=4, a₁=0 and j=1. For every prime p≥3, its actual uniform cell probability is1/(p−1), strictly greater than the printed bound11/(40p). Lean directly rules out any sufficiently-large-prime threshold for this specialization. This is a formalization of an author-acknowledged boundary issue, with no novelty claim or refutation of valid mixing Theorems1/2. [pcc:unverified]'),d,Spacer(1,6),
 p('<b>Faithful source distribution.</b> Actual nonzero elements of Fp have proved cardinality p−1. A genuine normalized uniform Mathlib PMF is constructed, together with an actual equivalence to all injective two-point supports whose first value is0. Its pushforward is proved uniform on that full conditional space. The actual source expression (a₁−a₂)·j equals−a₂. No selected support subclass or supplied probability/cardinality premise is used. [pcc:unverified]'),
 p('<b>Canonical representatives and exact event.</b> Lean proves congruence, half-open range(−p/2,p/2] and uniqueness for the actual centered representative. The literal real-power cell formulas and decimal1.1 reduce to endpoints1/p and5/(4p), with bound11/(40p). The cell is equivalent to centered residue1, hence exactly the actual choice a₂=−1. Uniform expectation gives1/(p−1). Euclid’s infinitude of primes closes the direct eventual-bound negation. [pcc:unverified]'),
 p('<b>Credited repair and positive control.</b> The later author preprint explicitly treats k=2 separately, restricts its counting lemma to k≥3 and flags the older failure as an exercise. The qualified two-support mixing result for any fixed positive normalized weights is included as a Lean positive control, using the preceding actual independent-law coupling proof; it does not depend on this invalid cell bound. Corrected k≥3 counting and mixing Theorems1/2 are retained. [pcc:unverified]'),
 p('<b>Primary review.</b> <link href="https://link.springer.com/content/pdf/10.1007/BF01199265.pdf" color="blue">Hildebrand1994, PTRF100,191–203</link>: full13 published pages read/visually inspected; original Lemma4p196/proof197 and k≥2 contextp195 rechecked. <link href="https://arxiv.org/abs/math/0401189" color="blue">Author preprint math/0401189v1</link>: selected section4.2/preprint13–16/reference14p36 read, pages15–16 visually inspected; not a full37-page read. Author corrections index found, attachment unread after recorded access failures; acknowledgement comes from the accessible preprint itself. [pcc:unverified]'),
 p(f'<b>Checks and remainder.</b> Wholegate{g["declarations"]}/0failures/0trusted; all{a["files"]} build/processing/provenance files retained in{a["unique_objects"]}objects, fivegate files and recursive source/config pins. Lean4.35.0/Mathlib56a00585…, standard foundations/default limits. Signed standing unverified. Stronger general random-support exponent and other14gaps remain open. KB/Conductor unavailable, fallback recorded before code. No holes/custom axioms/native evaluation/raised limits/global flags/waivers/native experiment/paid compute/upload. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/hildebrand-cell-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
