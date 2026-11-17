"""Render mathematical statements and provenance; no research computation."""
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
b=ParagraphStyle('b',fontName='DV',fontSize=8.9,leading=11.7,spaceAfter=8)
t=ParagraphStyle('t',fontName='DVB',fontSize=16.5,leading=21,spaceAfter=13)
def p(s):return Paragraph(s,b)
d=Drawing(500,120);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="120" viewBox="0 0 500 120">']
rows=[(81,'Actual C21 module',['F2[X]/(X^6+X^4+X+1): dimension6, cardinality64','Root unit has exact order21; action faithful and cyclic']),
      (42,'Counterexample',['Kernel of evaluation at1: proper, nonzero, invariant','Dimension=ord_21(2)=6 does not force irreducibility']),
      (3,'Proved repair',['Add f divides Phi_n and degree=ord_n(2), for odd n','Prime5/7 controls pass; actual composite example excluded'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,34,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{120-yy-34}" width="500" height="34" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+17,title,8.1),(116,yy+21,lines[0],7.6),(116,yy+7,lines[1],7.6)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{120-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'diem-cyclic.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0]['full_decl_sha256'];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Diem: a composite-degree proof gap',t),
 p('<b>Exact target.</b> The archived March24,2003 draft of The GHS Attack in odd Characteristic starts Section7 with general odd n. Lemma22 invokes Lemma17, which was stated for odd prime n. Lean disproves the composite-degree algebraic inference: a faithful cyclic action of order n on a cyclic F2-module of dimension ord_n(2) need not be irreducible. This is a proof-step counterexample, not a constructed GHS function-field counterexample or a refutation of the norm-conorm kernel theorem. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>Actual objects.</b> Let g=X⁶+X⁴+X+1=(X+1)(X²+X+1)(X³+X+1), and V=F2[X]/(g), the actual AdjoinRoot quotient ring. Its actual power basis gives dimension6 and cardinality64. An explicit polynomial division identity proves x²¹=1, while lower-degree obstructions exclude x³=1 and x⁷=1. Hence x is an actual unit of exact order21. Multiplication by its actual cyclic subgroup gives a faithful Mathlib representation. The orbit of1 spans V, and1 generates the actual F2[C21] module. No cyclicity, faithfulness or rank conclusion is assumed. V is not a field. [pcc:unverified]'),
 p('<b>Actual obstruction.</b> The kernel of evaluation at1 contains x+1, proved nonzero by a polynomial degree argument, and does not contain1. Multiplication preserves this kernel, so it is a proper nonzero invariant subspace. Mathlib’s actual invariant-submodule/group-algebra order isomorphism supplies a proper nonzero F2[C21] submodule. Lean also directly negates the zero-or-whole invariant-subspace claim. The actual multiplicative order of2 modulo21 is6;21 is odd, coprime to2 and composite. [pcc:unverified]'),
 p('<b>Sufficient repair and controls.</b> For every odd n, add the hypothesis that the actual quotient polynomial f divides the n-th cyclotomic polynomial and has degree ord_n(2). The official finite-field cyclotomic-factor theorem derives irreducibility; Lean then derives that every root-invariant subspace is zero or whole. The actual counterexample g does not divide Phi21. Actual irreducible quotient controls for n5 and n7 have dimensions4 and3, matching ord_5(2) and ord_7(2); both satisfy the repair and have no proper root-invariant subspace. The source’s prime applications are not refuted. [pcc:unverified]'),
 p('<b>Boundary.</b> No construction here realizes V by the paper’s regular function fields or verifies constant-field descent, Kummer pairing, intermediate-field correspondence, source circulant coordinates, genus, class groups, Jacobians or attack costs. Thus geometric Lemma22 and the norm-conorm kernel theorem remain open. No novelty or canonical corpus promotion. The exact archived33-page draft is distinguished from the journal’s32-page citation; published-version equivalence is not asserted. [pcc:unverified]'),
 p(f'<b>Checks and retained failures.</b> Whole project gate{g["declarations"]}/0failures/0trusted native; only propext/Classical.choice/Quot.sound. Complete1037-character binding{binding}; independent native1complete/0prefix/0unresolved. Nine snapshots retain six failed build states, plus a failed scratch exploration: {a["files"]}files/{a["unique_objects"]}objects/{size}-byte losslessXZ plus five gate files. Final09 is advisory-free. An early update-hook/build overlap is recorded; subsequent builds followed completed update. Lean4.35.0/Mathlib56a00585…/defaults; no holes/customaxioms/nativeproofeval/raisedlimits/globalflags/waivers/frozenedits/cryptoPython/newcryptoexperiment/spend/upload. Native header processing is procedural, not kernel refinement. Signed standing unverified. [pcc:unverified]'),
 p('<b>Sources checked.</b> Archived author PDF20220120091946,355922bytes/SHA7c25804a…/33pages. All33 OCR pages inclappendix/references read; PDF8/10/13/18/23 visuallychecked. Broken embedded-font extraction was not counted as reading; OCR does not verify every formula, matrix or numerical example. Copyright PDF/OCR/images remain local. Prior census/Momose work and official Mathlib quotient/power-basis/representation/order/cyclotomic proofs read. Fresh KB/Conductor127; prior/primary/official fallback. Bounded exact-lemma/composite/erratum search is not proof of absence of prior work. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/diem-cyclic-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
