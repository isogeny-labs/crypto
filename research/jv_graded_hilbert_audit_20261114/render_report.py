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
rows=[(96,'Canonical pieces',['P_d / ker(actual quotient map restricted to P_d)','Kernel = I intersect P_d; firstisomorphism to actual image']),
      (50,'Derived graded basis',['Degree:          0          1             2             3','Representatives:1       x,y,h       xh,yh,h²         h³']),
      (4,'Exact dimensions',['Hilbert function:1,3,3,1,0,0,...; first zero degree4','Total scalar dimension/length8; HF(0)=1 differs from8'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.4),(122,yy+25,lines[0],7.8),(122,yy+9,lines[1],7.8)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'jv-graded.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0]['full_decl_sha256'];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Canonical graded pieces of the literal JV quotient',t),
 p('<b>Result.</b> Lean proves that the first printed Joux–Vitse homogeneous F101 source ideal has canonical Hilbert function1,3,3,1,0 thereafter. A canonical degree piece is zero exactly when its degree is at least4. This closes the prior finite per-degree identification gap, supplementing the already-proved actual quotient basis and total dimension8. No native Hilbert or Gröbner result is assumed. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>Actual canonical construction.</b> Let P=F101[x,y,h] and I be the unchanged ideal of the three literal homogenized coordinate equations. Lean proves I homogeneous for Mathlib standard grading. For each natural d, restrict the actual quotient map P→P/I to Mathlib homogeneousSubmodule of degree d. Prove its kernel is exactly I intersected with that submodule. The canonical quotient piece P_d/(I∩P_d) is linearly equivalent to the actual image by the first isomorphism theorem. These definitions precede comparison with a proposed basis. [pcc:unverified]'),
 p('<b>Derived comparison.</b> The unchanged actual quotient proof supplies a basis and24 checked coordinate multiplication relations. Kernel-checked finite tables show every nonzero coefficient raises degree by one. Coordinate/power/monomial induction then proves that every actual homogeneous polynomial image lies in the matching-degree basis span. Conversely, every selected basis representative is an actual homogeneous polynomial image. The actual image equals that span; inherited linear independence gives its dimension, with no Hilbert/basis/regularity conclusion supplied as a premise. [pcc:unverified]'),
 p('<b>Control and boundaries.</b> HF(0)=1 while total scalar dimension and scalar module length are8; Lean directly negates their equality. The exact Hilbert cutoff4 is separate from maximal internal F4 processing degrees. Generic JV Assumption2, FGHR generic/n≥3 scope, larger-system cohorts, whole-curve order/scalar relations and native-C refinement remain open. The theorem does not claim a generic graded-module Hilbert polynomial API or a new source counterexample. [pcc:unverified]'),
 p(f'<b>Checks and provenance.</b> Whole new project gate{g["declarations"]}/0 failures/0 trusted native; only propext, Classical.choice and Quot.sound. Complete712-character written binding{binding}, independently parsed1complete/0prefix/0unresolved. Native parsing/hash evidence is procedural, not a kernel refinement theorem. Five build snapshots, including four failures, remain: {a["files"]}files/{a["unique_objects"]}objects/{size}-byte lossless XZ plus five final gate files. Frozen old sources unchanged. [pcc:unverified]'),
 p('<b>Sources.</b> Joux–Vitse author21-page manuscript, full selectedp7literal source equations andp9Assumption2/generic-degree discussion read and visually inspected; PDFSHA6f3268a0…. No fullpaper-read or novelty claim. Prior actual quotient/dual/cutoff/source-field work credited. Official Mathlib homogeneous modules/ideals/firstisomorphism/basis dimensions read; its standard grading instance is explicitly local as documented. HilbertPoly explicitly TODOs generic graded-module theory, which is not assumed here. KB/Conductor unavailable; primary/prior/official fallback. Lean4.35.0/Mathlib56a00585…, standard defaults; no holes/custom axioms/native proof evaluation/flags/limits/waivers/new crypto experiment/spend/upload. Signed standing remains unverified. Source-linked proof map/pins accompany the report. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/jv-graded-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
