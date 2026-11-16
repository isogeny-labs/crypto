#!/usr/bin/env python3
"""Append the complete s=3 solve and matched rho measurement to REPORT.pdf."""
import json
import subprocess
import tempfile
from pathlib import Path
from reportlab.lib import colors
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.platypus import Paragraph, SimpleDocTemplate, Spacer, Table, TableStyle

P=Path(__file__).resolve().parent;pdf=P/'REPORT.pdf'
assert 'Pages:           10' in subprocess.check_output(['pdfinfo',str(pdf)],text=True)
a=json.loads((P/'s3_full_attack.json').read_text())
b=json.loads((P/'s3_rho_standalone.json').read_text())
assert a['recovered_scalar']==b['validation_scalar']==12345 and a['relation_rank']==12
assert len(b['trials'])==30 and all(x['scalar']==12345 for x in b['trials'])
styles=getSampleStyleSheet()
styles.add(ParagraphStyle(name='S3T',parent=styles['Heading1'],fontName='Helvetica-Bold',fontSize=15,leading=18,spaceAfter=10))
styles.add(ParagraphStyle(name='S3B',parent=styles['Normal'],fontName='Helvetica',fontSize=9,leading=12,spaceAfter=8))
styles.add(ParagraphStyle(name='S3S',parent=styles['Normal'],fontName='Helvetica',fontSize=7.5,leading=10,spaceAfter=5))
story=[Paragraph('Complete s=3 solve: degree-seven transfer versus matched rho',styles['S3T'])]
def body(s):story.append(Paragraph(s,styles['S3B']))
body('Exact source: E/F_(3^15): v²=u³+2u²+a+a³, where a^5+2a+1=0. The saved degree-15 field polynomial and source coordinates pin the full instance. #E=14,350,286=2·109·65,827. The selected P has prime order 65,827; Q=12,345P is the validation target. The degree-seven cover to the fixed genus-five Y, corrected pole-fiber pullback, and F_(3^3)-Galois trace produced nonzero order-65,827 Jacobian points G and H.')
body('The index-calculus path built 12 nonzero projected factor-base classes from 27 finite rational points. It tested 3,406 divisors and accepted 52 split relations, solved a rank-12 modular system, checked every factor-base log on the Jacobian, and descended H after 138 offsets. It recovered 12,345 and verified the scalar on both E and J. An independent auditor reconstructed the cover and both mapped points, checked all 52 equations and the rank, and reproduced the target scalar. Earlier rank-cap and timing-output attempts remain preserved.')
t=a['timing'];rows=[['Measured CPU item','Seconds'],['GHS setup and factor base',f"{t['setup_and_factor_base_cpu_s']:.6f}"],['Pullback and trace',f"{t['transfer_cpu_s']:.6f}"],['Relations and linear algebra',f"{t['relations_and_la_cpu_s']:.6f}"],['Descent and final verification',f"{t['descent_verify_cpu_s']:.6f}"],['GHS total including setup',f"{t['ghs_total_cpu_s']:.6f}"],['Standalone Frobenius rho setup',f"{b['setup_cpu_s']:.6f}"],['Median rho trial, 30 verified solves',f"{b['median_trial_cpu_s']:.6f}"],['Median one-solve rho including setup',f"{b['median_one_solve_cpu_s']:.6f}"]]
tab=Table(rows,colWidths=[350,190],hAlign='LEFT')
tab.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),colors.HexColor('#233b57')),('TEXTCOLOR',(0,0),(-1,0),colors.white),('FONTNAME',(0,0),(-1,0),'Helvetica-Bold'),('FONTSIZE',(0,0),(-1,-1),8),('TOPPADDING',(0,0),(-1,-1),4),('BOTTOMPADDING',(0,0),(-1,-1),4),('ROWBACKGROUNDS',(0,1),(-1,-1),[colors.white,colors.HexColor('#edf2f7')])]))
story.extend([tab,Spacer(1,9)])
ratio=t['ghs_total_cpu_s']/b['median_one_solve_cpu_s']
body(f'The matched rho run used the same P and Q and canonicalized the order-three Frobenius orbit, with checked eigenvalue 2447 modulo 65,827. All 30 trials recovered 12,345; median walk length was 148 steps. The measured total CPU ratio was {ratio:.2f}: rho was faster on this exact instance. Both totals exclude Sage interpreter startup but include their own in-process setup. [pcc:unverified]')
body('Scope: this is a complete small-field attack calibration for mask 30. Five conjugate K-models in one Frobenius orbit have verified 69-bit subgroup transfers at s=11, plus one primitive-j isogeny-composed source. Their full index-calculus and matched-rho costs have not been measured. Diem\'s fixed-genus O-tilde(q^(8/5)) theorem is asymptotic and does not replace those measurements. Exact cover records are new to this project; global priority and a new attack method remain unconfirmed.')
story.append(Paragraph('Evidence: S3_FULL_ATTACK_RESULT.md; s3_full_attack.json; s3_rho_standalone.json; audit_s3_full_attack.py; DEGREE7_FINDING.md. Prior cost theorem: arxiv.org/abs/math/0606607.',styles['S3S']))
def footer(canvas,doc):
 canvas.saveState();canvas.setFont('Helvetica',8);canvas.setFillColor(colors.HexColor('#64748b'))
 canvas.drawString(36,26,'16 November 2026  |  Odd-degree-five carrier study');canvas.drawRightString(576,26,'11');canvas.restoreState()
with tempfile.TemporaryDirectory(dir=P) as tmp:
 add=Path(tmp)/'addendum.pdf';out=Path(tmp)/'merged.pdf'
 SimpleDocTemplate(str(add),pagesize=(612,792),leftMargin=36,rightMargin=36,topMargin=38,bottomMargin=42).build(story,onFirstPage=footer)
 assert 'Pages:           1' in subprocess.check_output(['pdfinfo',str(add)],text=True)
 subprocess.run(['pdfunite',str(pdf),str(add),str(out)],check=True,stderr=subprocess.DEVNULL)
 out.replace(pdf)
print(pdf)
