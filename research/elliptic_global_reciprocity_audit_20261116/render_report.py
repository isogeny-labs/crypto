"""Render checked proof and provenance; no mathematical computation."""
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
b=ParagraphStyle('body',fontName='DV',fontSize=8.6,leading=11.2,spaceAfter=8)
t=ParagraphStyle('title',fontName='DVB',fontSize=17,leading=22,spaceAfter=14)
def p(x):return Paragraph(x,b)
d=Drawing(500,120);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="120" viewBox="0 0 500 120">']
rows=[(81,'Full geometric product','Algebraically closed field; nonzero discriminant','Every actual function pair; both conventions'),(42,'Derived proof','Full fibers -> normalized norms -> P1','Actual quadratic basis -> Steinberg relation'),(3,'Guard controls','Singular ACF5: 4 / rational-only F5: 4','Nonsingular geometric and small-characteristic: 1')]
for yy,title,a,c in rows:
 d.add(Rect(0,yy,500,34,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')))
 svg.append(f'<rect x="0" y="{120-yy-34}" width="500" height="34" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+17,title,7.2),(104,yy+21,a,6.9),(104,yy+7,c,6.8)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{120-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'elliptic-reciprocity.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
bf=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0]
flow=[Paragraph('Geometric elliptic tame reciprocity',t),
p('<b>Result and guards.</b> Lean proves the global product of actual classical tame symbols is one for every pair of nonzero functions on every general Weierstrass model over an algebraically closed field with nonzero discriminant. Every characteristic is allowed. The corrected inverse convention also has product one. Actual finite support is established, and common zeros or poles are allowed through proper balanced local units. For disjoint principal divisors the proof derives the actual proper evaluation identity f(div g)=g(div f), with both support guards retained. [pcc:unverified]'),
d,Spacer(1,9),
p('<b>Actual fibers and mixed symbols.</b> The actual point map sends infinity to none and a finite point to its x-coordinate. Its fibers are the proved distinct full finite fibers and singleton infinity. Over an algebraically closed field, the actual quadratic fiber polynomial and nonzero discriminant give a nonsingular lift above every finite coordinate. An explicit finite-support restriction and regrouping identity turn the global elliptic product into products over these complete fibers. The independently normalized finite and infinity norm projections identify each with its canonical proper P1 symbol. Actual P1 reciprocity then proves the mixed-base product is one, without cancelling symbol squares or assuming a global product. [pcc:unverified]'),
p('<b>Arbitrary actual functions.</b> The localized coordinate-ring basis (1,y) gives actual base coefficients for every function. If f is outside K(x), its derived y coefficient is nonzero and the proof derives g=a*f+b with a,b in K(x). Zero a or b are handled explicitly using mixed reciprocity and geometric degree zero. When both are nonzero, actual units U=(-a/b)*f and V=g/b satisfy U+V=1. The proved global Steinberg relation and derived invariance under base factors give reciprocity for f,g. This uses the actual full function field, not a presumed abstract quadratic model. Known norm and Bass–Tate methods are credited. [pcc:unverified]'),
p('<b>Deletion controls and positives.</b> On the actual singular cusp y²=x³, the nonsingular-point divisor of x is minus two infinity: the singular finite zero is absent from that domain. The actual constant-symbol product is c^(-2), even over an algebraic closure. Over the actual algebraic closure of F5, c=2 gives embedded4, unequal to1, disproving deletion of the discriminant guard. The credited nonsingular F5 rational-points-only product4 remains a counterexample to omitting geometric points; its actual geometric extension gives1. On nonsingular y²=x³+1 over the algebraic closure of F5, both y and y-1 are proved outside K(x) and have classical/inverse product1. Actual binary and ternary geometric models give product1 for every function pair in both conventions. Qualified source theorems remain unrefuted. [pcc:unverified]'),
p('<b>Sources and remaining scope.</b> Fresh Gille–Szamuely PDF218–220/printed204–206 text and all three page visuals: complete bounded7.4.1–4 proof bodies and Remark7.4.5, including reference-based7.4.3 and the7.4.4 diagram proof. Earlier norm/completion/integral-closure and external dependencies were not all read. The complete bounded7.2.8–10 quadratic-generation proof is credited from the P1 predecessor. Six KB calls and initial/expanded Conductor were unavailable(127); bounded actual/official/primary fallback was used. General closed points with residue-field norms, scheme identification, arbitrary divisor representatives, isogeny comparison, descent, canonical Tate and general MOV/Frey–Rück remain open. All fourteen items remain active. [pcc:unverified]'),
p(f'<b>Checks.</b> Whole-project gate{g["declarations"]}/9 modules(8 concrete, one official bridge)/zero failures or trusted native evaluations. All330 predecessor pins match before/after9 states and gate; four failed states retained. Final new-source warnings zero;31 packages/no copies, hydration or update. Full{bf["full_characters"]}-character header SHA{bf["full_decl_sha256"]}; separate binding check after completed gate: one complete, zero prefixes/unresolved, empty suffix. Lossless{a["files"]} files/{a["unique_objects"]} objects/{a["compressed_bytes"]} bytes XZ, round-trip and actual restoration equal. Standard Lean foundations, no holes/custom axioms/native proof evaluation/raised limits/new flags/waivers/frozen edits. Same-host prover and procedural binding provide no independent compiler/parser/physical-host assurance. Signed status unverified; zero paid compute. [pcc:unverified]')]

SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595,842),leftMargin=44,rightMargin=44,topMargin=36,bottomMargin=36).build(flow)
doc=fitz.open(o/'report.pdf');pages=[]
for i,page in enumerate(doc):
 blocks=page.get_text('blocks');pages.append(dict(page=i+1,text_within_page=all(page.rect.contains(fitz.Rect(x[:4]))for x in blocks),text_characters=len(page.get_text())))
 page.get_pixmap(matrix=fitz.Matrix(1,1)).save('/workspace/scratch/elliptic-global-reciprocity-report-page'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(dict(pages=pages,visual_inspection='pending'),indent=2)+'\n')
print(json.dumps(dict(pages=len(pages),bytes=(o/'report.pdf').stat().st_size,layout=pages)))
