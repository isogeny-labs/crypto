#!/usr/bin/env python3
"""Render existing mathematical prose and recorded Rust diagnostics only.

This script performs document/figure layout. It does not execute field,
elliptic-curve, resultant, or benchmark computations.
"""
from pathlib import Path
import json
import subprocess
import tempfile
import hashlib
import fitz
from xml.sax.saxutils import escape

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.ticker import FuncFormatter
from reportlab.lib import colors
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib.enums import TA_LEFT
from reportlab.platypus import (
    SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, PageBreak,
    Image, KeepTogether,
)
from reportlab.graphics.shapes import Drawing, Rect, String, Line, Polygon
from reportlab.graphics import renderSVG, renderPDF
from pypdf import PdfReader

ROOT = Path(__file__).resolve().parent
FIG = ROOT / "figures"
FIG.mkdir(exist_ok=True)
NAVY = colors.HexColor("#153145")
TEAL = colors.HexColor("#087C80")
INK = colors.HexColor("#243746")
MUTED = colors.HexColor("#607482")
PALE = colors.HexColor("#EDF5F5")
GREY = colors.HexColor("#F2F5F7")
AMBER = colors.HexColor("#9B5B13")
RULE = colors.HexColor("#CCD8DE")
WIDTH = 504

# Embed portable subsets: unembedded base fonts can be substituted badly.
FONT_ROOT = Path("/usr/share/fonts/truetype/liberation")
pdfmetrics.registerFont(TTFont("ReportSans", str(FONT_ROOT/"LiberationSans-Regular.ttf")))
pdfmetrics.registerFont(TTFont("ReportSansBold", str(FONT_ROOT/"LiberationSans-Bold.ttf")))
pdfmetrics.registerFontFamily("ReportSans", normal="ReportSans", bold="ReportSansBold",
                             italic="ReportSans", boldItalic="ReportSansBold")

styles = getSampleStyleSheet()
styles.add(ParagraphStyle("BodyR", fontName="ReportSans", fontSize=10.2,
                          leading=14.5, textColor=INK, spaceAfter=8))
styles.add(ParagraphStyle("SmallR", parent=styles["BodyR"], fontSize=8.2,
                          leading=11.1, spaceAfter=5))
styles.add(ParagraphStyle("CaptionR", parent=styles["BodyR"], fontSize=8.1,
                          leading=10.8, textColor=MUTED, spaceAfter=8))
styles.add(ParagraphStyle("TitleR", fontName="ReportSansBold", fontSize=29,
                          leading=33, textColor=NAVY, spaceAfter=12))
styles.add(ParagraphStyle("H1R", fontName="ReportSansBold", fontSize=21,
                          leading=25, textColor=NAVY, spaceBefore=1, spaceAfter=14))
styles.add(ParagraphStyle("H2R", fontName="ReportSansBold", fontSize=12,
                          leading=15, textColor=TEAL, spaceBefore=9, spaceAfter=6))
styles.add(ParagraphStyle("EquationR", fontName="ReportSans", fontSize=13.6,
                          leading=20, textColor=NAVY, spaceAfter=5))
styles.add(ParagraphStyle("MonoR", fontName="ReportSans", fontSize=7.6,
                          leading=10.5, textColor=INK, spaceAfter=5))
styles.add(ParagraphStyle("LabelR", fontName="ReportSansBold", fontSize=8.2,
                          leading=11, textColor=TEAL, spaceAfter=7))
styles.add(ParagraphStyle("DisplayBoxR", parent=styles["BodyR"],
                          fontSize=14, leading=24))

def p(text, style="BodyR"):
    return Paragraph(text, styles[style])

def h(text):
    return p(text, "H2R")

def eq(text):
    return p(text, "EquationR")

def box(title, text, fill=PALE, body_style="BodyR"):
    data = [[p(title, "LabelR")], [p(text, body_style)]]
    t = Table(data, colWidths=[WIDTH - 20])
    t.setStyle(TableStyle([
        ("BACKGROUND", (0,0), (-1,-1), fill),
        ("BOX", (0,0), (-1,-1), .5, RULE),
        ("LEFTPADDING", (0,0), (-1,-1), 10),
        ("RIGHTPADDING", (0,0), (-1,-1), 10),
        ("TOPPADDING", (0,0), (-1,0), 9),
        ("BOTTOMPADDING", (0,-1), (-1,-1), 8),
    ]))
    return t

