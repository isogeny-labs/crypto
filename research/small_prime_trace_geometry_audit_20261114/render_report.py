"""Layout recorded formal model counts and scopes; no research computation."""
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
d=Drawing(500,109);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="109" viewBox="0 0 500 109">']
for yy,title,lines in [(59,'All F2 models',['32 raw / 16 nonsingular / 8 source-SS models','SS #E = 1, 3 or 5 → every count-divisor order | 4']),(5,'All F3 models',['243 raw / 162 nonsingular / 54 source-SS models','SS #E = 1, 4 or 7 → every count-divisor order | 6'])]:
 d.add(Rect(0,yy,500,45,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{109-yy-45}" width="500" height="45" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+24,title,9.5),(112,yy+28,lines[0],8.4),(112,yy+11,lines[1],8.2)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{109-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'all-model-trace-geometry.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('All-model F2/F3 trace, geometry and modular-order audit',t),
 p('<b>Result.</b> Kernel-check every general coefficient model over F2 and F3. For every nonsingular model, the actual source gcd(trace,q)≠1 predicate equals trivial geometric p-torsion. Actual Hasse inequalities and trace-square cases are proved here, with no Hasse/classification premise. Every actual point-count divisor r, including composite divisors, has gcd(r,q)=1 and modular order dividing4 over F2 or6 over F3. Generic actual point/equation-count semantics hold over every finite field. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>Actual enumeration and semantics.</b> A coefficient-five-tuple equivalence covers every Weierstrass model. An actual nonsingular-point equivalence proves group cardinality equals the number of literal equation pairs plus the identity under nonzero discriminant; the actual integer trace equals the counted trace. Closed kernel tables prove all F2/F3 coefficient, point-count and Hasse cases. Counts include isomorphic coefficient duplicates. The earlier native 16+162 nonsingular-model baseline is credited; its execution is not promoted by these independent formal semantics. [pcc:unverified]'),
 p('<b>Source and geometric bridges.</b> The source trace predicate is equivalent to a1=0 over F2 and b2=0 over F3. The preceding actual geometric j criterion identifies these with geometric killed-p subgroup cardinality1. Actual SS point-count cases derive the source trace-square alternatives. The older conditional modular-order arithmetic is credited; this packet supplies classification only over F2/F3. Root-of-unity powers give order-divisor bounds for every count divisor, without asserting every divisor occurs as a cyclic point order. General field-degree/full torsion-field bridges remain separate. [pcc:unverified]'),
 p('<b>Explicit positive controls.</b> F2:y²+y=x³+x has five actual rational points, P=(0,0) of exact order5, geometric two-torsion count1 and orderOf(2mod5)=4. F3:y²=x³+2x+1 has seven actual rational points, P=(0,1) of exact order7, geometric three-torsion count1 and orderOf(3mod7)=6. Both satisfy the actual source trace predicate and nonzero discriminant. Exact point orders use actual prime group cardinality/Lagrange. They lie outside the source p≥5 clause and do not refute it. [pcc:unverified]'),
 p('<b>Primary review.</b> <link href="https://link.springer.com/content/pdf/10.1007/s00145-009-9048-z.pdf" color="blue">FST2010 pp230/237, Definition2.1/Remark2.2/Section3</link>: exact trace predicate, embedding conditions, trace-square cases and p≥5 qualification retained. <link href="https://www.jmilne.org/math/Books/EC2.pdf" color="blue">Milne, Elliptic Curves, IV9/Theorem9.4</link>, printedp150: general Hasse proof uses degree/separability and is not assumed here. FST PDFpages7/14 and Milne PDF155 fully inspected; no full-paper/book read or corpus promotion. KB/Conductor unavailable, bounded fallback recorded. [pcc:unverified]'),
 p(f'<b>Validation and remaining scope.</b> Whole gate{g["declarations"]}/0failures/0trusted native exceptions. Complete statement binding/recursive source-config pins. All{a["files"]}attempt-provenance files retained losslessly in{a["unique_objects"]}objects; five gate records, including failed default-limit attempts. Pinned Lean4.35.0/Mathlib56a00585…, only standard foundations. Exact local PCC replay differs from signed standing. General prime/extension fields, Hasse/endomorphism-rank/geometric identifications beyond this cohort, other BK primes/full all-k p&lt;500 census and original fourteen-item gaps remain open. No novelty, qualified-source refutation, native experiment, paid compute, upload, holes/custom axioms/native evaluator/raised limits/new global flags/waivers. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/small-prime-trace-geometry-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
