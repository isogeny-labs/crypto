#!/usr/bin/env python3
"""Render the exact 27 October GHS findings as a compact evidence report."""
from pathlib import Path

from reportlab.lib import colors
from reportlab.lib.enums import TA_CENTER
from reportlab.lib.pagesizes import letter
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.lib.units import inch
from reportlab.platypus import (
    KeepTogether, Paragraph, Preformatted, SimpleDocTemplate,
    Spacer, Table, TableStyle,
)

HERE = Path(__file__).resolve().parent
OUT = HERE/'REPORT.pdf'


def footer(canvas, doc):
    canvas.saveState()
    w, _ = letter
    canvas.setStrokeColor(colors.HexColor('#AAB6C7'))
    canvas.line(44, 38, w-44, 38)
    canvas.setFont('Helvetica', 7.5)
    canvas.setFillColor(colors.HexColor('#526276'))
    canvas.drawString(44, 26, 'GHS transfer audit | 27 October 2026')
    canvas.drawRightString(w-44, 26, f'Page {doc.page}')
    canvas.restoreState()


def main():
    ss = getSampleStyleSheet()
    title = ParagraphStyle('TitleCustom', parent=ss['Title'], fontName='Helvetica-Bold',
                           fontSize=17, leading=21, textColor=colors.HexColor('#16324F'),
                           alignment=TA_CENTER, spaceAfter=8)
    sub = ParagraphStyle('SubtitleCustom', parent=ss['Normal'], fontName='Helvetica',
                         fontSize=9.2, leading=13, alignment=TA_CENTER,
                         textColor=colors.HexColor('#526276'), spaceAfter=16)
    h1 = ParagraphStyle('H1Custom', parent=ss['Heading1'], fontName='Helvetica-Bold',
                        fontSize=11.3, leading=15, textColor=colors.HexColor('#16324F'),
                        spaceBefore=10, spaceAfter=5)
    body = ParagraphStyle('BodyCustom', parent=ss['BodyText'], fontName='Helvetica',
                          fontSize=8.6, leading=12, spaceAfter=7)
    small = ParagraphStyle('SmallCustom', parent=body, fontSize=7.7, leading=10.5)
    code = ParagraphStyle('CodeCustom', parent=ss['Code'], fontName='Courier',
                          fontSize=7.2, leading=10.1, spaceAfter=8,
                          backColor=colors.HexColor('#F2F5F8'), borderPadding=6)
    table_cell = ParagraphStyle('CellCustom', parent=body, fontSize=7.5,
                                leading=10, spaceAfter=0)
    story = []

    def p(text, style=body):
        story.append(Paragraph(text,style))

    def head(text):
        story.append(Paragraph(text,h1))

    def pre(text):
        story.append(Preformatted(text,code))

    def tbl(data,widths):
        t=Table([[Paragraph(str(x),table_cell) for x in row] for row in data],
                colWidths=widths,repeatRows=1,hAlign='LEFT')
        t.setStyle(TableStyle([
            ('BACKGROUND',(0,0),(-1,0),colors.HexColor('#DFEAF3')),
            ('ROWBACKGROUNDS',(0,1),(-1,-1),[colors.white,colors.HexColor('#F6F9FC')]),
            ('VALIGN',(0,0),(-1,-1),'TOP'),
            ('LINEBELOW',(0,0),(-1,0),0.6,colors.HexColor('#9CB1C4')),
            ('BOTTOMPADDING',(0,0),(-1,-1),5),
            ('TOPPADDING',(0,0),(-1,-1),5),
            ('LEFTPADDING',(0,0),(-1,-1),5),
            ('RIGHTPADDING',(0,0),(-1,-1),5),
        ]))
        story.append(KeepTogether([t,Spacer(1,7)]))

    story.append(Paragraph('Residual quartics and a complete Hess transfer',title))
    story.append(Paragraph('Exact GF(16)/GF(4096) curves, verified subgroup transport, and measured attack cost',sub))
    p('<b>Result.</b> A fixed 30,000-draw search independently verified three plane-quartic equations in Jacobian isogeny classes absent from the earlier local screen. A separate, published genus-four Hess construction carried all four exact source subgroups through full split-divisor relation solves. Each measured transfer was at least 512 times slower than same-source rho in this backend. The new quartics still lack explicit source-to-quartic maps.')
    head('1. Exact quartic witnesses')
    p('Let K=GF(2)[w]/(w^12+w^7+w^6+w^5+w^3+w+1), k=GF(2)[a]/(a^4+a+1). Each source is E: Y^2+XY=X^3+w^5 X^2+b, with polynomial-basis integer encoding for b and point coordinates. Quartic vectors use x^i y^j z^(4-i-j), descending i and j.')
    tbl([
        ['#E=#Jac','b','prime r','G / Q=[7]G','(#C(k),#C(k^2),#C(K))','LMFDB class'],
        ['4006','198','2003','(199,2783) / (738,3018)','(17,257,3824)','3.16.a_a_adn'],
        ['4054','32','2027','(33,349) / (2098,1182)','(17,257,3968)','3.16.a_a_abr'],
        ['4078','44','2039','(45,1367) / (47,3257)','(17,257,4040)','3.16.a_a_at'],
        ['4126','63','2063','(62,1499) / (2633,1798)','(17,257,4184)','3.16.a_a_bd'],
    ],[60,25,47,122,137,88])
    pre('4006: [1,14,11,0,8,3,6,8,7,1,15,9,15,12,3]\n'
        '4054: [1,15,8,7,1,6,10,2,10,7,10,3,12,3,12]\n'
        '4078: [1,4,10,9,0,6,9,5,3,8,15,8,2,12,2]\n'
        '4126: [1,3,13,3,5,4,5,4,15,11,15,7,4,9,1]')
    p('Every quartic is geometrically smooth. Alternate-chart recounting and exact Jacobian orders were independently replayed; rational point-difference classes reach the selected prime subgroups. The first three orders were absent from the earlier 2,048-draw screen. Order 4126 already had a different local quartic witness. All four isogeny classes are cataloged in LMFDB, whose stored models for these labels are hyperelliptic only. Publication novelty of the equations remains unresolved.',small)

    head('2. Known direct Hess genus-four subgroup transfers')
    p('For each source in the table, choose Hess descent parameter c=1. The verified genus is four and Magma produced a nonzero order-r divisor-class image. For the order-4126 source E: Y^2+XY=X^3+w^5 X^2+63, the exact model and evaluated images are:')
    pre('C: y^2+(a^8 x^4+a^3 x^2+x+a^4)y\n'
        '   = a^11 x^9+a^13 x^8+a x^5+a^10 x^3\n'
        '     +a x^2+a^12 x+a^12.\n'
        '#Jac(C)(k)=57764=28*2063.')
    pre('h(G)=(x^4+a^6 x^3+a^13 x^2+x+a^8,\n'
        '      a^2 x^3+x^2+a^6 x+a^9, 4)\n'
        'h(Q)=(x^4+a^4 x^2+a^5 x+a^10,\n'
        '      a^12 x^3+a^13 x^2+a^2 x+a^12, 4)')
    p('The other three exact genus-four equations and Mumford image triples appear in the appendix. For every source, Magma checked h(G) nonzero, [r]h(G)=0, [r]h(Q)=0, and h(Q)=[7]h(G). The alternate plane model of the 4126 control has degree nine; Magma\'s documented line-sieve gate q >= d! fails because 16 < 9!. All four full runs used hyperelliptic split-divisor factor bases.')
    head('3. Full relation solve and cost')
    p('Each run factored the Mumford u-polynomial of sampled R=u h(G)+v h(Q) and reconstructed every retained R in Jac(C). Projecting by Jacobian cofactor m gives rows sum(n_i t_i)-m v x = m u (mod r). Independent Python elimination of the saved full-rank rows recovered x=7 for all four sources; Magma checked Q=[7]G on each source.')
    tbl([
        ['#E / r','Jacobian / m','FB; attempts; rows/rank','Full transfer','Rho per target','Ratio'],
        ['4006 / 2003','88132 / 44','21; 211; 22/22','0.350 s','0.000684 s','512x'],
        ['4054 / 2027','56756 / 28','13; 506; 14/14','0.450 s','0.000645 s','698x'],
        ['4078 / 2039','57092 / 28','13; 471; 18/14','0.450 s','0.000645 s','698x'],
        ['4126 / 2063','57764 / 28','13; 431; 15/14','0.430 s','0.000625 s','688x'],
    ],[70,82,122,76,77,52])
    p('Each rho time is amortized over 512 target-verified trials on that same source subgroup. Full transfer times include cover, map, factor-base setup, relations, linear solve, and source verification. They exclude field/source-object setup, as does the rho timer. The ratio range is 512-698. These are calculator-backend measurements, not EKS-qualified benchmarks; the configured AWS caller is outside the required Iso Lab account. These small-order timings are not asymptotic bounds.')
    head('4. Claim boundary and sources')
    p('An exact 8,190-model census of Hess\'s published c=1 choice gives 30 genus-one, 510 genus-three, and 7,650 genus-four descents. Of 2,875 models outside the genus-three Hess isogeny closure, 2,865 have a direct genus-four c=1 curve and ten are subfield genus-one exceptions. The proof uses the cubic Frobenius module decomposition and Hess\'s genus formula; alternate Sage and three Magma controls check the finite classification. This bounds genus in that exact construction, not vulnerability for all 2,865.')
    p('The quartics have exact equations and matching Jacobian classes, but no evaluated elliptic-to-quartic maps or complete attack costs. The genus-four controls have complete maps and solves, but are published and slower than rho. No newly vulnerable family survives these gates. The prior degree-at-most-three exclusion applies only to the listed order-4126 hyperelliptic genus-three models; it does not rule out the genus-four construction or new quartic correspondences.')
    p('Primary sources: Hess, <i>Generalising the GHS Attack on the Elliptic Curve Discrete Logarithm Problem</i> (LMS J. Comput. Math.); Magma Handbook, <i>Weil Descent in Characteristic Two</i> and <i>Linear Equivalence of Divisors / Index Calculus</i>. Exact scripts, receipts, audits, and checked Lean arithmetic/projection theorem accompany this PDF in the PR.',small)

    head('Appendix. Exact Hess outputs for the other three sources')
    p('All equations are over k=GF(16) with a^4+a+1=0. Source points and quartic vectors are on page 1. Each triple below is a reduced Mumford representative of the evaluated subgroup map.',small)
    pre('ORDER 4006, b=198, r=2003, #Jac=88132=44*2003\n'
        'C: y^2+(a^8*x^4+a^10*x^2+a^11*x+a^4)y\n'
        '   = a^5*x^9+a^4*x^8+a^9*x^5+x^4+a^11*x^3\n'
        '     +a^2*x^2+a^14*x+a^4.\n'
        'h(G)=(x^4+a^3*x^2+a^6*x+a^8, a^6*x^2+a^12*x+a^13, 4)\n'
        'h(Q)=(x^4+x^3+a^10*x^2+a^9*x+a^12,\n'
        '      a^10*x^3+a^3*x^2+a^4*x+a^11, 4)')
    pre('ORDER 4054, b=32, r=2027, #Jac=56756=28*2027\n'
        'C: y^2+(a^8*x^4+a^13*x+a^9)y\n'
        '   = a^10*x^9+a^12*x^8+a^10*x^4+a^5*x^3\n'
        '     +a^7*x^2+a^11*x+a^10.\n'
        'h(G)=(x^3+a^2*x^2+a^4*x+a^8, x^2+a^11*x+a, 3)\n'
        'h(Q)=(x^4+a^10*x^3+a^3*x^2+a^2*x+a^9,\n'
        '      a^9*x^3+a^3*x^2+a^5*x, 4)')
    pre('ORDER 4078, b=44, r=2039, #Jac=57092=28*2039\n'
        'C: y^2+(a^8*x^4+a^14*x^2+x+a^9)y\n'
        '   = a^14*x^9+x^8+a^11*x^5+a^11*x^4\n'
        '     +a^13*x^3+a^8*x^2+a^14*x+a^5.\n'
        'h(G)=(x^4+a^6*x^3+a^5*x^2+x+a^10,\n'
        '      a^5*x^3+a*x^2+a^13*x+a^14, 4)\n'
        'h(Q)=(x^4+a^12*x^3+a^5*x^2+a^2,\n'
        '      a^6*x^3+x^2+a^6*x+a^4, 4)')

    doc=SimpleDocTemplate(str(OUT),pagesize=letter,rightMargin=44,leftMargin=44,
                          topMargin=42,bottomMargin=48,
                          title='Residual quartics and a complete Hess transfer',
                          author='GHS research audit')
    doc.build(story,onFirstPage=footer,onLaterPages=footer)
    print(OUT)


if __name__ == '__main__':
    main()
