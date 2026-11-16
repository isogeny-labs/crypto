#!/usr/bin/env python3
"""Append the rigorous base-change family bounds and cost screen."""
import json
import subprocess
import tempfile
from pathlib import Path
from reportlab.lib import colors
from reportlab.lib.styles import ParagraphStyle,getSampleStyleSheet
from reportlab.platypus import Paragraph,SimpleDocTemplate,Spacer,Table,TableStyle

P=Path(__file__).resolve().parent;pdf=P/'REPORT.pdf'
assert 'Pages:           11' in subprocess.check_output(['pdfinfo',str(pdf)],text=True)
o=json.loads((P/'frobenius_orbits.json').read_text())
f=json.loads((P/'s11_frobenius_eigenvalue.json').read_text())
assert o['cover_positive_orbit']==[30,194,103,139,233]
assert f['order']==11 and f['checked_on_saved_point']
styles=getSampleStyleSheet()
styles.add(ParagraphStyle(name='BCT',parent=styles['Heading1'],fontName='Helvetica-Bold',fontSize=15,leading=18,spaceAfter=10))
styles.add(ParagraphStyle(name='BCB',parent=styles['Normal'],fontName='Helvetica',fontSize=9,leading=12,spaceAfter=8))
styles.add(ParagraphStyle(name='BCS',parent=styles['Normal'],fontName='Helvetica',fontSize=7.5,leading=10,spaceAfter=5))
story=[Paragraph('Base-change family: rigorous transfer and genus floor',styles['BCT'])]
def body(s):story.append(Paragraph(s,styles['BCB']))
body('Fix E/F_(3^5): v²=u³+2u²+a+a³, a^5+2a+1=0, and the saved degree-seven map from the fixed genus-five Y/F3. The five positive b masks are one Frobenius orbit, so this is one independent construction. For every s coprime to 5, put k_s=F_(3^s) and K_s=F_(3^(5s)). Linear disjointness gives an explicit base-changed degree-seven map Y_(K_s)→E_(K_s). This is an infinite field-size family, not a positive-dimensional moduli family.')
body('The five-term Galois trace of divisor pullback defines F: Res_(F_(3^5)/F3) E→J(Y). Its checked nonzero order-109 image proves F is nonzero. Both sides are simple dimension-five varieties with the same irreducible Weil polynomial X^10−26X^5+3^5, so F is an isogeny. It remains an isogeny after every coprime-to-five base change. For a prime subgroup with r coprime to the fixed degree of F, the map is injective. The simple Weil restriction cannot map nontrivially to a Jacobian of genus below five; Y attains that genus floor for the whole family. The fixed-pair minimum cover degree seven is proved at s=1 only. [pcc:unverified]')
rows=[['s','q=3^s','checked subgroup r','Evidence'],['1','3','109','nonzero trace'],['3','27','65,827','complete solve; rho faster'],['11','177,147','333,566,377,062,870,325,133','nonzero trace; full cost open']]
t=Table(rows,colWidths=[30,65,205,240],hAlign='LEFT')
t.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),colors.HexColor('#233b57')),('TEXTCOLOR',(0,0),(-1,0),colors.white),('FONTNAME',(0,0),(-1,0),'Helvetica-Bold'),('FONTSIZE',(0,0),(-1,-1),8),('TOPPADDING',(0,0),(-1,-1),5),('BOTTOMPADDING',(0,0),(-1,-1),5),('ROWBACKGROUNDS',(0,1),(-1,-1),[colors.white,colors.HexColor('#edf2f7')])]))
story.extend([t,Spacer(1,10)])
body('Diem\'s fixed-genus theorem gives expected O-tilde(q^(8/5)) for the genus-five Jacobian when its class group is cyclic or its full structure is known. The exact groups at s=1,3,11 are squarefree and hence cyclic. Frobenius-aware rho on prime order r has a leading sqrt(r/m) operation scale with orbit order m. The saved s=11 source point has checked m=11 and eigenvalue 213515707826203049379 modulo r. The bare exponent screen is r > m q^(16/5), up to constants and logarithms; Lean checks its integer form for s=3 and s=11 without formalizing the algorithms.')
body('At s=11, q^(8/5) is about 2.50e8 small-field units and sqrt(r/11) is about 5.51e9 source-group units. They are different units, and the omitted relation, linear-algebra, target-descent and implementation costs can dominate. The complete s=3 run measured GHS 11.47 times slower than matched rho. A measured s=11 speedup, worldwide publication priority, and a new general attack method remain unestablished.')
story.append(Paragraph('Full proof scope and receipts: DEGREE7_BASE_CHANGE_THEOREM.md, DEGREE7_FINDING.md, s11_frobenius_eigenvalue.json, S3_FULL_ATTACK_RESULT.md, Odd5Ledger.lean. Cost theorem: Claus Diem, arXiv:math/0606607. Direct-cover comparison: Iijima–Momose–Chao 2015, Section 5, Case 13.',styles['BCS']))
def footer(canvas,doc):
 canvas.saveState();canvas.setFont('Helvetica',8);canvas.setFillColor(colors.HexColor('#64748b'))
 canvas.drawString(36,26,'16 November 2026  |  Odd-degree-five carrier study');canvas.drawRightString(576,26,'12');canvas.restoreState()
with tempfile.TemporaryDirectory(dir=P) as tmp:
 add=Path(tmp)/'addendum.pdf';out=Path(tmp)/'merged.pdf'
 SimpleDocTemplate(str(add),pagesize=(612,792),leftMargin=36,rightMargin=36,topMargin=38,bottomMargin=42).build(story,onFirstPage=footer)
 assert 'Pages:           1' in subprocess.check_output(['pdfinfo',str(add)],text=True)
 subprocess.run(['pdfunite',str(pdf),str(add),str(out)],check=True,stderr=subprocess.DEVNULL)
 out.replace(pdf)
print(pdf)
