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
b=ParagraphStyle('b',fontName='DV',fontSize=9,leading=12.3,spaceAfter=8)
t=ParagraphStyle('t',fontName='DVB',fontSize=16.5,leading=21,spaceAfter=13)
def p(s):return Paragraph(s,b)
d=Drawing(500,120);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="120" viewBox="0 0 500 120">']
rows=[(81,'Actual source domain',['B = F5[X]/(X^5); lambda = x^2','Exact homogeneous degree 2, exact linear-generator rank 1']),
      (42,'Actual homology',['Ann(lambda) in B_k is zero for k=0,1,2','At k=3: x^3 kills lambda; trivial annihilators are zero']),
      (3,'Printed bound fails',['First fall degree = 3+2 = 5; printed bound = 4','Gamma_5(1,2,4,2) = 0; binary q=2 claim retained'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,34,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{120-yy-34}" width="500" height="34" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+17,title,8.1),(116,yy+21,lines[0],7.6),(116,yy+7,lines[1],7.6)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{120-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'hps-first-fall.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0]['full_decl_sha256'];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('HPS first fall: a general-field bound fails',t),
 p('<b>Source result.</b> Hodges–Petit–Schlather2014 Theorem4.9 asserts Dff(λ) ≤ (s(q−1)+d+2)/2 for degree d and rank s in a truncated polynomial algebra. Lean proves an actual counterexample over F5: λ=x² in B=F5[X]/(X⁵) has degree2, rank1 and first fall degree5, whereas the printed bound gives4. The same failure holds for every prime p≥5. This audits the printed general-field extension; the binary q=2 specialization in the corpus and its m²+1 bound are not refuted. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>Actual algebra and relations.</b> B is Mathlib’s actual AdjoinRoot quotient, with proved power basis, dimension, independent homogeneous pieces spanning B, graded multiplication and q-nilpotence of every positive-degree element. The formal first fall uses the source §4 single-polynomial formula: degree-k annihilators modulo λ^(q−1)B_(k−d(q−1)), with negative pieces zero. Its actual quotient is nontrivial exactly when an actual nontrivial fall exists. For q=5,d=2, the trivial multiplier λ⁴=x⁸ vanishes. Multiplication by λ is injective in degrees0,1,2, and nonzero x³ annihilates λ. Therefore the minimum total relation degree is3+2=5. No first-fall value or rank conclusion is assumed. [pcc:unverified]'),
 p('<b>Rank and family.</b> Definition4.6’s minimum number of linear algebra generators is implemented with actual Algebra.adjoin. The monomial lies in K[x] and cannot be constant, by independent quotient power coordinates, so its rank is exactly1. For every prime p≥5, actual Fp has cardinality p, B has dimension p, λ has exact degree2 and Dff=p; the printed bound is(p+3)/2&lt;p. The exact one-variable repair is Dff(x^d)=q and rank1 for every field K, q≥3 and2≤d&lt;q. This is a special-case repair, not a replacement general-rank theorem. [pcc:unverified]'),
 p('<b>Upstream failure and controls.</b> Theorem4.7 and AppendixA.2’s asserted strict positivity fail at q=5,n=1,d=2,k=2. Lean defines the actual q-nomial coefficients from(1+X+…+X^(q−1))^n and the actual bi-infinite integer sum Γ; it proves Γ₅(1,2,4,2)=0 and a direct negation with the printed guards retained. The proof’s strict-growth step encounters the flat n=1 coefficient sequence1,1,1,1,1. At q=3 the quadratic attains the printed bound3. At q=2, x² and its degree-two piece are zero, so this expression cannot refute the binary theorem. [pcc:unverified]'),
 p('<b>Boundary.</b> No full Weil-descent/filtered-associated-graded comparison, abstract all-subspace tensor comparison, generic regularity, solving-degree or F4 runtime theorem is established here. The actual single-polynomial homology formula is the one explicitly used in source §4. The corpus’s binary result remains separately open. Public exact-title/erratum/counterexample searches and prior local algebra work were consulted; no novelty or absence claim. Historical statuses and all frozen proof/claim bytes remain unchanged. [pcc:unverified]'),
 p(f'<b>Checks.</b> Whole project gate{g["declarations"]}/0failures/0trusted native, only propext/Classical.choice/Quot.sound. Complete1506-character binding {binding}; independent native1complete/0prefix/0unresolved. Native parsing/hashing is procedural, not kernel refinement. Eleven snapshots retain nine failed states: {a["files"]}files/{a["unique_objects"]}objects/{size}-byte losslessXZ plus five final gate files. Lean4.35.0/Mathlib56a00585…/defaults; no holes/customaxioms/nativeproofeval/raisedlimits/globalflags/waivers/newcryptoexperiment/spend/upload. Signed standing unverified. [pcc:unverified]'),
 p('<b>Sources checked.</b> First fall degree and Weil descent, Finite Fields and Their Applications30(2014)155–177, DOI10.1016/j.ffa.2014.07.001. Institutional version-of-record PDF490014bytes/SHA8edc2414…/24pages inclcover: all prose/math/appendices/references read; selected definitions,4.7/4.9/5.1/5.2 and AppA.2 visually inspected. Copyright PDF/OCR/screenshots remain local. Prior actual thickening/power-basis work and official Mathlib quotient/basis/subalgebra/homology APIs read. Fresh KB and Conductor127; prior/primary/official fallback before code. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/hps-first-fall-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
