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
b=ParagraphStyle('b',fontName='DV',fontSize=8.7,leading=11.8,spaceAfter=10)
t=ParagraphStyle('t',fontName='DVB',fontSize=16,leading=20,spaceAfter=12)
def p(s):return Paragraph(s,b)
d=Drawing(500,142);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="142" viewBox="0 0 500 142">']
rows=[(96,'Actual iid law',['Character expectation = λ^t·ψ(start)','Actual squared character moment = normSq(λ)^t']),
      (50,'Derived contraction',['Normalized unit-phase variance = pair-defect sum / 2','Positive pair + phase separation ⇒ strict contraction']),
      (4,'Domain controls',['Prime + positive pair + distinct supports + j≠0: normSq(λ)<1','Zero weight or composite0/2: coefficient1; j0 is trivial'])]
for yy,title,lines in rows:
 d.add(Rect(0,yy,500,40,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')));svg.append(f'<rect x="0" y="{142-yy-40}" width="500" height="40" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+21,title,8.8),(130,yy+25,lines[0],7.8),(130,yy+9,lines[1],8.1)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{142-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'walk-character-contraction.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text())
flow=[Paragraph('Actual walk characters and phase contraction',t),
 p('<b>Result.</b> Lean derives the Fourier coefficient law for the actual independent additive-walk PMF on every finite abelian group. Character expectation at time t equals λ^t·ψ(start), and actual squared modulus equals normSq(λ)^t, with λ the actual weighted character sum. Complex expectation and its bind law come from normalized finite PMF probabilities and checked real expectations; no supplied spectral formula is assumed. [pcc:unverified]'),d,Spacer(1,6),
 p('<b>Quantitative pair estimate.</b> For actual PMF unit phases, 1−normSq(E[z]) equals one half of the weighted double sum of normSq(zᵢ−zⱼ). Nonnegative defects and the two ordered terms for distinct labels u,v yield normSq(λ)≤1−2·qᵤ·qᵥ·(1−Re ψ(aᵤ−aᵥ)). Positive weights then make normSq(λ)>1−2·qᵤ·qᵥ·δ imply Re ψ(aᵤ−aᵥ)>1−δ. Thus δ=0.01 supplies the source’s0.99 phase cutoff for the chosen pair. [pcc:unverified]'),
 p('<b>Actual prime-field character.</b> The frequency character is Mathlib’s standard complex additive character at x·j. Lean proves its literal cosine formula using the natural residue representative and proves injectivity for prime p/nonzero j. For any injective support and any two distinct positive-weight labels, every nonzero mode has normSq(λ)<1. Character unit values and phase differences are derived, not numerical assumptions. [pcc:unverified]'),
 p('<b>Controls.</b> Index j=0 has actual coefficient1 and every-time squared moment1. Over every prime field, the actual pure-label PMF has weights1/0 and distinct supports0/1; its nonzero-index1 coefficient is1, directly refuting strict contraction with positivity deleted. On ZMod4, actual fair positive weights and distinct supports0/2 have coefficient1 at nonzero index2, checking deletion of primality. These preserve the qualified source. [pcc:unverified]'),
 p('<b>Sources and remaining work.</b> <link href="https://link.springer.com/content/pdf/10.1007/BF01199265.pdf" color="blue">Hildebrand1994</link>: original13-page read/inspection credited, Lemma3/5 on195–198 rechecked. <link href="https://arxiv.org/abs/math/0401189" color="blue">Author preprint math/0401189v1</link>: selected section4.2/Proposition2/explicit j≠0 on13–16, reference14p36; visual15–16 only. Official finite-character/complex APIs and actual weighted-kernel proof read. Full source minimum cutoff, Parseval/TV and averaged spectral sum remain open; corrected box asymptotic is proved separately. [pcc:unverified]'),
 p(f'<b>Checks.</b> Whole gate: {g["declarations"]} declarations, zero failures, zero trusted native exceptions. All {a["files"]} build/processing/provenance files retained losslessly in {a["unique_objects"]} objects, five gate files and recursive source/config pins. Lean4.35.0/Mathlib56a00585…, standard foundations/default limits. Signed standing unverified; KB/Conductor fallback recorded. No holes/custom axioms/native evaluation/raised limits/global flags/waivers/native experiment/paid compute/upload. [pcc:unverified]')]

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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/walk-character-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
