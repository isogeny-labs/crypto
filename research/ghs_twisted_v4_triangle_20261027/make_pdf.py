#!/usr/bin/env python3
"""Render a compact, evidence-linked PDF of the binary cubic V4-cover audit."""
import json
from pathlib import Path

from reportlab.lib import colors
from reportlab.lib.enums import TA_CENTER
from reportlab.lib.pagesizes import letter
from reportlab.lib.styles import ParagraphStyle
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import (
    BaseDocTemplate, Frame, PageBreak, PageTemplate, Paragraph, Spacer,
    Table, TableStyle,
)

HERE = Path(__file__).resolve().parent
OUT = HERE / "output/pdf/GHS_TRIANGLE_COVER_AUDIT.pdf"

pdfmetrics.registerFont(TTFont("ArialUnicode", "/System/Library/Fonts/Supplemental/Arial Unicode.ttf"))
pdfmetrics.registerFont(TTFont("ArialBold", "/System/Library/Fonts/Supplemental/Arial Bold.ttf"))
pdfmetrics.registerFontFamily("ArialUnicode", normal="ArialUnicode", bold="ArialBold")

NAVY = colors.HexColor("#193956")
BLUE = colors.HexColor("#2f5f83")
PALE = colors.HexColor("#e8f0f6")
LIGHT = colors.HexColor("#f5f8fa")
INK = colors.HexColor("#152535")

body = ParagraphStyle("body", fontName="ArialUnicode", fontSize=9, leading=12.7,
                      spaceAfter=7, textColor=INK)
small = ParagraphStyle("small", parent=body, fontSize=7.4, leading=10, spaceAfter=0)
code = ParagraphStyle("code", parent=body, fontName="Courier", fontSize=7.8,
                      leading=11, leftIndent=10, spaceAfter=5)
title = ParagraphStyle("title", parent=body, fontName="ArialBold", fontSize=19,
                       leading=23, alignment=TA_CENTER, spaceAfter=10,
                       textColor=NAVY)
subtitle = ParagraphStyle("subtitle", parent=body, fontSize=9,
                          leading=12, alignment=TA_CENTER, spaceAfter=17,
                          textColor=BLUE)
h1 = ParagraphStyle("h1", parent=body, fontName="ArialBold", fontSize=12,
                    leading=15, spaceBefore=10, spaceAfter=6,
                    keepWithNext=True, textColor=NAVY)
h2 = ParagraphStyle("h2", parent=body, fontName="ArialBold", fontSize=10,
                    leading=13, spaceBefore=7, spaceAfter=5,
                    keepWithNext=True, textColor=BLUE)


def P(text, style=body):
    return Paragraph(text, style)


def grid(rows, widths, header=True):
    data = [[P(str(cell), small) for cell in row] for row in rows]
    table = Table(data, colWidths=widths, repeatRows=1 if header else 0,
                  hAlign="LEFT")
    commands = [
        ("VALIGN", (0, 0), (-1, -1), "TOP"),
        ("TOPPADDING", (0, 0), (-1, -1), 5),
        ("BOTTOMPADDING", (0, 0), (-1, -1), 5),
        ("LEFTPADDING", (0, 0), (-1, -1), 6),
        ("RIGHTPADDING", (0, 0), (-1, -1), 6),
        ("ROWBACKGROUNDS", (0, 1 if header else 0), (-1, -1), [colors.white, LIGHT]),
        ("LINEBELOW", (0, -1), (-1, -1), .4, colors.HexColor("#c3d2dc")),
    ]
    if header:
        commands.extend([
            ("BACKGROUND", (0, 0), (-1, 0), PALE),
            ("LINEBELOW", (0, 0), (-1, 0), .8, BLUE),
        ])
    table.setStyle(TableStyle(commands))
    return table


def page(canvas, doc):
    canvas.saveState()
    width, height = letter
    canvas.setStrokeColor(colors.HexColor("#aabfce"))
    canvas.line(48, height - 45, width - 48, height - 45)
    canvas.setFillColor(BLUE)
    canvas.setFont("ArialUnicode", 8)
    canvas.drawString(48, 34, "Binary cubic V4 covers | research audit | 27 October 2026")
    canvas.drawRightString(width - 48, 34, str(doc.page))
    canvas.restoreState()