def table(rows, widths, font="SmallR"):
    data = [[p(str(c), font) for c in row] for row in rows]
    t = Table(data, colWidths=widths, repeatRows=1, hAlign="LEFT")
    t.setStyle(TableStyle([
        ("BACKGROUND", (0,0), (-1,0), PALE),
        ("LINEBELOW", (0,0), (-1,0), .8, TEAL),
        ("VALIGN", (0,0), (-1,-1), "TOP"),
        ("LEFTPADDING", (0,0), (-1,-1), 8),
        ("RIGHTPADDING", (0,0), (-1,-1), 8),
        ("TOPPADDING", (0,0), (-1,-1), 7),
        ("BOTTOMPADDING", (0,0), (-1,-1), 7),
        ("LINEBELOW", (0,1), (-1,-1), .3, RULE),
    ]))
    return t

def footnote(text):
    return p(text, "CaptionR")

def page(title, subtitle=None):
    out=[p(title, "H1R")]
    if subtitle: out.append(p(subtitle, "CaptionR"))
    return out

def diagram():
    d = Drawing(504, 284)
    def node(x,y,w,hh,title,lines,fill=PALE):
        d.add(Rect(x,y,w,hh,rx=5,ry=5,fillColor=fill,strokeColor=RULE,strokeWidth=.7))
        d.add(String(x+12,y+hh-18,title,fontName="ReportSansBold",fontSize=10.2,fillColor=NAVY))
        for i,s in enumerate(lines):
            d.add(String(x+12,y+hh-34-13*i,s,fontName="ReportSans",fontSize=8.5,fillColor=INK))
    def arrow(x1,y1,x2,y2):
        d.add(Line(x1,y1,x2,y2,strokeColor=TEAL,strokeWidth=1.2))
        if y2 < y1:
            pts=[x2,y2,x2-3.4,y2+6,x2+3.4,y2+6]
        else:
            pts=[x2,y2,x2-6,y2-3.4,x2-6,y2+3.4]
        d.add(Polygon(pts,fillColor=TEAL,strokeColor=TEAL))
    node(0,214,242,65,"Composed membership",["Full fibre: F_k(Y) = D_(2^k)(Y) - c", "Quadratic map applied k times"])
    node(262,214,242,65,"Public target and outer base",["R = (r,s), s nonzero; q = S3(X,Y,r)", "Outer modulus H(X): liftable x values"])
    node(44,116,416,65,"Homogeneous trace / norm state",["(Q,T,U) starts at the three quadratic coefficients", "Four quotient products per level; reduce modulo H"])
    arrow(121,214,121,181)
    arrow(383,214,383,181)
    node(44,40,256,51,"Target-conditioned projection",["G = U_k - c T_k + c^2 Q_k; J = gcd(H,G)"])
    node(320,27,184,64,"Recover exact signed rows",["Roots -> oriented line", "Then verify the group sum"],GREY)
    arrow(172,116,172,91)
    arrow(300,64,320,64)
    d.add(String(44,12,"A short evaluation circuit still needs paid root intersection and extraction.",
                 fontName="ReportSans",fontSize=8.4,fillColor=AMBER))
    return d

diag=diagram()
renderSVG.drawToFile(diag, str(FIG/"dickson-resultant-flow.svg"))
svg_path = FIG/"dickson-resultant-flow.svg"
svg_text = svg_path.read_text().replace("font-family: ReportSansBold;", "font-family: Liberation Sans, Arial, sans-serif; font-weight: bold;")
svg_text = svg_text.replace("font-family: ReportSans;", "font-family: Liberation Sans, Arial, sans-serif;")
svg_text = svg_text.replace("<title>...</title>", "<title>Exact Dickson-S3 quotient pipeline</title>")
svg_path.write_text(svg_text)
with tempfile.TemporaryDirectory(prefix="prime31-report-render-") as td:
    t=Path(td)
    renderPDF.drawToFile(diag,str(t/"diagram.pdf"))
    subprocess.run(["pdftoppm","-singlefile","-r","110","-png",str(t/"diagram.pdf"),str(t/"diagram")],check=True,capture_output=True)
    (FIG/"dickson-resultant-flow.png").write_bytes((t/"diagram.png").read_bytes())

