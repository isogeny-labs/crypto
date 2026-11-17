#!/usr/bin/env python3
"""Render a concise PDF companion to REPORT.md."""
from pathlib import Path

from reportlab.lib import colors
from reportlab.lib.enums import TA_CENTER
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.lib.units import inch
from reportlab.platypus import (BaseDocTemplate, Frame, KeepTogether, PageBreak,
                               PageTemplate, Paragraph, Preformatted, Spacer, Table,
                               TableStyle)

HERE = Path(__file__).resolve().parent
DEST = HERE/"output"/"GHS_NONSPLIT_QUADRATIC_POLES.pdf"
DEST.parent.mkdir(exist_ok=True)

styles = getSampleStyleSheet()
styles.add(ParagraphStyle(name="HeadingMain", parent=styles["Title"],
                          fontName="Helvetica-Bold", fontSize=19, leading=22,
                          textColor=colors.HexColor("#15324D"), spaceAfter=12))
styles.add(ParagraphStyle(name="Sub", parent=styles["Heading2"],
                          fontName="Helvetica-Bold", fontSize=11, leading=14,
                          textColor=colors.HexColor("#15324D"), spaceBefore=13,
                          spaceAfter=5))
styles.add(ParagraphStyle(name="BodyN", parent=styles["BodyText"],
                          fontName="Helvetica", fontSize=9, leading=13,
                          spaceAfter=7))
styles.add(ParagraphStyle(name="SmallN", parent=styles["BodyN"],
                          fontSize=7.7, leading=10, spaceAfter=5))
styles.add(ParagraphStyle(name="CenterN", parent=styles["BodyN"],
                          alignment=TA_CENTER))
styles.add(ParagraphStyle(name="CodeN", fontName="Courier", fontSize=8,
                          leading=11, backColor=colors.HexColor("#F2F5F8"),
                          borderPadding=8, spaceBefore=4, spaceAfter=9))


def P(s, style="BodyN"):
    return Paragraph(s, styles[style])


def footer(canvas, doc):
    canvas.saveState()
    canvas.setStrokeColor(colors.HexColor("#C9D4DD"))
    canvas.line(0.65*inch, 0.62*inch, 7.85*inch, 0.62*inch)
    canvas.setFont("Helvetica", 8)
    canvas.setFillColor(colors.HexColor("#526678"))
    canvas.drawString(0.67*inch, 0.43*inch,
                      "GHS quadratic-pole census | 27 October 2026")
    canvas.drawRightString(7.82*inch, 0.43*inch, str(doc.page))
    canvas.restoreState()


doc = BaseDocTemplate(str(DEST), pagesize=(8.5*inch, 11*inch),
                      leftMargin=0.67*inch, rightMargin=0.67*inch,
                      topMargin=0.67*inch, bottomMargin=0.78*inch)
frame = Frame(doc.leftMargin, doc.bottomMargin, doc.width, doc.height,
              leftPadding=0, rightPadding=0, topPadding=0, bottomPadding=0)
doc.addPageTemplates(PageTemplate(id="main", frames=[frame], onPage=footer))

