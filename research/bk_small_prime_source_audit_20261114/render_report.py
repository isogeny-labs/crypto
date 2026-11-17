"""Layout recorded formal results; no research computation."""
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
b=ParagraphStyle('b',fontName='DV',fontSize=8.9,leading=12.2,spaceAfter=11)
t=ParagraphStyle('t',fontName='DVB',fontSize=16,leading=20,spaceAfter=12)
def p(s):return Paragraph(s,b)
d=Drawing(500,108);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="108" viewBox="0 0 500 108">']
for yy,title,lines in [(58,'Prime two',['q−1 condition → characteristic two','actual count 1 or 2 ≠ required 4']),(4,'Prime three',['canonical closure of chosen K','original source theorem → count 9 iff 3 | q^k−1'])]:
 d.add(Rect(0,yy,500,46,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')))
 svg.append(f'<rect x="0" y="{108-yy-46}" width="500" height="46" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(10,yy+25,title,10),(112,yy+28,lines[0],8.7),(112,yy+11,lines[1],8.7)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{108-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'direct-small-prime-source.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('Original BK count criteria, directly over finite fields',t),
 p('<b>Result.</b> Primes r=2 and r=3, every nonsingular general Weierstrass model over every finite field F, every finite extension field K with its actual F-algebra structure and cardinality q^k. Retain the source hypotheses r dividing the actual rational point-group cardinality and r not dividing q−1. Exactly r² actual K-points killed by r, including the identity, iff r divides q^k−1. No supplied auxiliary algebraic closure, embedding, scalar tower, coprimality, rank, determinant, pairing or normal-form premise. The qualified source survives these checks. [pcc:unverified]'),d,Spacer(1,9),
 p('<b>Prime three.</b> Construct the canonical algebraic closure of the chosen K inside the proof. Mathlib already supplies the canonical F-algebra and scalar tower. These discharge the auxiliary parameters of the preceding original-source prime-three criterion. That proof handles arbitrary nonsingular models and characteristic three, where the actual torsion count is one or three and both nine-point count and cyclotomic divisibility are false. The public statement quantifies only F and K. [pcc:unverified]'),
 p('<b>Prime two.</b> The source q−1 condition forces 2 dividing q. Finite additive Cauchy and no zero divisors force characteristic two. Existing actual nonzero killed-two point uniqueness now gives a one-or-two count over every characteristic-two field, with no algebraic closure premise. The actual subgroup is finite and never has four points. Cyclotomic divisibility is false for positive k, while the chosen field cardinality excludes k=0. The public source point-count divisibility premise is preserved, although the prime-two proof does not need it. [pcc:unverified]'),
 p('<b>Actual controls and prior work.</b> Credited characteristic-two models have discriminant1 and actual killed-two subgroup counts2 and1 over every characteristic-two field. An actual point of order two in the ordinary model makes its rational group cardinality even over every finite such field. Previous two-torsion geometry, general three-torsion rank, actual normalization, characteristic-three classification and original-source criterion are reused, not presented as new. [pcc:unverified]'),
 p('<b>Primary source.</b> <link href="https://link.springer.com/content/pdf/10.1007/s001459900040.pdf" color="blue">BK1998 Theorem1/proof/Remarks pp142–143</link>, DOI10.1007/s001459900040. Original count includes the identity; the inclusion paraphrase remains coprime-qualified. Selected source sections and pinned Mathlib closure/tower APIs read; no full-source-read, novelty, source refutation or corpus promotion. KB/Conductor unavailable, bounded primary/repository/library fallback recorded. [pcc:unverified]'),
 p(f'<b>Validation and remaining scope.</b> Whole gate{g["declarations"]}/0failures/0trusted native exceptions; complete statement binding/recursive source-config pins. All{a["files"]}attempt-provenance files retained losslessly in{a["unique_objects"]}objects; five gate records. Failed manual-composition attempt retained; successful proof reuses canonical instances. Pinned Lean4.35.0/Mathlib56a00585…, standard foundations only. Exact local PCC replay is separate from signed standing. Other primes≥5, canonical Weil, full all-k p&lt;500 census, formal supersingularity/trace classification and original fourteen-item gaps remain open. No native experiment, paid compute, upload, holes/custom axioms/native evaluator/raised limits/new global flags/waivers. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/bk-small-prime-source-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