raw_path=ROOT/"evidence/correctness-cases-final.stdout"
raw=json.loads(raw_path.read_text())
cell=next(c for c in raw["cases"] if c["depth"]==6 and c["kind"]=="positive distinct pair")
chart_data={
    "source":"evidence/correctness-cases-final.stdout",
    "source_sha256":hashlib.sha256(raw_path.read_bytes()).hexdigest(),
    "cell":"depth 6; full fibre N=64; liftable base B=31; positive distinct pair",
    "unit":"counted base-field multiplications in the algebraic kernel",
    "standing":"[pcc:unverified]",
    "uncertainty":"deterministic operation counts; no statistical interval or wall time",
    "values":[
        {"arm":"Sparse full-fibre product","value":cell["sparse_product"]["field_multiplications"]},
        {"arm":"Dickson norm recurrence","value":cell["recurrence"]["field_multiplications"]},
    ],
}
(FIG/"kernel-counts.json").write_text(json.dumps(chart_data,indent=2)+"\n")
plt.rcParams.update({"font.family":"sans-serif","font.sans-serif":["Liberation Sans","DejaVu Sans"],
                     "font.size":10,"svg.fonttype":"none","pdf.use14corefonts":True})
fig,ax=plt.subplots(figsize=(7.0,2.18))
labels=[v["arm"] for v in chart_data["values"]]
vals=[v["value"] for v in chart_data["values"]]
ax.barh([1,0],vals,color=["#087C80","#B76C26"],height=.48)
ax.set_yticks([1,0],labels)
ax.set_xlim(0,21000)
ax.set_xticks([0,5000,10000,15000,20000])
ax.xaxis.set_major_formatter(FuncFormatter(lambda v,pos:format(int(v),",")))
ax.set_xlabel("Counted field multiplications (lower is better)",fontsize=9)
ax.grid(axis="x",color="#E2E9ED",linewidth=.7)
ax.set_axisbelow(True)
for yi,val in zip([1,0],vals):ax.text(val+350,yi,format(val,","),va="center",fontsize=10,fontweight="bold",color="#243746")
for s in ["top","right","left"]:ax.spines[s].set_visible(False)
ax.spines["bottom"].set_color("#CCD8DE")
ax.tick_params(axis="y",length=0,pad=9)
ax.tick_params(axis="x",length=3,color="#CCD8DE")
fig.subplots_adjust(left=.34,right=.97,bottom=.28,top=.93)
fig.savefig(FIG/"kernel-counts.svg",facecolor="white")
fig.savefig(FIG/"kernel-counts.png",dpi=150,facecolor="white")
plt.close(fig)

def count_chart():
    """Vector drawing of the two recorded counts, without recomputation."""
    d = Drawing(504, 146)
    x0, plot_width, maximum = 172, 322, 21000
    for tick in (0, 5000, 10000, 15000, 20000):
        x = x0 + plot_width*tick/maximum
        d.add(Line(x,35,x,132,strokeColor=RULE,strokeWidth=.5))
        d.add(String(x,21,format(tick,","),textAnchor="middle",fontName="ReportSans",fontSize=8.2,fillColor=MUTED))
    for y, value, label, fill in zip((112,66), vals, labels, (TEAL,colors.HexColor("#B76C26"))):
        w=plot_width*value/maximum
        d.add(Rect(x0,y-13,w,26,fillColor=fill,strokeColor=None))
        d.add(String(x0-12,y-3,label,textAnchor="end",fontName="ReportSans",fontSize=9,fillColor=INK))
        d.add(String(x0+w+6,y-3,format(value,","),fontName="ReportSansBold",fontSize=9.4,fillColor=INK))
    d.add(String(x0+plot_width/2,3,"Counted field multiplications (lower is better)",textAnchor="middle",fontName="ReportSans",fontSize=8.6,fillColor=INK))
    return d