story = [P("A third cubic GHS cover screen: quadratic conjugate poles", "HeadingMain"),
         P("Research audit | exact family census and an evaluated subgroup map", "SmallN"),
         P("The construction below isolates the elliptic-quotient case whose two poles are conjugate over F_(q^3). It adds exact, provable counts beyond the triangle and rational-pole Hess screens. Earlier even-characteristic cubic classifications probably include this geometric case, so global family novelty remains unverified."),
         P("Construction", "Sub"),
         P("Let q=2^m, k=F_q, K=F_(q^3), k2=F_(q^2), and L=F_(q^6). Choose t in k2 with t+t^q=1 and nonzero A in W=ker Tr_(L/k2). Set s=A A^(q^3), b=s^2, and f_i(x)=A^(q^i)/(x+t^(q^i))+A^(q^(i+3))/(x+t^(q^(i+3))) for i=0,1,2. The f_i lie in K(x), cycle under Frobenius, and sum to zero. Their V4 compositum descends to a genus-three curve over k. Its first elliptic quotient is the trace-one twist of Y^2+XY=X^3+aX^2+b."),
         P("Exact all-q bound", "Sub"),
         Preformatted("|Norm(W*)| = (q^3+q-2)/2\n"
                      "N_new(q) = (q^3-q^2-4)/4                 m odd\n"
                      "         = (q^3-q^2+4q-4)/4             m even\n"
                      "Union(q) = 3q^3/2-2                    m odd\n"
                      "         = 3q^3/2+q-2                  m even",
                      styles["CodeN"]),
         P("N_new counts models in this quadratic-pole screen outside the triangle screen. Union counts the triangle, Hess rational-pole, and quadratic-pole screens among 2(q^3-1) normalized ordinary models. These are exact cover-screen counts, not a global count of vulnerable curves or an attack-cost bound."),
         P("Proof in one paragraph", "Sub"),
         P("W is the k2-extension of the two-dimensional k-space ker Tr_(K/k). Multiplication Sym^2(W) to L is an isomorphism, so projective norm values are Frobenius orbits on P^1(k2). There are (q^2+q+2)/2 such orbits. Exactly two have zero relative trace when m is even and none when m is odd. Scalar absolute-trace counts then give N_new. The full argument and independently enumerated field values are in REPORT.md and population.py."),
         PageBreak(),
         P("Exact q=16 source and measured evidence", "HeadingMain"),
         P("Field L=F2[z]/(z^24+z^16+z^15+z^14+z^13+z^10+z^9+z^7+z^5+z^3+1). Decimal coefficients below are binary polynomial bitmasks in the basis 1,z,...,z^23. K is its unique 2^12 subfield."),
         Preformatted("E/K: Y^2+XY = X^3 + 5522909 X^2 + 9159130\n"
                      "#E(K) = 4054 = 2*2027\n"
                      "G=(9159131,11791722)\n"
                      "Q=(744530,10466263)=7G\n"
                      "t=16191718, A=2832837", styles["CodeN"]),
         P("The canonical curve over F16 is a smooth nonhyperelliptic quartic with L-polynomial 1-43T^3+4096T^6. Its exact equation and quotient rational map are in canonical_4054.json and quotient_map_4054.json. The evaluated traced correspondence satisfies Phi(G) nonzero, [2027]Phi(G)=0, Phi(Q) nonzero, [2027]Phi(Q)=0, and Phi(Q)=[7]Phi(G)."),
         P("Population and costs", "Sub")]

rows = [["q", "Norm values", "Added models", "Three-screen union / all"],
        ["2", "4", "0", "10 / 14"], ["4", "33", "15", "98 / 126"],
        ["8", "259", "111", "766 / 1022"],
        ["16", "2055", "975", "6158 / 8190"]]
table = Table(rows, colWidths=[0.65*inch, 1.2*inch, 1.3*inch, 2.3*inch])
table.setStyle(TableStyle([
    ("BACKGROUND", (0,0),(-1,0),colors.HexColor("#15324D")),
    ("TEXTCOLOR", (0,0),(-1,0),colors.white),
    ("FONTNAME", (0,0),(-1,0),"Helvetica-Bold"),
    ("FONTSIZE", (0,0),(-1,-1),8),
    ("BOTTOMPADDING",(0,0),(-1,-1),7),
    ("TOPPADDING",(0,0),(-1,-1),7),
    ("ROWBACKGROUNDS",(0,1),(-1,-1),[colors.white,colors.HexColor("#F2F5F8")]),
    ("LINEBELOW",(0,-1),(-1,-1),0.5,colors.HexColor("#C9D4DD"))]))
story += [table, Spacer(1,9),
          P("The full F256 Magma run verified the divisor orders, found 260 relations over 201 factor-base points, solved the matrix, and returned 7 mod 2027. Its proof-heavy job took 67.239 s. A separate timed run omitted those already checked assertions: setup 0.190 CPU s, index recovery 0.170 CPU s, and complete server job 0.459 s."),
          P("On the same source subgroup and target, explicit Magma Floyd rho solved 100/100 runs, median 168 group step calls, in 0.160 CPU s total (0.00160 s per solve on average). The measured Magma GHS core is about 225 times that average rho solve. Local Sage source-map setup and evaluation add 15.15 s and 5.04 s on another machine, so they are reported separately. This q=16 example shows no GHS-over-rho advantage."),
          P("Prior-work boundary", "Sub"),
          P("The order-4054 Jacobian isogeny class was already present in the repository and in LMFDB as 3.16.a_a_abr. Shimura, Momose, and Chao (SCIS 2010) studied cubic even-characteristic weak covers; a 2015 survey already attributes a three-quarters density to that work. Murai et al. (SCIS 2022) announced a corrected classification under the isogeny condition. The exact source equation and map are new to this audit; global family and method novelty remain unverified."),
          P("References and reproducibility", "Sub"),
          P("Primary sources: https://www.iwsec.org/scis/2022/_abst/1A3-2.pdf ; https://arxiv.org/abs/0905.0546 ; https://arxiv.org/abs/2012.07173 . Scripts, Magma receipts, the checked Lean arithmetic, and the full proof are in this research directory.", "SmallN")]

doc.build(story)
print(DEST)
