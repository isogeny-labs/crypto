"""Render recorded proof scopes and counts, never research computation."""
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
b=ParagraphStyle('b',fontName='DV',fontSize=8.8,leading=12,spaceAfter=11)
t=ParagraphStyle('t',fontName='DVB',fontSize=16,leading=20,spaceAfter=12)
def p(s):return Paragraph(s,b)
d=Drawing(500,117);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="117" viewBox="0 0 500 117">']
for y,txt,w,col,n in [(84,'char3: trivial killed-three group',25,'#2c788e','1'),(52,'char3: nontrivial killed-three group',75,'#2c788e','3'),(20,'Original BK count required',225,'#bd6248','9')]:
 d.add(String(0,y,txt,fontName='DV',fontSize=8.7));d.add(Rect(225,y-4,w,15,fillColor=colors.HexColor(col),strokeColor=None));d.add(String(233+w,y,n,fontName='DV',fontSize=9))
 svg.append(f'<text x="0" y="{117-y}" font-size="8.7" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text><rect x="225" y="{117-y-11}" width="{w}" height="15" fill="{col}"/><text x="{233+w}" y="{117-y}" font-size="9" font-family="DejaVu Sans,sans-serif">{n}</text>')
svg.append('</svg>');(o/'char3-count-source.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('Characteristic-three torsion: one or three, never nine',t),
 p('<b>Result.</b> Every nonsingular general Weierstrass model over every characteristic-three field has exactly one or three actual points killed by three. Its actual torsion subgroup is finite and its induced ZMod3 module has rank zero or one. No algebraic closure, normalization or torsion-rank premise. Credited actual algebraic-closure controls realize both counts. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>Original source at prime three.</b> For every finite base field and nonsingular model, assume 3 divides the actual rational point-group cardinality and 3 does not divide q−1. Every chosen compatible extension of cardinality q^k has exactly nine points killed by three, including the identity, iff 3 divides q^k−1. No extra coprimality or characteristic premise is added. The qualified source survives this check; inclusion of a rank-dropped torsion subgroup remains a different condition. [pcc:unverified]'),
 p('<b>Actual classification.</b> If a nonzero killed-three point exists, use the preceding actual tangent normalization and point-group AddEquiv. The resulting curve y²+a*x*y+b*y=x³ has discriminant b³a³ and literal division-polynomial value a²x³ in characteristic three. Nonsingularity forces a,b nonzero, the polynomial forces x=0, and the equation forces y=0 or−b. An explicit actual subgroup/OptionBool bijection proves count3. With no nonzero point the actual subgroup is a singleton. Finiteness and the actual induced module rank follow from these counts. [pcc:unverified]'),
 p('<b>Why no added coprimality.</b> If 3≠0 in the base field, finite additive Cauchy and no zero divisors imply gcd(3,q)=1; apply the preceding all-model qualified criterion. If 3=0, the new classification makes the nine-point side false. The additive order of 1 gives 3 dividing q, so cyclotomic divisibility is false for every positive k. The chosen field cardinality rules out k=0. The earlier ordinary characteristic-three inclusion counterexample remains valid and does not refute this original count formulation. [pcc:unverified]'),
 p('<b>Sources and controls.</b> <link href="https://link.springer.com/content/pdf/10.1007/s001459900040.pdf" color="blue">BK1998 Theorem1/proof/Remarks pp142–143</link>, DOI10.1007/s001459900040; <link href="https://web.math.pmf.unizg.hr/~duje/pdf/section15-3.pdf" color="blue">Dujella section15.3,p494</link>, normalization discussed over Q. Prior general rank/actual point criterion, tangent normalization, character criterion, short characteristic-three count1 and ordinary a=b=1 count3/rank1 credited. No full-paper/book read, novelty, source refutation or corpus promotion. KB/Conductor unavailable; bounded primary/repository/Mathlib fallback recorded. [pcc:unverified]'),
 p(f'<b>Validation and remaining scope.</b> Whole gate {g["declarations"]}/0failures/0trusted native exceptions; complete statement binding/recursive source-config pins. All{a["files"]}attempt-provenance files retained losslessly in{a["unique_objects"]}objects, including failures; five gate records. Initial build01 lacks a manifest snapshot while lake update finished; later changed builds have complete snapshots. Pinned Lean4.35.0/Mathlib56a00585…, only standard foundations. Exact local PCC replay is separate from signed standing. Other primes≥5, canonical Weil, full all-k p&lt;500 census and original fourteen-item gaps remain open. No formal supersingularity/trace classification, native experiment, paid compute, upload, holes/custom axioms/native evaluator/raised limits/new global flags/waivers. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/three-torsion-char3-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