def link(url, label):
    return f'<link href="{url}" color="#1c5789">{label}</link>'


def render():
    certs = {n: json.loads((HERE / f"certificate_{n}.json").read_text())
             for n in (3972, 4006, 4198, 4222)}
    maps = {n: json.loads((HERE / f"subgroup_map_{n}.json").read_text())
            for n in certs}
    quartics = {n: json.loads((HERE / f"canonical_data_{n}.json").read_text())
                for n in certs}
    two = json.loads((HERE / "two_pole_4132.json").read_text())
    two_map = json.loads((HERE / "two_pole_map_4132.json").read_text())
    two_quartic = json.loads((HERE / "canonical_two_pole_4132.json").read_text())

    flow = [
        P("Binary cubic V4 covers", title),
        P("Two exact eligibility gates, five canonical quartics, and evaluated subgroup maps", subtitle),
        P("<b>Status.</b> The constructions and maps are verified for five listed curves. "
          "This report establishes a low-genus transfer route and a conditional asymptotic "
          "attack bound. A complete attack-cost comparison has not yet been measured."),
        P("1. Family theorem and bound", h1),
        P("Let q=2^m, k=F_q, K=F_(q^3), and let E_(a,b)/K be the ordinary normalized "
          "curve Y^2+XY=X^3+aX^2+b with b nonzero. Let d be the inverse of 2(q+1) "
          "modulo q^3-1. The triangle-cover construction has a quotient isomorphic "
          "to E_(a,b) exactly when"),
        P("Tr_(K/F_2)(a) = Tr_(K/F_2)(b^d).", code),
        P("The exponent exists because gcd(2(q+1),q^3-1)=1. For each b in K*, "
          "exactly one of the two normalized twists passes the trace test. Therefore "
          "the eligible count is <b>q^3-1 of 2(q^3-1)</b> ordinary normalized "
          "K-isomorphism classes. At q=16 it is 4095 of 8190. This count is exact "
          "for this construction; it is neither a global classification of cover "
          "routes nor a count of curves with a measured attack advantage."),
        P("A 2012 seminar abstract reports an approximately three-quarter density "
          "for the wider binary cubic (2,...,2) weak-cover class when the base "
          "field has even degree over F2. The present exact one-half count is a "
          "construction-specific lower bound, not the full weak-cover density."),
        P("Choose nonzero delta in ker Tr_(K/k), and t0+t0^q=delta. With "
          "t1=t0^q, t2=t0^(q^2), lambda=b^d, and A=delta^(q^2-q+1)lambda, "
          "put f0=A/(x+t0)+A^q/(x+t1) and f1=A^q/(x+t1)+A^(q^2)/(x+t2). "
          "The smooth normalization of y0^2+y0=f0, y1^2+y1=f1 is a connected "
          "V4 cover. Its three distinct simple branch poles give genus 3 by "
          "Riemann-Hurwitz. Frobenius cycles its three elliptic quotients, so "
          "the cover descends to k. The differing inertia labels at the three "
          "branch poles rule out a hyperelliptic involution; its canonical model "
          "is a smooth plane quartic."),
        P("2. Exact q=16 source curves", h1),
        P("K=F_2[w]/(w^12+w^7+w^6+w^5+w^3+w+1). Integer coordinates and "
          "quartic coefficients are bitmasks in the basis 1,w,...,w^11. In every "
          "row Q=[7]G."),
    ]

    class_by_order = {3972:"3.16.a_a_aev", 4006:"3.16.a_a_adn",
                      4198:"3.16.a_a_dx", 4222:"3.16.a_a_ev"}
    rows = [["#E(K)", "(a,b)", "r", "G", "Q", "LMFDB class"]]
    for n, cert in certs.items():
        rows.append([str(n), f"({cert['source_a']},{cert['source_b']})",
                     str(cert["subgroup_prime"]), str(tuple(cert["source_G"])),
                     str(tuple(cert["source_Q"])), class_by_order[n]])
    flow += [grid(rows, [52, 65, 40, 102, 102, 155]), Spacer(1, 7),
             P("The 4006 and 4198 classes already had a positive Jacobian entry "
               "in LMFDB. The 3972 and 4222 classes were marked unknown on "
               "27 October 2026; these quartics provide explicit positive "
               "witnesses for those classes. Equivalent equations elsewhere "
               "have not been excluded."),
             PageBreak(),
             P("3. Explicit target quartics", h1),
             P("For each row, F_N(X,Y,Z) is the dot product of its coefficient "
               "vector with the ordered monomials below. Each curve F_N=0 is "
               "smooth over F16, geometrically nonhyperelliptic, and has the "
               "predicted L-polynomial. Magma 2.30-1 receipts and canonical-map "
               "polynomials are retained beside the source files."),
             P("(X^4, X^2Y^2, XY^3, Y^4, X^2YZ, XY^2Z, Y^3Z, X^2Z^2, "
               "XYZ^2, Y^2Z^2, XZ^3, YZ^3, Z^4)", code)]
    qrows = [["#E(K)", "Coefficient vector c_0,...,c_12"]]
    for n, data in quartics.items():
        coeffs = data["quartic_coefficients"]
        assert len(coeffs) == 13
        vector = ", ".join(str(z) for z in coeffs)
        qrows.append([str(n), f'<font name="Courier" size="7">[{vector}]</font>'])
    flow += [grid(qrows, [65, 451]), Spacer(1, 8),
             P("The exact base-field affine model has the form "
               "L(u)D(x)=N(x), with L(U)=U(U+p)(U+v)(U+p+v). "
               "For the 4222 row:"),
             P("(u^4+2622u^2+3158u)(x^6+3362x^4+2623x^2+2623)<br/>"
               "=3159x^5+3158x^4+2891x^3+1641x^2+1821x.", code),
             P("All coefficients in this equation are F16 elements under the "
               "bitmask convention. The other exact affine equations are in "
               "descent_*.json, and audit.py independently checks their rational "
               "identities at every unramified x in K."),
             P("4. Subgroup-preserving transfer", h1),
             P("Let pi_i:C_K -> E_i be the three degree-two quotient maps. "
               "For P in E_0(K), define Phi(P)=sum_i pi_i^*([P^(q^i)]-[O_i]). "
               "This is k-rational. Character orthogonality gives pi_0*Phi(P)=[2]P, "
               "so Phi is injective on every odd-order subgroup. The evaluator "
               "constructs degree-two quotient fibers, forms the divisor-class "
               "differences in the actual Sage quartic Jacobian, and sums the "
               "three Frobenius conjugates."),
             ]
    mrows = [["#E(K)", "r", "base n", "Jacobian setup", "map G,Q", "checks"]]
    for n, item in maps.items():
        assert all(item["checks"].values())
        mrows.append([str(n), str(item["subgroup_prime"]),
                      str(item["translated_base_scalar"]),
                      f"{item['jacobian_setup_seconds']:.2f} s",
                      f"{item['subgroup_map_seconds']:.2f} s", "all pass"])
    flow += [grid(mrows, [58, 45, 56, 112, 94, 151]),
             P("Every row checks Phi(G) nonzero, [r]Phi(G)=0, Phi(Q)=7Phi(G), "
               "and [r]Phi(Q)=0. The displayed times are observed map stages on "
               "a shared host; they exclude a discrete-logarithm solve and "
               "cannot be compared directly with rho."),
             PageBreak(),
             P("5. Attack bound and claim scope", h1),
             P("Diem's theorem gives expected soft-O(q) for discrete logarithms "
               "in a nonhyperelliptic genus-three Jacobian over F_q. Combining "
               "that theorem with this explicit transfer gives a conditional "
               "asymptotic route. If an eligible source sequence has an odd prime "
               "subgroup r=Theta(q^3), generic rho uses Theta(q^(3/2)) group "
               "operations. The trace count does not prove that such large prime "
               "subgroups occur infinitely often. The Hasse bound controls total "
               "source order only: q^3+1-2q^(3/2) <= #E(K) <= "
               "q^3+1+2q^(3/2)."),
             P("Magma's built-in plane-curve IndexCalculus rejects these q=16 "
               "quartics because its line-relation implementation requires "
               "q >= 4! = 24. Complete relation collection, linear algebra, "
               "target recovery, total cost, and matched source-subgroup rho "
               "remain unmeasured. Thus these four rows are transfer-eligible "
               "candidates, not established concrete speedups."),
             P("6. Prior work and novelty labels", h1),
             P("The three labels must stay separate: <b>exact equation</b>, "
               "<b>eligible family</b>, and <b>attack method</b>. The 3972 and "
               "4222 equations may be new catalog additions. The 4006 and 4198 "
               "classes are rediscoveries at isogeny-class level. The V4 cover "
               "type and genus-three transfer method have prior art. A SCIS 2024 "
               "abstract reports classification of all ordinary binary elliptic "
               "sources with extension degree at most nine outside the isogeny "
               "condition; a 2025 poster covers a domain that includes the "
               "normalized q=16 cubic source here. Exact equation-level overlap "
               "remains unresolved without their case tables. No global "
               "new-family or new-vulnerability claim follows from this audit."),
             P(link("https://arxiv.org/abs/0905.0546", "Nart and Ritzenthaler (2009)")
               + ": genus-three characteristic-two V4 covers and elliptic quotients."),
             P(link("https://www.iwsec.org/scis/2022/_abst/1A3-2.pdf",
                    "Murai, Shimura, Iijima, and Chao (2022)")
               + ": even-characteristic classification under the isogeny condition."),
             P(link("https://eprint.iacr.org/2013/487.pdf", "Iijima, Momose, and Chao (2013)")
               + ": references the earlier SCIS 2010/2011 cubic weak-cover papers."),
             P(link("https://sem-wnt.w.waseda.jp/kako/2012.html", "Shimura seminar (2012)")
               + ": reports about three-quarter weak-cover density for even base-field degree."),
             P(link("https://www.iwsec.org/scis/2024/program.html", "SCIS program (2024)")
               + ": reports non-isogeny ordinary binary classification through degree nine."),
             P(link("https://iss.iisec.ac.jp/sympo24/posters/M2_212poster24.pdf", "Uesato poster (2025)")
               + ": gives a wider even-characteristic classification domain."),
             P(link("https://www.numdam.org/item/10.5802/jtnb.815.pdf", "Diem (2012)")
               + ": expected soft-O(q) Jacobian DLP for nonhyperelliptic genus three."),
             P(link("https://arxiv.org/pdf/2012.07173", "Tian (2020)")
               + ": a separate cubic-extension genus-three transfer via isogenies."),
             P("7. Reproduction and formalization", h1),
             P("The full derivation, all exact coefficients, replay scripts, Magma "
               "inputs and responses, Lean file, and candidate-specific prior-work "
               "record live in research/ghs_twisted_v4_triangle_20261027/ in "
               + link("https://github.com/isogeny-labs/crypto/pull/2735", "merged PR #2735")
               + ". TriangleGate.lean kernel-checks the q=16 exponent/count "
               "arithmetic and trivial doubling kernel for the four named odd "
               "prime orders. It does not formalize the function-field geometry "
               "or the quartic Jacobian computations."),
             PageBreak(),
             P("8. Hess's known two-common-pole construction", h1),
             P("Let V=ker Tr_(K/k), a two-dimensional k-space. Choose A,B in V* "
               "with A B=sqrt(b), and put f_i=A^(q^i)x+B^(q^i)/x for i=0,1,2. "
               "The three f_i sum to zero. The connected V4 cover "
               "y0^2+y0=f0, y1^2+y1=f1 descends to k; its two common branch "
               "poles have full V4 inertia and total different 12, so its "
               "smooth normalization has genus three. Its first elliptic quotient "
               "is the normalized E_(a,b) precisely for the trace-zero twist "
               "Tr_(K/F2)(a)=0."),
             P("This is Hess's published double-trace-zero branch. The exact "
               "all-q product count was already proved in this repository's "
               "earlier Hess audit. The calculation below restates it to make "
               "the overlap with the triangle gate precise."),
             P("The product test is a binary-quadratic isotropy test: "
               "Q_s(A)=Tr_(K/k)(s A^q A^(q^2))=0 for s=sqrt(b). Over an algebraic "
               "closure, the three conjugates of A have coordinates (u,v,u+v); "
               "the map from s to the three coefficients of Q_s is invertible. "
               "Counting double and distinct rational linear factors gives the "
               "exact all-q product count below."),
             P("#(V*V*) = (q^2-1)(q+2)/2.", code),
             P("The all-q overlap has a closed form. Put e=(q+1)^(-1) modulo "
               "q^3-1 and t=s^e. Since b=s^2, the triangle trace is the "
               "absolute trace of t. Scaling s by c in k* scales t by "
               "sqrt(c). The projective product classes with relative trace "
               "t=0 are exactly t^(q+1) for t in V*, giving q+1 classes; "
               "the remaining q(q+1)/2 classes each contribute q/2 trace-one "
               "scalars. Hence exactly q^2(q+1)/4 source models pass Hess "
               "while failing the triangle gate."),
             P("Union of the two gates: q^3-1+q^2(q+1)/4 of 2(q^3-1).", code),
             P("At q=16, the Hess gate has 2295 b values, the new overlap count "
               "adds 1088 models, and the union is <b>5183 of 8190</b>. This "
               "is an exact cover-eligibility count and a lower bound for the "
               "wider cover universe; it is not an attack-cost count or a "
               "global vulnerability classification. The overlap formula is "
               "new to this audit; its publication novelty is unresolved."),
             P("9. Exact two-pole witness and subgroup map", h1),
             P("In the pinned F_(2^12) field, A=1079, B=2852, a=0, b=3399, "
               "and E: Y^2+XY=X^3+b. It has #E(K)=4132=4·1033, "
               "G=(2305,2550), and Q=(2273,2935)=7G. Its descended affine "
               "function field is"),
             P("x^2(u^4+2622u^2+3158u)=1821x^4+373x^3+1640x+1.", code),
             P("Magma 2.30-1 confirms genus three, a smooth nonhyperelliptic "
               "canonical quartic, and L-polynomial 1+35T^3+4096T^6. Its "
               "canonical quartic has the coefficient vector below in the "
               "monomial order X^4,X^3Y,X^2Y^2,Y^4,X^2YZ,XY^2Z,X^2Z^2,XYZ^2,YZ^3,Z^4."),
             P('<font name="Courier" size="7">['
               + ', '.join(str(z) for z in two_quartic["quartic_coefficients"])
               + ']</font>', code),
             P("Sage evaluates the quotient pullback and Frobenius trace in "
               "this quartic's actual Jacobian: Phi(G) is nonzero, "
               "[1033]Phi(G)=0, and Phi(Q)=7Phi(G). Jacobian setup was "
               f"{two_map['jacobian_setup_seconds']:.2f} s and the two map "
               f"evaluations were {two_map['subgroup_map_seconds']:.2f} s. "
               "A full index-calculus and matched-rho cost is still unmeasured."),
             P(link("https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_bj",
                    "LMFDB class 3.16.a_a_bj")
               + " already records a positive Jacobian. "
               + link("https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf",
                      "Hess (2004)")
               + " covers the factor branch. This is a new q=16 exact map "
               "witness in this audit, while its family and isogeny class "
               "are known. Its exact equation has not been checked against "
               "every earlier case table."),
             P("TwoPoleGate.lean kernel-checks the q=16 count arithmetic and "
               "the doubling kernel on the 1033-order subgroup; it does not "
               "formalize the field-product proof or geometric map."),
             ]

    OUT.parent.mkdir(parents=True, exist_ok=True)
    doc = BaseDocTemplate(str(OUT), pagesize=letter, leftMargin=48,
                          rightMargin=48, topMargin=58, bottomMargin=50,
                          title="Binary cubic V4 covers: research audit",
                          author="Cryptanalysis research audit")
    frame = Frame(48, 50, 516, 684, leftPadding=0, rightPadding=0,
                  topPadding=0, bottomPadding=0)
    doc.addPageTemplates(PageTemplate(id="main", frames=[frame], onPage=page))
    doc.build(flow)
    print(OUT)


if __name__ == "__main__":
    render()
