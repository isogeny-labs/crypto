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
b=ParagraphStyle('b',fontName='DV',fontSize=8.7,leading=11.7,spaceAfter=8)
t=ParagraphStyle('t',fontName='DVB',fontSize=16.5,leading=21,spaceAfter=13)
def p(s):return Paragraph(s,b)
d=Drawing(500,120);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="120" viewBox="0 0 500 120">']
rows=[(81,'Actual source fields',['|K|=q^2, |L|=q^6; derive [L:K]=3','epsilon^(q^3)=epsilon, epsilon^q!=epsilon: outside K']),
      (42,'Actual GL2 / PGL',['Construct an invertible matrix sending epsilon to theta','Prove denominator nonzero; uniqueness modulo unit scalars']),
      (3,'Degree control',['Actual F9 / F3: a^2=2, a outside F3','[0,2;1,0] fixes a and has nonidentity PGL class'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,34,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{120-yy-34}" width="500" height="34" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+17,title,8.1),(116,yy+21,lines[0],7.6),(116,yy+7,lines[1],7.6)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{120-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'momose-cubic.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0]['full_decl_sha256'];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Momose’s cubic homography lemma',t),
 p('<b>Source lemma survives.</b> Momose–Chao2005 Lemma1 uses a fractional-linear transformation over F_(q²) to move a specified element of F_(q³) outside F_q to any target in F_(q⁶) outside F_(q²), uniquely up to scalar. Lean proves this complete statement with actual field embeddings, actual invertible matrices and the actual Mathlib PGL quotient. It also verifies the printed explicit matrix. An actual quadratic-field counterexample shows why the cubic-degree condition cannot be deleted; the qualified primary lemma is not refuted. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>Finite-field bridge.</b> For actual fields K,L with an actual algebra embedding, |K|=q², |L|=q⁶ and q≥2 imply [L:K]=3 by the actual cardinality–dimension formula. If ε^(q³)=ε and ε^q≠ε, then ε cannot lie in the embedded K: that would make ε^(q²)=ε and hence ε^q=ε. These are the source’s subfield conditions. No extension-degree or nonbase-point conclusion is assumed. [pcc:unverified]'),
 p('<b>Construction and uniqueness.</b> In a degree-three extension, every nonbase element generates the field, by the actual intermediate-field lattice. Its actual power basis yields unique quadratic coordinates and excludes nonzero quadratic relations. A constructive matrix formula sends ε to an arbitrary nonbase target. Lean derives its nonzero denominator and determinant. A matrix fixing ε would supply a quadratic relation, forcing off-diagonal entries to zero and equal diagonal entries. Thus every stabilizer is an actual unit scalar matrix. Two matrices with the same source and target therefore have equal actual PGL classes and differ by a unit scalar. No numerical matrix or abstract group surrogate is substituted. [pcc:unverified]'),
 p('<b>Printed formula.</b> If ε³=rε+e and θ=aε²+bε+c, with(a,b)≠(0,0), Lean verifies A=[a(ar+c)−b², a²e−bc; a,−b]. Its determinant and fractional denominator are nonzero, and its actual fractional-linear value at ε equals θ. The proof also covers cubic minimal polynomials with a quadratic term through the more general constructive existence argument. [pcc:unverified]'),
 p('<b>Skeptical control.</b> Actual F9=GaloisField(3,2) has cardinality9 and degree2 over F3. Lean constructs a²=2 and proves a is outside the embedded F3. The actual GL2(F3) matrix[0,2;1,0] fixes a under fractional-linear action, yet its actual PGL class is not the identity. This directly refutes uniqueness after deleting the cubic-degree hypothesis. It does not refute the source lemma with that hypothesis. [pcc:unverified]'),
 p('<b>Boundary.</b> This checks Lemma1 and its explicit matrix, not the entire weak-cover classification, Jacobian/isogeny/kernel claims, densities or attack-runtime analysis. Lemma3.3’s omitted proof and Lemma5’s nonzero normalization requirement are not silently discharged. The 21-page extracted text was reviewed, with the exact Lemma1/PDF6 and Lemma3/PDF11 inspected visually; imperfect glyphs outside the checked target do not amount to independent verification of every formula or table. No novelty or canonical-status promotion. [pcc:unverified]'),
 p(f'<b>Checks.</b> Whole project gate{g["declarations"]}/0failures/0trusted native; only propext/Classical.choice/Quot.sound. Complete1206-character binding{binding}; independent native1complete/0prefix/0unresolved. Seven snapshots retain four failed states: {a["files"]}files/{a["unique_objects"]}objects/{size}-byte losslessXZ plus five final gate files. Lean4.35.0/Mathlib56a00585…/defaults, no holes/customaxioms/nativeproofeval/raisedlimits/globalflags/waivers/newcryptoexperiment/spend/upload. Native parsing/hashing is procedural, not kernel refinement. Signed standing unverified. [pcc:unverified]'),
 p('<b>Sources checked.</b> Momose–Chao, Scholten forms and elliptic/hyperelliptic curves with weak Weil restrictions, eprint2005/277, dated8Dec2005; public archive20100627054101,250076bytes/SHA137140ab…/21pages. Primary PDF/OCR/screenshots local only. Prior actual homography/power-basis work and official intermediate-field/GL/PGL/finite-field APIs read. Fresh KB/Conductor127; prior/primary/official fallback before code. Separate Diem2003 original retrieved but font extraction broken and only selected page3 OCR reviewed; CoverAttacks2003 only pages1–10 reviewed, not full papers. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/momose-cubic-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
