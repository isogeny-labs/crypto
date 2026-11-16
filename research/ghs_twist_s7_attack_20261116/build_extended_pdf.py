#!/usr/bin/env python3
"""Append the audited s=7 twist finding to the existing GHS PDF report."""
import json
import subprocess
import tempfile
from pathlib import Path

from reportlab.lib import colors
from reportlab.lib.pagesizes import letter
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.platypus import KeepTogether, PageBreak, Paragraph, SimpleDocTemplate, Spacer, Table, TableStyle

HERE=Path(__file__).resolve().parent
OLD=HERE.parent/'ghs_odd5_carrier_20261115'/'REPORT.pdf'
OUT=HERE/'REPORT_EXTENDED.pdf'
attack=json.loads((HERE/'s7_full_attack.json').read_text())
audit=json.loads((HERE/'audit_s7_full_attack.json').read_text())
rho=json.loads((HERE/'rho_s7_result.json').read_text())
rho_audit=json.loads((HERE/'audit_rho_s7.json').read_text())
orbit=json.loads((HERE/'isogeny_orbit_bound.json').read_text())
assert attack['recovered_scalar']==rho['solution']['scalar']==audit['target_scalar']==rho_audit['scalar']==12345
assert orbit['exceptional_factor']==21156227 and orbit['exceptional_factor_prime']
assert 'Pages:           12' in subprocess.check_output(['pdfinfo',str(OLD)],text=True)
gt=attack['timing']['ghs_one_solve_cpu_s_stitched']
rt=rho['one_solve_cpu_s']
ratio=rt/gt

styles=getSampleStyleSheet()
styles.add(ParagraphStyle(name='GTitle',parent=styles['Title'],fontName='Helvetica-Bold',fontSize=16,leading=19,spaceAfter=10,textColor=colors.HexColor('#193650')))
styles.add(ParagraphStyle(name='GHead',parent=styles['Heading2'],fontName='Helvetica-Bold',fontSize=11,leading=14,spaceBefore=10,spaceAfter=5,textColor=colors.HexColor('#24506f')))
styles.add(ParagraphStyle(name='GBody',parent=styles['Normal'],fontName='Helvetica',fontSize=8.5,leading=11.5,spaceAfter=6))
styles.add(ParagraphStyle(name='GSmall',parent=styles['Normal'],fontName='Helvetica',fontSize=7,leading=9.2,spaceAfter=4))
styles.add(ParagraphStyle(name='GMono',parent=styles['Normal'],fontName='Courier',fontSize=7.5,leading=10,spaceAfter=5))
story=[]
def body(s):story.append(Paragraph(s,styles['GBody']))
def head(s):story.append(Paragraph(s,styles['GHead']))
def mono(s):story.append(Paragraph(s,styles['GMono']))
def table(rows,widths):
    t=Table(rows,colWidths=widths,hAlign='LEFT',repeatRows=1)
    t.setStyle(TableStyle([
        ('BACKGROUND',(0,0),(-1,0),colors.HexColor('#233b57')),
        ('TEXTCOLOR',(0,0),(-1,0),colors.white),
        ('FONTNAME',(0,0),(-1,0),'Helvetica-Bold'),
        ('FONTSIZE',(0,0),(-1,-1),7.5),
        ('TOPPADDING',(0,0),(-1,-1),4),
        ('BOTTOMPADDING',(0,0),(-1,-1),4),
        ('ROWBACKGROUNDS',(0,1),(-1,-1),[colors.white,colors.HexColor('#edf2f7')])]))
    story.extend([t,Spacer(1,7)])

story.append(Paragraph('Negative-trace twist: complete s=7 cover attack',styles['GTitle']))
body('This supplement extends the twelve-page odd-degree-five GHS report. It records one complete source-target solve through a genus-five cover and a same-target Frobenius-aware rho solve. The result is an exact measured instance, not a classification of all weak curves. [pcc:unverified]')
head('Exact curves and subgroup-preserving map')
mono('K=F3[a]/(a^5+2a+1),  L=F_(3^35),  k=F_(3^7),  b=a+a^3')
mono('E^-/L: v^2=u^3+u^2+2b')
mono('Y^-/k: y^2=2(x^11+x^9+2x^8+x^7+2x^5+x^3+x^2+x+1)')
body('The degree-seven map is u=2A/B, v=yC/B^2, using the exact A,B,C saved with the earlier positive-trace cover. The corrected divisor pullback followed by five-term L/k Frobenius trace sends the saved P to nonzero G of order r=185,302,017,921,931, and Q=12345P to H=12345G. Source order is 270r. The map, coordinates, 37-fold check, and pole-fiber correction were independently rebuilt.')
head('Measured one-target costs')
t=attack['timing']
rows=[['Stage','CPU seconds'],
      ['Source/curve setup',f"{t['source_setup_cpu_s']:.6f}"],
      ['Pullback and trace',f"{t['transfer_cpu_s']:.6f}"],
      ['Factor-base setup',f"{t['factor_base_cpu_s']:.6f}"],
      ['Relation collection',f"{t['relation_collection_cpu_s']:.6f}"],
      ['Matrix build, rank, solve',f"{t['linear_algebra_cpu_s']:.6f}"],
      ['Target descent and check',f"{t['descent_verify_cpu_s']:.6f}"],
      ['GHS total',f'{gt:.6f}'],
      ['Rho total, same P and Q',f'{rt:.6f}']]
