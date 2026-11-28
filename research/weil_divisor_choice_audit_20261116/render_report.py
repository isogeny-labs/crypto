"""Render proof/provenance metadata; no mathematical computation."""
from pathlib import Path
import json,html,fitz
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
rows=[(81,'Actual quotient','Degree-zero divisors D,E; disjoint support','div(f)=nD, div(g)=nE; value g(D)/f(E)'),(42,'Choice proof','Replace D by D+div(h), f by f*h^n','Actual reciprocity cancels both changes'),(3,'Moving guard','Geometric moving avoids any finite set','Actual F2 curve: no suitable rational shift')]
for yy,title,a,c in rows:
 d.add(Rect(0,yy,500,34,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')))
 svg.append(f'<rect x="0" y="{120-yy-34}" width="500" height="34" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+17,title,7.2),(104,yy+21,a,6.9),(104,yy+7,c,6.8)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{120-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'divisor-choice.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());pre=json.loads((r/'preflight.json').read_text());bf=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0]
flow=[Paragraph('Actual Weil divisor representative choice',t),
p('<b>Result and guards.</b> Over every algebraically closed field and general Weierstrass model with nonzero discriminant, Lean proves that the actual quotient g(D)/f(E) is independent of both degree-zero divisor representatives and both nonzero functions with divisors nD,nE. Only disjointness inside each pair is required; old and new pairs need not be cross-disjoint. The source convention is Galbraith26.1, inverse to some Miller conventions. The proof covers every integer n, hence every natural n, and every characteristic. It derives n-th-root membership. At n=0 it separately proves the actual quotient is1, rather than relying on vacuous zero-power membership. [pcc:unverified]'),
d,Spacer(1,9),
p('<b>Actual proper evaluations.</b> Divisors are finitely supported integer combinations of actual curve points. Functions are units of the actual full function field. Every evaluation requires the function to be a unit in the actual regular local ring at every support point. Addition and integer scaling of divisors and powers of functions are derived from these proper local values. An extension-by-one helper is only a proof device; it does not assign meromorphic values at zeros or poles. Support guards are derived from actual function-divisor equalities. [pcc:unverified]'),
p('<b>Moving and cancellation.</b> The actual geometric Picard principal-divisor criterion and actual point avoidance produce a point-difference representative outside any supplied finite set, with an actual function h witnessing the difference. If D changes to D+div(h), the actual corrected function is f*h^n. Proper geometric reciprocity gives g(div(h))=h(div(g))=h(E)^n. Numerator and denominator therefore acquire the same factor and cancel. Function-choice invariance uses actual same-divisor scalar uniqueness and degree zero. For two arbitrary pairs, an actual intermediate representative avoids both right supports; one-coordinate invariance and swapping finish the proof. No reciprocity, population, pairing law or prescribed function is assumed. [pcc:unverified]'),
p('<b>Existence and negative control.</b> Every actual point pair killed by n has actual proper disjoint representatives and actual functions: degree-zero and torsion point sum imply the required principal multiples. Over F2 the actual nonsingular curve y²+y=x³ has exactly O, P=(0,0), and -P. Lean proves there is no rational S for which (P+S)-(S) avoids {O,P}. This refutes deleting the geometric-field condition from this shifted moving construction and agrees with the source footnote. It does not refute existence of all possible rational divisors or the qualified source theorem. [pcc:unverified]'),
p('<b>Sources and remaining scope.</b> Fresh Galbraith chapter26 PDF1–5/printed573–577 text and all five individual visuals were read. Exercise26.1.1, formula26.1, the reciprocity sketch, the complete displayed26.3.2 proof and footnotes2–3 informed the proof. External nondegeneracy and multiplication-isogeny comparison references were not read in full. Six KB wrapper calls and initial/expanded Conductor failed127; bounded actual/official/primary fallback was used. No novelty claim. Multiplication-isogeny comparison, full canonical nondegeneracy, descent, nonclosed residue fields, canonical Tate, general MOV/Frey–Rück and backend runtime remain open. All fourteen targets remain active. [pcc:unverified]'),
p(f'<b>Checks.</b> Whole-project gate{g["declarations"]}/{g["new_modules"]} modules/zero failures or trusted native evaluations.342 predecessor pins checked before/after{pre["build_snapshots"]} source states and gate;{len(pre["failed_states"])} failed states retained, including default-limit failures. Final new-source warnings zero;32 packages/no copies, hydration or update. Full{bf["full_characters"]}-character header SHA{bf["full_decl_sha256"]}; separate binding after completed gate: one complete, zero prefixes/unresolved, empty suffix. Lossless{a["files"]} files/{a["unique_objects"]} objects/{a["compressed_bytes"]}-byte XZ; round-trip and actual restoration equal. Standard foundations only; no holes/custom axioms/native proof evaluation/raised limits/new flags/waivers/frozen edits. Same host/prover and procedural binding are not independent compiler/parser/physical-host assurance. Signed status unverified; zero paid compute. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595,842),leftMargin=44,rightMargin=44,topMargin=36,bottomMargin=36).build(flow)
doc=fitz.open(o/'report.pdf');pages=[]
for i,page in enumerate(doc):
 blocks=page.get_text('blocks');pages.append(dict(page=i+1,text_within_page=all(page.rect.contains(fitz.Rect(x[:4]))for x in blocks),text_characters=len(page.get_text())))
 page.get_pixmap(matrix=fitz.Matrix(1,1)).save('/workspace/scratch/weil-divisor-choice-report-page'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(dict(pages=pages,visual_inspection='pending'),indent=2)+'\n')
print(json.dumps(dict(pages=len(pages),bytes=(o/'report.pdf').stat().st_size,layout=pages)))
