"""Build the terminal GHS cover-transfer audit as a shareable PDF."""
from pathlib import Path
from reportlab.lib import colors
from reportlab.lib.enums import TA_CENTER
from reportlab.lib.pagesizes import letter
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.lib.units import inch
from reportlab.platypus import (
    SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle,
    Preformatted, HRFlowable, KeepTogether,
)

HERE = Path(__file__).resolve().parent
OUT = HERE / "GHS_COVER_TRANSFER_FINAL_20261116.pdf"
NAVY = colors.HexColor("#142839")
TEAL = colors.HexColor("#0A6972")
PALE = colors.HexColor("#EAF3F3")
GRAY = colors.HexColor("#51606A")

styles = getSampleStyleSheet()
styles.add(ParagraphStyle(name="ReportTitle", fontName="Helvetica-Bold", fontSize=20,
                          leading=24, textColor=NAVY, spaceAfter=12))
styles.add(ParagraphStyle(name="Deck", fontName="Helvetica", fontSize=10.5,
                          leading=15, textColor=GRAY, spaceAfter=13))
styles.add(ParagraphStyle(name="SectionX", fontName="Helvetica-Bold", fontSize=12,
                          leading=15, textColor=NAVY, spaceBefore=16, spaceAfter=7))
styles.add(ParagraphStyle(name="BodyX", fontName="Helvetica", fontSize=9.3,
                          leading=13.6, textColor=NAVY, spaceAfter=7))
styles.add(ParagraphStyle(name="SmallX", fontName="Helvetica", fontSize=8.2,
                          leading=11.4, textColor=NAVY, spaceAfter=5))
styles.add(ParagraphStyle(name="Callout", fontName="Helvetica-Bold", fontSize=10,
                          leading=14, textColor=TEAL, spaceAfter=8))
styles.add(ParagraphStyle(name="MonoX", fontName="Courier", fontSize=7.4,
                          leading=10.2, textColor=NAVY, leftIndent=7, spaceAfter=8))
styles.add(ParagraphStyle(name="TableHead", fontName="Helvetica-Bold", fontSize=8.2,
                          leading=10, textColor=colors.white))
styles.add(ParagraphStyle(name="TableBody", fontName="Helvetica", fontSize=8.2,
                          leading=11, textColor=NAVY))
styles.add(ParagraphStyle(name="FootX", fontName="Helvetica", fontSize=7.5,
                          leading=10, textColor=GRAY, alignment=TA_CENTER))

P = lambda s, name="BodyX": Paragraph(s, styles[name])
story = []
story.append(P("GHS cover transfer: a complete trace-26 class and a measured mask-167 attack", "ReportTitle"))
story.append(P("Research audit | 16 November 2026 | Exact field, subgroup, maps, proofs, prior work and same-target CPU costs", "Deck"))
story.append(HRFlowable(width="100%", thickness=1.2, color=TEAL, spaceAfter=13))
story.append(P("Measured finding", "SectionX"))
story.append(P("A genus-five GHS route recovered the discrete logarithm of the exact order-185302017921931 subgroup target on the normalized mask-167 curve. A separate Frobenius-aware rho run recovered the same scalar on the same public points. The charged GHS cost was <b>231.294373 CPU seconds</b>; native rho took <b>3160.551528 CPU seconds</b>, a <b>13.664628</b> ratio for this one target.", "Callout"))
story.append(P("The GHS figure is a stitched phase total: its new source-side replay charges the saved relation collection and linear algebra on the same genus-five carrier. Rho is one terminal native trial, not a multi-seed median. The two routes each verified scalar 12345 directly on the public source points."))