story=[]
# 1: Decision and scope
story += [p("ISOGENY LABS  /  RESEARCH CHECKPOINT", "LabelR"),
          p("Prime31 summation<br/>polynomial compression", "TitleR"),
          p("Dickson membership, exact S3 resultants, and the next proof obligation", "BodyR"),
          footnote("November 22, 2026 (America/Los_Angeles)  |  Research lane: PR #4928"),
          Spacer(1,10),
          box("OUTCOME AT THIS CHECKPOINT", "A denominator-free resultant component is implemented in Rust and passes the original-lock hosted tests. On the recorded B=31 cell, it uses <b>17,923</b> field multiplications against <b>8,816</b> for the sparse-product control. The component has exact algebraic structure; the stronger control wins this diagnostic. [pcc:unverified]"),
          Spacer(1,15),
          table([
              ["Delivered", "Evidence and standing"],
              ["Composed membership evaluation", "Four quotient products per Dickson level; exact exceptional-point coverage. Mathematical derivation and unsigned native identity checks."],
              ["Reusable Rust component", "Five new tests and 41 native library tests pass with the original manifest and lockfile in hosted CI. Its diagnostic JSON matches the earlier local adapter byte for byte. [E4]"],
              ["Independent session review", "A distinct agent reviewed the algebra and replayed the five tests. This is not a signed PCC verdict or a formal proof-kernel result."],
              ["Successor degree theorem", "The full scalar-Dickson image has exact bidegree (2N,2N) under the stated hypotheses. It blocks a fixed-degree replacement, while leaving succinct representations open."],
          ],[168,336]),
          h("Research objective"),
          p("Reduce the complete cost of finding useful prime-field index-calculus relations, then recover an unknown scalar on the source curve. The principal fixed benchmark is p=2<super>31</super>-1. A future asymptotic claim must quantify a growing field family and charge setup, misses, extraction, rank, and recovery. [R1, P1]"),
          footnote("All mathematical and computational claims in this report have unsigned standing [pcc:unverified]. The diagram and chart explain recorded work; they do not promote a canonical performance row."),
          PageBreak()]
