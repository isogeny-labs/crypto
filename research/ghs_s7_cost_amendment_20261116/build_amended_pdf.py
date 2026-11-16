#!/usr/bin/env python3
"""Append the first-target cost accounting amendment to the 15-page report."""
import json
import subprocess
import tempfile
from pathlib import Path

from reportlab.lib import colors
from reportlab.lib.pagesizes import letter
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.platypus import Paragraph, SimpleDocTemplate, Spacer, Table, TableStyle

HERE=Path(__file__).resolve().parent
OLD=HERE.parent/'ghs_trace22_carrier_variants_20261116'/'REPORT_WITH_CARRIER_CENSUS.pdf'
OUT=HERE/'REPORT_COST_AMENDED.pdf'
d=json.loads((HERE/'cost_amendment.json').read_text())
assert d['source_mask']==30
assert abs(d['rho_over_given_cover_ghs']-14.13967551628992)<1e-8
assert abs(d['rho_over_search_plus_solve']-7.774460485888538)<1e-8
assert 'Pages:           15' in subprocess.check_output(['pdfinfo',str(OLD)],text=True)

styles=getSampleStyleSheet()
styles.add(ParagraphStyle(name='AHeading',parent=styles['Title'],fontName='Helvetica-Bold',fontSize=16,leading=20,spaceAfter=12,textColor=colors.HexColor('#193650')))
styles.add(ParagraphStyle(name='ASub',parent=styles['Heading2'],fontName='Helvetica-Bold',fontSize=11,leading=14,spaceBefore=10,spaceAfter=5,textColor=colors.HexColor('#24506f')))
styles.add(ParagraphStyle(name='ABody',parent=styles['Normal'],fontSize=8.8,leading=12,spaceAfter=7))
styles.add(ParagraphStyle(name='ASmall',parent=styles['Normal'],fontSize=7.4,leading=10,spaceAfter=5))
story=[]
def body(s):story.append(Paragraph(s,styles['ABody']))
def head(s):story.append(Paragraph(s,styles['ASub']))

story.append(Paragraph('Amendment: charge the targeted cover search',styles['AHeading']))
body('The 231.125-second GHS figure on pages 13 and 15 is the complete source-target relation solve <b>with the degree-seven cover supplied</b>. The cover was found in a separate targeted fixed-carrier search. This page preserves both measurements and records the combined cost for one target under an explicit narrower discovery model. [pcc:unverified]')
head('Saved CPU receipts')
rows=[['Phase or comparison','CPU seconds','Rho/GHS'],
      ['Targeted mask-30 cover search','189.23',''],
      ['Supplied-cover GHS solve','231.125131','14.14'],
      ['Search plus GHS solve','approximately 420.36','approximately 7.77'],
      ['Same-target order-seven Frobenius rho','3268.034356','1.00']]
t=Table(rows,colWidths=[305,115,100],hAlign='LEFT')
t.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),colors.HexColor('#233b57')),
                       ('TEXTCOLOR',(0,0),(-1,0),colors.white),
                       ('FONTNAME',(0,0),(-1,0),'Helvetica-Bold'),
                       ('FONTSIZE',(0,0),(-1,-1),8),
                       ('TOPPADDING',(0,0),(-1,-1),6),
                       ('BOTTOMPADDING',(0,0),(-1,-1),6),
                       ('ROWBACKGROUNDS',(0,1),(-1,-1),[colors.white,colors.HexColor('#edf2f7')])]))
story.extend([t,Spacer(1,8)])
body('The search receipt is the original positive-trace mask-30 quadratic-square run: 186.99 user plus 2.24 system CPU seconds, one exact cover hit, source SHA-256 verified. Quadratic twisting gives the negative-trace map used for the s=7 solve. The separate GHS receipt records 231.125131 in-process CPU seconds through relation collection, linear algebra, descent and verification; rho records 3268.034356 in-process CPU seconds on the same P and Q, recovering scalar 12,345.')
head('Measurement boundary')
body('The approximately 420.36-second sum combines an external process CPU measure rounded to 0.01 second with in-process solve timing. The search measure includes its interpreter startup; both solve measures exclude interpreter startup. It is therefore a documented mixed-convention estimate, not a uniformly instrumented contiguous run. It charges one targeted search on the already selected carrier. Carrier discovery, other screened sources and failed cover shapes are outside the sum; their combined first-discovery cost was not measured as one attack.')
body('The 14.14x ratio is the reusable-cover solve comparison. The approximately 7.77x ratio includes the recorded targeted cover search for a first target. Both are single-target, implementation-specific observations; rho collision time varies and optimized programs could change either ratio. The exact twist is a potentially vulnerable curve record under this model, while an independent new eligible family and global publication priority remain unestablished.')
story.append(Paragraph('Replay: research/ghs_s7_cost_amendment_20261116/{AMENDMENT.md,cost_amendment.json,reprice.py}. Primary source receipts: degree7_quadratic_square_30.json and .stderr; s7_full_attack.json; rho_s7_result.json. This page supersedes any reading of 231.125 seconds as including cover discovery.',styles['ASmall']))

def footer(canvas,doc):
    canvas.saveState()
    canvas.setFont('Helvetica',8)
    canvas.setFillColor(colors.HexColor('#64748b'))
    canvas.drawString(36,26,'16 November 2026  |  Odd-degree-five GHS research')
    canvas.drawRightString(576,26,'16')
    canvas.restoreState()

with tempfile.TemporaryDirectory(dir=HERE) as tmp:
    supplement=Path(tmp)/'supplement.pdf'
    merged=Path(tmp)/'merged.pdf'
    SimpleDocTemplate(str(supplement),pagesize=letter,leftMargin=36,rightMargin=36,topMargin=38,bottomMargin=42).build(story,onFirstPage=footer,onLaterPages=footer)
    assert 'Pages:           1' in subprocess.check_output(['pdfinfo',str(supplement)],text=True)
    subprocess.run(['pdfunite',str(OLD),str(supplement),str(merged)],check=True,stderr=subprocess.DEVNULL)
    merged.replace(OUT)
assert 'Pages:           16' in subprocess.check_output(['pdfinfo',str(OUT)],text=True)
print(OUT)
