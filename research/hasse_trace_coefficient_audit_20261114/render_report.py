"""Render proved statements and provenance; no research computation."""
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
rows=[(81,'Actual fibers',['Square-root counts give the Euler character contribution','Actual Mathlib Point/fiber equivalence includes infinity']),
      (42,'Actual coefficient',['Finite-field power sums isolate exponent q - 1','Proved strict degree bound for the actual cubic half-power']),
      (3,'Actual count / trace',['N cast = 1 - Hq; count-defined t cast = Hq','Hq = 0 iff characteristic divides the actual integer t'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,34,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{120-yy-34}" width="500" height="34" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+17,title,8.1),(116,yy+21,lines[0],7.6),(116,yy+7,lines[1],7.6)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{120-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'hasse-trace.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0]['full_decl_sha256'];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Actual elliptic counts and Hasse coefficients',t),
 p('<b>Result.</b> Lean proves the classical coefficient congruence for the actual point count of every nonsingular monic cubic elliptic model over every finite field of odd characteristic. Let q=|K|, f=X³+aX²+bX+c, N=Nat.card(E.Point), and let t=q+1−N be the actual count-defined integer trace. If Hq is the coefficient of X^(q−1) in f^((q−1)/2), then N cast into K equals 1−Hq and t cast equals Hq. Consequently Hq=0 iff the characteristic divides t. No count, trace or coefficient conclusion is supplied as a premise. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>Derived from actual objects.</b> Actual square-root fibers satisfy the Euler count formula in odd characteristic. Actual finite-field power sums give evaluation-sum = negative coefficient(q−1) for every polynomial of degree strictly below 2(q−1). Lean proves this strict degree bound for the actual cubic half-power, then reuses the unchanged actual Mathlib Point/fiber equivalence, including infinity. The closed certificate also specializes to every odd prime field with the source p-exponent coefficient and p-divisibility. [pcc:unverified]'),
 p('<b>Actual F13 ambiguity.</b> The nonsingular curves y²=x³−x and y²=x³+4 have actual Point counts 8 and 21, integer traces 6 and −7, and equal H13=6. Lean directly negates unrestricted recovery of the integer trace from that coefficient. The MIT source already asks for this p=13 control and correctly retains p&gt;13 for uniqueness; this is no source refutation. [pcc:unverified]'),
 p('<b>Extension and guard controls.</b> In actual F9=GaloisField(3,2), Lean constructs a nonsingular y²=x³+aX²+1 with a²=−1. Its characteristic-exponent coefficient Hp=a differs from Hq=1 and from the actual trace cast, which is 1. This distinguishes p and q exponents; the qualified source norm relation is not refuted. In F2 the root-fiber Euler helper fails at 1: this is a helper control, not a nonsingular cubic-curve counterexample. Actual F3 polynomial X⁴ directly disproves replacing the strict degree bound with a nonstrict bound. [pcc:unverified]'),
 p('<b>Remaining geometry.</b> The trace here is the actual count-defined integer q+1−N. This packet does not construct the canonical Frobenius endomorphism or identify its categorical trace, prove the general Hasse bound, geometric supersingularity/p-rank or full torsion-field bridge. The generic Hp-to-Hq norm identity and qualified integer count recovery beyond 13 also remain open. This closes a count-to-coefficient dependency in item 3, not the entire item. [pcc:unverified]'),
 p(f'<b>Checks and provenance.</b> Whole project gate {g["declarations"]}/0 failures/0 trusted native; only propext/Classical.choice/Quot.sound. Complete 1978-character binding {binding}; independent native 1complete/0prefix/0unresolved. Native parsing/hashing is procedural evidence, not kernel refinement. Nine source/config snapshots preserve five failed build states: {a["files"]} files/{a["unique_objects"]} objects/{size}-byte lossless XZ plus five final gate files. Frozen dependencies unchanged. Lean4.35.0/Mathlib56a00585…/defaults; legacy compatibility advisories retained. No holes/customaxioms/native evaluator/limits/flags/waivers/newcryptoexperiment/spend/upload. Signed standing unverified. [pcc:unverified]'),
 p('<b>Sources checked.</b> MIT18.783 Spring2019 ProblemSet4, Problem1(a–i), printed/PDF1–2 selected fullread/visual inspection before code; not full9pages. Classical finite-field coefficient method and p13 exercise credited; correct p&gt;13 preserved. Prior actual Point/fiber/count/trace proofs and official Mathlib finite-field, GaloisField, polynomial and characteristic APIs reviewed. KB/Conductor127; primary/prior/official fallback. No novelty claim. Copyright primary PDF/OCR/images remain local only. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/hasse-trace-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
