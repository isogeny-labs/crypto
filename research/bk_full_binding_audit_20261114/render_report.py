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
b=ParagraphStyle('b',fontName='DV',fontSize=9,leading=12.6,spaceAfter=12)
t=ParagraphStyle('t',fontName='DVB',fontSize=17,leading=21,spaceAfter=14)
def p(s):return Paragraph(s,b)
d=Drawing(500,142);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="142" viewBox="0 0 500 142">']
rows=[(96,'Frozen original',['Actual E/F5, #E=6, r=3; all-k proof retained','Issued receipt:176chars; full old written header913chars']),
      (50,'Additive wrapper',['Same theorem, explicit L and field assumptions','Actual rational / chosen-extension / GaloisField maps']),
      (4,'Two checks',['Kernel proof gate:1 / 0 failures / 0 trusted native','Independent official parser:934chars, complete hash match'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.5),(121,yy+25,lines[0],8),(121,yy+9,lines[1],8)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'bk-full-binding.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Complete binding for the actual F₅ BK certificate',t),
 p('<b>Result.</b> BKFullBindingAudit.full_bk_certificate writes the outer algebraically closed field L, Field/Algebra/DecidableEq assumptions and complete actual-object conclusion explicitly. It reuses the unchanged F₅ proof clause by clause. This adds a faithful receipt; it is not a new general Balasubramanian–Koblitz theorem. [pcc:unverified]'),d,Spacer(1,9),
 p('<b>Mathematical content.</b> For E/F₅: y²=x³+1, Lean proves nonsingularity, #E(F₅)=6 and the source conditions for r=3, including 3 not dividing 5−1. For every k, all geometric3-torsion points are fixed by Frobenius to the k exactly when 3 divides 5^k−1. An actual order3 point satisfies Frobenius(P)=−P≠P and lies outside the rational point image. For every compatible chosen finite extension K of cardinality5^k, the full geometric3-torsion lies in its actual point-map image exactly under the same divisibility condition. For every positive k an actual GaloisField5k embedding and cardinality are supplied. No rank, determinant, Hasse or pairing conclusion is assumed. [pcc:unverified]'),
 p('<b>Binding repair.</b> The old issued PCC-20261110-99983d stores a176-character prefix ending inside a named argument; the complete old written header has913characters. Original claim, source and whole37-declaration proof receipt stay unchanged. The new934-character written type includes its field telescope and contains no embedded named-argument assignment or bar delimiter. Its SHA256 is255334b4de88ddc92803431efa042b40aa02db63affd414666c7a20c01cb02ed. An independent native official Lean parser resolves exactly1complete header/0prefix/0unresolved and matches that hash. This procedural check is not proved by the wrapper theorem. [pcc:unverified]'),
 p('<b>Definitions and proof.</b> ActualPointMaps defines the three maps as Mathlib affine Point.map with explicit parameters. FullBKCertificate applies the frozen actual theorem, retaining all quantifiers, including positive k for the GaloisField clause. Recursive custom-source/config pins supplement the written signature; global semantic type/environment binding and the canonical shared-parser repair remain open. General r≥5, canonical Weil necessity and the complete all-k census are separate remaining BK work. [pcc:unverified]'),
 p(f'<b>Checks and reproducibility.</b> New-project whole gate{g["declarations"]}/0 failures/0 trusted native; printed transitive foundations are propext, Classical.choice and Quot.sound. Three builds, including two failed elaboration states, and all source/config/native protocol/input/output provenance are retained: {a["files"]}files/{a["unique_objects"]}objects, lossless XZ archive{size}bytes, with the five-file final gate. Build03 resolves duplicate-instance and conjunction elaboration issues without weakening the target. Lean4.35.0/Mathlib56a00585…, default settings; no holes/custom axioms/native proof evaluation/flags/limits/waivers/new cryptographic experiment/spend/upload. Signed standing remains unverified. [pcc:unverified]'),
 p('<b>Sources.</b> Original BK1998 Theorem1/conditions/proof pp141–143, earlier full141–145 read receipt credited; source PDF hash ac837b8e…. Frozen ThreeTorsion/QuadraticWitness/EmbeddingCriterion/GaloisFieldCase/BKCertificate and official Mathlib affine Point.map816–850 read. Fresh KB status/two mechanism-negative queries/exact theorem query all127; primary/repository/official fallback. No novelty claim. Source-linked proof map, sources.json and evidence/source-review.json accompany this report. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/bk-full-binding-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
