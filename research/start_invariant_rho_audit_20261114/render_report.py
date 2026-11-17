"""Lay out theorem statements and build provenance; no research computation."""
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
d=Drawing(500,142);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="142" viewBox="0 0 500 142">']
rows=[(96, 'Actual hash symmetry', ['Translation permutes actual vertices and static hash labels', 'Product weights and every hash expectation are preserved']), (50, 'Exact firstcollision law', ['Actual iterates conjugate; actual leastrepeat is transported', 'Every real statistic has the same expectation at every start']), (4, 'Same actual sampler', ['Actual hash/two-uniform-start mean = actual zero-start mean', 'Frozen local event and slow firstcollisions use the same PMF'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.1),(122,yy+25,lines[0],7.6),(122,yy+9,lines[1],7.6)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'start-invariant-rho.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0]['full_decl_sha256'];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Actual rho: the starting-point law bridge',t),
 p('<b>Result.</b> Lean proves that the actual static product-hash model gives the same expectation for every real statistic of firstcollision time at every starting state. It also proves that the mean under the exact earlier hash/two-independent-uniform-start probability space equals the zero-start mean. This makes the frozen Sidon counterfamily and verified local collision formula refer to the same actual probability model. Known finite reindexing/translation method credited, no novelty claim. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>Actual hash law.</b> For any finite additive commutative group, domain translation gives an explicit equivalence of actual hash functions, with both inverse identities proved. Actual product weights are preserved by that finite vertex permutation. Reindexing the actual finite weighted sum preserves every real hash-observable expectation; no equality of distributions is a premise. The algebraic identity even permits arbitrary real label weights. [pcc:unverified]'),
 p('<b>Actual trajectories and leastrepeat.</b> For F(x)=x+g(h(x)) in every nonzero Z/p, Lean proves conjugacy of the actual iterates after translating the hash. It derives equivalence of the actual collision predicate and equality of the actual Nat.find leastrepeat. Applying the proved hash-expectation transport yields equal expectations for every real testfunction of leastrepeat at every start, including indicator functions. No degreegenericity/mixing/collision-time estimate is needed for this starting-point result. [pcc:unverified]'),
 p('<b>Genuine joint probability model.</b> For any finite label type and normalized nonnegative label law, jointMean is the finite expectation of actual leastrepeat from the first sampled starting point under the unchanged genuine static-hash/two-independent-uniform-ordered-start PMF. The second starting point remains in the probability space. Its actual pointmass, finite sum exchange, derived start invariance and groupcardinality cancellation prove jointMean equal to the actual zero-start mean. No iid resampling, new sampler or independence conclusion replaces the actual static hash. [pcc:unverified]'),
 p('<b>Source-qualified counterfamily transferred.</b> The unchanged20 shifted Sidon steps satisfy literal degree0/1/2genericity for p&gt;1902. The pointwise p≤951T and T≤p bounds now hold for every hash and every starting state. The actual uniform-start mean is exactly the frozen zero-start E, so E²/p remains unbounded over primes with a direct real uniform-bound negation retaining the source degree guard. Prime10000019 and E²&gt;4p transfer to this exact sampler. Its frozen actual fresh-event probability remains7581/(8000p) under the same jointPMF. Fixed table versus random-table ensemble remains explicit; no qualified source/random-table/asymptotic-constant/practicalhash refutation. [pcc:unverified]'),
 p(f'<b>Checks and provenance.</b> Whole project gate{g["declarations"]}/0 failures/0 trusted native, only propext/Classical.choice/Quot.sound. Complete1697-character binding{binding}; independent native1complete/0prefix/0unresolved. Native parsing/hash processing is procedural evidence, not kernel execution refinement. Four source/config snapshots preserve firstbuild overloaded-lemma/recursive-rewrite failures: {a["files"]}files/{a["unique_objects"]}objects/{size}-byte lossless XZ plus five final gate files. Frozen dependencies byte-identical. Lean4.35.0/Mathlib56a00585…/defaults/legacy compatibility advisories retained; no holes/customaxioms/native evaluator/flags/limits/waivers/newcryptoexperiment/spend/upload. Signed standing unverified. [pcc:unverified]'),
 p('<b>Sources checked.</b> Galbraith14.2.1printed288/PDF4 starting point/static hash/random-table qualifier rechecked. Prior Bernstein–Lange2013§4printed96–97/PDF11–12 read/visual inspection and unchanged actual jointPMF/event proof retained. Prior leastrepeat/productPMF/counterfamily and official Mathlib finite reindexing/Nat.find/PMF APIs read. KB/Conductor127, primary/prior/official fallback before code; no fullpaper-read or novelty claim. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/start-rho-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
