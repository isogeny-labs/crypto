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
b=ParagraphStyle('b',fontName='DV',fontSize=8.7,leading=11.5,spaceAfter=8)
t=ParagraphStyle('t',fontName='DVB',fontSize=16.5,leading=21,spaceAfter=13)
def p(s):return Paragraph(s,b)
d=Drawing(500,120);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="120" viewBox="0 0 500 120">']
rows=[(81,'Actual points','Cubic Euler function nonconstant → actual weak point bound','Source gcd condition + prime p≥5 → trace0 and countp+1'),
      (42,'Actual root field','All roots adjoin = simple primitive-root extension','Finiteness derived; actual Frobenius has order2 = field degree'),
      (3,'Exact controls','F3 counts1/7; actual F5 point of order2 gives field degree1','Actual F7 point of order8 gives root-adjoin field degree2')]
for yy,title,a,c in rows:
 d.add(Rect(0,yy,500,34,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')))
 svg.append(f'<rect x="0" y="{120-yy-34}" width="500" height="34" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+17,title,8.1),(116,yy+21,a,7.6),(116,yy+7,c,7.6)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{120-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'prime-trace.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Prime-field embedding degree: an actual proof',t),
 p('<b>Repaired source clause proved.</b> For every prime p≥5 and every nonsingular general Weierstrass model over Fp, FST2010’s actual trace/gcd definition of supersingularity forces actual trace0 and point count p+1. Every integer r>2 dividing that count, including composite r, has exact modular order2 and actual cyclotomic extension degree2. No Hasse bound, point count, trace-zero or field-degree identification is assumed. The literal r=2 claim is refuted by an actual curve and field extension. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>Actual weak count, without Hasse.</b> Over every odd finite field of size q>3, a degree3 polynomial f has nonconstant Euler-power function f(x)^h, h=(q−1)/2. If constant epsilon, X^q−X divides f^h−epsilon. Writing this as Lg, formal differentiation gives L(fg′−hf′g)=fg+h epsilon f′. The right side has degree below q, so is zero; degree(fg)≥3 contradicts degree(h epsilon f′)≤2. Actual completed-square and nonsingular-point/fiber equivalences then give 1&lt;#E(K)&lt;2q+1. Prime p dividing actual trace and the strict interval −p&lt;t&lt;p force trace0. This is a weak bound, not Hasse’s theorem. [pcc:unverified]'),
 p('<b>Actual cyclotomic field.</b> The field is IntermediateField.adjoin Fp of every z in AlgebraicClosure(Fp) satisfying z^r=1. From r dividing p+1, characteristic p does not divide r, so an actual primitive r-root exists. Every root is its power; the root-adjoin is a simple algebraic extension. Finite dimension and the finite carrier are derived. Actual p-Frobenius squared fixes every root generator. Frobenius cannot be identity, since a fixed primitive root would have square1 and force r dividing2. Its order is2; the actual finite-field Frobenius-order theorem gives actual Module.finrank2. This proves the field degree itself. [pcc:unverified]'),
 p('<b>Skeptical controls.</b> Actual nonsingular curves y²=x³−x±1 over F3 have counts7/1 and traces−3/3, both source-supersingular: deleting q>3 or p≥5 fails. Over F5, y²=x³+1 has count6, trace0 and an actual point of order2, but adjoining square roots of unity gives the base field and degree1. This directly refutes literal degree2 at r=2. Over F7, y²=x³+x has count8 and an actual point of order8; its actual eighth-root field has degree2, a positive composite-order control. Existing examples and fiber/gcd/order/Frobenius arguments are credited. [pcc:unverified]'),
 p('<b>Scope.</b> Supersingularity here is the exact source definition gcd(q+1−#E(K),q)≠1. Alternate geometric p-rank/Hasse-invariant/endomorphism equivalences, the general nonprime-field at-most6 classification and full geometric torsion fields remain separate. No canonical corpus promotion, novelty claim or full fourteen-item completion. [pcc:unverified]'),
 p(f'<b>Checks.</b> Whole gate {g["declarations"]}/0 failures/0 trusted native; only propext/Classical.choice/Quot.sound. Full {binding["full_characters"]}-character header, SHA {binding["full_decl_sha256"]}; independent native1complete/0prefix/0unresolved. Thirteen snapshots retain eight failed states: {a["files"]} files/{a["unique_objects"]} objects/{size}-byte losslessXZ plus five gate files. All final13 logs searched: no warnings/errors. Lean4.35.0/Mathlib56a00585…/defaults. No holes/custom axioms/native proof evaluation/raised limits/flags/waivers/frozen edits/cryptoPython/new native crypto/spend/upload. Header processing procedural; signed standing unverified. [pcc:unverified]'),
 p('<b>Sources checked.</b> Freeman–Scott–Teske, DOI10.1007/s00145-009-9048-z, primary723726bytes/57pages/SHA2acd937b…; printed230/PDF7 and237/PDF14 fully text-read/visually inspected, not full57pages. Exact source definition/Definition2.1/Remark2.2/prime-field claim distinguished. Earlier EquationCount/SquareFibers/TorsionBoundaries/CompositeTorsion/SourceFSTBoundary/procyclic proofs and pinned official APIs read; credited originals retained. KB/Conductor127, prior/primary/official fallback. Primary PDF/text/images local only. Initial protocol derivative sign error corrected separately before code; original preserved. [pcc:unverified]')]

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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/prime-trace-weak-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
