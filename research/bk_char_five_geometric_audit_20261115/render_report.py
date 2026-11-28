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
b=ParagraphStyle('b',fontName='DV',fontSize=8.5,leading=11.1,spaceAfter=8)
t=ParagraphStyle('t',fontName='DVB',fontSize=17,leading=22,spaceAfter=14)
def p(s):return Paragraph(s,b)
d=Drawing(500,120);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="120" viewBox="0 0 500 120">']
rows=[(81,'Actual E/F5','y²=x³+3x+2; discriminant nonzero; actual count5','Four nonzero actual base points have exact order5'),
      (42,'Whole kernel','Every extension: all five-torsion points descend to E(F5)','Actual point-map bijection; geometric point kernel has size5'),
      (3,'Source boundary','Inclusion paraphrase false: all geometric5-torsion rational','Original r²-count criterion survives: both sides false')]
for yy,title,a,c in rows:
 d.add(Rect(0,yy,500,34,fillColor=colors.HexColor('#eaf2f6'),strokeColor=colors.HexColor('#2c788e')))
 svg.append(f'<rect x="0" y="{120-yy-34}" width="500" height="34" fill="#eaf2f6" stroke="#2c788e"/>')
 for xx,y,txt,size in [(9,yy+17,title,8.1),(104,yy+21,a,7.6),(104,yy+7,c,7.6)]:
  d.add(String(xx,y,txt,fontName='DV',fontSize=size));svg.append(f'<text x="{xx}" y="{120-y}" font-size="{size}" font-family="DejaVu Sans,sans-serif">{html.escape(txt)}</text>')
svg.append('</svg>');(o/'char-five.svg').write_text('\n'.join(svg)+'\n')
g=json.loads((r/'evidence/gate.json').read_text());a=json.loads((r/'evidence/attempts.json').read_text());binding=json.loads((r/'evidence/binding-check.json').read_text())['observations'][0];size=(r/'evidence/attempts.tar.xz').stat().st_size
flow=[Paragraph('BK at characteristic five: actual torsion counterexample',t),
 p('<b>Inclusion paraphrase disproved.</b> The actual nonsingular curve y²=x³+3x+2 over F5 has five actual rational points and four nonzero points of exact order5. Over every field extension of F5, all points killed by5 lie in the actual base point-map image, and the whole point kernel has cardinality5. In particular all geometric5-torsion points are rational over F5, while5 does not divide5^k−1 for any k&gt;0. This refutes the unrestricted inclusion iffdivisibility wording even with r=5prime, r dividing the actual count and r not dividing q−1. [pcc:unverified]'),d,Spacer(1,9),
 p('<b>Original source survives.</b> BK1998 Theorem1 counts r² torsion points; it does not merely assert inclusion of the geometric point kernel in characteristic r. Here the kernel has5points rather than25, and5 does not divide the actual finite-extension cardinality minus1. Both sides of the original count criterion are false in every actual finite extension. No extra characteristic hypothesis is attributed to that source theorem. The distinction was already exposed by the earlier characteristic3 audit; this actual characteristic5/r≥5 control carries no novelty claim. [pcc:unverified]'),
 p('<b>Coordinate restriction from actual S3.</b> For nonzero actual P killed by5, Q=2P is nonzero and2Q=−P. Two proved repeated-coordinate S3 identities give N(x)=D(x)u and N(u)−D(u)x=0, with D=4(x³+3x+2), N=x⁴−6x²−16x+9. The homogenized eliminant factors in characteristic5 as −(3x⁴+18x²+24x−9)(x−1)^5(x−2)^5. This concrete polynomial identity is checked coefficientwise in the kernel and transported to every characteristic5 field. The first factor cannot vanish: actual S3 would give three-torsion, incompatible with nonzero five-torsion. Thus x=1 or2; the actual curve equation gives y=±1. No canonical fifth-division-polynomial identification is assumed. [pcc:unverified]'),
 p('<b>Actual whole-kernel descent and size.</b> The base count is derived through Mathlib’s actual nonsingular-point/subtype equivalence and a kernel-checked arithmetic fiber count. Actual finite-group cardinality then kills every base point by5. Explicit additive curve-equality transport and Mathlib’s actual Point.map send the four coordinate possibilities to their actual base points. Map injectivity and complete descent give a bijection from E(F5) to the whole five-torsion point kernel over every extension, including AlgebraicClosure(F5). Cardinality5 is derived, not supplied. [pcc:unverified]'),
 p('<b>Boundary.</b> This concerns the geometric point kernel, not an identification of the nonreduced p-torsion group scheme. General prime≥5/canonical Weil/rank/determinant source proofs, the full-population all-k census and other fourteen-item obligations remain. No canonical corpus promotion or full-request completion. [pcc:unverified]'),
 p(f'<b>Checks and retained failures.</b> Whole gate {g["declarations"]}/0 failures/0 trusted native; standard foundations only. Complete {binding["full_characters"]}-character binding {binding["full_decl_sha256"]}; independent native1complete/0prefix/0unresolved. Fifteen snapshots retain13failedstates, successes09/15, one failed API probe and a pre-edit module compatibility amendment: {a["files"]}files/{a["unique_objects"]}objects/{size}-byte losslessXZ plus five gate files. Full final15 logs have exactly7inherited S3 argument advisories, zero new-module advisories/errors. Seven modules include two credited prior argument cores adapted only with module/public wrappers; exact argument bodies checked. Lean4.35.0/Mathlib56a00585…/defaults. No holes/custom axioms/native proof evaluation/raised limits/global flags/waivers/frozen edits/cryptoPython/spend/upload. Header processing procedural; signed standing unverified. [pcc:unverified]'),
 p('<b>Sources checked.</b> Balasubramanian–Koblitz1998, J.Cryptology11:141–145; primary73656bytes/5PDFpages/SHAac837b8e…; printed142–143/PDF2–3 freshly fully text-read/visually inspected, no fresh all-five-page claim. Exact source hypotheses/proof/Remarks distinguished. Prior S3Roots/S3Completion, equal-x and characteristic3 source arguments and pinned official point-map/group-order/cardinality APIs read and credited; frozen originals unchanged. Original raw copies/snapshots and wrapper amendment retained. KB/Conductor127, prior/primary/official fallback; no absence claim. Primary copyrighted PDF/text/images local only. [pcc:unverified]')]

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
 page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/bk-char-five-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
