"""Render proof results/provenance only; no research arithmetic."""
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
rows=[(81,'Actual steps','One at zero; vertical for negatives; chord/vertical otherwise','Nonzero functions with divisor P+Q-(P+Q)-O'),(42,'All n','Construct fn recursively; div fn = n(P)-([n]P)-(n-1)(O)','Actual finite support, torsion specialization and f1=1'),(3,'Normalize','Actual source X/Y parameter gives leading residue one','Normalized i+j invariant; actual vertical-guard countercontrol')]
for yy,title,a,c in rows:
 d.add(Rect(0,yy,500,34,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')))
 svg.append(f'<rect x="0" y="{120-yy-34}" width="500" height="34" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+17,title,8.1),(104,yy+21,a,7.6),(104,yy+7,c,7.4)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{120-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'miller-recursion.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Actual all-n Miller functions and divisors',t),p('<b>Result.</b> Lean constructs nonzero actual rational Miller functions for every n and actual nonsingular curve point. Their actual principal point divisors and leading residue-one normalization at infinity are derived. The addition rule preserves both properties for every point branch, including zero and vertical additions. [pcc:unverified]'),d,Spacer(1,9),p('<b>Actual function and divisor.</b> The step g(P,Q) is one if either point is O, the actual vertical function when Q=−P, and the actual chord divided by the vertical through P+Q otherwise. Its actual normalized local orders give div(g)=(P)+(Q)−(P+Q)−(O), with nonzero and finite-support proofs. Define f0=1 and f(n+1)=fn·g([n]P,P). Induction proves div(fn)=n(P)−([n]P)−(n−1)(O) for every n. The coefficient n−1 is integer subtraction, specifying the consistent n0 convention. If [n]P=O the actual divisor is n(P)−n(O). The actual constructed f1 equals1. [pcc:unverified]'),p('<b>Actual normalization and addition invariant.</b> Scale each actual function by t raised to minus its actual infinity order, with source parameter t=X/Y. The norm-degree valuation equals the official adic valuation of the proved actual infinity chart local ring. Valuation algebra proves closure of leading residue-one under products and quotients; actual line and vertical normalizations give it for every step and every constructed fn. Any valid normalized i,j functions yield a valid normalized i+j function fi·fj·g([i]P,[j]P). Equality of differently constructed functions is not inferred from their divisors; uniqueness and binary-program/runtime refinement are not claimed. [pcc:unverified]'),p('<b>Skeptical control.</b> On the actual nonsingular F5 curve y²=x³+1, T=(4,0) is nonzero and T+T=O. The correct step is the vertical function with actual infinity order−2. Treating this case as an ordinary nonvertical chord gives infinity order−3, contradicting the actual prescribed chord divisor. Lean proves that divisor assertion false. The source explicitly requires exceptional simplifications; its qualified statements survive. [pcc:unverified]'),p(f'<b>Checks.</b> Whole tracked project gate {g["declarations"]}/0 failures/0 trusted native, only propext/Classical.choice/Quot.sound. Thirty-three modules:27byte-identical credited cores and6new. Complete {binding["full_characters"]}-character binding {binding["full_decl_sha256"]}; native1complete/0prefix/0unresolved. Seven snapshots/two failures/five successes, setup/KB/Conductor/protocol retained: {a["files"]}files/{a["unique_objects"]}objects/{size}-byte losslessXZ plus five gate files. Final07/project logs have no warnings/errors; frozen auditor has one inherited String.mk deprecation. No holes/custom axioms/nativeproofeval/raisedlimits/globalflags/waivers/frozenedits/cryptoPython/spend/upload. [pcc:unverified]'),p('<b>Sources and remaining bridges.</b> Galbraith chapter26, primary222784bytes/16pages/SHA c06cf2df…: PDF2–6/printed574–578 fully text-read, PDF2–6 visually inspected across prior and fresh readings, not all16pages. Definition26.3.4, Exercise26.3.5, Algorithm29 and Exercise26.3.6 supply the targets and exceptions. Prior chapter7 local-order/divisor definitions and official valuation/log/zpow/Finsupp/point-group APIs are credited. KB/Conductor127, explicit primary/prior/official fallback; no novelty claim. Over arbitraryK these are actual nonsingularKpoint divisors; for an elliptic model over algebraically closedK they cover all geometric points. Separate Mathlib Scheme.localRing/Divisor API comparison, general all-function divisor finiteness, translation/reciprocity/isogeny/Lang/Cartier/Jacobian/canonical pairing/MOV-FR/all14 remain open. Same host/prover/Mathlib, procedural header check and unverified signed standing. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595,842),leftMargin=44,rightMargin=44,topMargin=36,bottomMargin=36).build(flow)
doc=fitz.open(o/'report.pdf');pages=[]
for i,page in enumerate(doc):
 blocks=page.get_text('blocks');rect=page.rect
 pages.append(dict(page=i+1,text_within_page=all(rect.contains(fitz.Rect(x[:4]))for x in blocks),text_characters=len(page.get_text())))
 page.get_pixmap(matrix=fitz.Matrix(1.5,1.5)).save('/workspace/scratch/miller-recursion-report-page'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(dict(pages=pages,visual_inspection='pending'),indent=2)+'\n')
print(json.dumps(dict(pages=len(pages),bytes=(o/'report.pdf').stat().st_size,layout=pages)))
