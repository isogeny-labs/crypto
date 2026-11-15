"""Render formal results and provenance only; no research computation."""
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
b=ParagraphStyle('b',fontName='DV',fontSize=8.7,leading=11.5,spaceAfter=8)
t=ParagraphStyle('t',fontName='DVB',fontSize=17,leading=22,spaceAfter=14)
def p(s):return Paragraph(s,b)
d=Drawing(500,120);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="120" viewBox="0 0 500 120">']
rows=[(81,'Generic chord','Actual point-ideal product = principal line ideal','Every field, general model, nonsingular points, nonvertical guard'),
      (42,'Three-torsion','Actual tangent ideal = point ideal cubed','Actual ring-hom evaluation gives the numerical Miller expression'),
      (3,'Boundary','F5 positive and negative guard controls both proved','Infinity valuation and canonical projective divisor still open')]
for yy,title,a,c in rows:
 d.add(Rect(0,yy,500,34,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')))
 svg.append(f'<rect x="0" y="{120-yy-34}" width="500" height="34" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+17,title,8.1),(104,yy+21,a,7.6),(104,yy+7,c,7.1)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{120-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'affine-bridge.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Miller lines: actual affine principal ideals',t),
 p('<b>Result and scope.</b> Lean proves the actual coordinate-ring ideal identity I(P)I(Q)I(−P−Q) = (line(P,Q)) for every field, general Weierstrass model, actual nonsingular affine points and nonvertical guard. This advances the finite-affine ideal/function/evaluation bridge beyond the earlier polynomial intersection certificates. The projective infinity pole, canonical principal divisor and canonical pairing identification remain separate obligations. [pcc:unverified]'),d,Spacer(1,9),
 p('<b>Actual ideal argument.</b> Mathlib supplies actual chord-product and negative-point-product identities in its coordinate ring. A new generic cancellation proof removes a nonzero principal X-ideal factor using explicit ideal membership and integral-domain cancellation. Actual 3P=0 derives a nonvertical tangent and 2P=−P; their actual coordinates yield I(P)³ = (tangent(P)). No group rank, pairing nondegeneracy or infinity-valuation hypothesis is inserted. [pcc:unverified]'),
 p('<b>Actual functions and evaluation.</b> The regular line class Y−line(X) is nonzero, and its image in the actual function field is nonzero by fraction-field injectivity. AdjoinRoot.evalEval at an actual affine point is a ring homomorphism. Its values on X and Y classes prove evaluation(line) = yR−yP−slope(xR−xP). When a1=a2=a3=0 the tangent slope is (3xP²+a4)/(2yP), giving the usual numerical Miller tangent expression. Evaluation at infinity is not defined by an ad hoc zero branch. [pcc:unverified]'),
 p('<b>Skeptical guard control.</b> On actual nonsingular E/F5:y²=x³+1, P=(0,1) is killed by3 and satisfies the tangent cube identity. Q=(2,2) is nonsingular, has a nonvertical tangent, and is not killed by3. Its cube identity is false: the tangent line vanishes at P, whereas (X−2)³ evaluates to2 there. This is a kernel-checked counterexample to deleting the three-torsion guard, not a refutation of the correctly qualified Miller identity. [pcc:unverified]'),
 p(f'<b>Checks and provenance.</b> Whole project gate {g["declarations"]}/0 failures/0 trusted native; only propext/Classical.choice/Quot.sound. Complete {binding["full_characters"]}-character binding {binding["full_decl_sha256"]}; native1complete/0prefix/0unresolved. Eight snapshots retain five failed states and successful04/07/08; the attempt03 edit-path error and unchanged rebuild are recorded. Final08 and project build logs contain no warnings/errors. The frozen audit tool emits one existing String.mk deprecation warning. {a["files"]} files/{a["unique_objects"]} objects/{size}-byte losslessXZ plus five gate files. Four new modules import no prior custom proof modules. Lean4.35.0/Mathlib56a00585…/defaults. No holes/custom axioms/nativeproofeval/raised limits/global flags/waivers/frozen edits/cryptoPython/spend/upload. [pcc:unverified]'),
 p('<b>Source fidelity and remaining work.</b> Galbraith chapter26, extended corrected version2.0: primary222784bytes/16pages/SHA c06cf2df…; printed574–577/PDF2–5 freshly fully text-read and visually inspected in preceding work, not all16pages. Definition26.3.4 and equations26.1/26.2 motivate the bridge. Prior MillerCurve/Expressions/TangentCertificate were fully read and byte-checked against their frozen source plan; original files untouched. Official coordinate-ring and evaluation APIs were read before implementation. KB/Conductor127; prior/primary/official fallback, no absence or novelty claim. Canonical projective divisor/valuation, pullback/reciprocity/isogeny/Lang/Cartier/Jacobian/Weil-Tate/MOV-FR and full fourteen-item completion remain open. Same prover/Mathlib/host; no independent physical-host/compiler/parser refinement or signed-standing claim. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595,842),leftMargin=44,rightMargin=44,topMargin=36,bottomMargin=36).build(flow)
doc=fitz.open(o/'report.pdf');pages=[]
for i,page in enumerate(doc):
 blocks=page.get_text('blocks');rect=page.rect
 pages.append(dict(page=i+1,text_within_page=all(rect.contains(fitz.Rect(x[:4]))for x in blocks),text_characters=len(page.get_text())))
 page.get_pixmap(matrix=fitz.Matrix(1.5,1.5)).save('/workspace/scratch/miller-ideal-report-page'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(dict(pages=pages,visual_inspection='pending'),indent=2)+'\n')
print(json.dumps(dict(pages=len(pages),bytes=(o/'report.pdf').stat().st_size,layout=pages)))