# 2: Inputs and architecture
story += page("1  Inputs and architecture", "The same ordinary cofactor-one source curve and maintained arithmetic are retained.")
story += [table([
    ["Parameter", "Inherited source value"],
    ["Prime field", "p = 2147483647"],
    ["Elliptic curve", "y<super>2</super> = x<super>3</super> + 847514846 x + 2087444724"],
    ["Full group order", "n = 2147446831 (prime); cofactor one"],
    ["Generator", "G = (625037715, 2100627251)"],
],[140,364]),
footnote("[R2] These curve properties are inherited from the native source and its checks; this iteration does not supply a new point-counting certificate."),
diag,
footnote("Figure 1. The inner full fibre remains a quadratic composition. Reduction modulo the outer base bounds the state dimension but can make its coefficients dense. Discovery takes public coordinates; signed points are reconstructed after projection. [R1, R3]"),
h("The prime-specific fibre"),
p("The norm-one torus over the quadratic extension has order p+1=2<super>31</super>. Its trace fibres support F<sub>k</sub>(Z)=D<sub>2<super>k</super></sub>(Z)-c, using D<sub>1</sub>(Z)=Z and D<sub>2n</sub>(Z)=D<sub>n</sub>(Z)<super>2</super>-2. The abstract primitive-torus recipe is split and squarefree for the stated range 1 &le; k &le; 29; c=+2 or -2 must be rejected. The existing Base::build has a smaller bounded implementation range. [R1]"),
p("Write N=2<super>k</super> for the full coordinate fibre and B for the actual liftable subset. B is observed, not assumed to be exactly N/2. This uses torus structure, not a proper subfield of the prime field."),PageBreak()]
# 3: Algebra
story += page("2  Exact denominator-free recurrence", "The polynomial identity retains tangent and degree-drop cases.")
story += [p("Let R=(r,s) be the public target, with s nonzero, and write f(X)=X<super>3</super>+aX+b. Regard S3 as a quadratic in Y: [R1, R3]"),
eq("q<sub>X</sub>(Y) = d(X)Y<super>2</super> - t(X)Y + u(X)"),
eq("d=(X-r)<super>2</super>; &nbsp; t=2[(X+r)(Xr+a)+2b]"),
eq("u=(Xr-a)<super>2</super>-4b(X+r)"),
p("Initialize (Q<sub>0</sub>,T<sub>0</sub>,U<sub>0</sub>)=(d,t,u). Each level applies:"),
box("HOMOGENEOUS TRACE AND NORM", "Q' = Q<super>2</super><br/>T' = T<super>2</super> - 2UQ - 4Q<super>2</super><br/>U' = U<super>2</super> - 2T<super>2</super> + 4UQ + 4Q<super>2</super>", body_style="DisplayBoxR"),
Spacer(1,8),
eq("G<sub>k</sub> = U<sub>k</sub> - cT<sub>k</sub> + c<super>2</super>Q<sub>k</sub> = Res<sub>Y</sub>(F<sub>k</sub>,q<sub>X</sub>)"),
eq("J(X) = gcd(H(X), G<sub>k</sub>(X) mod H(X))"),
h("Why the recurrence is correct"),
p("Away from d=0, let z<sub>1</sub>,z<sub>2</sub> be the quadratic roots, counted with multiplicity. For v<sub>i</sub>=D<sub>2<super>j</super></sub>(z<sub>i</sub>), write τ=v<sub>1</sub>+v<sub>2</sub> and ν=v<sub>1</sub>v<sub>2</sub>. The map v to v<super>2</super>-2 gives:"),
eq("τ' = τ<super>2</super>-2ν-4; &nbsp; ν' = ν<super>2</super>-2τ<super>2</super>+4ν+4"),
p("With e<sub>j</sub>=2<super>j</super>, take Q<sub>j</sub>=d<super>e<sub>j</sub></super>, T<sub>j</sub>=Q<sub>j</sub>τ and U<sub>j</sub>=Q<sub>j</sub>ν. Clearing denominators gives the recurrence. Polynomial identity extends it to d=0. [R1]"),
h("Exceptional point retained"),
p("At X=r, q becomes the linear polynomial -4s<super>2</super>(Y-x(2R)). The monic-first resultant convention therefore retains the tangent contribution exactly. No division by X-r occurs. The recurrence also makes sense in quotient rings with zero divisors. Native tests cover linear, constant and zero q, as well as a nonreduced outer ring. [E1, E2]"),
footnote("Identity records: homogeneous trace IDC1hd3786f53034b6b9d; homogeneous norm IDC1h3366a93945da9204. Evaluated native certificates have proof:null and do not certify the function-field theorem. [E3]"),PageBreak()]
# 4: Lift and cost
story += page("3  Lifting, signs and complete cost", "The algebraic projection must return usable, normalized source-curve relations.")
story += [h("Full-fibre membership is exact on the admitted target domain"),
eq("t(X)<super>2</super>-4d(X)u(X) = 16 f(X)s<super>2</super>"),
p("For a rational S3 solution and s nonzero, this discriminant forces rational lifting away from X=r; the diagonal is handled separately. Therefore the full rational fibre may remain the inner membership predicate even when only its liftable subset forms the point base. The implementation compares full-fibre and filtered outer support on every retained case. [R1, E1]"),
h("Recover the signs without new square roots"),
eq("λ = [(x-r)(z-r)-(3r<super>2</super>+a)] / (2s)"),
eq("P=(x,λ(x-r)-s), &nbsp; Q=(z,λ(z-r)-s)"),
p("The identity f(X)-(λ(X-r)-s)<super>2</super>=(X-r)(X-x)(X-z) puts P and Q on the curve with P+Q=R, including multiplicities. One inverse of 2s serves the target. Source-curve replay and factor-base sign/index normalization remain mandatory. The affine API excludes s=0 and the identity; inverse pairs handle the identity separately. [R1, E2]"),
h("Cost that the circuit count does not remove"),
table([
    ["Method", "Algebraic kernel model"],
    ["Expanded inner Horner", "O(N B<super>2</super>) with naive dense products"],
    ["Degree-sensitive recurrence", "O(B<super>2</super>[1+log<super>+</super>(N/B)]) with trimmed schoolbook arithmetic"],
    ["Sparse quadratic product", "O(NB); each known factor has degree at most two"],
    ["Fast product-tree reference", "Approximately O(M(N) log N) when B and N are comparable; implementation comparison pending"],
    ["Group-complement scan", "O(B) group operations plus charged membership work; required relation-search control"],
],[174,330]),
footnote("[R1] Setup, inversion/reduction preparation, gcd, root selection, sign recovery, normalization and validation are additional. Under standard fast multiplication and prepared reduction, the recurrence kernel can be O(M(B)[1+log<super>+</super>(N/B)]). These are derived models, not measured speed ratios."),
p("Before saturation, state degrees grow geometrically. If N/B is bounded, only a constant number of layers saturate and the schoolbook recurrence is O(B<super>2</super>), like the sparse product. Logarithmic layer count alone does not give an exponent advantage."),PageBreak()]
# 5: data
story += page("4  The stronger control wins", "Deterministic operation diagnostic: one regular depth-six relation query, N=64 and B=31.")
story += [count_chart(),
footnote("Figure 2. Recorded algebraic-kernel field multiplications for the positive distinct-pair cell. Counts are deterministic; no confidence interval or comparative wall time exists. The recurrence uses about 2.03 times the sparse-product count. [E1]"),
table([
    ["Instrumented kernel", "Quotient calls", "Field multiplications"],
    ["Dickson recurrence", "24", "17,923"],
    ["Sparse full-fibre product", "64", "8,816"],
    ["Filtered-base Horner comparison", "81", "126,511"],
    ["Full-fibre Horner coefficient control", "145", "254,913"],
],[266,100,138]),
Spacer(1,8),
box("INTERPRETATION", "The reduction from 64 quotient calls to 24 is not an equal-work comparison. The product control multiplies by degree-at-most-two factors; the recurrence eventually multiplies dense polynomials. A gain against expanded Horner therefore does not establish a gain against the strongest elementary algebraic control. [pcc:unverified]",GREY),
h("Recorded density"),
p("All three final recurrence polynomials contain 31 nonzero coefficients, for 93 retained coefficient slots. The largest single polynomial allocation is 61 slots in the recurrence and 33 in sparse product. Those are allocation statistics, not peak live memory. The observed density rejects bounded sparsity on these fixtures; it does not rule out other succinct circuits. [E1]"),
h("Accounting qualification"),
p("Kernel counters exclude factor-base setup, extraction and group verification. Both Horner controls include denominator-clearing rescaling for the coefficient comparison; the earlier normalized-GCD oracle can omit this unit. The table consequently does not give complete oracle costs or a qualified speed ratio against that incumbent. [E1, E2]"),
    footnote("Source: evidence/correctness-cases-final.stdout, the final retained Rust diagnostic JSON. Canonical benchmark plots remain unchanged."),PageBreak()]
