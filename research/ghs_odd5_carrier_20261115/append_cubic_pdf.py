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

story = [Paragraph('Degree-three cover exclusion for the finite carrier census', styles['ReportTitle'])]
def body(s): story.append(Paragraph(s,styles['ReportBody']))
body('<b>Exact field and scope.</b> K = F3[a]/(a^5+2a+1). The earlier report gives 84 smooth monic degree-11 genus-five hyperelliptic models over F3 in the eight selected trace classes. The claim below concerns morphisms from these 84 specific models after base change to K. It does not classify all genus-five curves or all Jacobian correspondences.')
body('<b>Geometric degree-two bound.</b> A genus-five hyperelliptic curve cannot also have a degree-two map to an elliptic curve: the two maps would give the Castelnuovo-Severi bound g &lt;= 3. Degree one is impossible by genus.')
body('<b>Complete cubic criterion.</b> A degree-three elliptic map sends the 12 Weierstrass points three at a time to the four elliptic 2-torsion points. After taking the rational Weierstrass point at infinity to the elliptic origin, it forces f = B(A^3+c2 A^2 B+c1 A B^2+c0 B^3), with B a squarefree monic quadratic divisor of f, A a monic cubic coprime to B, and the target cubic smooth. Conversely this identity gives (x,y) -&gt; (A/B,y/B^2). For each B, cubing in K[x]/(B) fixes A modulo B; only the 243 lifts A=A0+B(x+v), v in K, remain.')

data = [['Finite screen','Models','Cubic candidates','Elliptic maps'],
        ['Eight selected witnesses','8','9,720','0'],
        ['All selected-trace models','84','102,060','0'],
        ['Independent Magma: traces +/-26','2','2,430','0']]
t=Table(data,colWidths=[178,58,112,95],hAlign='LEFT')
t.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),colors.HexColor('#233b57')),
                       ('TEXTCOLOR',(0,0),(-1,0),colors.white),
                       ('FONTNAME',(0,0),(-1,0),'Helvetica-Bold'),
                       ('FONTSIZE',(0,0),(-1,-1),8),
                       ('BOTTOMPADDING',(0,0),(-1,-1),6),
                       ('TOPPADDING',(0,0),(-1,-1),6),
                       ('ROWBACKGROUNDS',(0,1),(-1,-1),[colors.white,colors.HexColor('#edf2f7')]),
                       ('LINEBELOW',(0,-1),(-1,-1),0.5,colors.HexColor('#cbd5e1'))]))
story += [t,Spacer(1,12)]
body('<b>Conclusion.</b> Each of these 84 particular K-curves has elliptic cover degree at least four. The 84-model exclusion is an exact Sage finite certificate with one implementation; the two catalog-uncertain representatives were independently replayed by Magma 2.30-1. This is a cover-degree lower bound, not an evaluated subgroup isogeny or a GHS/rho speedup. A different genus-five representative or a degree-four-or-higher cover remains possible. [pcc:unverified]')
story.append(Paragraph('Reproduction: CUBIC_PROTOCOL.md; cubic_all.sage and cubic_all.json (102,060 trials); cubic_replay2.json (public Magma success). The first Magma intrinsic-name failure is preserved in cubic_replay.json. The algebraic search is not formalized in Lean; Odd5Ledger.lean checks the earlier arithmetic ledger only.',styles['ReportSmall']))
story.append(Paragraph('Prior work: Iijima-Momose-Chao 2013, eprint.iacr.org/2013/487.pdf (direct cover family); cubic transfer concept in Tian, arxiv.org/abs/2012.07173; bielliptic genus bound in mat.uab.cat/~francesc/xzeroquotientgeneral.pdf. Global novelty and practical vulnerability remain unestablished.',styles['ReportSmall']))

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
