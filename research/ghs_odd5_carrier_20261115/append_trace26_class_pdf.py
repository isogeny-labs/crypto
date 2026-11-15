#!/usr/bin/env python3
"""Append the ten-target trace +26 cover bound as page seven."""
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
assert 'Pages:           6' in info, 'expected six-page predecessor report'
addendum = tmp / 'trace26_class_addendum.pdf'
merged = tmp / 'trace26_class_merged.pdf'
styles = getSampleStyleSheet()
styles.add(ParagraphStyle(name='ClassTitle', parent=styles['Heading1'], fontName='Helvetica-Bold', fontSize=15, leading=18, spaceAfter=11))
styles.add(ParagraphStyle(name='ClassBody', parent=styles['Normal'], fontName='Helvetica', fontSize=8.8, leading=11.5, spaceAfter=7))
styles.add(ParagraphStyle(name='ClassSmall', parent=styles['Normal'], fontName='Helvetica', fontSize=7.5, leading=9.5, spaceAfter=5))
story = [Paragraph('Trace +26 isogeny class: ten exact K-cover bounds', styles['ClassTitle'])]
def body(s): story.append(Paragraph(s, styles['ClassBody']))
body('<b>Amendment to page six.</b> The [6,20] minimum-degree window now holds for the fixed genus-five Y/F3 and all ten normal-form K-isomorphism classes E_b: v^2=u^3+2u^2+b of trace +26 over K=F3[a]/(a^5+2a+1). Page six considered only b=a. This is a finite-class cover-degree result, not an attack-cost or vulnerability finding. [pcc:unverified]')
rows = [['b-mask','b in F3[a]','degree 4 A','degree 5 A','degree 5 maps'],
        ['3','a','0','2','0'],
        ['27','a^3','0','2','0'],
        ['30','a^3+a','0','1','0'],
        ['103','a^4+2a^2+a+1','0','1','0'],
        ['139','a^4+2a^3+a+1','0','1','0'],
        ['167','2a^4+a+2','0','2','0'],
        ['182','2a^4+2a^2+2','0','2','0'],
        ['194','2a^4+a^3+a+2','0','1','0'],
        ['230','2a^4+2a^3+a^2+a+2','0','2','0'],
        ['233','2a^4+2a^3+a^2+2a+2','0','1','0']]
t=Table(rows,colWidths=[48,211,81,81,99],hAlign='LEFT')
t.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),colors.HexColor('#233b57')),
                       ('TEXTCOLOR',(0,0),(-1,0),colors.white),
                       ('FONTNAME',(0,0),(-1,0),'Helvetica-Bold'),
                       ('FONTSIZE',(0,0),(-1,-1),7.4),
                       ('TOPPADDING',(0,0),(-1,-1),3),
                       ('BOTTOMPADDING',(0,0),(-1,-1),3),
                       ('ROWBACKGROUNDS',(0,1),(-1,-1),[colors.white,colors.HexColor('#edf2f7')])]))
story += [t,Spacer(1,9)]
body('<b>Proof boundary.</b> The earlier geometric certificate excludes degrees one to three for this Y and every elliptic target. For each of the nine newly screened E_b, an independently frozen CRT producer and direct-linear replay checked all 59,697 degree-four assignments and returned no degree-four numerator. Both methods checked 14,446,377 degree-five assignments per target, agreed on all thirteen interpolation survivors, and found no full-equation map. Magma 2.30-1 independently verified that each of those thirteen required degree-eight quotients is a nonsquare. The b=a pair had separate earlier Sage and Magma checks. Howe Theorem 4.17 gives a K-map of degree at most twenty to each target. [pcc:unverified]')
body('<b>Subgroup and novelty scope.</b> Every E_b(K) has order 218=2*109. A cover in [6,20] would preserve the order-109 subgroup under pullback into J(Y)(K), since its degree is coprime to 109. The cover remains nonconstructive. A usable F3-rational transfer and full matched GHS/rho costs remain open. Isogeny-assisted GHS and Howe-type cover bounds are prior methods; this is an exact finite-class obstruction relative to the inspected records, with worldwide priority unconfirmed. [pcc:unverified]')
story.append(Paragraph('Full equations, source hashes and receipts: TRACE26_CLASS_THEOREM.md and the degree4_isogeny / degree5_isogeny files in this directory. Eighteen axiom-free Lean arithmetic declarations check counts and coprimality; geometry and finite-search exhaustiveness are paper-level. Sources: Howe arxiv.org/pdf/2110.04221; Galbraith-Smart cryptrec.go.jp/exreport/cryptrec-ex-1029-2001.pdf; Tian arxiv.org/pdf/2012.07173.',styles['ClassSmall']))

def footer(canvas,doc):
    canvas.saveState()
    canvas.setFont('Helvetica',8)
    canvas.setFillColor(colors.HexColor('#64748b'))
    canvas.drawString(36,26,'15 November 2026  |  Odd-degree-five carrier study')
    canvas.drawRightString(576,26,'7')
    canvas.restoreState()

SimpleDocTemplate(str(addendum),pagesize=(612,792),leftMargin=36,
                  rightMargin=36,topMargin=40,bottomMargin=44).build(story,onFirstPage=footer)
subprocess.run(['pdfunite',str(pdf),str(addendum),str(merged)],check=True,
               stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
merged.replace(pdf)
addendum.unlink()
try:
    tmp.rmdir()
except OSError:
    pass
print(pdf)
