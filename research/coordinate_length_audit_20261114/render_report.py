"""Lay out checked formal identities and controls; no research computation."""
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
b=ParagraphStyle('b',fontName='DV',fontSize=8.7,leading=11.8,spaceAfter=9);t=ParagraphStyle('t',fontName='DVB',fontSize=16,leading=20,spaceAfter=12)
def p(s):return Paragraph(s,b)
d=Drawing(500,122);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="122" viewBox="0 0 500 122">']
def label(x,y,s,size=8.5):
 d.add(String(x,y,s,fontName='DV',fontSize=size,fillColor=colors.HexColor('#234b70')));svg.append(f'<text x="{x}" y="{122-y}" font-family="DejaVu Sans,sans-serif" font-size="{size}" fill="#234b70">{html.escape(s)}</text>')
def box(x,y,w,h):
 d.add(Rect(x,y,w,h,rx=4,fillColor=colors.HexColor('#edf3f8'),strokeColor=colors.HexColor('#8eb0cb')));svg.append(f'<rect x="{x}" y="{122-y-h}" width="{w}" height="{h}" rx="4" fill="#edf3f8" stroke="#8eb0cb"/>')
label(8,109,'Actual bivariate family, every field and every positive m',9.2)
for x,lines in[(4,['Ideal (x^m, y−x)','Full ideal vanishing']),(173,['Quotient ≃ K[t]/t^m','Dimension / K-length m']),(342,['Every field extension','Exactly one solution'])]:
 box(x,48,151,46);label(x+7,76,lines[0],8.4);label(x+7,59,lines[1],8)
box(4,3,489,33);label(12,23,'Controls: m=0 empty; m=1 reduced; m>1 nonreduced, arbitrary gap.',8.4);label(12,10,'Target-domain control: F3 dual numbers have ≥3 endomorphisms, dimension 2.',7.9)
svg.append('</svg>');(o/'support-length.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text())
flow=[Paragraph('Actual solutions and coordinate-algebra length',t),p('<b>Generic formal bridge.</b> For any multivariate ideal I over a field K, Lean constructs an equivalence between its actual solutions over any field extension L and K-algebra maps K[variables]/I → L. When the quotient is finite dimensional, the solution set is finite and its cardinality is at most the quotient dimension, which equals its K-module length. Nilpotents in the source algebra are permitted; no reducedness or basis premise is supplied. Signed standing unverified. [pcc:unverified]'),d,Spacer(1,9),p('<b>Actual family.</b> For every field K and every natural m, the coordinate algebra of the actual two-variable ideal (x^m, y−x) is explicitly isomorphic to K[t]/(t^m). Both compositions are proved on actual quotient coordinates/root. A monic power basis gives dimension and scalar-module length m. The actual coordinate x has mth power zero; for m>0 its previous power is nonzero. For m>1 the actual quotient is nonreduced. [pcc:unverified]'),p('<b>Support and controls.</b> The full ideal vanishing condition, over every field extension, is equivalent to x^m=0 and y=x. For m>0 this gives exactly the origin and solution cardinality one. Lean proves an arbitrarily large dimension with unchanged support and directly negates universal support=dimension. At m=0 the solution set is empty and dimension/length zero; at m=1 the actual quotient is reduced and isomorphic to K. These are formal controls over every field, without solver enumeration. [pcc:unverified]'),p('<b>Why the target condition matters.</b> The checked homomorphism bound requires a domain target. For the actual finite F3 algebra A=F3[t]/t², scaling its nonzero nilpotent root by each field element gives an injective family of endomorphisms. Lean proves card End(A)≥3 while dim A=2, and directly negates the bound with this nonreduced target. This tests deleting a stated hypothesis; it does not refute the qualified theorem. [pcc:unverified]'),p('<b>Sources checked.</b> Original Diem (2011), Proposition 2.5, DOI10.1112/S0010437X10005075; existing actual bivariate multiplicity/JV basis-length packets; pinned Mathlib quotient lifts/ext, evaluation uniqueness, algebra-map independence/cardinality, monic AdjoinRoot power bases and module length. Exact Stacks09HS/0CKM statements and proofs were read and retained: they concern field extensions and are narrower than the actual checked ring-algebra statement. Neither citation annotations nor an irreducibility docstring replace actual formal hypotheses. No full-paper read or novelty claim. [pcc:unverified]'),p(f'<b>Validation.</b> Whole gate {g["declarations"]} declarations / 0 failures / 0 trusted native exceptions. Lean4.35.0 / Mathlib56a00585…; complete closed binding, custom source/config and artifact pins retained. All failed/successful source/log/exit records and the full gate are losslessly archived. Only propext/Classical.choice/Quot.sound; no holes, custom axioms, native evaluator, raised limits, new global flags or waivers. Exact PCC replay is separate; signed standing unverified. [pcc:unverified]'),p('<b>Remaining geometry and execution.</b> This proves scalar-module length accounting for finite affine coordinate algebras. Scheme-cycle/local intersection multiplicity identification, isolated points within positive dimensional components, the equal-multidegree projective n!d^n bound and Rojas/C refinement remain open. Neither Diem’s qualified theorem nor the corpus node is refuted. Existing 16 msolve calibration calls remain unchanged; no new solver experiment, paid compute or agent upload. KB unavailable; bounded source/formal fallback recorded. All fourteen original requirements retain their precise open scopes. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595.276,841.89),leftMargin=47,rightMargin=47,topMargin=35,bottomMargin=35).build(flow)
doc=fitz.open(o/'report.pdf');layout={'pages':[],'visual_inspection':'pending'}
for i,page in enumerate(doc):
 bad=[]
 for block in page.get_text('dict')['blocks']:
  for line in block.get('lines',[]):
   for span in line['spans']:
    x0,y0,x1,y1=span['bbox']
    if x0<0 or y0<0 or x1>page.rect.width or y1>page.rect.height:bad.append(span['text'])
 layout['pages'].append({'page':i+1,'text_within_page':not bad,'out_of_bounds':bad});page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/coordinate-length-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n');print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
