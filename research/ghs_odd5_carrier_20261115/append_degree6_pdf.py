#!/usr/bin/env python3
"""Append the ten-target degree-six replay amendment to REPORT.pdf."""
from pathlib import Path
import json
import subprocess
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle

P = Path(__file__).resolve().parent
pdf = P / 'REPORT.pdf'
assert 'Pages:           7' in subprocess.check_output(['pdfinfo', str(pdf)], text=True)
manifest = json.loads((P / 'degree6_receipt_manifest.json').read_text())
replay = json.loads((P / 'degree6_replay_manifest.json').read_text())
assert len(manifest['rows']) == len(replay['rows']) == 10
assert all(row['shape_counters_agree'] and row['replay_hits'] == 0 for row in replay['rows'])
tmp = P / 'tmp_pdfs'
tmp.mkdir(exist_ok=True)
addendum = tmp / 'degree6_addendum.pdf'
merged = tmp / 'degree6_merged.pdf'
styles = getSampleStyleSheet()
styles.add(ParagraphStyle(name='D6Title', parent=styles['Heading1'], fontName='Helvetica-Bold', fontSize=15, leading=18, spaceAfter=10))
styles.add(ParagraphStyle(name='D6Body', parent=styles['Normal'], fontName='Helvetica', fontSize=9, leading=12, spaceAfter=7))
styles.add(ParagraphStyle(name='D6Small', parent=styles['Normal'], fontName='Helvetica', fontSize=8, leading=10, spaceAfter=5))
story = [Paragraph('Trace +26 class: first K-cover degree 7 to 20', styles['D6Title'])]
def body(s): story.append(Paragraph(s, styles['D6Body']))
body('<b>Amendment to page seven.</b> For the exact genus-five Y/F3 and all ten elliptic targets E_b/K listed there, no K-defined map of degree six exists under the complete pole-shape classification. Howe\'s degree-20 upper bound still applies. The minimum degree for each target is therefore <b>[7,20]</b>. This is a finite-class, computer-assisted bound, not an explicit cover or an attack cost. [pcc:unverified]')
body('<b>Exhaustive degree-six search.</b> Translate infinity to the elliptic origin. The elliptic x-coordinate has degree six and a denominator of odd degree 1, 3 or 5. Finite branch poles have odd multiplicity; other poles have even multiplicity. With branch polynomial (x+1)Q1...Q5, this yields ten complete denominator shapes, 60,518 monic denominators, and 14,506,074 logical branch assignments per target. The LQiQj shape tests another 242 nonzero free numerator coefficients for every assignment. Both implementations check f C^2=B(A^3+2A^2B+bB^3). [pcc:unverified]')
rows = [['b-mask', 'Degree-6 A', 'Coprime and f-divisible', 'Maps', 'Replay']]
for prod, rep in zip(manifest['rows'], replay['rows']):
    assert prod['mask'] == rep['mask']
    rows.append([str(prod['mask']), f"{prod['numerator_degree_six']:,}",
                 f"{prod['f_divides']:,}", str(prod['hits']), 'all counters agree'])
rows.append(['Total', f"{sum(r['numerator_degree_six'] for r in manifest['rows']):,}",
             f"{sum(r['f_divides'] for r in manifest['rows']):,}", '0', '10/10'])
t = Table(rows, colWidths=[65,90,150,50,135], hAlign='LEFT')
t.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),colors.HexColor('#233b57')),
                       ('TEXTCOLOR',(0,0),(-1,0),colors.white),
                       ('FONTNAME',(0,0),(-1,0),'Helvetica-Bold'),
                       ('FONTNAME',(0,-1),(-1,-1),'Helvetica-Bold'),
                       ('FONTSIZE',(0,0),(-1,-1),8),
                       ('TOPPADDING',(0,0),(-1,-1),3),
                       ('BOTTOMPADDING',(0,0),(-1,-1),3),
                       ('ROWBACKGROUNDS',(0,1),(-1,-2),[colors.white,colors.HexColor('#edf2f7')])]))
story += [t, Spacer(1,8)]
body('<b>Independent replay.</b> The producer solves the dominant LH^2 shape by resultants, with an exhaustive fallback for rank-deficient cases, and uses Sage polynomial square testing. The second implementation scans the first field parameter directly, solves a quadratic for the second, and reconstructs each candidate square root coefficient by coefficient. All ten shapes and five counters agree on every target. The 21 Lean declarations verify arithmetic inputs and ceilings without axioms; they do not formalize the geometry. [pcc:unverified]')
body('<b>Cryptanalytic boundary.</b> Any cover in [7,20] preserves the chosen order-109 subgroup under pullback into J(Y)(K), because its degree is coprime to 109. A rational map, transfer into J(Y)(F3), complete relation and descent pipeline, and matched rho timing remain open. Accordingly none of these ten curves is classified here as a new GHS vulnerability. [pcc:unverified]')
story.append(Paragraph('Evidence: TRACE26_CLASS_THEOREM.md, DEGREE6_PROTOCOL.md, DEGREE6_REPLAY_PROTOCOL.md, source-bound per-mask stdout/stderr, and degree6_replay_manifest.json. Upper bound: Howe, Theorem 4.17, arxiv.org/pdf/2110.04221.', styles['D6Small']))

def footer(canvas, doc):
    canvas.saveState()
    canvas.setFont('Helvetica', 8)
    canvas.setFillColor(colors.HexColor('#64748b'))
    canvas.drawString(36, 26, '15 November 2026  |  Odd-degree-five carrier study')
    canvas.drawRightString(576, 26, '8')
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
