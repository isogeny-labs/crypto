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
rows=[(96,'Finite algebraic core',['G killed by q−1; Q = G / range(σ−1)','Invariant chars ≃ Hom(Q,K*); Q ≃ Hom(chars,K*)']),
      (50,'Actual F3 positive',['Nonsingular y²=x³−x; P=(0,0) has exact order 2','Actual two-torsion 4; characters 4; coinvariants 4; units 2']),
      (4,'Actual char2 control',['Ordinary y²+xy=x³+1; identity action','Characters 1 / coinvariants 2; source annihilation fails'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.8),(130,yy+25,lines[0],7.8),(130,yy+9,lines[1],8.1)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'finite-character-duality.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('Finite character duality behind the Tate pairing',t),
 p('<b>Result.</b> For every finite field K of size q, finite abelian group G killed by q−1 and automorphism σ, Lean constructs both evaluation isomorphisms between invariant K*-valued characters and the actual quotient Q=G/range(σ−1). Evaluation formulas, both trivial radicals and equal cardinalities are proved. No primitive root, basis, rank, supplied duality isomorphism or nondegeneracy premise. This is the finite character/coinvariant algebraic core; actual Galois H1 and canonical divisor pairings remain separate. [pcc:unverified]'),d,Spacer(1,6),
 p('<b>Construction and credit.</b> The actual finite field unit group supplies a primitive(q−1)th root. Source annihilation derives enough roots for the quotient exponent. The invariant-character subgroup descends to all quotient characters by explicit inverse lift/pullback maps. Credited Mathlib finite abelian biduality then constructs the reverse evaluation isomorphism, with its formula proved. The maps, source quotient and character groups are actual homomorphism/quotient types. [pcc:unverified]'),
 p('<b>Actual positive control.</b> Every finite odd field’s actual rational two-torsion is killed by q−1, so the result applies to any automorphism of that actual group. On F3, y²=x³−x has nonzero discriminant and actual point(0,0) of exact order2. Its actual rational two-torsion has four elements by the actual cubic-root equivalence. With identity action, invariant characters and coinvariants each have four elements, units have two, and evaluation is perfect. No assumed group table or pairing matrix. [pcc:unverified]'),
 p('<b>Actual failed-hypothesis control.</b> Over every finite characteristic-two field, the credited y²+xy=x³+1 model supplies an order-two point. Identity coinvariants have cardinality2 but invariant characters cardinality1. A constructed nonidentity quotient class is killed by every character. Lean proves source(q−1)annihilation fails and directly negates the universal right-nondegeneracy statement after deleting it. The qualified result is retained. [pcc:unverified]'),
 p('<b>Primary review.</b> <link href="https://www.numdam.org/article/JTNB_2011__23_2_323_0.pdf" color="blue">Peter Bruin, JTNB23(2011),323–328, DOI10.5802/jtnb.764</link>: all six article pages/proofs/references read; all seven PDF pages including cover fully rendered and inspected. Lemma2.1’s finite/annihilation conditions are distinguished from section3’s separability/Lang/Cartier steps and Theorem4.1’s divisor comparison. Those bridges are not assumed or proved here. The original FR1994 remains unread after separate author403/503 and archive503 routes; Bruin is a separate primary source. Cited books/papers are not automatically marked read. [pcc:unverified]'),
 p(f'<b>Validation and remainder.</b> Wholegate{g["declarations"]}/0failures/0trusted native exceptions; all{a["files"]} build/processing/provenance files losslessly retained in{a["unique_objects"]}objects, five gate files and recursive source/config pins. Lean4.35.0/Mathlib56a00585…, standard foundations/default limits. Exact local PCC replay differs from signed standing. KB/Conductor unavailable; bounded fallback recorded. Continuous H1/Lang/isogeny divisibility/Cartier/polarisation/canonical Weil-Tate-MOV-FR and other fourteen-item gaps remain open. No novelty/corpus promotion, holes/custom axioms/native evaluation/raised limits/new global flags/waivers/native experiment/paid compute/upload. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/finite-tate-duality-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
