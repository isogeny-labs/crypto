#!/usr/bin/env python3
"""Build the auditable short report from the retained cost summary."""

import json
from pathlib import Path

from reportlab import rl_config
from reportlab.lib import colors
from reportlab.lib.enums import TA_CENTER
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.lib.units import inch
from reportlab.platypus import (
    BaseDocTemplate, Frame, KeepTogether, PageTemplate, Paragraph,
    Preformatted, Spacer, Table, TableStyle,
)

root=Path(__file__).resolve().parent
rl_config.useA85=0
audit=json.loads((root/'audit.json').read_text())
pdf=root/'REPORT.pdf'
styles=getSampleStyleSheet()
styles.add(ParagraphStyle(name='TitleCustom',parent=styles['Title'],fontName='Helvetica-Bold',fontSize=17,leading=21,spaceAfter=12))
styles.add(ParagraphStyle(name='SubCustom',parent=styles['Heading2'],fontName='Helvetica-Bold',fontSize=10.5,leading=13,spaceBefore=12,spaceAfter=5))
styles.add(ParagraphStyle(name='BodyCustom',parent=styles['BodyText'],fontName='Helvetica',fontSize=9.1,leading=13,spaceAfter=7))
styles.add(ParagraphStyle(name='SmallCustom',parent=styles['BodyText'],fontName='Helvetica',fontSize=7.7,leading=10,spaceAfter=5))
styles.add(ParagraphStyle(name='MonoCustom',fontName='Courier',fontSize=7.2,leading=10,spaceAfter=6))
styles.add(ParagraphStyle(name='BylineCustom',parent=styles['BodyText'],fontSize=8,leading=10,textColor=colors.HexColor('#405066'),spaceAfter=14))

story=[]
def p(s, style='BodyCustom'): story.append(Paragraph(s,styles[style]))
def section(s): p(s,'SubCustom')

p('An exact cubic Hess family and two complete transfer benchmarks','TitleCustom')
p('Research note | 26 October 2026 | reproducible inputs, raw receipts, and checked claim boundaries','BylineCustom')
p('<b>Finding.</b> For k=F_q and K=F_(q^3), q a power of two, the double trace-zero Hess genus-three branch has an exact membership test and exactly (q-1)(q+1)(q+2)/2 nonzero coefficients. This refines Hess\'s published heuristic count for an already published eligible family. Two exact GF(512) curves have certified nonzero prime-subgroup transfer and complete index-calculus solves. Their measured total attack costs exceed same-source rho by factors 2,304 and 2,784. Neither is a measured vulnerability.')

section('Rigorous criterion and scope')
p('Set U=ker Tr_(K/k). Write delta=sqrt(b)=A u^2+B uv+C v^2 for any k-basis u,v of U. Multiplication Sym^2(U) -> K is an isomorphism: u/v has degree three, so u^2,uv,v^2 are linearly independent. The Hess genus-three factorization delta=gamma beta with gamma,beta in U* holds exactly when B=0, or when B is nonzero and Tr_(k/F_2)(AC/B^2)=0. Counting unordered projective factor pairs and their nonzero scalings gives |P_q|=(q-1)(q+1)(q+2)/2. Its intersection with the direct trace-zero branch has exactly q^2-1 coefficients; the distinct-factor branch outside it has (q-1)q(q+1)/2. For q=2^ell with even ell, subtract q-1 for coefficients of full cubic degree; with odd ell, all counted coefficients have full cubic degree.')
p('This is an upper bound on measured vulnerable source models <b>within this normalized branch</b>, since a speedup requires a useful subgroup image and lower full attack cost. It does not bound all GHS covers, isogenies, or other transfer attacks. The proof is written in EXACT_CUBIC_FAMILY.md; Sage checks q=2,4,8,16 give 6,45,315,2295. Lean checks count arithmetic and abstract scalar preservation, while the field and geometric map remain external certificates.')

section('Exact source and cover identities')
p('K=F_2[w]/(w^9+w^4+1), k=F_2[a]/(a^3+a+1), with the compatible Magma embedding. Integer field coordinates are bits of the Conway polynomial basis. Sources have y^2+xy=x^3+b; in both rows Q=[7]G.')
rows=[['b','|E(K)|','r','G','Q','(gamma,beta)','Jacobian L(T)'],
      ['155','548','137','(478,460)','(328,58)','(392,295)','T^6+35T^3+512'],
      ['32','556','139','(255,59)','(206,467)','(218,439)','T^6+43T^3+512']]
t=Table(rows,colWidths=[31,41,29,68,68,72,119],hAlign='LEFT',repeatRows=1)
t.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),colors.HexColor('#dfe8f1')),('FONTNAME',(0,0),(-1,0),'Helvetica-Bold'),('FONTSIZE',(0,0),(-1,-1),7.1),('BOTTOMPADDING',(0,0),(-1,-1),5),('TOPPADDING',(0,0),(-1,-1),5),('GRID',(0,0),(-1,-1),0.3,colors.HexColor('#aab7c2'))]))
story.append(t)
p('The two smooth canonical quartics over k are as follows. Coordinates are [X:Y:Z].','SmallCustom')
story.append(Preformatted('b=155: X^4+a^5 X^2Y^2+a^5 X^3Z+a XY^2Z+a^3 Y^3Z\n       +a^6 X^2Z^2+a^3 XYZ^2+a^3 Y^2Z^2\n       +a^4 XZ^3+a^6 YZ^3+Z^4=0',styles['MonoCustom']))
story.append(Preformatted('b=32:  X^4+a X^2Y^2+a X^3Z+a^4 XY^2Z+a^6 Y^3Z\n       +a^6 X^2Z^2+a^6 XYZ^2+a^3 Y^2Z^2\n       +XZ^3+a^2 YZ^3+Z^4=0',styles['MonoCustom']))

