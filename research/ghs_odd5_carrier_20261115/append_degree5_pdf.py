#!/usr/bin/env python3
"""Append the exact-pair degree-five exclusion as page six of REPORT.pdf."""
from pathlib import Path
import subprocess
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle

P = Path(__file__).resolve().parent
pdf = P / 'REPORT.pdf'
tmp = P / 'tmp_pdfs'
tmp.mkdir(exist_ok=True)
info = subprocess.check_output(['pdfinfo', str(pdf)], text=True)
assert 'Pages:           5' in info, 'expected the five-page predecessor report'
addendum = tmp / 'degree5_addendum.pdf'
merged = tmp / 'degree5_merged.pdf'
styles = getSampleStyleSheet()
styles.add(ParagraphStyle(name='D5Title', parent=styles['Heading1'], fontName='Helvetica-Bold', fontSize=15, leading=18, spaceAfter=11))
styles.add(ParagraphStyle(name='D5Body', parent=styles['Normal'], fontName='Helvetica', fontSize=9, leading=12.2, spaceAfter=8))
styles.add(ParagraphStyle(name='D5Small', parent=styles['Normal'], fontName='Helvetica', fontSize=8, leading=10, spaceAfter=6))
story = [Paragraph('Exact +26 witness: first K-cover degree 6 to 20', styles['D5Title'])]
def body(s): story.append(Paragraph(s, styles['D5Body']))
body('<b>Amendment to page five.</b> The earlier [5,20] interval is superseded for the exact pair E/K: v^2=u^3+2u^2+a and Y/F3: y^2=x^11+x^9+2x^8+x^7+2x^5+x^3+x^2+x+1, with K=F3[a]/(a^5+2a+1). There is no K-defined cover of degree five to this E. The prior exact degree-four exclusion and geometric degree-one-to-three exclusion supply the lower endpoint; Howe Theorem 4.17 still supplies degree at most 20. The new exact-pair window is <b>[6,20]</b>. [pcc:unverified]')
body('<b>Complete degree-five screen.</b> The rational elliptic x-coordinate is A/B with degree five. Infinity has odd ramification index one, three or five. The branch polynomial factors over K as (x+1) times five irreducible quadratics Qi. Parity of finite branch and nonbranch poles gives the six complete monic denominator shapes below. At every unpoled branch pair there are three Galois-compatible finite elliptic 2-torsion images. [pcc:unverified]')
rows = [['Denominator B','Count','Assignments','Degree-5 A'],
        ['1','1','243','0'],
        ['Qi','5','405','0'],
        ['(x-c)^2','242','58,806','0'],
        ['QiQj','10','270','2'],
        ['Qi(x-c)^2','1,210','98,010','0'],
        ['H^2, gcd(H,f)=1','58,801','14,288,643','0'],
        ['Total','60,269','14,446,377','2']]
t = Table(rows, colWidths=[170,75,105,80], hAlign='LEFT')
t.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),colors.HexColor('#233b57')),
                       ('TEXTCOLOR',(0,0),(-1,0),colors.white),
                       ('FONTNAME',(0,0),(-1,0),'Helvetica-Bold'),
                       ('FONTNAME',(0,-1),(-1,-1),'Helvetica-Bold'),
                       ('FONTSIZE',(0,0),(-1,-1),8),
                       ('TOPPADDING',(0,0),(-1,-1),4),
                       ('BOTTOMPADDING',(0,0),(-1,-1),4),
                       ('ROWBACKGROUNDS',(0,1),(-1,-2),[colors.white,colors.HexColor('#edf2f7')]),
                       ('LINEABOVE',(0,-1),(-1,-1),0.7,colors.HexColor('#64748b'))]))
story += [t, Spacer(1,10)]
body('<b>Independent checks.</b> A frozen Sage producer used CRT; a separately frozen replay built direct quotient-ring coefficient systems. Both covered all 14,446,377 logical cases and found the same two exact-degree-five (A,B) pairs. A public Magma 2.30-1 run checked those two hard-coded pairs. Their required degree-eight quotients are nonsquares: the first has irreducible factor degrees 2+2+4 and the second 1+1+2+4, all with exponent one. Neither pair satisfies the elliptic equation. The Magma response is successful and untruncated. [pcc:unverified]')
body('<b>Scope and next step.</b> This bound is for the displayed Y and E over K. It does not exclude degree five for another target elliptic curve or carrier. Any K-cover in [6,20] would have injective pullback on the order-109 source subgroup into J(Y)(K), but a computable transfer into J(Y)(F3) and a complete same-target GHS/rho cost remain open. Fifteen axiom-free Lean declarations check finite arithmetic inputs; the cover geometry is paper-level. [pcc:unverified]')
story.append(Paragraph('Full proof and source hashes: DEGREE5_THEOREM.md; frozen protocols, Sage producer/replay JSON and hard-coded Magma source/response in this directory. The first producer receipt had an output-only source-binding error and is preserved in DEGREE5_AMENDMENT.md. Prior method: Fan 2022; upper bound: Howe, arxiv.org/pdf/2110.04221.', styles['D5Small']))

def footer(canvas, doc):
    canvas.saveState()
    canvas.setFont('Helvetica', 8)
    canvas.setFillColor(colors.HexColor('#64748b'))
    canvas.drawString(36, 26, '15 November 2026  |  Odd-degree-five carrier study')
    canvas.drawRightString(576, 26, '6')
    canvas.restoreState()

SimpleDocTemplate(str(addendum), pagesize=(612,792), leftMargin=36,
                  rightMargin=36, topMargin=40, bottomMargin=44).build(story, onFirstPage=footer)
subprocess.run(['pdfunite', str(pdf), str(addendum), str(merged)], check=True,
               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
merged.replace(pdf)
addendum.unlink()
try:
    tmp.rmdir()
except OSError:
    pass
print(pdf)
