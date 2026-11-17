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
b=ParagraphStyle('b',fontName='DV',fontSize=9,leading=12.3,spaceAfter=11)
t=ParagraphStyle('t',fontName='DVB',fontSize=16.5,leading=21,spaceAfter=13)
def p(s):return Paragraph(s,b)
d=Drawing(500,142);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="142" viewBox="0 0 500 142">']
rows=[(96,'Literal local guards',['20 explicit Sidon steps, shifted into [477,951]','For p > 1902: full degree0/1/2 genericity holds']),
      (50,'Exact local statistics',['Actual uniform20-label static product-hash law','In-degree variance19/20; fresh two-step rate7581/(8000p)']),
      (4,'Slow first collision',['Every hash: p <= 951 T; actual mean p <= 951 E','Over primes: E²/p unbounded; no uniform square-root bound'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.1),(122,yy+25,lines[0],7.6),(122,yy+9,lines[1],7.6)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'sidon-rho-counterfamily.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0]['full_decl_sha256'];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('Source-qualified local collisions can coexist with slow rho',t),
 p('<b>Result.</b> Lean constructs an actual fixed20-step rho family that satisfies the literal source degree-two genericity conditions and the exact local collision formula, yet has unbounded E[T]²/p over primes. Thus those finite local conditions alone cannot imply a uniform square-root firstcollision bound. This strengthens the prior bounded fixed-support control; no novelty or qualified random-increment-ensemble refutation is claimed. [pcc:unverified]'),d,Spacer(1,8),
 p('<b>Actual finite table.</b> The first20 A005282 entries are explicitly written in Lean, including148 and182. Lean checks injectivity and all unordered pair sums, including diagonal pairs, in the kernel. Shift by476 gives natural increments477 through951. For every p&gt;1902, all relevant degree0/1/2 sums lie below p; natural bounds and Sidon uniqueness prove the unchanged full DegreeTwoGeneric predicate for the actual residues in Z/p. No infinite greedy-generation property is assumed. [pcc:unverified]'),
 p('<b>Actual walk and original probability law.</b> F(x)=x+step(h(x)), where each of the p vertex labels is sampled independently and uniformly from20 labels. A pointwise hash embedding into Fin951 identifies this exact transition with the earlier bounded-step transition. The embedding is never assumed uniform over951 labels. Every actual leastrepeat T satisfies p≤951T and T≤p. Taking the original20-label product PMF yields p≤951E and E≤p, with an explicit actual-PMF expectation identity. No hugehash enumeration or assumed collision-time conclusion is used. [pcc:unverified]'),
 p('<b>Local formula retained, global bound disproved.</b> Under that same static hash law the in-degree mean is1 and variance19/20. With two independent uniform ordered starting points, sampled with replacement, the actual event that x,F(x),y,F(y) are distinct and F²(x)=F²(y) has measure7581/(8000p). The general actual-group coefficient theorem is reused unchanged. Nevertheless, for every natural C, an actual prime p&gt;1902 exists with E²&gt;Cp. Lean directly negates a real uniform bound, even when the literal degree guard is retained. It also checks prime10000019 and E²&gt;4p. [pcc:unverified]'),
 p('<b>Source qualifications and credit.</b> Bernstein–Lange2013§4printed96–97/PDF11–12 explicitly warns about step-vector and higher/mixed-degree dependencies; the result is consistent with that warning. Galbraith14.2.1printed288/PDF4 selects step tables randomly. The present table is fixed, so this counterfamily does not refute that ensemble. Official OEIS A005282 dataset mirror supplies the finite entries/definition/examples; original site/bfile returned403 and Mian–Chowla1944 full paper was not read. KB/Conductor unavailable; prior/primary/official fallback completed before code. No fullpaper-read or novelty claim. [pcc:unverified]'),
 p(f'<b>Checks and provenance.</b> Whole project gate{g["declarations"]}/0 failures/0 trusted native, only propext/Classical.choice/Quot.sound. Complete1043-character binding{binding}; independent native1complete/0prefix/0unresolved. Native parsing/hash evidence is procedural, not kernel execution refinement. Five snapshots preserve initial pre-Lean harness failure and cast-elaboration failure: {a["files"]}files/{a["unique_objects"]}objects/{size}-byte lossless XZ plus five final gate files. Old dependencies byte-identical to HEAD. Lean4.35.0/Mathlib56a00585…, defaults, retained legacy compatibility advisories; no holes/customaxioms/native evaluator/flags/limits/waivers/new crypto experiment/spend/upload. Exact local replay is recorded separately; signed standing remains unverified. [pcc:unverified]')]
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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/sidon-rho-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
