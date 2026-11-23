"""Layout proved algebra statements and provenance; no research computation."""
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
b=ParagraphStyle('b',fontName='DV',fontSize=9,leading=12.3,spaceAfter=11)
t=ParagraphStyle('t',fontName='DVB',fontSize=16.5,leading=21,spaceAfter=13)
def p(s):return Paragraph(s,b)
d=Drawing(500,142);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="142" viewBox="0 0 500 142">']
rows=[(96,'Actual quotients',['A = K[t]/(t^m); B = K[x,y]/(x^m,y-x)','Self-module length(A)=length(B)=m for every field K']),
      (50,'Local support',['Positive m: A and B local; exactly one geometric point','Arbitrarily large ring length at support1; m=0 nonlocal']),
      (4,'Residue control',['F4 over F2: field, hence local; self-module length1','F2-module length2; locality alone does not equate them'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.4),(122,yy+25,lines[0],7.8),(122,yy+9,lines[1],7.8)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'local-length.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0]['full_decl_sha256'];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Actual local-algebra length and residue-degree control',t),
 p('<b>Result.</b> Lean proves actual ring-module length m for K[t]/(t^m) and the literal bivariate quotient K[x,y]/(x^m,y-x), for every field K and natural m. For positive m these are local rings with exactly one solution over every field extension. Their ring length can be arbitrarily large while geometric support remains one. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>Derived ideal chain.</b> A generic theorem starts from an actual power basis whose generator is nilpotent at the basis dimension. Tail spans of its powers are proved closed under multiplication by every algebra element, giving actual ideals. Independence supplies a strict ideal chain of length dim. Restricting scalars embeds the ring-submodule lattice into the field-subspace lattice, giving the upper bound; together these prove the actual self-module length. No module-length or local-structure conclusion is assumed. [pcc:unverified]'),
 p('<b>Actual applications and locality.</b> The monic AdjoinRoot power basis supplies K[t]/(t^m); its map through the previously proved actual bivariate quotient equivalence supplies K[x,y]/(x^m,y-x). Every element of the truncated algebra is a scalar plus a nilpotent term. A nonzero scalar gives a unit; zero gives nilpotence, yielding locality for positive m. Unchanged support proofs apply over every extension field. At m=0 both quotients are zero rings with length0, and are not local rings under Mathlib convention. [pcc:unverified]'),
 p('<b>Negative control.</b> Actual F4 over F2 is a field and a local ring. Lean proves cardinality4, scalar dimension2, self-module length1, scalar-module length2 and the direct inequality. Thus a local algebra need not have scalar length equal to its ring length; a residue-field degree factor matters. This control guards the change from the earlier scalar-length result to the present actual ring-length result. [pcc:unverified]'),
 p('<b>Boundary.</b> This closes the self-module length and positive locality gap for this actual truncated/bivariate family. General scheme cycles, localization/intersection multiplicities, isolated points among positive-dimensional components, Diem’s qualified n!d^n bound and the separate JV ring-length8 remain open. No qualified source theorem is refuted. [pcc:unverified]'),
 p(f'<b>Checks.</b> Whole new-project gate {g["declarations"]}/0 failures/0 trusted native; only propext, Classical.choice and Quot.sound. Complete771-character written binding {binding}, independently parsed1complete/0prefix/0unresolved. Native parser/hash evidence is procedural, not kernel refinement. Five build snapshots including three failures: {a["files"]}files/{a["unique_objects"]}objects/{size}-byte lossless XZ, plus final gate files. [pcc:unverified]'),
 p('<b>Sources and assurance.</b> Diem2011Prop2.5 printed79–81/PDF6–8 fully read and visually inspected; 31-page PDFSHA74902b58…, no fullpaper claim. Stacks00IY/00J2/06GG selected statements/proofs read, residue-degree qualification retained. Prior actual coordinate-length study and official Mathlib Length/PowerBasis/AdjoinRoot/LocalRing/Nilpotent/GaloisField proofs credited; classical filtration method, no novelty claim. KB/Conductor unavailable, primary/prior/official fallback. Lean4.35.0/Mathlib56a00585…, standard defaults, no holes/custom axioms/native evaluator/flags/limits/waivers/new crypto experiment/spend/upload. Signed standing unverified. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/local-length-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