section('Checked subgroup map and complete solve')
p('For each source, the Hess map sends G and Q to divisor differences D=h(G)-h(O) and T=h(Q)-h(O). After the canonical map, Magma checks D nonprincipal, [r]D principal, and T-[7]D principal. The divisor ideals are reconstructed after base change of the quartic to GF(64); the same three checks pass there. Magma IndexCalculus finds the logarithm 7, then verifies [7]G=Q on the original elliptic curve. The original singular GF(8) sextic did not satisfy the line-sieve field-size condition; the retained failed receipt is separate from the successful canonical-quartic run.')

cost_heading=Paragraph('Full attack versus source-subgroup rho',styles['SubCustom'])
cost_intro=Paragraph('Three fixed-script Magma runs per row; each includes source setup, Hess cover, canonical map, GF(64) base change and checks, index calculus, and source verification. Each run then times 512 deterministic rho solves of the same source pair. Table values are medians across runs; rho is the per-solve mean within a run. Timing resolution is about 0.01 s.',styles['BodyCustom'])
rows=[['b','relations / base','full GHS median','rho mean median','cost ratio']]
for b in ('548','556'):
    d=audit['native_index'][b]
    bval='155' if b=='548' else '32'
    rows.append([bval,f"{d['runs'][0]['relations']} / {d['runs'][0]['factor_base_size']}",
                 f"{d['attack_median_seconds']:.3f} s",
                 f"{d['rho_mean_median_seconds']:.7f} s",
                 f"{d['ratio_of_median_costs']:,.0f}x"])
t=Table(rows,colWidths=[35,95,95,103,90],hAlign='LEFT',repeatRows=1)
t.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),colors.HexColor('#dfe8f1')),('FONTNAME',(0,0),(-1,0),'Helvetica-Bold'),('FONTSIZE',(0,0),(-1,-1),8),('BOTTOMPADDING',(0,0),(-1,-1),6),('TOPPADDING',(0,0),(-1,-1),6),('GRID',(0,0),(-1,-1),0.3,colors.HexColor('#aab7c2'))]))
story.append(KeepTogether([cost_heading,cost_intro,t]))
p('The pure line-sieve stages were 0.140 s (b=155) and 0.240-0.260 s (b=32). A separate Sage relation-walk control solved both fixed source logs in 792.828 s and 261.920 s. It is not used for the native cost ratio.')

section('Prior-work classification and remaining claim')
p('<b>Known eligible family and method:</b> Hess 2004, Section 6, already discusses two trace-zero factors; Menezes-Teske 2004 quantifies a different cubic hyperelliptic branch. Karabina et al. 2010 give broad asymptotic genus and coefficient-count bounds for growing extension degree. <b>Potentially new exact result:</b> the cubic factorization test and closed count relative to the primary literature checked; global publication novelty remains unverified. <b>Exact equations:</b> new witnesses in this project, but their Jacobian isogeny classes are already in LMFDB (3.8.a_a_bj and 3.8.a_a_br); equation-level publication novelty remains open. <b>Measured vulnerability:</b> neither q=8 source beats rho. Diem\'s later plane-quartic result motivates larger-q tests but does not provide a measured crossover for these cases.')

section('Primary sources and reproducibility')
p('<link href="https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf" color="blue">Hess (2004)</link>, Theorems 11, 12, 14 and Section 6; <link href="https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf" color="blue">Menezes-Teske (2004)</link>, Section 7; <link href="https://math.dartmouth.edu/~carlp/WeilDescent-Product.pdf" color="blue">Karabina et al. (2010)</link>, Theorems 8-9; <link href="https://www.numdam.org/articles/10.5802/jtnb.815/" color="blue">Diem (2012)</link>, plane-quartic index calculus; <link href="https://arxiv.org/pdf/0905.0546" color="blue">Nart-Ritzenthaler (2009)</link>, genus-three geometry. Run python3 audit.py, sage verify_split_criterion.sage, and lean Bounds.lean in this directory. Raw Magma receipts, complete scripts, and input hashes are retained alongside this report.','SmallCustom')

def page(canvas,doc):
    canvas.saveState()
    canvas.setStrokeColor(colors.HexColor('#bdc8d4'))
    canvas.line(50,52,562,52)
    canvas.setFont('Helvetica',8)
    canvas.setFillColor(colors.HexColor('#405066'))
    canvas.drawString(50,39,'Exact cubic Hess family - research audit')
    canvas.drawRightString(562,39,f'{doc.page}')
    canvas.restoreState()

doc=BaseDocTemplate(str(pdf),pagesize=(612,792),leftMargin=50,rightMargin=50,topMargin=49,bottomMargin=67,title='Exact cubic Hess family and complete transfer benchmarks')
frame=Frame(50,67,512,676,leftPadding=0,rightPadding=0,topPadding=0,bottomPadding=0)
doc.addPageTemplates(PageTemplate(id='normal',frames=frame,onPage=page))
doc.build(story)
print(pdf)
