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
rows=[(81,'Actual coefficient',['Sparse product convolution has one contributing exponent','Proved degree bound on the actual cubic half-power']),
      (42,'Source recurrence',['H_(p^(r+1)) = H_(p^r) times Hp^(p^r)','Actual Frobenius powers derive the explicit product']),
      (3,'Actual trace bridge',['Hq = 0 iff Hp = 0 for positive extension degree','Actual count-defined t divisible by p iff Hp = 0'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,34,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{120-yy-34}" width="500" height="34" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+17,title,8.1),(116,yy+21,lines[0],7.6),(116,yy+7,lines[1],7.6)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{120-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'hasse-norm.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0]['full_decl_sha256'];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Actual Hasse coefficients: the extension law',t),
 p('<b>Result.</b> Lean proves the source prime-power coefficient recurrence for every odd prime p, every field of characteristic p, every actual polynomial f of degree at most 3 and every r≥0. Define H(Q)=[X^(Q−1)]f^((Q−1)/2). Then H(p^(r+1))=H(p^r)·H(p)^(p^r), and H(p^r)=H(p)^(sum of p^i for 0≤i&lt;r). For r&gt;0, H(p^r)=0 iff H(p)=0. This is the explicit Frobenius product; no recurrence or coefficient conclusion is a premise. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>Actual polynomial proof.</b> A generic sparse convolution theorem proves the coefficient of A·map(σ,expand(M,B)) at Mn+M−1 equals A[M−1]·σ(B[n]) whenever M&gt;0 and deg(A)&lt;2M−1. Actual antidiagonal convolution and a proved residue window exclude every other contribution. Lean proves the strict degree bound for the actual half-power, derives its exponent decomposition, then applies official actual iterated Frobenius. Induction gives the product and positive-extension vanishing test. [pcc:unverified]'),
 p('<b>Actual elliptic trace.</b> For every finite field K with |K|=p^r and actual nonsingular monic cubic E, the unchanged actual Mathlib Point-count coefficient bridge gives count-defined integer t=|K|+1−Nat.card(E.Point) cast equal to that explicit H(p) product. For r&gt;0, actual p-divisibility of t is equivalent to H(p)=0. No point-count, trace or supersingularity conclusion is assumed. [pcc:unverified]'),
 p('<b>Skeptical guard controls.</b> Over F2, f=X³ has H(2)=0 but H(4)=1, directly disproving recurrence without odd characteristic. Over F3, f=X³ has H(1)=1 but H(3)=0, directly disproving vanishing equivalence at r=0. Over F3, f=X+X⁵ has H(3)=0 but H(9)=1, directly disproving unrestricted polynomial degree. These are coefficient-helper controls, not nonsingular elliptic counterexamples. Degree3 sufficiency is proved, not claimed sharp. [pcc:unverified]'),
 p('<b>Actual F9 distinction resolved.</b> The frozen actual nonsingular F9 curve y²=x³+a·x²+1 has H(3)=a differing from H(9)=1. The new generic theorem proves H(9)=a⁴=1, equal to the actual trace cast. The p-exponent coefficient cannot simply replace the q-exponent coefficient, while the qualified source Frobenius product is correct. [pcc:unverified]'),
 p('<b>Boundary.</b> No canonical Algebra.norm identification, Frobenius endomorphism/categorical trace bridge, general Hasse bound, geometric supersingularity/p-rank, full torsion-field bridge or integer count recovery beyond13 is proved here. The known MIT/Sutherland method is credited; no novelty or qualified-source refutation. [pcc:unverified]'),
 p(f'<b>Checks and provenance.</b> Whole project gate {g["declarations"]}/0 failures/0 trusted native; only standard propext/Classical.choice/Quot.sound. Complete1909-character binding {binding}; independent native1complete/0prefix/0unresolved, procedural parsing/hashing, not kernel refinement. Nine snapshots retain four proof failures, one premature-setup harness failure and one unavailable process completion: {a["files"]}files/{a["unique_objects"]}objects/{size}-byte losslessXZ plus five final gate files. Only03/07/09 pass. Lean4.35.0/Mathlib56a00585…/defaults/legacy advisories retained; no holes/customaxioms/native evaluator/limits/flags/waivers/newcryptoexperiment/spend/upload. Signed standing unverified. [pcc:unverified]'),
 p('<b>Sources checked.</b> MIT18.783 Spring2019 ProblemSet4 Problem1(g) exact statement reread before approach; prior selected1(a–i) printed/PDF1–2 fullread/visual inspection, not full9pages. Prior actual coefficient/count/trace/F9 proofs and official Mathlib Expand/Coeff/Basic/Frobenius/CharP/prime parity definitions and proofs reviewed. KB/Conductor127, primary/prior/official fallback. Copyright primary materials local only. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/hasse-norm-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
