#!/usr/bin/env python3
"""Append the exact +26 degree-four exclusion as page five of REPORT.pdf."""
from pathlib import Path
import subprocess
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle

P = Path(__file__).resolve().parent
pdf = P / 'REPORT.pdf'
tmp = P / 'tmp_pdfs'
tmp.mkdir(exist_ok=True)
addendum = tmp / 'degree4_addendum.pdf'
merged = tmp / 'degree4_merged.pdf'
styles = getSampleStyleSheet()
styles.add(ParagraphStyle(name='D4Title', parent=styles['Heading1'], fontName='Helvetica-Bold', fontSize=15, leading=18, spaceAfter=12))
styles.add(ParagraphStyle(name='D4Body', parent=styles['Normal'], fontName='Helvetica', fontSize=9, leading=12.3, spaceAfter=8))
styles.add(ParagraphStyle(name='D4Small', parent=styles['Normal'], fontName='Helvetica', fontSize=8, leading=10, spaceAfter=6))
story = [Paragraph('Exact +26 witness: first cover degree 5 to 20', styles['D4Title'])]
def body(s): story.append(Paragraph(s, styles['D4Body']))
body('<b>Exact pair.</b> K=F3[a]/(a^5+2a+1), E: v^2=u^3+2u^2+a with group order 218=2·109, and Y/F3: y^2=x^11+x^9+2x^8+x^7+2x^5+x^3+x^2+x+1. The full Weil numerator of Y is 1-26T^5+243T^10. There is no degree-four K-map Y_K to this exact E. Earlier geometric work excludes degrees one through three; Howe Theorem 4.17 guarantees a K-map of degree at most 20. Thus the first degree lies in [5,20]. This does not give rational functions for a map or a base-F3 transfer. [pcc:unverified]')
body('<b>Complete degree-four pole shapes.</b> Translate infinity to the elliptic origin. Hyperelliptic involution then acts as elliptic negation, so the elliptic x-coordinate is A(x)/B(x). Its degree is four. Infinity has odd ramification index one or three. Finite branch poles have odd denominator multiplicity, and nonbranch poles even multiplicity. Since the branch polynomial factors as one linear factor x+1 and five irreducible quadratics over K, exactly 249 monic denominator shapes are possible: (x+1)Qi; (x+1)(x-c)^2 for 242 c; (x+1)^3; and x+1. [pcc:unverified]')

data = [['Pole shape','Assignments','Linear-consistent','Degree-four A'],
        ['(x+1)Qi','405','5','0'],
        ['(x+1)(x-c)^2','58,806','242','0'],
        ['(x+1)^3','243','1','0'],
        ['x+1','243','1','0']]
t = Table(data, colWidths=[190,95,110,95], hAlign='LEFT')
t.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),colors.HexColor('#233b57')),
                       ('TEXTCOLOR',(0,0),(-1,0),colors.white),
                       ('FONTNAME',(0,0),(-1,0),'Helvetica-Bold'),
                       ('FONTSIZE',(0,0),(-1,-1),8),
                       ('BOTTOMPADDING',(0,0),(-1,-1),6),
                       ('TOPPADDING',(0,0),(-1,-1),6),
                       ('ROWBACKGROUNDS',(0,1),(-1,-1),[colors.white,colors.HexColor('#edf2f7')]),
                       ('LINEBELOW',(0,-1),(-1,-1),0.5,colors.HexColor('#cbd5e1'))]))
story += [t, Spacer(1,12)]
body('<b>Independent finite certificate.</b> Each unpoled quadratic branch pair has three Galois-compatible elliptic 2-torsion images. The producer solved the resulting congruences by CRT for all 59,697 cases. A separately frozen replay built eight- or ten-equation linear systems in the five numerator coefficients and checked their left kernels. Both found zero degree-four numerators in every shape. The replay found one lower-degree consistent system per denominator; these cannot define a degree-four map. Source hashes and exact counts are in DEGREE4_THEOREM.md and the two JSON receipts. [pcc:unverified]')
body('<b>Scope.</b> This is an exact-pair, K-rational cover-degree bound. It does not classify other genus-five carriers or elliptic targets, does not construct a degree 5-20 map, and does not measure a small-field GHS attack. The order-109 subgroup pullback of any degree 5-20 K-cover is injective by push-pull, but its image lies in J(Y)(K); an evaluable map into J(Y)(F3) remains open. Thirteen Lean declarations check finite arithmetic inputs without axioms; the denominator classification and finite replay are paper-level. [pcc:unverified]')
story.append(Paragraph('Source: Howe, arxiv.org/pdf/2110.04221, Theorem 4.17 and Remark 4.18. Reproduction: DEGREE4_ALL_PROTOCOL.md, DEGREE4_REPLAY_PROTOCOL.md, degree4_all.sage/json, degree4_linear_replay.sage/json. Bounded prior-work search has not established publication priority.', styles['D4Small']))

def footer(canvas, doc):
    canvas.saveState()
    canvas.setFont('Helvetica',8)
    canvas.setFillColor(colors.HexColor('#64748b'))
    canvas.drawString(36,26,'15 November 2026  |  Odd-degree-five carrier study')
    canvas.drawRightString(576,26,'5')
    canvas.restoreState()

SimpleDocTemplate(str(addendum), pagesize=(612,792), leftMargin=36, rightMargin=36,
                  topMargin=40, bottomMargin=44).build(story, onFirstPage=footer)
subprocess.run(['pdfseparate','-f','1','-l','4',str(pdf),str(tmp/'base-%d.pdf')], check=True,
               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
subprocess.run(['pdfunite',*(str(tmp/f'base-{i}.pdf') for i in range(1,5)),
                str(addendum),str(merged)], check=True, stderr=subprocess.DEVNULL)
merged.replace(pdf)
print(pdf)
