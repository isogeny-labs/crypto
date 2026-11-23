"""Render recorded proofs/provenance; no mathematical computation."""
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
for x,c,head,line1,line2 in [(0,'#dcfce7','Algebraic-closure points','Actual point doubling: surjective','Every nonsingular model; all characteristics'),(260,'#fef3c7','Actual rational F5 control','E:y²=x³+x; Q=(0,0) nonzero','Every double zero; Q has no half')]:
 d.add(Rect(x,5,247,88,fillColor=colors.HexColor(c),strokeColor=colors.HexColor('#64748b')))
 d.add(String(x+9,71,head,fontName='DVB',fontSize=10))
 d.add(String(x+9,47,line1,fontName='DV',fontSize=8.5))
 d.add(String(x+9,26,line2,fontName='DV',fontSize=8.5))
renderSVG.drawToFile(d,str(o/'point-domains.svg'))
flow=[Paragraph('Actual halves exist over algebraic closure',title),d,
p('Actual positive theorem. For every algebraically closed field K, every nonsingular general Weierstrass model W, and every actual point Q, Lean produces an actual point P with P+P=Q. The statement covers every characteristic, including2 and3, without a separability premise. It concerns the actual point group and its actual addition. The source theorem uses geometric points; this is a formalization of its doubling case, not a new general isogeny theorem. [pcc:unverified]'),
p('Actual coordinate construction. Write N=X^4-b4*X²-2b6*X-b8 and D=4X³+b2*X²+2b4*X+b6. For a desired abscissa a, the actual polynomial N-C(a)D has degree4. Algebraic closure supplies a root u, and the actual quadratic curve fibre supplies v. Nonzero discriminant makes the actual point(u,v) nonsingular. If D(u)=0, the root equation also gives N(u)=0, contradicting the official no-common-zero duplication theorem at that actual point. Thus the tangent is nonvertical and the official doubling formula gives doubled abscissa a. No abstract surjectivity is assumed. [pcc:unverified]'),
p('Actual point and dyadic lifting. The doubled ordinate is either the target ordinate or its actual Weierstrass negative: the difference of the two actual curve equations factors into these two alternatives. In the first case the constructed point is a half. In the second, its actual group negative is a half. Infinity is handled by the zero point. Induction then proves surjectivity of every actual multiplication-by-2^n point map. The order of scalar iteration is checked explicitly. This is unbounded point divisibility for dyadic levels, not a theorem about arbitrary multiplication degrees or general Weil nondegeneracy. [pcc:unverified]'),
p('Actual skeptical control. On the nonsingular rational F5 curve y²=x³+x, kernel evaluation of all coordinate pairs proves that every actual affine curve point equals its negative. Every actual rational point therefore doubles to zero, including infinity. The actual point(0,0) is nonzero and has no actual rational half. Lean negates the universal surjectivity reading without algebraic closure. It does not refute Galbraith9.6.3, whose E(kbar) domain is visible in the primary source. Actual algebraic-closure curves in characteristics2,3 and0 satisfy all dyadic surjectivity controls. Point surjectivity must be kept distinct from surjectivity of a function-field pullback; the earlier actual degree4 and binary inseparability results remain intact. [pcc:unverified]'),
p(f'Checks retained. {pre["build_snapshots"]} actual source states; failures{",".join(pre["failed_states"])}; passes{",".join(pre["successful_states"])}. Whole-project gate{pre["gate"]["declarations"]}/0failures/0trustednative, six roots from01/final newwarnings0. Separate complete{binding["full_characters"]}-character main binding after completedgate:1complete/0prefix/0unresolved/empty suffix; full normalized/parsed headers inspected separately. All7906preservation entries/57initial plans match;28packages/no copied imports/hydration/update. Lossless{a["files"]}files/{a["unique_objects"]}objects/{a["compressed_bytes"]}XZbytes/{a["parts"]}parts; actual restore matches. Default limits/ordinary foundations, no holes/customaxioms/nativeproofevaluation/newflags/waivers/frozenedits. [pcc:unverified]'),
p('Sources and limits. Galbraith author ch9.pdf387389B/SHA f9a3272af43e74723accaed2e5e1d19405779141d1ed7f516d822203a40cc495: complete printed172/PDF8 text and individual1300px visual read before protocol/setup/code. Theorem9.6.3 explicitly uses algebraic-closure points; its external8.2.7 proof was not read in full here. Earlier bounded source ranges credited separately, no fullchapter claim. Official actual duplication/point APIs and canonical actual polynomial/fibre proofs reviewed. SixKB wrapper calls and Conductor127; bounded fallback, no absence/novelty claim. Full Scheme/isogeny API comparison, general nonzero[n] surjectivity and larger pairing levels remain open. Samehost checks do not prove compiler/parser/host independence; signed standing unverified, zero paid compute. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595,842),leftMargin=44,rightMargin=44,topMargin=32,bottomMargin=32).build(flow)
doc=fitz.open(o/'report.pdf');pages=[]
for i,page in enumerate(doc):pages.append(dict(page=i+1,text_within_page=all(page.rect.contains(fitz.Rect(x[:4]))for x in page.get_text('blocks')),text_characters=len(page.get_text())))
(o/'layout.json').write_text(json.dumps(dict(pages=pages,visual_inspection='pending'),indent=2)+'\n');print(json.dumps(dict(pages=len(doc),bytes=(o/'report.pdf').stat().st_size,layout=pages)))
