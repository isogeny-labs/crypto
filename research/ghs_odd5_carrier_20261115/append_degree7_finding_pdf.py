#!/usr/bin/env python3
"""Append the independently checked degree-seven finding to the research PDF."""
import json
import subprocess
import tempfile
from pathlib import Path

from reportlab.lib import colors
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.platypus import Paragraph, SimpleDocTemplate, Spacer, Table, TableStyle

P=Path(__file__).resolve().parent
pdf=P/'REPORT.pdf'
assert 'Pages:           9' in subprocess.check_output(['pdfinfo',str(pdf)],text=True)
m=json.loads((P/'degree7_evidence_manifest.json').read_text())
assert [(x['bmask'],x['hits']) for x in m['rows']]==[(3,0),(27,0),(30,1),(103,1),(139,1),(167,0),(182,0),(194,1),(230,0),(233,1)]
assert all(x.get('map_checks_passed') for x in m['rows'] if x['hits'])
mag=json.loads((P/'degree7_hits_replay_magma.json').read_text())
assert mag['response']['success'] and mag['response']['stdout'].count('PASS degree7 identity')==5
prim=json.loads((P/'primitive_cover_s11.json').read_text())
assert prim['primitive_j_degree']==55 and all(prim['checks'].values())
styles=getSampleStyleSheet()
styles.add(ParagraphStyle(name='A7Title',parent=styles['Heading1'],fontName='Helvetica-Bold',fontSize=15,leading=18,spaceAfter=10))
styles.add(ParagraphStyle(name='A7Body',parent=styles['Normal'],fontName='Helvetica',fontSize=9,leading=12,spaceAfter=8))
styles.add(ParagraphStyle(name='A7Tiny',parent=styles['Normal'],fontName='Helvetica',fontSize=7.5,leading=10,spaceAfter=5))
story=[Paragraph('Five degree-seven covers: explicit transfer and scoped bound',styles['A7Title'])]
def body(s):story.append(Paragraph(s,styles['A7Body']))
body('Fix K=F3[a]/(a^5+2a+1) and Y/F3: y²=x^11+x^9+2x^8+x^7+2x^5+x³+x²+x+1. For b in the table, E_b/K has equation v²=u³+2u²+b and order 218=2·109. Each saved cover Y_K→E_b has u=A/B, v=yC/B², degree 7, and exact identity fC²=B(A³+2A²B+bB³). The complete degree-at-most-six exclusion for this fixed carrier and ten trace-+26 targets proves degree 7 is minimal for these five. [pcc:unverified]')
rows=[['b mask','Exact b','Pole Q index','Min. degree'],['30','a+a^3','3','7'],['103','1+a+2a^2+a^4','1','7'],['139','1+a+2a^3+a^4','2','7'],['194','2+a+a^3+2a^4','4','7'],['233','2+2a+a^2+2a^3+2a^4','0','7']]
t=Table(rows,colWidths=[57,330,80,73],hAlign='LEFT')
t.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),colors.HexColor('#233b57')),('TEXTCOLOR',(0,0),(-1,0),colors.white),('FONTNAME',(0,0),(-1,0),'Helvetica-Bold'),('FONTSIZE',(0,0),(-1,-1),8),('TOPPADDING',(0,0),(-1,-1),4),('BOTTOMPADDING',(0,0),(-1,-1),4),('ROWBACKGROUNDS',(0,1),(-1,-1),[colors.white,colors.HexColor('#edf2f7')])]))
story.extend([t,Spacer(1,10)])
body('Orbit classification: b masks 30, 194, 103, 139, 233 are one F3-Frobenius cycle. Their five saved cover equations are conjugate, with the C polynomial sometimes differing by sign. The other five trace-+26 masks form the second orbit. These are five exact K-models but one independent positive orbit. Sage, Magma and axiom-free Lean arithmetic check the orbit relation.')
body('Magma 2.30-1 independently checked all five polynomial identities, degrees and coprimality. The pullback must subtract the pole-fiber 2-torsion class (Q_i,0). For every curve, the resulting Galois trace is nonzero of order 109 in J(Y)(F3), and the separately evaluated image of 37P equals 37 times the image of P. At s=11, all five traces are nonzero of prime order r=333566377062870325133 in J(Y)(F_(3^11)); source and target group orders equal 2·109·2399·r. Exact coefficient vectors and group images are in the linked JSON receipts.')
body('A further exact E′/F_(3^55) has j-invariant of full degree 55. A saved 23-isogeny to and from E_30, followed by the same corrected cover pullback and trace, gives a nonzero order-r image for E′. Independent kernel-polynomial replay reconstructed the two isogenies and the Jacobian image. The forward 23-isogeny composed with the degree-seven cover gives a degree-161 map Y→E′ over F_(3^55).')
body('Cost boundary: Diem’s 2006 fixed-genus theorem gives expected Õ(q^(8/5)) for a cyclic genus-five Jacobian, subject to its model hypotheses. J(Y)(F_(3^11)) is cyclic because its order is squarefree. This exponent is a theoretical bound, not a measured cost. At s=11 the full relation, decomposition, linear-algebra, target-descent and final-verification pipeline has not been timed against a Frobenius-aware matched rho run. Page 11 reports the complete measured s=3 run. The exact records are new to this project; global publication priority and a new attack method remain unconfirmed.')
story.append(Paragraph('Evidence: DEGREE7_FINDING.md; degree7_evidence_manifest.json; degree7_hits_replay_magma.json; primitive_cover_s11.json; verify_primitive_cover_s11.py. Prior work: Diem (arXiv:math/0606607); Iijima–Momose–Chao (2015 full text, Case 13); Tian (arXiv:2012.07173); Fan et al. (DOI 10.1155/2022/9833393).',styles['A7Tiny']))
def footer(canvas,doc):
 canvas.saveState();canvas.setFont('Helvetica',8);canvas.setFillColor(colors.HexColor('#64748b'))
 canvas.drawString(36,26,'16 November 2026  |  Odd-degree-five carrier study');canvas.drawRightString(576,26,'10');canvas.restoreState()
with tempfile.TemporaryDirectory(dir=P) as tmp:
 add=Path(tmp)/'addendum.pdf';out=Path(tmp)/'merged.pdf'
 SimpleDocTemplate(str(add),pagesize=(612,792),leftMargin=36,rightMargin=36,topMargin=38,bottomMargin=42).build(story,onFirstPage=footer)
 assert 'Pages:           1' in subprocess.check_output(['pdfinfo',str(add)],text=True)
 subprocess.run(['pdfunite',str(pdf),str(add),str(out)],check=True)
 out.replace(pdf)
print(pdf)
