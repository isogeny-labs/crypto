#!/usr/bin/env python3
"""Append the bounded cubic-cover theorem to the existing two-page report."""
from pathlib import Path
import subprocess
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle

P = Path(__file__).resolve().parent
pdf = P/'REPORT.pdf'
tmp = P/'tmp_pdfs'
tmp.mkdir(exist_ok=True)
addendum = tmp/'cubic_addendum.pdf'
merged = tmp/'report_merged.pdf'
styles = getSampleStyleSheet()
styles.add(ParagraphStyle(name='ReportTitle',parent=styles['Heading1'],fontName='Helvetica-Bold',fontSize=15,leading=18,spaceAfter=12))
styles.add(ParagraphStyle(name='ReportBody',parent=styles['Normal'],fontName='Helvetica',fontSize=9.1,leading=12.3,spaceAfter=8))
styles.add(ParagraphStyle(name='ReportSmall',parent=styles['Normal'],fontName='Helvetica',fontSize=8,leading=10,spaceAfter=6))

story = [Paragraph('Geometric degree-three cover exclusion', styles['ReportTitle'])]
def body(s): story.append(Paragraph(s,styles['ReportBody']))
body('<b>Exact scope.</b> The exhaustive monic degree-11 screen has 84 smooth genus-five hyperelliptic models over F3 in the eight selected trace classes. For each of these specific curves, every elliptic cover over the algebraic closure has degree at least four. The bound survives every finite base change of these models. It does not classify other genus-five curves or Jacobian correspondences.')
body('<b>Geometric degree-two bound.</b> A genus-five hyperelliptic curve cannot also have a degree-two map to an elliptic curve: the two maps would give the Castelnuovo-Severi bound g &lt;= 3. Degree one is impossible by genus.')
body('<b>Complete cubic criterion.</b> A degree-three elliptic map sends the 12 Weierstrass points three at a time to the four elliptic 2-torsion points. After taking infinity to the elliptic origin, it forces f = B(A^3+c2 A^2 B+c1 A B^2+c0 B^3), with B a squarefree monic quadratic divisor of f and A a coprime monic cubic. This identity constructs (x,y) -&gt; (A/B,y/B^2). Over F_(3^10), all 11 finite branch roots split, giving 55 possible B. For each, inverse Frobenius fixes A modulo B and leaves one symbolic parameter v in A=A0+B(x+v).')

data = [['Screen','Models','Cases','Survivors'],
        ['Finite F243 cubic lifts','84','102,060','0'],
        ['Geometric symbolic pole pairs','84','4,620','0'],
        ['Independent Magma symbolic replay','84','4,620','0']]
t=Table(data,colWidths=[230,60,85,68],hAlign='LEFT')
t.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),colors.HexColor('#233b57')),
                       ('TEXTCOLOR',(0,0),(-1,0),colors.white),
                       ('FONTNAME',(0,0),(-1,0),'Helvetica-Bold'),
                       ('FONTSIZE',(0,0),(-1,-1),8),
                       ('BOTTOMPADDING',(0,0),(-1,-1),6),
                       ('TOPPADDING',(0,0),(-1,-1),6),
                       ('ROWBACKGROUNDS',(0,1),(-1,-1),[colors.white,colors.HexColor('#edf2f7')]),
                       ('LINEBELOW',(0,-1),(-1,-1),0.5,colors.HexColor('#cbd5e1'))]))
story += [t,Spacer(1,12)]
body('<b>Certificate.</b> The frozen Sage program computed residual coefficient polynomials in F_(3^10)[v]. Their gcd was constant for all 4,620 pole pairs, excluding every value of v over the algebraic closure. Public Magma 2.30-1 independently repeated all 4,620 gcd calculations; all 84 output rows matched the frozen source order, with no nonconstant gcd and no output truncation. This is an independently replayed finite computation. [pcc:unverified]')
body('<b>Limit.</b> The bound concerns curve-to-elliptic covers of these 84 models. An isogeny or correspondence between their Jacobians and the Weil restrictions may still give a transfer. Degree-four or larger covers and different genus-five representatives remain open. No odd-five subgroup map or full GHS/rho cost has been measured.')
story.append(Paragraph('Reproduction: CUBIC_PROTOCOL.md; cubic_geometric_all.sage and cubic_geometric_all.json; cubic_geometric_all.magma and cubic_geometric_all_magma.json. Odd5Ledger.lean checks the 102,060 and 4,620 arithmetic counts without axioms; the geometric theorem itself is not Lean-formalized.',styles['ReportSmall']))
story.append(Paragraph('Prior work: Iijima-Momose-Chao 2013, eprint.iacr.org/2013/487.pdf (direct cover family); Tian, arxiv.org/abs/2012.07173 (cubic transfer); Kudo-Harashita 2022, arxiv.org/pdf/2204.06805 (F3 genus-five enumeration, maximal F9 count 20; this slice has 10). Global novelty and practical vulnerability remain unestablished.',styles['ReportSmall']))

def footer(canvas,doc):
    canvas.saveState()
    canvas.setFont('Helvetica',8)
    canvas.setFillColor(colors.HexColor('#64748b'))
    canvas.drawString(36,26,'15 November 2026  |  Odd-degree-five carrier study')
    canvas.drawRightString(576,26,'3')
    canvas.restoreState()

SimpleDocTemplate(str(addendum),pagesize=(612,792),leftMargin=36,rightMargin=36,
                  topMargin=40,bottomMargin=44).build(story,onFirstPage=footer)
subprocess.run(['pdfseparate','-f','1','-l','2',str(pdf),str(tmp/'base-%d.pdf')],check=True)
subprocess.run(['pdfunite',str(tmp/'base-1.pdf'),str(tmp/'base-2.pdf'),
                str(addendum),str(merged)],check=True)
merged.replace(pdf)
print(pdf)