story.append(P("Exact source, carrier and transfer", "SectionX"))
story.append(P("Let K=F<sub>3</sub>[a]/(a<super>5</super>+2a+1), L=F<sub>3<super>35</super></sub>, and k=F<sub>3<super>7</super></sub>. The newly certified source model is E<sub>167</sub><super>-</super>: V<super>2</super>=U<super>3</super>+U<super>2</super>+(a<super>4</super>+2a+1). Its 2-isogenous covered partner is E<sub>30</sub><super>-</super>: v<super>2</super>=u<super>3</super>+u<super>2</super>+2(a+a<super>3</super>). Both have 270 points over K; over L their order is 270r, where r=185302017921931. The saved P<sub>167</sub>,Q<sub>167</sub> have exact order r and Q<sub>167</sub>=12345P<sub>167</sub>."))
story.append(P("The reverse degree-two isogeny is evaluated by an explicit rational map. Put W=U+s, s=2a^3+2a^2+1 and d=-a^3-a^2-1. Then:", "SmallX"))
story.append(Preformatted("psi_x = (W^2+dW+a^3+a^2)/(W+d)-1\npsi_y = -V*(W^2+(a^3+a^2+1)W+a^4+a^3-a^2+a-1)\n           /(W^2+(a^3+a^2+1)W+a^4-a^3+a-1)", styles["MonoX"]))
story.append(P("Independent Sage identities check the map, its degree and its action on both public points. The forward map composed with the saved degree-seven cover gives a separable degree-fourteen map from Y<super>-</super><sub>K</sub> to E<sub>167</sub><super>-</super>. Here Y<super>-</super>/F<sub>3</sub> is y<super>2</super>=2(x<super>11</super>+x<super>9</super>+2x<super>8</super>+x<super>7</super>+2x<super>5</super>+x<super>3</super>+x<super>2</super>+x+1). The five-term trace puts its selected subgroup image in J(Y<super>-</super>)(k)."))

story.append(P("Same-target cost ledger", "SectionX"))
rows = [
    [P("Route / phase", "TableHead"), P("CPU seconds", "TableHead"), P("Receipt status", "TableHead")],
    [P("GHS source setup, isogeny and trace", "TableBody"), P("0.401305", "TableBody"), P("New source-native replay", "TableBody")],
    [P("GHS factor base", "TableBody"), P("6.407235", "TableBody"), P("New replay", "TableBody")],
    [P("GHS relations and linear algebra", "TableBody"), P("224.358897", "TableBody"), P("Saved same-carrier work charged", "TableBody")],
    [P("GHS descent and verification", "TableBody"), P("0.126936", "TableBody"), P("New replay", "TableBody")],
    [P("GHS total", "TableBody"), P("231.294373", "TableBody"), P("Scalar 12345 verified", "TableBody")],
    [P("Native rho setup + walk", "TableBody"), P("3160.551528", "TableBody"), P("7,541,531 steps; collision audited", "TableBody")],
]
t = Table(rows, colWidths=[2.3*inch, 1.05*inch, 2.18*inch], repeatRows=1, hAlign="LEFT")
t.setStyle(TableStyle([
    ("BACKGROUND", (0,0), (-1,0), NAVY), ("ROWBACKGROUNDS", (0,1), (-1,-1), [colors.white, PALE]),
    ("VALIGN", (0,0), (-1,-1), "TOP"), ("LEFTPADDING", (0,0), (-1,-1), 7),
    ("RIGHTPADDING", (0,0), (-1,-1), 7), ("TOPPADDING", (0,0), (-1,-1), 6),
    ("BOTTOMPADDING", (0,0), (-1,-1), 6), ("LINEBELOW", (0,-1), (-1,-1), 0.8, TEAL),
]))
story.append(t)
story.append(Spacer(1, 8))
story.append(P("Charging the fixed-carrier targeted cover search adds 189.23 external user+system CPU seconds: GHS becomes 420.524373 seconds and the native-rho ratio becomes 7.515739. That sensitivity row mixes timing conventions and omits broader carrier discovery and failed search arms.", "SmallX"))

