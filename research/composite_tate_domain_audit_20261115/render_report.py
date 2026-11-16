"""Render checked statements/provenance; no research computation."""
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
b=ParagraphStyle('b',fontName='DV',fontSize=8.7,leading=11.5,spaceAfter=8)
t=ParagraphStyle('t',fontName='DVB',fontSize=17,leading=22,spaceAfter=14)
def p(s):return Paragraph(s,b)
d=Drawing(500,120);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="120" viewBox="0 0 500 120">']
rows=[(81,'Known control','Existing F5/n6 source counterexample reconstructed','Same prover, Mathlib and host; no new coverage claim'),
      (42,'Actual domains','E(F5)[6] and E(F5)/6E(F5): both cardinality6','F5 units modulo sixth powers: cardinality2, exponent2'),
      (3,'Two radicals','Actual nonzero order3 point and its nonzero quotient class','Every bilinear map kills both; primitive6root is absent')]
for yy,title,a,c in rows:
 d.add(Rect(0,yy,500,34,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')))
 svg.append(f'<rect x="0" y="{120-yy-34}" width="500" height="34" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+17,title,8.1),(104,yy+21,a,7.6),(104,yy+7,c,7.6)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{120-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'known-control.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Known Tate counterexample: reconstruction check',t),
 p('<b>Known-result cross-check.</b> This reconstructs the actual F5/n6 case already recorded in pairing_composite_order_audit_20261110, PCC-20261110-5e63e9, whose earlier gate covered50declarations and also an F3/n4 example. The duplicate was found at attempt05 after an initially too-narrow prior search; the user was informed and scope amended. No new source coverage, fourteen-item gap closure or novelty is claimed. Four new modules import Mathlib and no earlier custom proof modules, on the same prover/Mathlib/host. This is not an independent implementation or physical-host validation. [pcc:unverified]'),d,Spacer(1,9),
 p('<b>Actual objects.</b> E/F5:y²=x³+1 has actual discriminant nonzero/count6 and actual P=(0,1) of order3. The first domain is the actual kernel of multiplication6; the second is the actual curve group modulo its multiplication6 image. Their cardinalities6/6 are derived. The target is actual F5 units modulo the image of the sixth-power homomorphism, not a substitute cyclic-group table. Its power image is exactly1 and−1, and the quotient has cardinality2 and exponent2. [pcc:unverified]'),
 p('<b>Both explicit radicals.</b> Every actual function on these exact domains satisfying the two multiplicative bilinearity laws sends P against every right input to1, because the output is killed by both3 and2. The nonzero quotient class of P similarly lies in every right radical. Identity/power preservation is derived from the two laws; no extra rank, primitive-root or nondegeneracy premise is inserted. Lean directly negates a necessary existence consequence of the literal composite-order source condition. No canonical divisor/Miller map identification is claimed. [pcc:unverified]'),
 p('<b>Root guard, correctly scoped.</b> All printed arithmetic source guards hold: gcd(6,5)=1,6 divides the actual curve count and actual−1 is a nonidentity sixth root. It has exact order2, and no primitive sixth root exists in F5. A separate generic commutative-monoid theorem proves that a nonidentity root of prime order is primitive. This is an arithmetic guard comparison; the general correctly qualified Tate nondegeneracy theorem remains open here. [pcc:unverified]'),
 p(f'<b>Checks.</b> Whole gate {g["declarations"]}/0 failures/0 trusted native; only propext/Classical.choice/Quot.sound. Complete {binding["full_characters"]}-character declaration binding {binding["full_decl_sha256"]}; independent native1complete/0prefix/0unresolved. Eight snapshots retain six failed states and successes02/08, a failed API probe and all three pre-edit protocols/amendments: {a["files"]}files/{a["unique_objects"]}objects/{size}-byte losslessXZ plus five gate files. Full final08 and gate logs have no warnings/errors. Lean4.35.0/Mathlib56a00585…/defaults. No holes/custom axioms/native proof evaluation/raised limits/global flags/waivers/frozen edits/cryptoPython/nativecrypto/spend/upload. Header processing is procedural; signed standing unverified. [pcc:unverified]'),
 p('<b>Source fidelity.</b> Galbraith chapter26, extendedcorrected version2.0; primary222784bytes/16pages/SHA c06cf2df…; printed574–577/PDF2–5 freshly fully text-read/visually inspected, not all16pages. Tate Exercise26.3.1 and Theorem26.3.3 give natural n, coprimality, divisibility, proper quotient domains and literal nontrivial-root wording. The prime-order/full-primitive-root qualified result is not refuted. Author listed errata23–71 were read; no chapter26entry in that bounded list, no global absence claim. Original Frey–Rück1994 remains unread. The old known-control README, prior actual count pattern and pinned official APIs are credited; original sources unchanged. KB/Conductor127, prior/primary/official fallback; initial duplicate-search error and correction retained. Copyrighted primary files remain local. [pcc:unverified]'),
 p('<b>Still open.</b> Canonical divisor/valuation Weil/Tate identification, generic MOV/Frey–Rück and other original fourteen-item obligations. This auxiliary control does not replace the separate original solving-degree negative control or establish new validator independence. [pcc:unverified]')]

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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/composite-tate-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
