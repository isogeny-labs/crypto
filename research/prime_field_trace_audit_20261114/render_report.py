"""Layout recorded formal results and provenance; no research computation."""
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
d=Drawing(500,109);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="109" viewBox="0 0 500 109">']
for yy,title,lines in [(59,'All F5 models',['Normalize → 25 short pairs → actual Hasse','Source SS ↔ trace 0 ↔ #E = 6; r > 2 → order 2']),(5,'All F7 models',['Normalize → 49 short pairs → actual Hasse','Source SS ↔ trace 0 ↔ #E = 8; r > 2 → order 2'])]:
 d.add(Rect(0,yy,500,45,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{109-yy-45}" width="500" height="45" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+24,title,9.5),(112,yy+28,lines[0],8.4),(112,yy+11,lines[1],8.2)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{109-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'prime-field-trace.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('Actual F5/F7 trace bounds and composite-order controls',t),
 p('<b>Result.</b> Lean proves actual Hasse and source trace-zero criteria for every nonsingular general Weierstrass model over F5 and F7. The exact source predicate gcd(q+1−#E,q)≠1 is equivalent to actual point count q+1. For source-SS models, every count divisor is coprime to q and has modular order dividing two; if r&gt;2, the order is exactly two, including composite r. No general Hasse or geometric supersingularity classification premise is assumed. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>Actual model coverage.</b> An explicit unit-u1 coordinate shift eliminates a1,a2,a3 when two and three are nonzero. Discriminants agree. The preceding actual additive point-group equivalence is reused, including identity, inverse, secant and tangent cases. Actual point cardinality, integer trace and source predicate transport follow. Closed kernel tables cover all 25/49 short coefficient pairs; normalization supplies the universal general-model quantifier. Prior native all-model censuses and conditional trace-zero/modular arithmetic are credited. [pcc:unverified]'),
 p('<b>Composite positive controls.</b> F5:y²=x³+1 has six actual points; P=(2,2) has exact order six and orderOf(5mod6)=2. F7:y²=x³+x has eight actual points; P=(3,3) has exact order eight and orderOf(7mod8)=2. All proper positive multiples and final zero multiples are checked in the Lean kernel. In both fields the modular order modulo two is one. No cyclic point order is inferred from count divisibility alone; the F5 model is credited to the earlier S3/r=2 controls. [pcc:unverified]'),
 p('<b>Negative control.</b> The F5 singular cusp y²=x³ has discriminant zero, raw equation count plus identity six, and actual nonsingular point-group cardinality five. Its actual trace predicate is false while the raw counted predicate is true. Lean directly negates the universal counting bridge after deleting nonsingularity. This tests the hypothesis; it does not refute an elliptic-curve theorem. [pcc:unverified]'),
 p('<b>Primary review.</b> <link href="https://link.springer.com/content/pdf/10.1007/s00145-009-9048-z.pdf" color="blue">FST2010 pp230/237, Definition2.1/Remark2.2/Section3</link>: trace predicate, embedding conditions and p≥5 qualification retained. <link href="https://www.jmilne.org/math/Books/EC2.pdf" color="blue">Milne II1.5/1.6 and II2/Theorem2.1, printedp50/PDF55</link>: coordinate elimination and actual group transport; PDF55 rendered and fully inspected. Official Mathlib NormalForms and prior shiftEquiv credited. Selected sections read; no full-paper/book read or corpus promotion. KB/Conductor unavailable, bounded fallback recorded. [pcc:unverified]'),
 p(f'<b>Validation and scope.</b> Whole gate {g["declarations"]} declarations, zero failures, zero trusted native exceptions. All {a["files"]} build/provenance files retained losslessly in {a["unique_objects"]} objects, including failed source/config/log/exit states; initial setup path error recorded separately. Complete statement and recursive source/config pins. Pinned Lean4.35.0/Mathlib56a00585…, only standard foundations. Exact local PCC replay differs from signed standing. General prime/extension-field Hasse, trace/geometric/Endrank identifications beyond preceding bounded cohorts, field-degree/full-torsion-field bridges and other original fourteen-item gaps remain open. No qualified-source refutation, novelty, new native experiment, paid compute, agent upload, holes/custom axioms/native evaluator/raised limits/new global flags/waivers. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/prime-field-trace-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
