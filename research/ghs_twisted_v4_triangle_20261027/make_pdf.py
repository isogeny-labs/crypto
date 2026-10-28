#!/usr/bin/env python3
"""Render a compact, evidence-linked PDF of the triangle-cover audit."""
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
    canvas.drawString(48, 34, "Binary cubic triangle covers | research audit | 27 October 2026")
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

    flow = [
        P("Binary cubic triangle covers", title),
        P("An exact eligibility criterion, four canonical quartics, and evaluated subgroup maps", subtitle),
        P("<b>Status.</b> The construction and maps are verified for the four listed curves. "
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
               "type and genus-three transfer method have prior art; exact "
               "overlap with the SCIS 2010/2011 even-characteristic cubic "
               "classification remains unresolved because the full case tables "
               "were unavailable. No global new-family or new-vulnerability "
               "claim follows from this audit."),
             P(link("https://arxiv.org/abs/0905.0546", "Nart and Ritzenthaler (2009)")
               + ": genus-three characteristic-two V4 covers and elliptic quotients."),
             P(link("https://www.iwsec.org/scis/2022/_abst/1A3-2.pdf",
                    "Murai, Shimura, Iijima, and Chao (2022)")
               + ": even-characteristic classification under the isogeny condition."),
             P(link("https://eprint.iacr.org/2013/487.pdf", "Iijima, Momose, and Chao (2013)")
               + ": references the earlier SCIS 2010/2011 cubic weak-cover papers."),
             P(link("https://www.numdam.org/item/10.5802/jtnb.815.pdf", "Diem (2012)")
               + ": expected soft-O(q) Jacobian DLP for nonhyperelliptic genus three."),
             P(link("https://arxiv.org/pdf/2012.07173", "Tian (2020)")
               + ": a separate cubic-extension genus-three transfer via isogenies."),
             P("7. Reproduction and formalization", h1),
             P("The full derivation, all exact coefficients, replay scripts, Magma "
               "inputs and responses, Lean file, and candidate-specific prior-work "
               "record live in research/ghs_twisted_v4_triangle_20261027/ in "
               + link("https://github.com/isogeny-labs/crypto/pull/2735", "draft PR #2735")
               + ". TriangleGate.lean kernel-checks the q=16 exponent/count "
               "arithmetic and trivial doubling kernel for the four named odd "
               "prime orders. It does not formalize the function-field geometry "
               "or the quartic Jacobian computations."),
             ]

    OUT.parent.mkdir(parents=True, exist_ok=True)
    doc = BaseDocTemplate(str(OUT), pagesize=letter, leftMargin=48,
                          rightMargin=48, topMargin=58, bottomMargin=50,
                          title="Binary cubic triangle covers: research audit",
                          author="Cryptanalysis research audit")
    frame = Frame(48, 50, 516, 684, leftPadding=0, rightPadding=0,
                  topPadding=0, bottomPadding=0)
    doc.addPageTemplates(PageTemplate(id="main", frames=[frame], onPage=page))
    doc.build(flow)
    print(OUT)


if __name__ == "__main__":
    render()