# 6: theorem
story += page("5  Exact degree obstruction", "A second mathematical iteration narrows the surviving representation choices.")
story += [box("THEOREM - UNSIGNED MATHEMATICAL PROOF", "Let K be algebraically closed of characteristic p&gt;3, E/K a smooth elliptic curve, and R a nonzero point of finite odd order. Put N=2<super>k</super>, f(P)=D<sub>N</sub>(x(P)) and g(P)=D<sub>N</sub>(x(R-P)). Then K(f,g)=K(E). The irreducible affine relation F(U,V)=0 has exact degree 2N in each variable. [pcc:unverified]"),
Spacer(1,8),
p("This statement does not require ordinariness or rational fibre splitting. Its proof has been reviewed independently within the session; it has not been checked by Lean or signed PCC. The complete argument is in NEXT_PROOF.md. [T1]"),
h("Proof in four steps"),
table([
    ["Step", "Argument"],
    ["1. Pole divisors", "The unique poles are (f)<sub>infinity</sub>=2N[O] and (g)<sub>infinity</sub>=2N[R]. Both maps have degree 2N, prime to p, and are separable."],
    ["2. Common subcover", "For L0=K(f,g), let e=[K(E):L0]. Then e divides 2N, hence is a power of two. The corresponding separable cover has singleton fibres e[O] and e[R] over distinct points."],
    ["3. Rule out e&gt;1", "Riemann-Hurwitz with the effective different gives genus at most one. A genus-one quotient is unramified, contradicting total ramification. A genus-zero quotient makes e[O]-e[R] principal, forcing [e]R=O, impossible for nonzero odd-order R."],
    ["4. Read the bidegree", "Thus e=1. Each projection of the normalized image to the projective line has degree 2N, giving deg<sub>U</sub>F=deg<sub>V</sub>F=2N."],
],[118,386]),
footnote("[T1; G1-G3] A prime-to-p degree does not justify assuming every ramification index of a non-Galois cover is tame. The argument uses separable Riemann-Hurwitz and its different instead."),
h("What this excludes, and what remains open"),
p("A scalar replacement that keeps one full-image equation at constant bidegree cannot work along growing k. The theorem does not bound arithmetic-circuit size, monomial sparsity, finite-fibre predicates, vector auxiliary states, or common-root extraction. It is not a lower bound for ECDLP. The successor must specify which surviving representation it exploits and price its extraction algorithm."),PageBreak()]
# 7: evidence
story += page("6  Evidence and reproducibility", "The original manifest and lockfile now pass hosted replay; the CI record is producer evidence.")
story += [table([
    ["Evidence", "Retained outcome"],
    ["New example tests", "5 passed in original-lock CI and the local adapter; 25 source-curve cases plus synthetic exceptions"],
    ["Existing library tests", "41 passed in original-lock CI, matching the prior adapter pass on the same native sources"],
    ["Correctness instrument", "12 cases; hosted JSON is byte-identical to the local record, including all deterministic counters"],
    ["Independent session replay", "Distinct algebra reviewer reran the five tests and reviewed source; no correctness defect found"],
    ["Identity records", "Four native identity.certificate/v1 records, each proof:null; 32 evaluations over 2<super>61</super>-1 and additional replay/checked-integer expansion"],
],[168,336]),
h("Successful original-lock hosted replay"),
p("<link href='https://github.com/isogeny-labs/crypto/actions/runs/45089569480' color='#087C80'>Run 45089569480</link>, job 137415823110, succeeded at checkout 109fc024e33f122fd678d2c93a18b2c0438a8668. The original native manifest and lockfile were used. Retained logs record 41 library and 5 component tests passing. [E4]","SmallR"),
p("Hosted and local diagnostic SHA-256:<br/>2f491f3250b71bca1597668098f0feecd9a40d9c76714986b482b2c8b8fc6efb","MonoR"),
p("This resolves the original-lock replay question. It supplies CI producer evidence, with no independent signed execution verdict, admitted timing benchmark, or repository-wide test claim. [E4]","SmallR"),
h("Earlier local build qualification retained"),
p("The first offline locked build failed because required versions were absent from the local cache. Its failure receipt and the separate adapter's eight package-version deviations remain in the evidence. The original library, manifest, lockfile and component source were preserved. The later hosted success is recorded separately. [E2, E4]","SmallR"),
h("Native reproduction entry points"),
p("From the repository root, these are the original-lock CI test entry points:","SmallR"),
p("cargo test --locked --release \\\n  --manifest-path research/prime31_ic_20261115/native/Cargo.toml \\\n  --example prime31_dickson_quotient".replace("\n","<br/>"),"MonoR"),
p("cargo test --locked --release \\\n  --manifest-path research/prime31_ic_20261115/native/Cargo.toml --lib".replace("\n","<br/>"),"MonoR"),
    p("PCC records: recurrence PCC-20261123-5f10b5; development diagnostic PCC-20261123-acf74d; degree theorem PCC-20261123-f61d26; hosted replay PCC-20261123-526846. Canonical standing is <b>unverified</b> for all four.","SmallR"),
    p("Final example SHA-256:<br/>b76cd5a1efeff77bc065554d89acc9b53be051e6b17811cca589c2dc93a018b1","MonoR"),
footnote("Identity ids [E3]: centered S3 IDC1hf62e2402e4871a82; discriminant IDC1h869923a46a34c70c; trace IDC1hd3786f53034b6b9d; norm IDC1h3366a93945da9204. These do not certify the function-field proof. Lean, lake and elan were unavailable."),PageBreak()]
# 8: next and citations
story += page("7  Next research gate and sources", "Continue from the exact kernel and the loss against sparse product.")
story += [box("NEXT TECHNICAL TARGET", "Construct and analyze a common-root algorithm that acts on the succinct fibre and resultant circuits without enumerating the full fibre or materializing all degree-B residues. A vector state, sparse high-degree circuit or finite-set-specific predicate must state its exact output contract and charge witness extraction. The degree theorem rules out only the fixed-bidegree scalar route."),
Spacer(1,10),
p("The prospective component protocol freezes depths 2-10, natural targets and separate planted controls; compares filtered Horner, both recurrence domains, sparse product, batch group scanning and a fast-polynomial reference; and charges cold setup, misses, extraction and normalization. It remains queued behind the repository's execution and checkpoint gates. [P1]"),
p("Higher-arity integration must supply actual unknown-index relations, useful independent rank, and a checked cold scalar recovery. A growing-field argument must not assume infinitely many Mersenne primes. The final matched controls remain the applicable index-calculus frontier and optimized negation-map Pollard rho. [R1, P1]"),
h("Local evidence and proof sources"),
p("<b>R1</b> REPORT.md; <b>R2</b> native/lib.rs at snapshot 26cc445bdc6b0b3518535d5f7b8b4ec6cc08fb19; <b>R3</b> native/examples/prime31_dickson_quotient.rs. <b>E1</b> evidence/correctness-cases-final.stdout. <b>E2</b> evidence/independent_review.md, source-manifest.json, dependency-differences.json, and local test receipts. <b>E3</b> identity-certificates/*.json. <b>E4</b> evidence/original-lock-ci/: receipt.json, source-sha256.txt, library-tests.txt, component-tests.txt, correctness-cases.json and job.log. <b>T1</b> NEXT_PROOF.md. <b>P1</b> PROTOCOL.md.","SmallR"),
h("Primary literature and mathematical inputs"),
p("<b>L1.</b> Semaev (2015), linked S3 chain: <link href='https://arxiv.org/pdf/1504.01175' color='#087C80'>arXiv:1504.01175</link>. <b>L2.</b> Petit, Kosters, Messeng (2016), composed factor-base maps and prime-field root finding: <link href='https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf' color='#087C80'>PKC paper</link>. <b>L3.</b> Amadori, Pintore, Sala (2018), one-dependency architecture: <link href='https://pure.tue.nl/ws/files/91158650/609.pdf' color='#087C80'>author manuscript</link>. <b>L4.</b> Kosters and Yeo (2015), first-fall-degree limitations: <link href='https://arxiv.org/pdf/1503.08001' color='#087C80'>arXiv:1503.08001</link>.","SmallR"),
p("<b>G1.</b> Stacks Project, separable Riemann-Hurwitz with different: <link href='https://stacks.math.columbia.edu/tag/0C1B' color='#087C80'>Tag 0C1B</link>. <b>G2.</b> Stein, elliptic curves and Pic<super>0</super>: <link href='https://www.wstein.org/books/ant/ant/node65.html' color='#087C80'>Abelian Groups Attached to Elliptic Curves</link>. <b>G3.</b> Lynn, Weierstrass pole divisor: <link href='https://crypto.stanford.edu/pbc/notes/elliptic/divisor.html' color='#087C80'>Zeroes and Poles</link>. The degree theorem is an application of these standard inputs, not attributed to them as a previously stated result.","SmallR"),
h("Prior-work and overlap boundary"),
p("Discriminant factorization, signed coordinates, quotient/GCD elimination, Dickson fibres and Brent-Kung acceleration already exist in the repositories. The linked S3 chain is prior art. This bounded audit supports no global novelty claim. PR #4901 and the extraction work in #4692/#4812/#4822 remain separate dependencies. PR #4829 maintains summation-root machinery. [R1]","SmallR"),
footnote("Research record: <link href='https://github.com/isogeny-labs/crypto/pull/4928' color='#087C80'>isogeny-labs/crypto PR #4928</link>. Hosted original-lock correctness replay succeeded. No admitted timing benchmark, independent signed execution verdict, signed PCC standing or asymptotic speedup is asserted.")]

