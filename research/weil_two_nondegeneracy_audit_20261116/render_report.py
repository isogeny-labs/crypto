"""Lay out recorded proof/provenance only; no mathematical computation."""
from pathlib import Path
import json,fitz
from reportlab.platypus import SimpleDocTemplate,Paragraph
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib import colors
from reportlab.graphics.shapes import Drawing,Rect,String
from reportlab.graphics import renderSVG
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
s=Path(__file__).resolve().parent;o=s/'reports';o.mkdir(exist_ok=True)
pre=json.loads((s/'preflight.json').read_text());a=pre['archive']
binding=json.loads((s/'evidence/binding-check.json').read_text())['observations'][0]
pdfmetrics.registerFont(TTFont('DV','/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf'))
pdfmetrics.registerFont(TTFont('DVB','/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf'))
pdfmetrics.registerFontFamily('DV',normal='DV',bold='DVB')
b=ParagraphStyle('body',fontName='DV',fontSize=9,leading=12,spaceAfter=11)
title=ParagraphStyle('title',fontName='DVB',fontSize=17,leading=22,spaceAfter=12)
def p(x):return Paragraph(x,b)
d=Drawing(507,100)
for x,c,head,line1,line2 in [(0,'#dcfce7','Qualified level two','Actual geometric pairing: nondegenerate','Every nonsingular model; 2 nonzero'),(260,'#fef3c7','Actual characteristic-two control','Ordinary y²+xy=x³+1; P=(0,1)','Nonzero two-torsion; all values one')]:
 d.add(Rect(x,5,247,88,fillColor=colors.HexColor(c),strokeColor=colors.HexColor('#64748b')))
 d.add(String(x+9,71,head,fontName='DVB',fontSize=10))
 d.add(String(x+9,47,line1,fontName='DV',fontSize=8.5))
 d.add(String(x+9,26,line2,fontName='DV',fontSize=8.5))
renderSVG.drawToFile(d,str(o/'proof-domains.svg'))
flow=[Paragraph('Weil pairing: the level-two domain matters',title),d,
p('Actual positive theorem. For every algebraically closed field K with 2 nonzero and every nonsingular general Weierstrass model W, Lean proves left and right nondegeneracy of the existing geometric divisor pairing at integer level2. This confirms the qualified modern theorem in this scope. The map is the previously constructed quotient on actual torsion points, with actual proper divisor representatives and functions. No abstract pairing, assumed perfectness, or assumed rank replaces it. [pcc:unverified]'),
p('Actual partners from the cubic. The official two-torsion cubic has coefficients 4,b2,2b4,b6 and discriminant16Delta. Since2 and Delta are nonzero, its leading coefficient and discriminant are nonzero. Algebraic closure and the official cubic root theorem give three distinct roots. Each root produces an actual nonsingular nonzero point killed by2. For any abscissa x a different root u therefore supplies a partner; no precomputed torsion table or asserted group structure is needed. [pcc:unverified]'),
p('Vertical-function calculation. For a nonzero actual two-torsion point P=(x,y), the actual function X-x has full divisor2(P)-2(O), including its repeated zero and infinity pole. The quotient of two vertical functions has normalized infinity residue1. Actual translation, negation fixing X, and regular-ring evaluation of the auxiliary scalar identity prove c*(x-u)=u-x. For x different from u, cancellation gives c=-1. Actual admissible moving representatives and the existing arbitrary representative agreement identify the source26.1 quotient with this value. Inverting the source convention preserves -1. A point in the left radical would pair to1 with its different-root partner, forcing -1=1 and2=0. Swapping proves the right statement. [pcc:unverified]'),
p('Actual skeptical control. Over the algebraic closure of F2, the ordinary model y²+xy=x³+1 has nonzero discriminant and the actual nonzero point(0,1) is killed by2. Every actual level2 pairing value is a square root of1; in characteristic2 this forces value1. This point is therefore a nonzero radical element. Lean explicitly negates the universal nondegeneracy reading with no characteristic qualification. This does not refute Definition26.2.2, which requires coprimality. Healthy actual characteristic3 and characteristic0 curves y²=x³+x have nontrivial pairing values; zero and self values are checked separately. [pcc:unverified]'),
p(f'Checks retained. {pre["build_snapshots"]} actual source states; failures {",".join(pre["failed_states"])}; passes {",".join(pre["successful_states"])}. Whole-project gate {pre["gate"]["declarations"]} declarations,0failures,0trustednative; six roots from01, final new warnings0. Separate complete {binding["full_characters"]}-character main binding after completed gate:1complete,0prefix,0unresolved,empty suffix; full normalized and parsed statements inspected separately. All7538preservation entries/56initial plans match;34packages, no copied imports/hydration/update. Lossless {a["files"]}files/{a["unique_objects"]}objects/{a["compressed_bytes"]}XZbytes/{a["parts"]}parts; actual restore matches. Default limits and ordinary foundations; no holes/customaxioms/native proof evaluation/newflags/waivers. [pcc:unverified]'),
p('Sources and limits. Galbraith chapter26: fresh pre-code reread covers extraction lines1–230/247, including all26.2 and26.3.3. Final17 Miller-definition lines read after state09; earlier full bounded text/visuals credited separately. Initial broad wording corrected with snapshot retained. Definition26.2.2 and Theorem26.2.3 are qualified; formula26.1 requires proper moving representatives. Full external Silverman/Washington proofs were not read. Six KB wrapper calls and Conductor failed127; bounded canonical/official/primary fallback, no absence or novelty claim. Full general levels, multiplication-isogeny definition comparison, descent/canonical Tate/MOV/Frey–Ruck remain open. Samehost checks do not prove parser/compiler/host independence; signed standing unverified, zero paid compute. Source review, exact proof map and retained failures accompany this report. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595,842),leftMargin=44,rightMargin=44,topMargin=32,bottomMargin=32).build(flow)
doc=fitz.open(o/'report.pdf');pages=[]
for i,page in enumerate(doc):pages.append(dict(page=i+1,text_within_page=all(page.rect.contains(fitz.Rect(x[:4]))for x in page.get_text('blocks')),text_characters=len(page.get_text())))
(o/'layout.json').write_text(json.dumps(dict(pages=pages,visual_inspection='pending'),indent=2)+'\n')
print(json.dumps(dict(pages=len(doc),bytes=(o/'report.pdf').stat().st_size,layout=pages)))
