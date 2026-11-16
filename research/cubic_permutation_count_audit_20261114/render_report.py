"""Layout checked formal statements and provenance; no research computation."""
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
t=ParagraphStyle('t',fontName='DVB',fontSize=16.4,leading=21,spaceAfter=13)
def p(s):return Paragraph(s,b)
d=Drawing(500,142);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="142" viewBox="0 0 500 142">']
rows=[(96,'Derived permutation',['q mod3=2; e=(2q−1)/3; 3e=2(q−1)+1','Actual cube inverse x^e, including x=0']),
      (50,'Actual point count',['E_b: y²=x³+b, char≠2,3 and b≠0','E_b.Point ≃ Option K; #E_b=q+1; trace0']),
      (4,'Actual point orders',['Every divisor r>2 of #E_b has ord_r(q)=2','Lagrange derives the result for each actual point order'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.3),(128,yy+25,lines[0],8),(128,yy+9,lines[1],8)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'cubic-family.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0]['full_decl_sha256'];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Exact counts and embedding orders for a cubic family',t),
 p('<b>Result.</b> Over every finite field K of cardinality q with q mod3=2, characteristic different from2 and3, and b nonzero, Lean proves the actual elliptic curve y²=x³+b is nonsingular and has exactly q+1 points. The point-count trace is0. Every divisor r&gt;2 of its actual cardinality, including composite r, has modular embedding order2. The proof also derives this for every actual point of order&gt;2. [pcc:unverified]'),d,Spacer(1,7),
 p('<b>Proof and scope.</b> Derive the inverse cube exponent and its Fermat identities at zero and nonzero elements. For each y there is exactly one x, the cube root of y²−b. A literal equivalence between actual nonsingular Mathlib points and Option K supplies the cardinality; no count or equivalence is assumed. The discriminant is proved nonzero. Lagrange supplies each actual point-order divisor, and q≡−1 modulo that order gives exact order2 when r&gt;2. For every prime p≥5 with p mod3=2, the specialization derives both characteristic guards; no upper p bound. [pcc:unverified]'),
 p('<b>Omitted-condition controls.</b> Actual nonsingular-point counts are5 for F5,b=0 (versus predicted6),2 for F2,b=1 (versus3), and12 for F7,b=1 (versus8). Lean directly negates the corresponding coefficient/characteristic/congruence strengthenings. The actual F5,b=1 curve has6points and r=2 divides its count, yet ord₂(5)=1, directly negating exact2. These are omitted-hypothesis controls, not refutations of the qualified family statement. Prior F5 cusp/r2 and the textbook F7 count are credited. [pcc:unverified]'),
 p('<b>What stays open.</b> This proves the textbook count family and its trace/order consequences. It does not prove the general equivalence between trace, geometric p-torsion, p-rank or endomorphism definitions of supersingularity, general Hasse, canonical full torsion fields or the whole prime-field source theorem. The modular order is distinguished from unproved geometric full-torsion inclusion. The original fourteen-item request remains partial. [pcc:unverified]'),
 p(f'<b>Checks and binding.</b> Whole project gate{g["declarations"]}/0 failures/0 trusted native; only propext, Classical.choice and Quot.sound. Complete explicit1230-character main type hash{binding}; independent native official parser1complete/0prefix/0unresolved. The parser/hash observation is procedural, not a kernel refinement theorem. Six source/config build snapshots, including every failure, are preserved: {a["files"]}files/{a["unique_objects"]}objects/{size}-byte lossless XZ archive plus five final gate files. [pcc:unverified]'),
 p('<b>Sources and reproducibility.</b> Galbraith author chapter9, Exercise9.10.4p184 and Theorem9.11.2/Definition9.11.3/Example9.11.4p189 read; both selected pages visually inspected. PDF SHA f9a3272a…; no fullchapter/bookread or novelty claim. Official Mathlib finite-field Fermat, affine Point equivalences and ZMod order APIs; credited prior CompositeTorsionAudit minus-one argument. KB/Conductor unavailable, primary/prior/official fallback. Lean4.35.0/Mathlib56a00585…, standard defaults; no holes/custom axioms/native proof evaluation/flags/limits/waivers/new crypto experiment/spend/upload. Signed standing remains unverified. Source-linked proof map and pinned sources accompany this report. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/cubic-permutation-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
