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
rows=[(96,'Every finite char2',['Source gcd(trace,q)≠1 ↔ a1=0 ↔ j=0','↔ odd #E ↔ rational/geometric two-torsion count 1']),
      (50,'Drop finiteness',['F2(X): y²+xy=x³+X, Δ=X≠0, j≠0','Rational two-torsion count 1; geometric count 2']),
      (4,'Change to char3',['Credited F3 curve: actual #E=5','Rational three-torsion count 1; source predicate false'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.8),(130,yy+25,lines[0],8.1),(130,yy+9,lines[1],8.1)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'finite-char2-trace.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('Every finite characteristic-two field: trace and geometric torsion',t),
 p('<b>Result.</b> For every finite field of characteristic two and every nonsingular general Weierstrass model, Lean proves the exact source gcd(q+1−#E,q)≠1 predicate iff a1=0 iff j=0 iff odd actual point count iff rational two-torsion cardinality one iff geometric two-torsion cardinality one. No field-size or extension-degree bound, Hasse, trace-square classification or endomorphism-rank premise. [pcc:unverified]'),d,Spacer(1,6),
 p('<b>Actual finite-field bridge.</b> If a1≠0, a nonzero point killed by two has x=−a3/a1. Its equation becomes a square-root equation; the official finite reduced-ring square theorem supplies its ordinate. Actual Cauchy and Lagrange give even point count iff a1≠0. Official FiniteField.card supplies q=2ⁿ, n&gt;0; coprime-power/parity arithmetic connects this to the source trace predicate. The preceding actual geometric j theorem supplies the final geometric interpretation. [pcc:unverified]'),
 p('<b>Modules and scalar extensions.</b> Actual rational two-torsion has cardinality one/two and induced ZMod2 rank zero/one according as j=0/nonzero. Its cardinality equals geometric two-torsion cardinality over every algebraically closed scalar extension. The source predicate is invariant under every finite field extension, using the actual scalar j identity. Prior group, module, Cauchy, geometric j and finite-field perfectness proofs are credited. [pcc:unverified]'),
 p('<b>Negative controls.</b> Over F2(X), y²+xy=x³+X has Δ=X≠0, a1=1 and j≠0. Rational two-torsion count is one, geometric count two. X is not a square because a square has even integer degree; Lean directly negates ordinary rational descent after deleting finiteness. The credited F3 curve y²=x³+2x²+1 has five actual rational points/rational three-torsion count one/source predicate false; Lean directly negates changing characteristic two to three in the rational criterion. These test hypotheses, not qualified-source theorems. [pcc:unverified]'),
 p('<b>Primary review.</b> <link href="https://link.springer.com/content/pdf/10.1007/s00145-009-9048-z.pdf" color="blue">FST2010 selected pp230/237</link>: exact trace predicate and embedding qualifications. <link href="https://www.jmilne.org/math/Books/EC2.pdf" color="blue">Milne Remark3.16/context pp98–99 and p75 notes</link>: geometric p-rank versus endomorphism-rank distinction; prior PDF104/80 inspections credited. Official Mathlib finite reduced-ring squares, finite-field cardinality, rational-function degree and j identities read. KB/Conductor unavailable; bounded fallback preceded code. No full-paper/book read, novelty or corpus promotion. [pcc:unverified]'),
 p(f'<b>Validation and scope.</b> Whole gate {g["declarations"]} declarations, zero failures, zero trusted native exceptions. All {a["files"]} build/provenance files retained losslessly in {a["unique_objects"]} objects; five gate files, complete statement and recursive source/config pins. Lean4.35.0/Mathlib56a00585…, standard foundations only. One unsuppressed legacy/modern-module compatibility advisory remains; attempted module migration failed against frozen legacy imports and was reverted. Exact local replay differs from signed standing. General characteristic-three/other-prime trace-geometric and endomorphism-rank identifications, Hasse/embedding bounds and remaining original fourteen-item gaps stay open. No qualified-source refutation, holes/custom axioms/native evaluator/raised limits/new global flags/waivers/native experiment/paid compute/agent upload. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/finite-char-two-trace-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
