"""Render checked statements and provenance; no research computation."""
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
b=ParagraphStyle('b',fontName='DV',fontSize=9,leading=12.3,spaceAfter=9)
t=ParagraphStyle('t',fontName='DVB',fontSize=16.5,leading=21,spaceAfter=13)
def p(s):return Paragraph(s,b)
d=Drawing(500,120);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="120" viewBox="0 0 500 120">']
rows=[(81,'Actual characters','All-Gamma equivariant M → Omega units','Unique descent to K units, using annihilation and fixed fields'),
      (42,'Actual classes','All continuous crossed cocycles / all principal cocycles','Frobenius evaluation gives the actual coinvariant group'),
      (3,'Perfect / control','Both duality isomorphisms and exact source formula','Characteristic 2, M=Z/2: characters trivial, classes nonzero')]
for yy,title,a,c in rows:
 d.add(Rect(0,yy,500,34,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')))
 svg.append(f'<rect x="0" y="{120-yy-34}" width="500" height="34" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+17,title,8.1),(116,yy+21,a,7.6),(116,yy+7,c,7.6)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{120-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'galois-pairing.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Actual Galois characters: perfect finite pairing',t),
 p('<b>Proved qualified case.</b> For every finite field K of order q, every finite Abelian M with an actual continuous Galois action and exponent dividing q−1 has the finite-module pairing displayed in Bruin2011 Lemma2.1. Lean constructs both perfectness isomorphisms, proves the exact formula on every representative, bilinearity, both trivial radicals and equal cardinality. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>Actual character domain.</b> Omega is AlgebraicClosure(K); Gamma is its actual K-automorphism group with the Krull topology. Characters are all homomorphisms M→Omega units equivariant under every Gamma element. A unit killed by q−1 is Frobenius-fixed and therefore comes from K. This proves unique descent for every character. Conversely, invariance under Frobenius implies invariance under all Gamma by proved density and continuity. No descent or equivariance conclusion is assumed. [pcc:unverified]'),
 p('<b>Actual class domain and formula.</b> The second factor is the quotient of all continuous crossed cocycles by the complete image of all principal cocycles. Credited prior proofs identify this actual quotient with Frobenius coinvariants. Finite-field character descent and biduality then give both multiplicative equivalences to the other factor’s K-unit character group. The pairing satisfies baseMap(pair(chi,[c])) = chi(c(Frob)), in actual Omega units. This proves the displayed finite-module pairing for these standard cocycle-class definitions. [pcc:unverified]'),
 p('<b>Hypothesis deletion fails in Lean.</b> For every finite K of characteristic2, take M=Multiplicative(ZMod2) with trivial action. Actual Omega units have no nontrivial square root of1, so every character is trivial. The proved continuous-class/coinvariant equivalence produces an actual nonzero cocycle class. Consequently no bilinear homomorphism on these same character/class domains can be right-nondegenerate. The annihilation condition is false. This refutes the hypothesis-deleted statement and retains the qualified source lemma. [pcc:unverified]'),
 p('<b>Boundary and reuse.</b> Seven prior formal core files are copied byte-identical and credited to PCC-20261115-ed6495 and PCC-20261114-cdb4f1. The characteristic2 unit-root argument credits PowerTargets. Comparison with Mathlib’s separate continuous homogeneous-complex H1 remains open, as do isogeny kernels, Lang, Cartier group schemes, Jacobians, divisors and full Tate/MOV/Frey–Rück conditions. Original FR94 remains unread. No novelty claim or canonical corpus promotion. [pcc:unverified]'),
 p(f'<b>Checks.</b> Whole gate {g["declarations"]}/0 failures/0 trusted native; only propext/Classical.choice/Quot.sound. Full {binding["full_characters"]}-character written declaration, SHA {binding["full_decl_sha256"]}; independent native 1 complete/0 prefixes/0 unresolved. Six snapshots retain four failed states: {a["files"]} files/{a["unique_objects"]} objects/{size}-byte losslessXZ plus five gate files. Final06 has no advisories. Lean4.35.0/Mathlib56a00585…; defaults. No holes/custom axioms/native proof evaluation/raised limits/flags/waivers/frozen edits/cryptographic Python/new native cryptographic experiment/spend/upload. Native header processing is procedural. Signed standing unverified. [pcc:unverified]'),
 p('<b>Sources checked.</b> Bruin, JTNB23(2011)323–328, DOI10.5802/jtnb.764, primary649044bytes/SHA528b6379…; all six article pages previously read/seven PDF pages inspected. Current printed325/PDF4 Galois-Cartier definition, Eq2.1 and Lemma2.1 rechecked before code. Prior actual cocycle and finite-duality proofs plus official Mathlib APIs read. Primary copyright files remain local. KB/Conductor unavailable127; prior/primary/official fallback. No failed-search absence claim. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/galois-character-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
