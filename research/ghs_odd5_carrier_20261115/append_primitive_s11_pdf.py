#!/usr/bin/env python3
"""Append the s=11 primitive-source and prior-work amendment to REPORT.pdf."""
from pathlib import Path
import json
import subprocess
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle

P = Path(__file__).resolve().parent
pdf = P / 'REPORT.pdf'
assert 'Pages:           8' in subprocess.check_output(['pdfinfo', str(pdf)], text=True)
d = json.loads((P / 'primitive_isogeny_s11.json').read_text())
assert d['isogeny_count'] == 24 and d['primitive_j_count'] == 22
assert d['r'] == 333566377062870325133 and not d['codomain_j_fixed_by_243']
assert all(d['subgroup_checks'].values())
tmp = P / 'tmp_pdfs'; tmp.mkdir(exist_ok=True)
addendum, merged = tmp / 'primitive_s11_addendum.pdf', tmp / 'primitive_s11_merged.pdf'
styles = getSampleStyleSheet()
styles.add(ParagraphStyle(name='S11Title', parent=styles['Heading1'], fontName='Helvetica-Bold', fontSize=15, leading=18, spaceAfter=9))
styles.add(ParagraphStyle(name='S11Body', parent=styles['Normal'], fontName='Helvetica', fontSize=9, leading=12, spaceAfter=7))
styles.add(ParagraphStyle(name='S11Small', parent=styles['Normal'], fontName='Helvetica', fontSize=7.5, leading=10, spaceAfter=5))
story = [Paragraph('Primitive degree-55 source: verified elliptic isogeny, open Jacobian transfer', styles['S11Title'])]
def body(s): story.append(Paragraph(s, styles['S11Body']))
body('<b>Exact source.</b> Let K=F3[a]/(a^5+2a+1), L=F_(3^55), and E/L: y^2=x^3+2x^2+a. The fixed Sage modulus and embedding, all five Weierstrass coefficients of E and E\', the 23-isogeny kernel polynomials, and checked point coordinates are encoded in <font face="Courier">primitive_isogeny_s11.json</font> (SHA-256 f4d2e1726dd845e0e40e9a0a1f75f4451d1fe384bff227bc4d864c91048f7ca9). The selected E\' has j-invariant of full degree 55 over F3. [pcc:unverified]')
rows = [['Measured or checked item','Result'],['Rational 23-isogenies from E','24'],['Codomains with j outside F243','22'],['Selected j field degree over F3','55'],['#E(L)=#E\'(L)','174449211009094048378706606'],['Selected prime subgroup r','333566377062870325133 (69 bits)'],['Forward and signed reverse on subgroup','Q != 0; [r]Q = 0; reverse(Q) = [23]P']]
t=Table(rows,colWidths=[270,270],hAlign='LEFT')
t.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),colors.HexColor('#233b57')),('TEXTCOLOR',(0,0),(-1,0),colors.white),('FONTNAME',(0,0),(-1,0),'Helvetica-Bold'),('FONTSIZE',(0,0),(-1,-1),8),('TOPPADDING',(0,0),(-1,-1),4),('BOTTOMPADDING',(0,0),(-1,-1),4),('ROWBACKGROUNDS',(0,1),(-1,-1),[colors.white,colors.HexColor('#edf2f7')])]))
story += [t,Spacer(1,9)]
body('<b>Replay and bounds.</b> An independent script reconstructed both 23-isogenies from their saved kernels and rechecked the subgroup images and full-degree j. Lean checked three new arithmetic declarations for the s=11 trace step, the group order and U_11=23*131*74673301; all 24 ledger declarations are axiom-free. The earlier ten-target K-cover bound remains [7,20] for the fixed Y/F3 and ten E_b/K. It does not supply the map from E\' to the small-field Jacobian. [pcc:unverified]')
body('<b>Prior-work boundary.</b> Direct odd-characteristic (2,...,2) cover classification for prime degrees 2,3,5,7 is due to Iijima-Momose-Chao (2015). Tian et al. (2018) gave cofactor-two covers; Tian (2020) explicitly transferred via a cubic Weil-restriction isogeny; Fan et al. (2022) gave quadratic prime-order hyperelliptic covers. The exact primitive-j witness is new to this project; global priority and a new vulnerable-family claim are unresolved. [pcc:unverified]')
body('<b>Attack gate.</b> The known 23-isogeny can also move an E\' DLP back to descended E, so matched rho must include that transport and Frobenius symmetry. A usable GHS claim requires an evaluable E\' to J(Y) image over F_(3^11), verified target recovery, and complete transfer, relation, linear-algebra and descent costs below that matched rho baseline.')
story.append(Paragraph('Sources: arxiv.org/pdf/2012.07173; eprint.iacr.org/2015/805.pdf (indexed abstract); doi.org/10.1007/s10623-018-0457-6 (indexed abstract); doi.org/10.1155/2022/9833393. Exact evidence: PRIMITIVE_S11_PROTOCOL.md and verify_primitive_s11.py.', styles['S11Small']))
def footer(canvas, doc):
    canvas.saveState();canvas.setFont('Helvetica',8);canvas.setFillColor(colors.HexColor('#64748b'))
    canvas.drawString(36,26,'16 November 2026  |  Odd-degree-five carrier study');canvas.drawRightString(576,26,'9');canvas.restoreState()
SimpleDocTemplate(str(addendum),pagesize=(612,792),leftMargin=36,rightMargin=36,topMargin=40,bottomMargin=44).build(story,onFirstPage=footer)
subprocess.run(['pdfunite',str(pdf),str(addendum),str(merged)],check=True,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
merged.replace(pdf);addendum.unlink()
try:tmp.rmdir()
except OSError:pass
print(pdf)
