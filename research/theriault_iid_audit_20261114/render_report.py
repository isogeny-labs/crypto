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
label(4,119,'Three draws, eight elements: paired orbits versus unpaired colors',10)
label(4,85,'Paired expectation');bar(79,220,'#2c788e');label(390,85,'11/16')
label(4,48,'Unpaired expectation');bar(42,115,'#607e96');label(285,48,'23/64')
label(4,12,'Actual uniform finite sums; exact rational values, no statistical uncertainty.',8.4)
svg.append('</svg>');(o/'iid-bridge.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text())
flow=[Paragraph('Actual iid sampling: recurrence and repaired bound',t),p('<b>Result.</b> Lean derives Thériault’s collision recurrence and closed form from actual uniform finite samples, then proves the corrected quadratic bounds directly. The recurrence is a conclusion, not a hypothesis. The original finite lower bound has a source-domain counterfamily with arbitrarily large even pools at three draws. Signed standing unverified. [pcc:unverified]'),d,Spacer(1,12),p('<b>Exact sampling proof.</b> For every finite nonempty color type of cardinality m and every sample size s, the actual sample type is all functions Fin s→colors, equally weighted. The natural repeat count is s minus the number of distinct sampled colors; support cardinality≤s justifies its rational cast. Functions avoiding a fixed color are equivalent to functions into the complement subtype, so their number is (m−1)ˢ. Summing missing-color indicators and interchanging the two finite sums yields E=s−m+m(1−1/m)ˢ. No probabilistic approximation or recurrence premise enters this derivation. [pcc:unverified]'),p('<b>Actual paired model.</b> An explicit function-to-product equivalence proves uniform product draws preserve the projected color average for every finite nonempty nuisance-label type. With Boolean signs, the pool has n=2m elements, negation flips the sign, has no fixed points, and equal colors characterize equality up to negation. The actual expectation equals the source closed form (n/2)(1−2/n)ˢ+s−n/2 and satisfies E₀=0, Eₛ₊₁=(1−2/n)Eₛ+2s/n. [pcc:unverified]'),p('<b>Direct repair and family.</b> Applying the byte-identical prior recurrence-bound module now gives actual 4s²/(9n)≤Eₛ≤s²/n whenever 3≤s≤n/2. The actual pools Fin(t+4)×Bool have unbounded cardinalities n=2(t+4). At s=3 each satisfies the strict source domain and violates the printed2s²/(3n) lower bound. Paired eight-element draws give11/16; treating the same number of elements as eight distinct colors gives23/64, a negative control against forgetting the negation quotient. The asymptotic runtime theorem is not refuted by this finite constant error. [pcc:unverified]'),p('<b>Sources and validation.</b> N.Thériault, ASIACRYPT2003,LNCS2894,75–92, DOI10.1007/978-3-540-40061-5_5, relevant definitions and original pp85–86 inspected in the preceding immutable packet. Pinned Mathlib finite-cardinality/subtype/function-equivalence/sum APIs read. Prior source review and recurrence module provenance are retained. Scalar supplement IDC1h939108711fe75407 is reused from its original immutable source, with the same symbolic cubic proof and official32+32-point check; no new native issuance. KB unavailable, bounded primary/formal fallback recorded. [pcc:unverified]'),p(f'<b>Assurance and boundary.</b> Whole gate {g["declarations"]}/0failures/0trusted; Lean4.35.0/Mathlib56a00585… pinned. Complete source binding, custom source/config/artifact pins, failed/successful attempts, SVG and this inspected PDF retained. Only propext/Classical.choice/Quot.sound; no holes, custom axioms, native evaluator, raised limits, global flags or waivers. Exact PCC replay and signed standing are separate. Actual algorithm walks and Jacobian/subgroup distributions are not proved iid; full runtime/rank/invertibility remain open. No full-paper read, corpus promotion, novelty claim, native experiment, paid compute or upload. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595.276,841.89),leftMargin=47,rightMargin=47,topMargin=35,bottomMargin=35).build(flow)
doc=fitz.open(o/'report.pdf');layout={'pages':[],'visual_inspection':'pending'}
for i,page in enumerate(doc):
 bad=[]
 for block in page.get_text('dict')['blocks']:
  for line in block.get('lines',[]):
   for span in line['spans']:
    x0,y0,x1,y1=span['bbox']
    if x0<0 or y0<0 or x1>page.rect.width or y1>page.rect.height:bad.append(span['text'])
 layout['pages'].append({'page':i+1,'text_within_page':not bad,'out_of_bounds':bad});page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/theriault-iid-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n');print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
