"""Layout recorded formal statements and provenance; no research computation."""
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
b=ParagraphStyle('b',fontName='DV',fontSize=8.7,leading=11.8,spaceAfter=10)
t=ParagraphStyle('t',fontName='DVB',fontSize=16,leading=20,spaceAfter=12)
def p(s):return Paragraph(s,b)
d=Drawing(500,142);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="142" viewBox="0 0 500 142">']
rows=[(96,'Valid count theorem',['Every finite odd field; nonsquare d, Δ≠0','Actual N+N′=2(q+1), t′=−t; source predicates agree']),
      (50,'Literal Ex9.5.3',['F7, j=1728, nonsquare d=3; both #E=8','Isomorphism (4X:Y:Z), inverse (2X:Y:Z)']),
      (4,'Delete hypotheses',['F3, d=1: actual sum 10 ≠ 8','F5 singular cusp, nonsquare d=2: sum 10 ≠ 12'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.8),(130,yy+25,lines[0],8.1),(130,yy+9,lines[1],8.1)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'twist-count-counterexample.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('Actual quadratic-twist counts and a literal isomorphism counterexample',t),
 p('<b>Result.</b> Lean proves the actual count formula for every finite odd field and nonsingular cubic model y²=x³+a·x²+b·x+c: nonsquare scaling (d·a,d²·b,d³·c) gives N+N′=2(q+1), actual trace t′=−t and equal source gcd(trace,q) predicates. It also supplies an explicit counterexample to the literal non-isomorphism assertion in Galbraith Exercise9.5.3. [pcc:unverified]'),d,Spacer(1,6),
 p('<b>Actual semantics.</b> Actual points are equated with literal equation pairs plus the identity, then with square-root fibers. Zero has one root, a nonzero square two, a nonsquare none. Official finite-field square criteria prove complementary fibers for v and d³v have total two. Reindex x by d·x, add the two identities and derive actual counts/traces. Discriminants scale by d⁶. No non-isomorphism, Frobenius trace or point-count premise is assumed. This proves the source short-model count exercise and the explicit cubic-with-x² extension. [pcc:unverified]'),
 p('<b>Literal primary counterexample.</b> Over F7, y²=x³+x is nonsingular with j=1728. The nonsquare d=3 gives y²=x³+2x, yet (X:Y:Z)↦(4X:Y:Z) has inverse (2X:Y:Z). Lean proves the actual admissible variable change, actual additive point-group equivalence, both counts eight, and the homogeneous cubic/inverse identities over every commutative ring of characteristic seven. This is a trivial nonsquare scaling in the special-j case. The separately qualified generic-j Lemma9.5.7 retains its explicit exceptions. [pcc:unverified]'),
 p('<b>Hypothesis controls.</b> Deleting nonsquare scaling fails on the credited F3 curve with five actual points: d=1 gives sum10 rather than8. Deleting nonsingularity fails on the credited F5 cusp: nonsquare d=2 leaves both actual nonsingular point groups of count5, giving10 rather than12. Both are direct universal negations. F7 addition compatibility is a closed kernel check on actual point types through a proved equation-point equivalence. [pcc:unverified]'),
 p('<b>Primary review.</b> <link href="https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch9.pdf" color="blue">Galbraith author version2 chapter9</link>, SHA f9a3272a…, selected Theorem9.3.4/Definition9.5.1/Example9.5.2/Exercises9.5.3–9.5.4; printedp171/PDF7 fully rendered and inspected. <link href="https://www.math.auckland.ac.nz/~sgal018/crypto-book/crypto-book.html" color="blue">Author landing/errata</link> checked, including adjacent9.5.8 correction; no novelty or previously-unreported-error claim. FST selected trace/7.3 background; Milne IV7 restriction char≠2,3 retained. KB/Conductor unavailable; bounded primary/official/prior fallback recorded. No full-book read or corpus promotion. [pcc:unverified]'),
 p(f'<b>Validation and scope.</b> Whole gate {g["declarations"]} declarations, zero failures, zero trusted native exceptions. All {a["files"]} build/provenance files retained losslessly in {a["unique_objects"]} objects; five gate files, complete statement and recursive source/config pins. Lean4.35.0/Mathlib56a00585…, standard foundations only. Exact local PCC replay differs from signed standing. General characteristic-three trace-to-geometric criterion is the next component; general Hasse/Endrank/embedding/full-torsion-field and remaining original fourteen-item gaps stay open. No holes/custom axioms/native evaluator/raised limits/new global flags/waivers/native experiment/paid compute/agent upload. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/quadratic-twist-count-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
