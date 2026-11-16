#!/usr/bin/env python3
"""Append the audited independent-carrier screens to the GHS report."""
import json
import subprocess
import tempfile
from pathlib import Path

from reportlab.lib import colors
from reportlab.lib.pagesizes import letter
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.platypus import Paragraph, SimpleDocTemplate, Spacer, Table, TableStyle

HERE=Path(__file__).resolve().parent
BASE=HERE.parent/'ghs_twist_s7_attack_20261116'
OLD=BASE/'REPORT_EXTENDED.pdf'
OUT=HERE/'REPORT_WITH_CARRIER_CENSUS.pdf'
audit=json.loads((HERE/'audit_manifest.json').read_text())
manifest=json.loads((HERE/'manifest.json').read_text())
attack=json.loads((BASE/'s7_full_attack.json').read_text())
rho=json.loads((BASE/'rho_s7_result.json').read_text())
assert audit['screened_representative_pairs']==24
assert audit['pairs_covered_by_frobenius']==120
assert audit['total_hits']==manifest['total_hits']==0
gt=attack['timing']['ghs_one_solve_cpu_s_stitched']
rt=rho['one_solve_cpu_s']
assert abs(rt/gt-14.13967551628992)<1e-8
assert 'Pages:           14' in subprocess.check_output(['pdfinfo',str(OLD)],text=True)

styles=getSampleStyleSheet()
styles.add(ParagraphStyle(name='Title2',parent=styles['Title'],fontName='Helvetica-Bold',fontSize=16,leading=20,spaceAfter=12,textColor=colors.HexColor('#193650')))
styles.add(ParagraphStyle(name='Heading2b',parent=styles['Heading2'],fontName='Helvetica-Bold',fontSize=11,leading=14,spaceBefore=10,spaceAfter=5,textColor=colors.HexColor('#24506f')))
styles.add(ParagraphStyle(name='Body2',parent=styles['Normal'],fontSize=8.6,leading=11.6,spaceAfter=6))
styles.add(ParagraphStyle(name='Small2',parent=styles['Normal'],fontSize=7.2,leading=9.5,spaceAfter=4))
story=[]
def body(s):story.append(Paragraph(s,styles['Body2']))
def head(s):story.append(Paragraph(s,styles['Heading2b']))

story.append(Paragraph('Independent carriers: finite cover screens',styles['Title2']))
body('This final supplement follows the completed trace minus 26 twist attack on pages 13-14. It tests two different genus-five Jacobian isogeny classes for degree-seven covers. Both searches were frozen before execution and retain all cell receipts. The result narrows concrete construction leads; it is not a classification of every GHS-vulnerable curve. [pcc:unverified]')
head('What completed')
rows=[['Isogeny class','Exact work','Outcome'],
      ['trace -26, s=7','Full cover, transfer, relation solve and rho','GHS 231.125 s; rho 3268.034 s; 14.14x'],
      ['trace +10','One carrier, two source Frobenius orbits','Zero hits in two denominator shapes'],
      ['trace +22','Twelve carriers, 24 representative pairs','Zero hits; 120 pairs by conjugacy']]
t=Table(rows,colWidths=[95,215,210],hAlign='LEFT')
t.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),colors.HexColor('#233b57')),
                       ('TEXTCOLOR',(0,0),(-1,0),colors.white),
                       ('FONTNAME',(0,0),(-1,0),'Helvetica-Bold'),
                       ('FONTSIZE',(0,0),(-1,-1),7.1),
                       ('VALIGN',(0,0),(-1,-1),'TOP'),
                       ('TOPPADDING',(0,0),(-1,-1),6),
                       ('BOTTOMPADDING',(0,0),(-1,-1),6),
                       ('ROWBACKGROUNDS',(0,1),(-1,-1),[colors.white,colors.HexColor('#edf2f7')])]))
story.extend([t,Spacer(1,8)])
head('Finite bound for the trace +22 screen')
body('The twelve fixed carriers have Weil polynomial X^10 - 22X^5 + 243. Their equations are the twelve carrier masks listed in the saved census. Two source masks represent the ten trace +22 equations over F_(3^5); Lean checks both five-element Frobenius orbits. Exact Sage elimination completed 24 representative cells, each with 243 H-squared and 405 Q_i H-squared systems, and found zero degree-seven identities. Frobenius conjugacy extends that result to 12 x 10 = 120 exact carrier/source pairs in the same shapes. The coverage auditor checked all receipts, source hashes, counters and timings; it is not an independent algebraic elimination.')
body(f'The 24 cells represent {audit["total_logical_assignments"]:,} logical assignments and used {audit["aggregate_user_cpu_s"]:.2f} aggregate user CPU seconds. The trace +22 s=11 group has an 80-bit prime factor, but no evaluated cover or nonzero subgroup map was found. Its favorable leading exponent screen is therefore not an attack cost or vulnerability claim.')
head('Claim boundary and next construction')
body('The trace minus 26 negative-twist equation is a potentially vulnerable exact instance under the measured cost model. It is a quadratic twist of the positive cover orbit, so it does not define an independent new eligible family or a new method. The trace +10 and trace +22 null screens rule out only their fixed carriers and denominator shapes. A genuinely separate family requires a new explicit correspondence or cover on another isogeny class, a nonzero selected-subgroup image, and a complete same-target comparison. Exact-equation searches did not establish worldwide publication priority.')
story.append(Paragraph('Replay: research/ghs_trace10_cover_screen_20261116/RESULT.md and research/ghs_trace22_carrier_variants_20261116/{PROTOCOL.md,manifest.json,audit_manifest.json,Trace22Orbit.lean,RESULT.md}. Prior baselines: Iijima-Momose-Chao 2015; Joux-Vitse 2012; Diem 2006.',styles['Small2']))

def footer(canvas,doc):
    canvas.saveState()
    canvas.setFont('Helvetica',8)
    canvas.setFillColor(colors.HexColor('#64748b'))
    canvas.drawString(36,26,'16 November 2026  |  Odd-degree-five GHS research')
    canvas.drawRightString(576,26,'15')
    canvas.restoreState()

with tempfile.TemporaryDirectory(dir=HERE) as tmp:
    supplement=Path(tmp)/'supplement.pdf'
    merged=Path(tmp)/'merged.pdf'
    SimpleDocTemplate(str(supplement),pagesize=letter,leftMargin=36,rightMargin=36,topMargin=38,bottomMargin=42).build(story,onFirstPage=footer,onLaterPages=footer)
    assert 'Pages:           1' in subprocess.check_output(['pdfinfo',str(supplement)],text=True)
    subprocess.run(['pdfunite',str(OLD),str(supplement),str(merged)],check=True,stderr=subprocess.DEVNULL)
    merged.replace(OUT)
assert 'Pages:           15' in subprocess.check_output(['pdfinfo',str(OUT)],text=True)
print(OUT)