table(rows,[340,180])
body(f'The measured rho/GHS one-solve ratio is {ratio:.2f}. GHS used 250,000 walk candidates, 2,069 split relations, full rank 1,092, and target offset 124. Rho used {rho["steps"]:,} walk steps and its checked order-seven Frobenius eigenvalue. Both CPUs exclude Sage interpreter startup. The GHS total is stitched from checkpointed relation collection and separate measured setup, linear algebra and descent; rho is one checkpointed walk, so its collision time has high variance.')
body('Independent auditors checked the exact cover and source/target maps, all 1,092 factor-base logs, all 2,069 relation group equalities, the modular rank and target recovery, and the rho collision coefficients and scalar.')
story.append(PageBreak())

story.append(Paragraph('Isogeny reach, prior work, and claim boundary',styles['GTitle']))
head('A sharp bound for this sevenfold base change')
body('Let pi be the 243-Frobenius on E^- over F_(3^5). It satisfies pi^2+26pi+243=0. Put U_0=0, U_1=1 and U_(n+1)=-26U_n-243U_(n-1). Then pi^n=U_n pi-243U_(n-1), and U_7=-21,156,227. The absolute value is prime: Lean checked the recurrence, the square-root bracket, and all potential divisors through 4,600; Sage independently checked primality.')
body('If a separable isogeny kernel H of order n is stable under pi^7 and gcd(n,3*21,156,227)=1, the identity gives pi=(pi^7+243U_6)/U_7 modulo n. Thus H is already stable under pi. Every such isogeny descends to F_(3^5) and its codomain j stays in that subfield. A single prime-to-three isogeny producing full-degree-35 j must have degree divisible by 21,156,227. This is a bound for the fixed source and extension, not a universal GHS criterion. Directly screened degrees 2,3,5,7,13,29,41,43 yielded only degree-five j values or no rational isogenies. Lean checks the integer part; the kernel and isogeny-descent argument is a mathematical proof outside Lean.')
head('Novelty and cost limits')
body('The exact negative-trace cover and completed 48-bit subgroup solve are new explicit records in this project. Quadratic twisting, isogeny-assisted transfer, and cover-plus-decomposition index calculus are established mechanisms. The five conjugate positive models and their twists form one construction orbit per trace sign; these are field-size base changes rather than an independent parameter family. The result therefore does not establish a new attack method or worldwide publication priority.')
body('The 2015 Iijima-Momose-Chao direct degree-five genus-five (2,...,2) case requires four extension-rational branch roots. This exact source has order 2 modulo 4 and its map has degree seven, so it lies outside that specified direct case. The paper does not classify every possible curve map. Joux-Vitse 2012 already treats cover and decomposition attacks with optional isogeny walks. Diem 2006 supplies the fixed-genus asymptotic class under its group-structure hypotheses; it is not a concrete runtime bound.')
head('Replay artifacts')
body('The exact cover coefficients, source and target coordinates, transfer images, relation data, factor-base logs, timings, Lean checks and independent audit receipts are linked from S7_RESULT.md in the accompanying pull request. The producer scripts and source-hash-bound checkpoint are retained with the research artifacts.')
story.append(Paragraph('Primary prior work: Iijima-Momose-Chao, ePrint 2015/805, Section 5; Joux-Vitse, Eurocrypt 2012; Diem, arXiv:math/0606607. All exact-instance claims remain [pcc:unverified] until PCC admission.',styles['GSmall']))

def footer(canvas,doc):
    canvas.saveState()
    canvas.setFont('Helvetica',8)
    canvas.setFillColor(colors.HexColor('#64748b'))
    canvas.drawString(36,26,'16 November 2026  |  Odd-degree-five GHS research')
    canvas.drawRightString(576,26,str(12+doc.page))
    canvas.restoreState()

with tempfile.TemporaryDirectory(dir=HERE) as tmp:
    supplement=Path(tmp)/'supplement.pdf'
    merged=Path(tmp)/'merged.pdf'
    SimpleDocTemplate(str(supplement),pagesize=letter,leftMargin=36,rightMargin=36,topMargin=38,bottomMargin=42).build(story,onFirstPage=footer,onLaterPages=footer)
    assert 'Pages:           2' in subprocess.check_output(['pdfinfo',str(supplement)],text=True)
    subprocess.run(['pdfunite',str(OLD),str(supplement),str(merged)],check=True)
    merged.replace(OUT)
assert 'Pages:           14' in subprocess.check_output(['pdfinfo',str(OUT)],text=True)
print(OUT)