story.append(P("Finite class and rigorous limits", "SectionX"))
story.append(P("The fundamental trace discriminant is -296, with class number 10. The existing ten-model census covers every K-isomorphism class of trace +26; quadratic twists give all ten trace -26 classes. The masks 30, 103, 139, 194 and 233 have direct degree-seven covers (one Frobenius orbit). The masks 3, 27, 167, 182 and 230 are their 2-isogenous partners and have degree-fourteen composed covers. Thus all <b>twenty</b> exact K-classes have a cover route. The measured speed result above is for one negative representative; it is not a twenty-curve benchmark."))
story.append(P("Here is a compact specification of all twenty equations. For a mask m, set b<sub>m</sub>=sum<sub>i=0..4</sub>((floor(m/3<super>i</super>) mod 3)a<super>i</super>). The positive model is v<super>2</super>=u<super>3</super>+2u<super>2</super>+b<sub>m</sub>; its negative quadratic twist is v<super>2</super>=u<super>3</super>+u<super>2</super>+2b<sub>m</sub>. Use m in {3, 27, 30, 103, 139, 167, 182, 194, 230, 233}. The degree-two pairings are (30,167), (194,182), (103,230), (139,3) and (233,27).", "SmallX"))
story.append(P("The subgroup-size bound is sharp here. For any g-dimensional abelian variety A/F<sub>3<super>7</super></sub> receiving a nonzero image of this order-r subgroup, r divides #A(k). Weil gives #A(k) at most (sqrt(2187)+1)<super>2g</super>. For g at most 4 this is below 48<super>8</super>=28,179,280,429,056, which is below r. The explicit genus-five carrier attains the minimum dimension."))
story.append(P("A separate isogeny escape bound controls primitive-j transport: for trace +/-26, the Frobenius Lucas value U<sub>7</sub>=-21,156,227 is prime. Any separable isogeny over F<sub>243<super>7</super></sub> from these K-curves to a codomain with j outside K has degree at least 21,156,227. This is a degree bound, not an isogeny evaluation-time or GHS-cover bound."))

story.append(P("Prior work and claim boundary", "SectionX"))
story.append(P("Joux and Vitse (EUROCRYPT 2012) already describe isogeny-assisted cover and decomposition attacks. Iijima, Momose and Chao (2015) classify a specified direct odd-characteristic covering type; its degree-five genus-five case does not exhaust the route above. Howe (2022) supplies cover-existence bounds. The mask-167 equation was already in this project's census. Focused searches of those sources and exact equations found no published instance of this specific paired-orbit map or measured source, but they cannot establish global priority."))
story.append(P("Three labels remain separate: <b>new exact vulnerable curve certificate</b> for mask-167 negative in this project; <b>new eligible parameter family</b> unproved; <b>new attack method</b> unproved. The 20-class cover result is a complete finite stratum, while the performance result is one selected subgroup and one rho seed. Lean checks finite arithmetic, including the 2-isogeny subgroup condition, Lucas recurrence and Weil-bound inequalities; Sage checks geometric maps and collision identities. Elliptic geometry is not formalized in Lean.", "SmallX"))

story.append(P("Evidence and references", "SectionX"))
story.append(P("GitHub PR #4301: https://github.com/isogeny-labs/crypto/pull/4301<br/>Primary receipts: research/ghs_isogenous2_native_20261116/ and research/ghs_isogenous2_rho_final_20261116/. Cover and orbit: research/ghs_twist_isogenous2_20261116/. Bounds: research/ghs_isogeny_escape_20261116/ and research/ghs_genus_floor_s7_20261116/.", "SmallX"))
story.append(P("Joux-Vitse 2012: https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf<br/>Iijima-Momose-Chao 2015: https://eprint.iacr.org/2015/805.pdf<br/>Howe 2022: https://arxiv.org/pdf/2110.04221<br/>Weil bound: https://hal.science/hal-00662352v2/file/Jacobians.pdf", "SmallX"))

def footer(canvas, doc):
    canvas.saveState()
    canvas.setStrokeColor(colors.HexColor("#C4D2D7"))
    canvas.line(48, 41, letter[0]-48, 41)
    canvas.setFont("Helvetica", 7.5)
    canvas.setFillColor(GRAY)
    canvas.drawString(48, 30, "GHS cover-transfer research audit | 16 Nov 2026")
    canvas.drawRightString(letter[0]-48, 30, f"Page {doc.page}")
    canvas.restoreState()

doc = SimpleDocTemplate(str(OUT), pagesize=letter, rightMargin=49, leftMargin=49,
                        topMargin=48, bottomMargin=52, title="GHS cover-transfer audit",
                        author="Isogeny Labs research")
doc.build(story, onFirstPage=footer, onLaterPages=footer)
print(OUT)
