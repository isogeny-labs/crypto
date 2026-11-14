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
rows=[(96,'Actual equivalence',['Fresh ordered pairs at terminal v <-> admissible labels','Four actual queries distinct from actual increment equations']),
      (50,'Derived expectation',['Actual product-hash cylinder probabilities + label sum','Expected pairs at v = c; all terminals = |G| c']),
      (4,'Actual probability',['I2=sum p_i²; I4=sum p_i⁴; c=1-I2-I2²+I4','Independent uniform ordered starts: fresh event mass = c / |G|'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.4),(122,yy+25,lines[0],7.8),(122,yy+9,lines[1],7.8)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'general-two-step-collision.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0]['full_decl_sha256'];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('General actual degree-two local collision probability',t),
 p('<b>Result.</b> Lean proves the exact actual fresh two-step collision probability(1-I2-I2²+I4)/|G| for every finite additive commutative group, finite labels, nonnegative normalized label law and source-qualified increment table. The actual joint probability law samples static vertex labels independently and two starting points uniformly and independently with replacement. This closes the previous general actual-group coefficient gap. Known Bernstein–Lange2013 local calculation credited, no novelty claim. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>Actual source conditions.</b> The unchanged source degree predicate checks actual increment equations: distinct nonzero steps, nonzero degree-two sums, no equality between degree-one and degree-two sums, and degree-two sum equality only from equal or swapped labels. These are structural conditions on the actual table. No query-independence, count, coefficient or probability conclusion is a premise. [pcc:unverified]'),
 p('<b>Actual pair-to-label equivalence.</b> At a fixed terminal v, the four labels determine actual preimages v-(g_i+g_j),v-g_j,v-(g_k+g_l),v-g_l. Lean derives their injectivity exactly for admissible labels, using the actual source equations. Conversely, a fresh colliding pair determines its labels through the actual static hash. The explicit pair/label equivalence proves both inverse identities from actual step and terminal equations. No abstract assumed collision table replaces the graph. [pcc:unverified]'),
 p('<b>Probability derived from the equivalence.</b> Actual subtype cardinalities give the finite indicator count identity. The unchanged arbitrary injective-query cylinder theorem supplies product label probabilities; the unchanged generic label identity supplies c=1-I2-I2²+I4. Thus the expected fresh-pair count at each terminal is c. Unique actual terminal decomposition gives total expected count |G|c. Lean constructs the genuine joint PMF, proves real pointmass equal to the product of all vertex-label weights divided by |G|², identifies the actual event measure, and derives c/|G|. [pcc:unverified]'),
 p('<b>Event and boundary.</b> The actual event requires x,F(x),y,F(y) pairwise distinct and F²(x)=F²(y). The theorem now applies to all finite abelian groups and source-qualified tables, beyond the prior complete Z7control. The earlier finite omitted-degree-guard counter remains unchanged. This is a finite local event probability; the limiting firstcollision penalty/constants, higher-degree asymptotic corrections and practical fixed/keyedhash refinement remain open. No qualified source theorem is refuted. [pcc:unverified]'),
 p(f'<b>Checks and provenance.</b> Whole new-project gate{g["declarations"]}/0 failures/0 trusted native, only propext/Classical.choice/Quot.sound. Complete777-character binding{binding}, independent native1complete/0prefix/0unresolved. Native source parsing/hash evidence is procedural, not kernel execution refinement. Seven build snapshots include both failed probability states: {a["files"]}files/{a["unique_objects"]}objects/{size}-byte lossless XZ plus five final gate files. Old dependencies are byte-identical to their frozen versions. [pcc:unverified]'),
 p('<b>Sources and assurance.</b> Bernstein–Lange Two grumpy giants and a baby2013§4printed96–97/PDF11–12 rechecked before approach; earlier full selected94–97/PDF9–12 read and visually inspected. Degree/query distinctness and ideal-random-hash qualifications retained; limiting-factor conversion remains heuristic. Prior generic cylinder/label proofs credited and reused unchanged; official Mathlib subtype/cardinality/finite-sum/PMF/ENNReal APIs read. KB/Conductor unavailable, primary/prior/official fallback; no fullpaper-read claim. Lean4.35.0/Mathlib56a00585…, defaults, legacy compatibility advisories retained; no holes/custom axioms/native evaluator/flags/limits/waivers/new crypto experiment/spend/upload. Signed standing unverified. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/general-two-step-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