def on_page(canvas,doc):
    canvas.saveState()
    canvas.setStrokeColor(RULE);canvas.setLineWidth(.6)
    canvas.line(54,750,558,750)
    canvas.setFont("ReportSansBold",8);canvas.setFillColor(TEAL)
    canvas.drawString(54,760,"ISOGENY LABS  /  PRIME-FIELD RELATION RESEARCH")
    canvas.setFont("ReportSans",7.6);canvas.setFillColor(MUTED)
    canvas.drawRightString(558,760,"PR #4928  |  2026-11-22 PT")
    canvas.line(54,42,558,42)
    canvas.setFont("ReportSans",7.4)
    canvas.drawString(54,29,"Unsigned research checkpoint  [pcc:unverified]")
    canvas.drawRightString(558,29,f"{doc.page}  /  8")
    canvas.restoreState()

doc=SimpleDocTemplate(str(ROOT/"REPORT.pdf"),pagesize=(612,792),
    rightMargin=54,leftMargin=54,topMargin=58,bottomMargin=58,
    title="Prime31 summation polynomial compression - research checkpoint",
    author="Isogeny Labs",subject="Unsigned Dickson/S3 resultant development checkpoint",
    pageCompression=1)
doc.build(story,onFirstPage=on_page,onLaterPages=on_page)
# Lossless stream/object compression keeps the embedded-font report modest.
with fitz.open(ROOT/"REPORT.pdf") as pdf:
    compact = pdf.tobytes(garbage=4, clean=True, deflate=True,
                          deflate_fonts=True, use_objstms=1)
(ROOT/"REPORT.pdf").write_bytes(compact)
pages=len(PdfReader(ROOT/"REPORT.pdf").pages)
if pages != 8: raise RuntimeError(f"Expected 8 pages; got {pages}")
print(json.dumps({"pdf":str(ROOT/"REPORT.pdf"),"pages":pages,
    "bytes":(ROOT/"REPORT.pdf").stat().st_size,
    "source":"REPORT.md, NEXT_PROOF.md, recorded Rust diagnostic, and retained original-lock CI evidence",
    "figures":[str(p.relative_to(ROOT)) for p in sorted(FIG.iterdir())]},indent=2))
