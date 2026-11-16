"""Lay out already checked formal claims and source evidence; no experiment."""
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
b=ParagraphStyle('b',fontName='DV',fontSize=8.8,leading=12,spaceAfter=9);t=ParagraphStyle('t',fontName='DVB',fontSize=16,leading=20,spaceAfter=12)
def p(s):return Paragraph(s,b)
d=Drawing(500,127);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="127" viewBox="0 0 500 127">']
def label(x,y,s,size=8.5):
 d.add(String(x,y,s,fontName='DV',fontSize=size,fillColor=colors.HexColor('#234b70')));svg.append(f'<text x="{x}" y="{127-y}" font-family="DejaVu Sans,sans-serif" font-size="{size}" fill="#234b70">{html.escape(s)}</text>')
def box(x,y,w,h):
 d.add(Rect(x,y,w,h,rx=4,fillColor=colors.HexColor('#edf3f8'),strokeColor=colors.HexColor('#8eb0cb')));svg.append(f'<rect x="{x}" y="{127-y-h}" width="{w}" height="{h}" rx="4" fill="#edf3f8" stroke="#8eb0cb"/>')
label(9,114,'Actual F2 obstruction: P1 has3 points; each torus has1 point.',9.2)
for x,s in[(4,'[1:0]'),(173,'[1:1]'),(342,'[0:1]')]:
 box(x,64,151,35);label(x+55,77,s,10)
box(4,8,489,42);label(12,34,'Three injective charts: [1:u], [1:u−1], [u−1:1] (u≠0).',9);label(12,18,'Products need3^n charts overF2. Proved overhead: 3^n ≤5n!.',8.7)
svg.append('</svg>');(o/'chart-repair.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text())
flow=[Paragraph('Diem’s torus charts: disproof and repair',t),p('<b>Source-helper failure.</b> Original Diem2011 Proposition2.5(b), printedp81, uses a two-copy multiplicative-group cover of the projective line for every primepowerq. Lean disproves the rational-point consequence overF2 using actual Mathlib Projectivization(F2,F2×F2), not an assumed three-element model. Its cardinality is3; actual F2units have cardinality1. Any two maps fail to cover. All positive products also defeat2^ncharts. Signed standing unverified. [pcc:unverified]'),d,Spacer(1,9),p('<b>Exact qualification.</b> For every finite fieldK, two rational-point maps from K-units cover the actual projective line if and only if cardK≥3. Necessity follows from actual surjection/cardinality bounds. Sufficiency choosesa≠0,1 and proves that [1:u] and [u−1:u−a] are injective and cover. This includes fields of characteristic2 with more than2elements; characteristic2alone is not the obstruction. [pcc:unverified]'),p('<b>Uniform repair.</b> Three explicit injective rational-point charts cover over every field: [1:u], [1:u−1], [u−1:1], withu nonzero. Actual projective induction and nonzero-scaling equivalence prove the cover, including both coordinate-axis points. Product covers hold at everyarity. OverF2, any cover fromm actual n-tori requires m≥3^n, even allowing arbitrary maps. [pcc:unverified]'),p('<b>Call-count allowance.</b> The actual chart-index type has3^nelements, and Lean proves3^n≤5n! for every naturaln. For d>0 and B=n!d^n, it proves3^nB≤5B². Thus the repaired chart count has a polynomial allowance in the stated size parameter. This is arithmetic accounting, not a formal proof of Rojas’ runtime or the full decomposition algorithm. [pcc:unverified]'),p('<b>Primary-source check.</b> C.Diem, Compositio Math147(2011),75–104, DOI10.1112/S0010437X10005075. Read notation/setup p78, Definition2.4 and Proposition2.5 pp80–81, its proof and laterCondition2.7; printedp81 visually inspected in full. Proposition2.5 explicitly acceptsq2; laterCondition2.7 is not its hypothesis. PDF878749B, SHA74902b58…a934a58a. Exact full corpus node THM-SP-DIEM11-DECOMPOSITION-ALG states the bound/main algorithm and is not refuted. No full-paper read or novelty claim. [pcc:unverified]'),p(f'<b>Checks and evidence.</b> Whole gate {g["declarations"]} declarations /0 failures /0 trusted native exceptions; Lean4.35.0, Mathlib56a00585…. Closed declaration, all custom source/config and evidence/report hashes are pinned. All failed/successful source/log/exits and first no-sources gate failure are retained losslessly. Only propext/Classical.choice/Quot.sound; no holes, custom axioms, native evaluator, raised limits, new global flags or waivers. Exact PCC replay is separate; signed standing unverified. [pcc:unverified]'),p('<b>Remaining scope.</b> The literal source cover step fails. The main geometric Bezout bound and decomposition theorem are not refuted. Explicit rational-point formulas and chart arithmetic do not formalize scheme open embeddings, multihomogeneous substitution, isolated-point multiplicities, toric resultants or the full algorithm runtime. Existing solver-length/support evidence remains unchanged. KB unavailable; bounded source/formal fallback retained. No new native experiment, paid compute or agent upload. Other all-fourteen obligations remain open at their recorded scopes. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595.276,841.89),leftMargin=47,rightMargin=47,topMargin=35,bottomMargin=35).build(flow)
doc=fitz.open(o/'report.pdf');layout={'pages':[],'visual_inspection':'pending'}
for i,page in enumerate(doc):
 bad=[]
 for block in page.get_text('dict')['blocks']:
  for line in block.get('lines',[]):
   for span in line['spans']:
    x0,y0,x1,y1=span['bbox']
    if x0<0 or y0<0 or x1>page.rect.width or y1>page.rect.height:bad.append(span['text'])
 layout['pages'].append({'page':i+1,'text_within_page':not bad,'out_of_bounds':bad});page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/diem-chart-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n');print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
