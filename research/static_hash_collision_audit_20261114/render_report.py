"""Layout proved algebra statements and provenance; no research computation."""
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
b=ParagraphStyle('b',fontName='DV',fontSize=9,leading=12.3,spaceAfter=11)
t=ParagraphStyle('t',fontName='DVB',fontSize=16.5,leading=21,spaceAfter=13)
def p(s):return Paragraph(s,b)
d=Drawing(500,142);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="142" viewBox="0 0 500 142">']
rows=[(96,'Actual models',['Static labels: one independently sampled label per state','Chronological labels: independent choices at each time']),
      (50,'First collision',['Survival probabilities agree at every finite time','Actual E[first repeat] = sum of iid self-avoidance tails']),
      (4,'Boundary control',['Z3, steps1,2: word0,1,1 revisits0 with conflicting labels','Static word mass0; iid mass1/8; unrestricted law false'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.4),(122,yy+25,lines[0],7.8),(122,yy+9,lines[1],7.8)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'static-hash-collision.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0]['full_decl_sha256'];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Static random hashes: exact first-collision law',t),
 p('<b>Result.</b> Lean proves that an arbitrary finite state update driven by independently sampled static vertex labels has exactly the same first-collision survival probabilities as a chronological iid label walk stopped when a state repeats. The proof uses actual deterministic paths and actual product PMFs, without a survival/mixing/collision conclusion supplied as a premise. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>Independent queries are derived.</b> A prescribed finite word determines an actual state path. Lean proves that a static hash produces this word exactly when its labels agree at the corresponding queried states. If the path is self-avoiding, these states are distinct. An arbitrary finite injective-query cylinder theorem derives the word probability as the product of label weights, using actual coordinate gates, products of sums and extension by one. Independence is not asserted for a repeated state. [pcc:unverified]'),
 p('<b>Exact survival and expectation.</b> Decomposing the survival event into disjoint chronological word events gives equality with the actual iid self-avoidance mass at every time, for all finite state/label types and normalized nonnegative weights. On every nonzero modulus p, Lean links the unchanged actual Nat.find least-repeat definition to prefix injectivity. Its tail probability is the iid self-avoidance mass; its expectation is the finite sum of those tails for times0 through p-1. Any state rule and starting state are allowed, so adding walks are included. [pcc:unverified]'),
 p('<b>Direct disproof beyond the stopping boundary.</b> The actual prime-order group Z3 has distinct nonzero increments1,2 and strictly positive uniform labels. Word0,1,1 would move0→1→0, then require label1 at state0 although its first label was0. Lean proves this word impossible under every static hash, with static mass0 and iid mass1/8, and directly negates the unrestricted chronological word-mass identity. Kernel-checked finite decisions supply the small control. [pcc:unverified]'),
 p('<b>Boundary and prior work.</b> This closes a first-collision model bridge. It does not prove the limiting random-increment-table penalty, practical fixed/keyedhash refinement or compiler execution. Bernstein–Lange2013 already identifies higher-degree local anticollisions beyond the degree-one factor; those asymptotic heuristic corrections are credited and are not proved here. A finite counterexample is not presented as an asymptotic refutation. [pcc:unverified]'),
 p(f'<b>Checks.</b> Whole new-project gate{g["declarations"]}/0 failures/0 trusted native, only propext/Classical.choice/Quot.sound. Complete1421-character binding{binding}, independent native1complete/0prefix/0unresolved; procedural parser/hash evidence, not kernel refinement. Eight complete build snapshots including six failures: {a["files"]}files/{a["unique_objects"]}objects/{size}-byte lossless XZ, plus final gate files. [pcc:unverified]'),
 p('<b>Sources and assurance.</b> Galbraithch14v2 full selected printed288/296/297 (PDF4/12/13) and Bernstein–Lange Two grumpy giants and a baby full selected94–97 (PDF9–12) read/visually inspected. No fullpaper-read or novelty claim. Prior actual hash-PMF/moment and least-repeat proofs reused unchanged; official Mathlib finite products/extension/iteration/find APIs checked. KB/Conductor unavailable, primary/prior/official fallback. Lean4.35.0/Mathlib56a00585…, defaults, no holes/custom axioms/native evaluator/global flags/raised limits/waivers/new crypto experiment/spend/upload. Signed standing unverified. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/static-hash-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
