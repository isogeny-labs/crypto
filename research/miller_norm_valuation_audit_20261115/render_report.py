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
rows=[(81,'Actual valuation','Norm degree: multiplicative and ultrametric','Extended to the actual function field; no valuation premise'),
      (42,'Actual DVR','X/Y is a uniformizer and generates the maximal ideal','X weight2, Y and line weight3; normalized residues equal1'),
      (3,'Counter-control','Reciprocal X is not a uniformizer; normalization fails','Canonical projective local-ring identification remains open')]
for yy,title,a,c in rows:
 d.add(Rect(0,yy,500,34,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')))
 svg.append(f'<rect x="0" y="{120-yy-34}" width="500" height="34" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+17,title,8.1),(104,yy+21,a,7.6),(104,yy+7,c,7.4)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{120-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'valuation-bridge.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Miller functions: actual valuation and residue',t),
 p('<b>Result and scope.</b> Over every field and general Weierstrass model, Lean constructs an actual valuation of Mathlib’s actual curve function field. Its actual valuation ring is a DVR; t=X/Y is an actual uniformizer and generates the maximal ideal. Normalized nonvertical and vertical lines have actual residue1. This advances the valuation/normalization bridge. Equality with the canonical projective local ring at infinity and the full canonical divisor/pairing remain open. [pcc:unverified]'),d,Spacer(1,9),
 p('<b>Construction.</b> For the actual coordinate-ring element p(X)+q(X)Y, the official rank-two algebra norm has degree max(2degp,2degq+3). Its triangle bound is proved, multiplication follows from actual norm/degree multiplicativity, and its zero support is proved. Mapping bottom to0 and a finite weight n to exp(n) gives an actual Valuation; the official localization API extends it to the actual FractionRing. No valuation, coordinate weights, rank, DVR or normalization statement is assumed. [pcc:unverified]'),
 p('<b>Coordinates, lines and parameter.</b> The derived values are exp2 for X and vertical lines, exp3 for Y and nonvertical lines, and exp(−1) for t=X/Y. The parameter’s powers prove full surjectivity onto the integer-exponent value group. Its derived nontrivial cyclic value group supplies the actual DVR and official IsUniformizer/maximal-ideal results. This convention grows at poles, like Mathlib’s RatFunc.inftyValuation; canonical infinity identification is not silently assumed. [pcc:unverified]'),
 p('<b>Actual residue normalization and negative control.</b> The actual Weierstrass equation implies v(X³/Y²−1)&lt;1. Lower-weight terms give v(t³ line−1)&lt;1 and v(t² vertical−1)&lt;1. The actual valuation-ring residue homomorphism sends both normalized elements to1. Reciprocal X has value exp(−2), is proved not to be a uniformizer, and replacing t with it makes the normalized-line difference have value1. This disproves the substitution, while retaining the correctly qualified source formula. [pcc:unverified]'),
 p(f'<b>Checks and provenance.</b> Whole project gate {g["declarations"]}/0 failures/0 trusted native; only propext/Classical.choice/Quot.sound. Complete {binding["full_characters"]}-character binding {binding["full_decl_sha256"]}; native1complete/0prefix/0unresolved. Seven new modules, no prior custom proof modules. Fourteen snapshots/nine failed states/success03,05,08,12,14 retained: {a["files"]}files/{a["unique_objects"]}objects/{size}-byte losslessXZ plus five gate files. Four missing exit-file writes caused by set -e were recovered from actual retained PTY completions, all exit1; no reruns manufactured those records. Final14/project build logs have no warnings/errors; the frozen auditor emits one inherited String.mk deprecation. Lean4.35.0/Mathlib56a00585…/defaults. [pcc:unverified]'),
 p('<b>Source fidelity and remaining work.</b> Galbraith chapter26, extended corrected version2.0, primary222784bytes/16pages/SHA c06cf2df…; printed577/PDF5 freshly fully text-read/visually inspected, especially Definition26.3.4 and Exercise26.3.5. Earlier574–577/PDF2–5 scope retained, not all16pages. Official norm-degree, valuation/localization/residue and discrete/uniformizer APIs are credited; official formal Serre I.1 Proposition1 used, no new primary Serre reading claimed. Prior Miller proof packets credited and unchanged. KB/Conductor127, prior/primary/official fallback. Canonical projective local-ring equality, full principal divisor, reciprocity/isogeny/Lang/Cartier/Jacobian/Weil-Tate/MOV-FR and all14 completion remain open. Same host/prover/Mathlib, no independent physical-host/parser/compiler refinement or signed standing. No holes/custom axioms/nativeproofeval/limits/global flags/waivers/frozen edits/cryptoPython/spend/upload/novelty/canonical promotion. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595,842),leftMargin=44,rightMargin=44,topMargin=36,bottomMargin=36).build(flow)
doc=fitz.open(o/'report.pdf');pages=[]
for i,page in enumerate(doc):
 blocks=page.get_text('blocks');rect=page.rect
 pages.append(dict(page=i+1,text_within_page=all(rect.contains(fitz.Rect(x[:4]))for x in blocks),text_characters=len(page.get_text())))
 page.get_pixmap(matrix=fitz.Matrix(1.5,1.5)).save('/workspace/scratch/miller-infinity-report-page'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(dict(pages=pages,visual_inspection='pending'),indent=2)+'\n')
print(json.dumps(dict(pages=len(pages),bytes=(o/'report.pdf').stat().st_size,layout=pages)))
