"""Render recorded formal results and provenance; no mathematical computation."""
from pathlib import Path
import json, fitz
from reportlab.platypus import SimpleDocTemplate, Paragraph
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib import colors
from reportlab.graphics.shapes import Drawing, Rect, String
from reportlab.graphics import renderSVG
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont

s = Path(__file__).resolve().parent
o = s / 'reports'
pre = json.loads((s / 'preflight.json').read_text())
a = pre['archive']
binding = json.loads((s / 'evidence/binding-check.json').read_text())['observations'][0]
pdfmetrics.registerFont(TTFont('DV', '/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf'))
pdfmetrics.registerFont(TTFont('DVB', '/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf'))
pdfmetrics.registerFontFamily('DV', normal='DV', bold='DVB')
body = ParagraphStyle('body', fontName='DV', fontSize=9, leading=12, spaceAfter=10)
title = ParagraphStyle('title', fontName='DVB', fontSize=17, leading=22, spaceAfter=12)
def p(text): return Paragraph(text, body)
d = Drawing(507, 100)
for x, colour, head, line1, line2 in [
    (0, '#dcfce7', 'Characteristic different from 2', 'Actual pairing on E[2^n]: nondegenerate', 'Every n; every nonsingular model over AC field'),
    (260, '#fef3c7', 'Characteristic 2', 'All actual dyadic pairing values equal 1', 'Ordinary binary curve: nonzero radical each level')]:
    d.add(Rect(x, 5, 247, 88, fillColor=colors.HexColor(colour), strokeColor=colors.HexColor('#64748b')))
    d.add(String(x+9, 71, head, fontName='DVB', fontSize=10))
    d.add(String(x+9, 47, line1, fontName='DV', fontSize=8.1))
    d.add(String(x+9, 26, line2, fontName='DV', fontSize=7.7))
renderSVG.drawToFile(d, str(o / 'dyadic-domains.svg'))
flow = [Paragraph('Dyadic Weil pairing: the characteristic guard matters', title), d,
 p('Actual positive theorem. For every algebraically closed field K with2 nonzero, every nonsingular general Weierstrass curve W, and every n>=0, the existing geometric divisor Weil pairing on actual E[2^n] has zero left and right radicals. This is an unbounded theorem about the actual point and pairing objects. It formalizes established qualified mathematics; it does not assert full general isogeny-definition agreement. Level1 torsion is trivial. [pcc:unverified]'),
 p('Actual induction. Suppose P pairs trivially at level2t. The actual compatibility law and level-t nondegeneracy show2P=0. Given any actual two-torsion point Q, actual point divisibility supplies R with tR=Q. Its actual torsion membership and both level transports are proved. Compatibility and swapping reduce the original hypothesis to the actual level-two pairing, whose checked nondegeneracy forces P=0. Integer/natural scalar conversion, dependent torsion subtypes and equal-level transports are explicit. No abstract nondegeneracy or surjectivity premise remains in the main theorem. [pcc:unverified]'),
 p('Actual skeptical control. In characteristic2 every unit satisfying u^(2^n)=1 equals1. The actual pairing root law therefore makes every actual dyadic pairing value1. On the nonsingular ordinary curve y²+xy=x³+1 over an algebraic closure of F2, the actual point(0,1) is nonzero and lies in E[2^(n+1)] for every n. It gives a nonzero radical at every positive dyadic level and refutes the explicitly unguarded universal reading. This point has order2; no exact higher order or universal nonzero characteristic2 radical is asserted. The modern coprimality-qualified theorem survives. [pcc:unverified]'),
 p('Healthy controls. The qualified theorem also produces actual nontrivial pairs at every positive dyadic level, including the existing nonsingular curves in characteristics3 and0. A nonzero two-torsion point embeds into every higher dyadic torsion group; nondegeneracy supplies a partner. These controls use the actual proved curve and pairing, without a finite table or assumed torsion cardinality. [pcc:unverified]'),
 p(f'Checks retained. {pre["build_snapshots"]} source states; failures{",".join(pre["failed_states"])}; passes{",".join(pre["successful_states"])}. Whole gate{pre["gate"]["declarations"]}/0failures/0trustednative; six roots from01, no new-source warnings. Separate complete{binding["full_characters"]}-character main binding after completed gate:1complete/0prefix/0unresolved/empty suffix; full normalized and parsed signatures read separately. All8210preservation entries/58initial plans match.39packages/no copied imports/hydration/update. Full404formal files comprise9new root/config files plus395frozen dependency records pinned across2lossless manifest parts. Archive{a["files"]}files/{a["unique_objects"]}objects/{a["compressed_bytes"]}XZbytes/{a["parts"]}parts, restored and checked. Default limits/ordinary foundations, no holes/customaxioms/nativeproofevaluation/waivers/frozenedits. [pcc:unverified]'),
 p('Sources and limits. Galbraith author ch26.pdf222784B/SHA c06cf2df9b8003e19694a4ebfe298a1557d61169f87ec01542258438ceac015e: fresh bounded extracted lines58–151/printed574–576 before protocol/setup/code. Definition26.2.2 coprimality, all five properties of26.2.3, and26.1 quotient/convention read; no fresh full-page/full-chapter/external-proof claim. Canonical actual level-two, dyadic point division and level laws plus official scalar APIs reviewed. SixKB calls/Conductor127: bounded fallback, no absence or novelty claim. General odd levels, perfect-character duality, Scheme/isogeny comparison, descent and canonical Tate/MOV/FR remain open. Samehost checks do not establish compiler/parser/host independence. Signed standing unverified; zero spend. [pcc:unverified]')]
SimpleDocTemplate(str(o / 'report.pdf'), pagesize=(595,842), leftMargin=44, rightMargin=44, topMargin=32, bottomMargin=32).build(flow)
doc = fitz.open(o / 'report.pdf')
pages = [dict(page=i+1, text_within_page=all(page.rect.contains(fitz.Rect(x[:4])) for x in page.get_text('blocks')), text_characters=len(page.get_text())) for i,page in enumerate(doc)]
(o / 'layout.json').write_text(json.dumps(dict(pages=pages, visual_inspection='pending'), indent=2)+'\n')
print(json.dumps(dict(pages=len(doc), bytes=(o/'report.pdf').stat().st_size, layout=pages)))
