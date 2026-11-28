"""Render proved statements and provenance; no research computation."""
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
b=ParagraphStyle('b',fontName='DV',fontSize=9,leading=12.3,spaceAfter=8)
t=ParagraphStyle('t',fontName='DVB',fontSize=16.5,leading=21,spaceAfter=13)
def p(s):return Paragraph(s,b)
d=Drawing(500,120);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="120" viewBox="0 0 500 120">']
rows=[(81,'Source guards',['Actual n=2M points with a fixed-point-free involution','Actual independent uniform draws; 3 <= s < n/2']),
      (42,'Exact counterexample',['n=8, s=3, actual expectation = 11/16','Printed lower bound = 3/4; 11/16 < 3/4']),
      (3,'Proved repair',['For the same whole window: 4s^2/(9n) <= E <= s^2/n','Same quadratic order; runtime exponent not refuted'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,34,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{120-yy-34}" width="500" height="34" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+17,title,8.1),(116,yy+21,lines[0],7.6),(116,yy+7,lines[1],7.6)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{120-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'theriault-intersections.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0]['full_decl_sha256'];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Thériault 2003: a finite-bound correction',t),
 p('<b>Source claim and result.</b> Theorem1 of the original ASIACRYPT2003 paper claims 2s²/(3n)≤E(n,s)≤s²/n whenever 3≤s&lt;n/2. Lean directly refutes the lower bound in the actual stated iid paired-point model. For n=8 and s=3, the exact expectation is 11/16, strictly below the printed lower bound 3/4. More generally, for every M≥4, n=2M and s=3 retain all stated guards but E=3/M−1/M²&lt;3/M. This is a qualified finite-theorem counterexample, not a deleted-hypothesis control. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>The same actual model.</b> Points are Fin(M)×Bool, with cardinality n=2M and actual involution flipping Bool. Lean proves involutivity, absence of fixed points and that equal first coordinates mean equality or negation. The unchanged genuine uniformPMF on all point words has pointmass equal to the product of independent uniform pointmasses. The actual intersection count is s minus the number of distinct orbits observed, so each repeated orbit contributes one intersection. No expected-value or recurrence conclusion is a premise. [pcc:unverified]'),
 p('<b>Actual source Q and E.</b> Q(n,s,k) is the probability of actual count k under that PMF. Nonnegativity and normalization over 0≤k&lt;s are proved for s&gt;0. The exact source expression E=sum(k·Q) is proved equal to the actual paired-word expectation. An explicit equivalence splitting orbit labels and Bool labels derives the label-occupancy formula; finite absent-label indicator products give E=s−M+M(1−1/M)^s and the printed recurrence E(s+1)=(1−1/M)E(s)+s/M. The n=8 fixture has exactly512 actual point triples. [pcc:unverified]'),
 p('<b>Where the proof fails.</b> The source correctly derives its closed expectation, then its last inequality chain on printed86 drops the finite s−1 correction too aggressively. For s=3 the exact E is 6/n−4/n², while the asserted lower bound is 6/n; every even n≥8 violates it. This identifies a literal finite constant error without claiming a new occupancy method or an error in every source statement. [pcc:unverified]'),
 p('<b>Proved repair and boundary.</b> From the actual derived recurrence, Lean proves s(s−1)/(3M)≤E for s≤M+1 and E≤s(s−1)/(2M). Hence 4s²/(9n)≤E≤s²/n throughout the original 3≤s&lt;n/2 window. The quadratic order is retained. Theorem3’s single-large-prime q-exponent is not refuted by this finite-constant correction. Actual Jacobian/Picard, smoothness, random-walk sampling and sparse-linear-algebra/runtime bridges are separate open obligations; no full algorithm verification is claimed. [pcc:unverified]'),
 p(f'<b>Checks and provenance.</b> Whole project gate {g["declarations"]}/0 failures/0 trusted native; only propext/Classical.choice/Quot.sound. Complete1265-character binding {binding}; independent native1complete/0prefix/0unresolved. Native parsing/hashing is procedural, not kernel refinement. Nine snapshots retain five failed states: {a["files"]}files/{a["unique_objects"]}objects/{size}-byte losslessXZ plus five final gate files. Lean4.35.0/Mathlib56a00585…/defaults/frozen dependencies unchanged; no holes/customaxioms/native evaluator/limits/flags/waivers/newcryptoexperiment/spend/upload. Signed standing unverified. [pcc:unverified]'),
 p('<b>Sources checked.</b> Nicolas Thériault, Index Calculus Attack for Hyperelliptic Curves of Small Genus, ASIACRYPT2003/LNCS2894 printed75–92, DOI10.1007/978-3-540-40061-5_5. All18pages fullyread; printed85–86/PDF11–12 visually inspected for exact theorem/recurrence/last chain. Free-pair qualification on printed81 retained. Prior actual uniform PMF and official finite-sum/product/cardinality definitions/proofs reviewed. KB/Conductor127, primary/prior/official fallback; no novelty claim. Primary PDF/OCR/images remain local. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/theriault-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
