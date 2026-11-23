"""Render checked statements and provenance; no research computation."""
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
b=ParagraphStyle('b',fontName='DV',fontSize=8.8,leading=11.8,spaceAfter=8)
t=ParagraphStyle('t',fontName='DVB',fontSize=16.5,leading=21,spaceAfter=13)
def p(s):return Paragraph(s,b)
d=Drawing(500,120);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="120" viewBox="0 0 500 120">']
rows=[(81,'Actual complex','Homogeneous kernel ↔ all continuous crossed cocycles','Actual d0 image ↔ all principals; actual homology projection'),
      (42,'Canonical H1','Existing continuousCohomology1 ↔ actual class quotient','Finite Galois coefficients: continuity derived, coinvariants proved'),
      (3,'Perfect / control','Actual H1 pairing: exact chi(F(1,Frob)) formula','Characteristic 2, M=Z/2: characters trivial, actual H1 nonzero')]
for yy,title,a,c in rows:
 d.add(Rect(0,yy,500,34,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')))
 svg.append(f'<rect x="0" y="{120-yy-34}" width="500" height="34" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+17,title,8.1),(116,yy+21,a,7.6),(116,yy+7,c,7.6)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{120-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'continuous-h1.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Actual continuous H1: comparison and pairing',t),
 p('<b>Canonical comparison proved.</b> Lean identifies Mathlib’s actual continuousCohomology1, defined by the homogeneous cochain complex, with all continuous crossed cocycles modulo all principal cocycles. For every finite field and finite Abelian coefficient group with a continuous Galois action, this gives the actual Frobenius coinvariants and both perfectness maps for Bruin2011’s finite-module pairing under exponent dividing q−1. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>All actual kernels and boundaries.</b> The actual differential formulas are d0(f)(x,y)=f(y)−f(x) and d1(F)(x,y,z)=F(y,z)−F(x,z)+F(x,y). Invariance and the actual kernel condition give the crossed cocycle c(g)=F(1,g). Its inverse is the actual continuous homogeneous cochain F(x,y)=rho(x)c(x⁻¹y), constructed by continuous currying. Both inverse laws are proved. The actual d0 image maps onto the complete principal image g↦rho(g)a−a. No restricted cocycles or supplied cohomology values are used. [pcc:unverified]'),
 p('<b>The existing homology object.</b> Actual categorical kernel/cokernel universal properties and proved index isomorphisms identify the existing continuousCohomology1 object with the kernel quotient. Representatives are sent through the actual categorical homology projection, and the quotient/representative formula is proved. The application constructs the coefficient TopRep on Additive(M) and derives joint continuity from the original continuous action into the discrete automorphism group. An explicit common ambient universe covers all ordinary finite fields and modules. [pcc:unverified]'),
 p('<b>Exact pairing and skeptical control.</b> Composing the actual H1/class comparison with credited finite-field character duality gives both perfectness isomorphisms and both trivial radicals for actual all-Galois-equivariant algebraic-closure unit characters. For every actual homogeneous kernel representative F, the pairing value maps to chi(F(1,Frob)) in actual algebraic-closure units. For every finite characteristic2 field and M=Z/2 with trivial action, actual continuous H1 has a nonzero class while every character is trivial. No bilinear homomorphism on these same canonical domains can be right-nondegenerate without the annihilation guard. Qualified source survives. [pcc:unverified]'),
 p('<b>Reuse, correction and boundary.</b> Eleven prior core files are copied byte-identical, credited to PCC-ed6495/4711ad/cdb4f1 and PowerTargets. Two inherited unused-section advisories remain; prior advisory-free report wording is corrected outside its frozen pinset. Six new modules have no advisories. Isogeny kernels, Lang, Cartier group schemes, Jacobians, divisors and the full Tate/MOV/Frey–Rück conditions remain open. Original FR94 remains unread. No novelty or canonical corpus promotion. [pcc:unverified]'),
 p(f'<b>Checks.</b> Whole gate {g["declarations"]}/0 failures/0 trusted native; only propext/Classical.choice/Quot.sound. Full {binding["full_characters"]}-character header, SHA {binding["full_decl_sha256"]}; independent native1complete/0prefix/0unresolved. Twelve snapshots/five failed states plus initial setup working-directory failure retained: {a["files"]} files/{a["unique_objects"]} objects/{size}-byte losslessXZ and five gate files. Final12 passes with two inherited advisories. Lean4.35.0/Mathlib56a00585…/defaults. No holes/custom axioms/native proof evaluation/raised limits/flags/waivers/frozen edits/cryptoPython/new native crypto/spend/upload. Header processing procedural; signed standing unverified. [pcc:unverified]'),
 p('<b>Sources checked.</b> Bruin, JTNB23(2011)323–328, DOI10.5802/jtnb.764; full six article pages previously read/seven PDF pages inspected, selected325/PDF4 rechecked. Actual pinned Mathlib continuous-cohomology, coinduction, TopRep, categorical homology/kernel/cokernel/index and quotient APIs read. KB/Conductor127; prior/primary/official fallback before approach/code. Copyright primary files local only. No failed-search absence claim. [pcc:unverified]')]

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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/continuous-h1-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
