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

story = [Paragraph('Hyperelliptic cover floor in the eight classes', styles['ReportTitle'])]
def body(s): story.append(Paragraph(s,styles['ReportBody']))
body('<b>Theorem and scope.</b> Every genus-five hyperelliptic curve over F3 whose Jacobian has Weil numerator 1-tT^5+243T^10 for t in {-26,-22,-14,-10,10,14,22,26} has elliptic cover degree at least four, even over the algebraic closure. The bound survives every finite base change. It does not cover nonhyperelliptic carriers or Jacobian correspondences.')
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
body('<b>Complete hyperelliptic slice.</b> An independent even-degree screen checked all 531,441 monic degree-12 models and retained 84 in these traces. Sage replay matched every row. Each retained polynomial has two F3 branch roots and one irreducible degree-10 factor. For both roots of each model, x=r+f\'(r)/u and y\'=u^6 y give a monic odd-degree equation in the prior 84-row screen: 168 exact transformations. Nonsquare-leading even models are quadratic twists; t changes sign and remains in the symmetric trace set. Thus every hyperelliptic curve over F3 in these classes has a representative in the odd-degree screen.')
body('<b>Limit.</b> An isogeny or correspondence between these Jacobians and the Weil restrictions may still give a transfer. Degree-four or larger covers and nonhyperelliptic genus-five carriers remain open. No odd-five subgroup map or full GHS/rho cost has been measured.')
story.append(Paragraph('Reproduction: CUBIC_PROTOCOL.md, EVEN_PROTOCOL.md, cubic_geometric_all.{sage,magma,json}, cubic_geometric_all_magma.json, screen_even.json and audit_even.json. Odd5Ledger.lean checks eight arithmetic equalities without axioms; the geometric and enumeration proofs are paper-level with exact independent receipts.',styles['ReportSmall']))
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
