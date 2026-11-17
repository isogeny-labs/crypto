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
rows=[(96,'Weighted labels',['I2 = sum p_i²; I4 = sum p_i⁴; actual admissible label sum','Degree-two coefficient = 1 - I2 - I2² + I4']),
      (50,'Actual good table',['Z7; steps1,3; 128static hashes x49ordered start pairs','One-step count448/rate1/14; fresh two-step336/rate3/56']),
      (4,'Hypothesis control',['Steps1,2: still distinct/nonzero, violates degree condition','Fresh two-step count112/rate1/56; omitted guard false'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.4),(122,yy+25,lines[0],7.8),(122,yy+9,lines[1],7.8)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'two-step-collision.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0]['full_decl_sha256'];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Degree-two local collisions: exact finite checks',t),
 p('<b>Result.</b> Lean proves the normalized weighted admissible-label coefficient1-I2-I2²+I4, and checks its actual fresh two-step collision probability on the complete Z7 random-hash population with steps1,3. A second distinct/nonzero table1,2 violates the source degree condition and directly disproves removing it. This checks a known Bernstein–Lange2013 correction; no novelty or limiting collision constant is claimed. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>Generic label identity.</b> Sum the products p_i p_j p_k p_l over different final labels j,l, excluding oppositely ordered copies of the same two steps. For every finite real weight list summing to1, Lean derives1-I2-I2²+I4 by explicit label exclusions and off-diagonal finite sums. This is an actual weighted sum identity, not a coefficient assumption or native oracle. The conversion to actual graph probabilities is checked separately at the stated finite tables. [pcc:unverified]'),
 p('<b>Actual graph and probability law.</b> The state map is x→x+g(h(x)) on Z7, with every one of the128 maps h:Z7→Fin2 and all49 ordered starts. A fresh two-step event requires all four queried states x,y,F(x),F(y) distinct and F²(x)=F²(y). Lean checks degree0/1/2 sum distinctness modulo commutativity for g=(1,3). Kernel enumeration gives one-step448 and fresh two-step336 among6272 samples. A genuine PMF has pointmass equal to the product of seven uniform vertex-label factors and two uniform-start factors. Its exact event rates are1/14 and3/56, and3/56 equals the proved uniform coefficient3/8 divided by7. [pcc:unverified]'),
 p('<b>Direct negative control.</b> The table g=(1,2) remains injective and nonzero but fails the degree condition:1+1=2 mixes degrees. Its actual fresh two-step count is112 and rate1/56. Lean directly negates the formula after replacing the source degree qualification by injectivity/nonzero alone. The two actual good-table event rates also differ; a degree-one local coefficient does not equal the checked degree-two coefficient. No qualified source theorem is refuted. [pcc:unverified]'),
 p('<b>Boundary.</b> The generic label identity and actual finite graph checks do not prove the general actual-group/support probability coefficient, a limiting random-increment-table firstcollision penalty or practical keyedhash behavior. The separate exact static-hash/iid firstcollision survival bridge remains unchanged. Source higher-degree asymptotic heuristic corrections are credited, not promoted to proved limiting results. [pcc:unverified]'),
 p(f'<b>Checks and retained failure.</b> Whole final gate{g["declarations"]}/0 failures/0 trusted native; standard propext/Classical.choice/Quot.sound only. Complete1347-character binding{binding}, independent native1complete/0prefix/0unresolved. Initial native parser failed on scoped ENNReal notation; equivalent explicit type name rebuilt/regated, frozen parser unchanged. Seven build snapshots/four failed builds, both native inputs/protocols and both gates retained: {a["files"]}files/{a["unique_objects"]}objects/{size}-byte lossless XZ plus final gate files. [pcc:unverified]'),
 p('<b>Sources and assurance.</b> Bernstein–Lange Two grumpy giants and a baby2013§§3–4, full selected printed94–97/PDF9–12 read/visually inspected; degree≤2 and distinct-query qualifications retained. Galbraithch14v2 selected288/296–297 and prior actual variance/firstcollision studies credited. Official Mathlib finite sums/PMF/ENNReal APIs checked. KB/Conductor unavailable, primary/prior/official fallback; no fullpaper-read claim. Lean4.35.0/Mathlib56a00585…, defaults; all finite counts by kernel decisions, no native evaluator/holes/custom axioms/flags/limits/waivers/new crypto experiment/spend/upload. Signed standing unverified. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/two-step-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
