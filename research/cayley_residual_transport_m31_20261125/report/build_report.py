#!/usr/bin/env python3
"""Lay out retained research evidence; no experiment or scientific computation.

Usage in the published study: python3 report/build_report.py
Usage elsewhere: python3 build_report.py --study-root /path/to/frozen/study
"""
from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from xml.sax.saxutils import escape

from reportlab.lib import colors
from reportlab.lib.enums import TA_LEFT
from reportlab.lib.styles import ParagraphStyle
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.pdfgen import canvas
from reportlab.platypus import Paragraph
from reportlab.graphics.shapes import Drawing, Rect, Line, Polygon, String
from reportlab.graphics import renderPDF, renderSVG


REPO = "isogeny-labs/crypto"
BRANCH = "codex/cayley-residual-transport-m31-20261126"
STUDY = "research/cayley_residual_transport_m31_20261125"
UPSTREAM = "eb55e93110676c91213d91d6c8d00489311a4bd8"
ROOT_URL = f"https://github.com/{REPO}/blob/{BRANCH}/{STUDY}"
SOURCE_URLS = {
    1: f"{ROOT_URL}/MATHEMATICS.md",
    2: f"{ROOT_URL}/evidence/receipt.json",
    3: f"{ROOT_URL}/PROTOCOL.md",
    4: f"https://github.com/{REPO}/blob/{UPSTREAM}/research/prime31_ic_20261115/native/lib.rs",
    5: f"https://github.com/{REPO}/pull/5232",
    6: f"{ROOT_URL}/README.md",
}

INK = colors.HexColor("#14283D")
MUTED = colors.HexColor("#526579")
BLUE = colors.HexColor("#176B90")
TEAL = colors.HexColor("#0A766E")
PALE = colors.HexColor("#EDF5F7")
PAPER = colors.HexColor("#F7F9FB")
RULE = colors.HexColor("#CEDBE3")
ORANGE = colors.HexColor("#AF6427")
W, H = 612.0, 792.0
LEFT, RIGHT = 42.0, 570.0
CW = RIGHT - LEFT


def register_fonts(folder: Path) -> None:
    for name, filename in (
        ("ReportSans", "DejaVuSans.ttf"),
        ("ReportBold", "DejaVuSans-Bold.ttf"),
        ("ReportItalic", "DejaVuSans-Oblique.ttf"),
        ("ReportMono", "DejaVuSansMono.ttf"),
    ):
        path = folder / filename
        if not path.exists():
            raise FileNotFoundError(f"Required font missing: {path}; pass --font-dir")
        pdfmetrics.registerFont(TTFont(name, str(path)))
    pdfmetrics.registerFontFamily(
        "ReportSans", normal="ReportSans", bold="ReportBold", italic="ReportItalic"
    )


def style(size=9.5, leading=13.0, color=INK, bold=False):
    return ParagraphStyle(
        "body", fontName="ReportBold" if bold else "ReportSans",
        fontSize=size, leading=leading, textColor=color, alignment=TA_LEFT,
        spaceBefore=0, spaceAfter=0, allowWidows=0, allowOrphans=0,
    )


def par(c, text, top, x=LEFT, width=CW, size=9.5, leading=13.0, color=INK, bold=False):
    p = Paragraph(text, style(size, leading, color, bold))
    _, height = p.wrap(width, H)
    p.drawOn(c, x, H - top - height)
    return top + height


def textline(c, text, top, x=LEFT, font="ReportSans", size=10, color=INK):
    c.setFont(font, size)
    c.setFillColor(color)
    c.drawString(x, H - top - size * 0.78, text)


def section(c, title, top):
    c.setFillColor(TEAL)
    c.rect(LEFT, H-top-12, 3, 12, fill=1, stroke=0)
    textline(c, title, top, x=LEFT+10, font="ReportBold", size=11.3)
    return top + 23


def box(c, top, height, fill=PALE, x=LEFT, width=CW):
    c.setFillColor(fill)
    c.setStrokeColor(RULE)
    c.roundRect(x, H-top-height, width, height, 7, fill=1, stroke=1)


def linked(n, title):
    return f'<link href="{SOURCE_URLS[n]}" color="#176B90">[{n}] {escape(title)}</link>'


def header(c, number, title, subtitle):
    textline(c, "ISOGENY LABS / RESEARCH COMPONENT", 30, font="ReportBold", size=8.7, color=TEAL)
    c.setFont("ReportSans", 8.1)
    c.setFillColor(MUTED)
    c.drawRightString(RIGHT, H-36.5, "DERIVED / PCC-UNVERIFIED")
    textline(c, title, 54, font="ReportBold", size=21.2)
    par(c, subtitle, 85, size=9.0, leading=12.4, color=MUTED)
    c.setStrokeColor(RULE)
    c.line(LEFT, 46, RIGHT, 46)
    textline(c, "Publication: 2026-11-26 | Frozen evidence: 2026-11-25", 756, size=7.5, color=MUTED)
    c.setFont("ReportBold", 8)
    c.drawRightString(RIGHT, 29.8, f"{number} / 3")


def check_bottom(y, page):
    if y > 736:
        raise RuntimeError(f"Page {page} content reaches {y:.1f}; exceeds safe content area")


def table(c, top, headers, rows, widths, *, x=LEFT, font_size=8.6, leading=11.2, pad=6):
    all_rows = [headers] + rows
    y = top
    for idx, row in enumerate(all_rows):
        paragraphs = [
            Paragraph(str(cell), style(font_size, leading, colors.white if idx == 0 else INK, idx == 0))
            for cell in row
        ]
        heights = [p.wrap(width-pad*2, H)[1] for p, width in zip(paragraphs, widths)]
        height = max(heights) + pad*2
        c.setFillColor(TEAL if idx == 0 else (PAPER if idx % 2 else colors.white))
        c.rect(x, H-y-height, sum(widths), height, fill=1, stroke=0)
        xx = x
        for p, width, ph in zip(paragraphs, widths, heights):
            p.drawOn(c, xx+pad, H-y-pad-ph)
            xx += width
        c.setStrokeColor(RULE)
        c.setLineWidth(0.35)
        c.line(x, H-y-height, x+sum(widths), H-y-height)
        y += height
    return y


def arrow(d, points, color=BLUE, width=2):
    for a, b in zip(points, points[1:]):
        d.add(Line(*a, *b, strokeColor=color, strokeWidth=width))
    (x0,y0),(x,y) = points[-2:]
    if x == x0:
        sign = 1 if y > y0 else -1
        tri = [x,y, x-6,y-sign*10, x+6,y-sign*10]
    else:
        sign = 1 if x > x0 else -1
        tri = [x,y, x-sign*10,y-6, x-sign*10,y+6]
    d.add(Polygon(tri, fillColor=color, strokeColor=color))


def diagram() -> Drawing:
    d = Drawing(900, 390)
    d.add(Rect(0,0,900,390,fillColor=colors.white,strokeColor=None))

    def node(x,y,w,h,title,lines,fill=PALE):
        d.add(Rect(x,y,w,h,rx=10,ry=10,fillColor=fill,strokeColor=RULE,strokeWidth=1.3))
        d.add(String(x+16,y+h-27,title,fontName="ReportBold",fontSize=18,fillColor=INK))
        for i, line in enumerate(lines):
            d.add(String(x+16,y+h-53-i*23,line,fontName="ReportSans",fontSize=16,fillColor=MUTED))

    node(12,304,355,75,"Public inputs",["E, R, P1, P2, certified supports"])
    node(533,304,355,75,"Supplied sets A1 and A2",["Label discovery is outside this kernel"])
    node(12,174,355,94,"Fix the prefix",["A = P1 + P2; K = R - A", "Compute factors η1, η2 and map T"])
    node(533,174,355,94,"Apply both constraints",["z1 ∈ A1 and T(z1) ∈ A2", "One residual slope, two allowed sets"])
    node(249,32,401,97,"Recover signed residual points",["z1 → m → quadratic → (P3, P4)", "Signed sum checked; base test is downstream"],PAPER)
    arrow(d,[(189,304),(189,268)])
    arrow(d,[(710,304),(710,268)])
    arrow(d,[(367,221),(533,221)])
    d.add(String(443,234,"T",fontName="ReportBold",fontSize=18,fillColor=BLUE))
    arrow(d,[(189,174),(189,82),(249,82)])
    d.add(String(80,133,"K, η1",fontName="ReportSans",fontSize=16,fillColor=BLUE))
    arrow(d,[(710,174),(710,82),(650,82)])
    d.add(String(725,133,"compatible z1",fontName="ReportSans",fontSize=15,fillColor=BLUE))
    d.add(String(450,9,"Derived / PCC-unverified | cayley-residual-transport-m31-v1 | Supplied inputs, no discovery",
                 textAnchor="middle",fontName="ReportSans",fontSize=14.5,fillColor=MUTED))
    return d


def outcomes_chart(result) -> Drawing:
    """Scale only the two recorded empty-fiber counts; do not infer probabilities."""
    d = Drawing(340,185)
    d.add(Rect(0,0,340,185,fillColor=colors.white,strokeColor=None))
    d.add(String(170,169,"Recorded empty fibers",textAnchor="middle",fontName="ReportBold",fontSize=16,fillColor=INK))
    ymax = max(result["matched_empty_fibers_random"], result["matched_empty_fibers_controlled"])
    base, extent = 62, 70
    d.add(Line(30,base,315,base,strokeColor=RULE,strokeWidth=1))
    for x, count, title, n, col in (
        (94,result["matched_empty_fibers_random"],"Random",result["random_slope_cases"],BLUE),
        (244,result["matched_empty_fibers_controlled"],"Controlled",result["controlled_slope_cases"],ORANGE),
    ):
        ht = extent*count/ymax
        d.add(Rect(x-28,base,56,ht,fillColor=col,strokeColor=None))
        d.add(String(x,base+ht+9,f"{count:,}",textAnchor="middle",fontName="ReportBold",fontSize=16,fillColor=INK))
        d.add(String(x,45,title,textAnchor="middle",fontName="ReportSans",fontSize=14,fillColor=INK))
        d.add(String(x,28,f"n = {n:,} cases",textAnchor="middle",fontName="ReportSans",fontSize=12.8,fillColor=MUTED))
    d.add(String(170,5,"cayley-residual-transport-m31-v1 / recorded counts",textAnchor="middle",fontName="ReportSans",fontSize=10.8,fillColor=MUTED))
    return d


def draw_scaled(c, drawing, x, top, width):
    scale = width/drawing.width
    c.saveState()
    c.translate(x, H-top-drawing.height*scale)
    c.scale(scale,scale)
    renderPDF.draw(drawing,c,0,0)
    c.restoreState()
    return top+drawing.height*scale


def save_svg(drawing, path):
    renderSVG.drawToFile(drawing, str(path))
    # Portable font names while keeping editable vector text.
    s = path.read_text()
    s = s.replace('font-family: ReportBold;', "font-family: 'DejaVu Sans', sans-serif; font-weight: bold;")
    s = s.replace('font-family: ReportSans;', "font-family: 'DejaVu Sans', sans-serif;")
    s = s.replace('font-family="ReportBold"', 'font-family="DejaVu Sans" font-weight="bold"')
    s = s.replace('font-family="ReportSans"', 'font-family="DejaVu Sans"')
    desc = f"Retained study cayley-residual-transport-m31-v1. Derived/PCC-unverified. Sources: {SOURCE_URLS[1]} ; {SOURCE_URLS[2]}. No new scientific execution."
    s = re.sub(r"(<svg[^>]*>)",lambda m:m.group(1)+"\n<desc>"+escape(desc)+"</desc>",s,count=1)
    path.write_text(s)


def page_one(c, flow):
    header(c, 1, "Two labels, one residual slope", "Exact transport of trace-zero chord-function labels over p = 2³¹ - 1.")
    box(c,112,63)
    y = par(c,"<b>Completed component.</b> For a fixed public prefix, an invertible projective map links the two labels. A second allowed set can filter this graph; it does not add another residual degree of freedom once the first label is fixed.",124,x=54,width=CW-24,size=10.0,leading=14)
    y = section(c,"Definitions and exact transport",190)
    y = par(c,"On E: y² = x³ + ax + b, fix K = (w,v), v ≠ 0, and certified nonrational supports S<sub>j</sub> = (s<sub>j</sub>, iδ<sub>j</sub>), with i² = -1 and δ<sub>j</sub>² = -(s<sub>j</sub>³ + as<sub>j</sub> + b). Then h<sub>j</sub> = s<sub>j</sub> - w ≠ 0. The chord through -K gives the normalized label and raw label:",y,size=9.3,leading=12.8)
    y += 9
    box(c,y,118,PAPER)
    textline(c,"C(X:Z) = (X+iZ)/(X-iZ)",y+12,x=56,font="ReportMono",size=10.4)
    textline(c,"t_j(m) = C(v-m h_j : δ_j),    z_j = η_j t_j(m)",y+31,x=56,font="ReportMono",size=10.4)
    textline(c,"N =",y+61,x=56,font="ReportBold",size=11)
    mx, my, mw, mh = 96, H-(y+52), 235, 40
    c.setStrokeColor(INK); c.setLineWidth(1)
    c.lines([(mx+6,my,mx,my),(mx,my,mx,my-mh),(mx,my-mh,mx+6,my-mh),
             (mx+mw-6,my,mx+mw,my),(mx+mw,my,mx+mw,my-mh),(mx+mw,my-mh,mx+mw-6,my-mh)])
    textline(c,"h2 δ1",y+56,x=mx+20,font="ReportMono",size=10.3)
    textline(c,"(h1-h2) v",y+56,x=mx+111,font="ReportMono",size=10.3)
    textline(c,"0",y+76,x=mx+20,font="ReportMono",size=10.3)
    textline(c,"h1 δ2",y+76,x=mx+111,font="ReportMono",size=10.3)
    par(c,"det N = h1 h2 δ1 δ2 ≠ 0",y+64,x=352,width=200,size=9.5,leading=12)
    textline(c,"T = M_η2 N adj(M_η1),   M_η = [[E,-F],[F,E]], η=C(E:F)",y+100,x=56,font="ReportMono",size=8.7)
    y += 130
    y = draw_scaled(c,flow,LEFT,y,CW)
    y = par(c,"<b>Exact count.</b> For fixed K, supports and factors, 2³¹ label pairs are compatible among 2⁶² possible pairs. Independent uniform labels pass with probability 2⁻³¹. Changing the prefix changes T; the statement does not exclude a shared index across prefixes or labels on different unresolved branches. "+linked(1,"Derivation, §§2-5"),y+10,size=9.0,leading=12.3)
    check_bottom(y,1)
    c.showPage()


def page_two(c):
    header(c,2,"Public prefixes to signed points","The four-point construction uses a principal-divisor label, with explicit identity cases.")
    y = section(c,"A function attached to the four-summand relation",113)
    y = par(c,"For P1 + P2 + P3 + P4 = R, set A = P1 + P2 and K = R - A. For ordinary affine inputs, f<sub>U,V</sub> is the chord/tangent function divided by x - x(U+V). Use the vertical function for opposite nonidentity points and the constant 1 when either input is O.",y,size=9.4,leading=13)
    y += 9
    box(c,y,80)
    textline(c,"div(f_U,V) = [U] + [V] - [U+V] - [O]",y+13,x=56,font="ReportMono",size=10.2)
    textline(c,"F = f_P1,P2 · f_P3,P4 · f_A,K",y+35,x=56,font="ReportMono",size=10.2)
    textline(c,"div(F) = [P1]+[P2]+[P3]+[P4]-[R]-3[O]",y+57,x=56,font="ReportMono",size=10.2)
    y += 91
    y = par(c,"Vertical factors have equal evaluations at S<sub>j</sub> and -S<sub>j</sub>. Thus F(S<sub>j</sub>)/F(-S<sub>j</sub>) = η<sub>j</sub>t<sub>j</sub>, where η<sub>j</sub> is the product of the f<sub>P1,P2</sub> and f<sub>A,K</sub> evaluation ratios. <b>Frame::from_prefix</b> computes these factors from E, R, P1, P2 and the supports, with no target scalar or residual witness input. This function label is distinct from the earlier product of x-coordinate Cayley values. "+linked(1,"Divisor proof, §4"),y,size=9.3,leading=12.8)
    y = section(c,"Exact residual extraction and outcome types",y+15)
    y = par(c,"For a finite compatible label, recover m and solve one quadratic. Ordinates follow from the same signed line, without further square roots:",y,size=9.2,leading=12.6)
    y += 7
    box(c,y,46,PAPER)
    textline(c,"X² - (m²-w)X + (w m²+w²+a+2vm) = 0",y+10,x=56,font="ReportMono",size=10.1)
    textline(c,"y = m(x-w)-v",y+28,x=56,font="ReportMono",size=10.1)
    y += 55
    y = table(c,y,["Input / fiber","Returned result","Coordinate sqrt calls"],[
        ["Incompatible labels","None (rejected before extraction)","0"],
        ["Compatible, finite and split","All ordered signed pairs; repeated pair once","1"],
        ["Compatible, finite and nonsplit","Some(empty)","1"],
        ["Vertical chord: raw label η1","(O,K) and (K,O); O is null in records","0"],
    ],[164,260,104],font_size=8.6,leading=11.5,pad=5)
    y = par(c,"K = O and zero-ordinate K are outside this interface. Noncanonical residues, singular curves, off-curve inputs and invalid supports are rejected; certified frame state is immutable. "+linked(6,"API and limits"),y+9,size=8.8,leading=12)
    y = section(c,"Two established ordinary M31 fixtures",y+15)
    y = par(c,"<b>Primary:</b> a = 2147483644; b = 21; r = 2147487637.<br/>G = (1391443431, 177979044). Retained panel: 64 targets and 6,144 case rows.",y,size=8.7,leading=12.1)
    y = par(c,"<b>Secondary:</b> a = 847514846; b = 2087444724; r = 2147446831.<br/>G = (625037715, 2100627251). Retained panel: 64 targets and 6,144 case rows.",y+6,size=8.7,leading=12.1)
    y = par(c,"Field/group primality, nonsingularity, generator order and the Hasse interval were checked in the frozen panel. The models remain separate. "+linked(2,"Receipt")+" · "+linked(4,"Pinned upstream arithmetic"),y+9,size=8.2,leading=11)
    check_bottom(y,2)
    c.showPage()


def page_three(c,receipt,chart):
    r = receipt["result"]
    n = lambda key: f'{r[key]:,}'
    header(c,3,"Frozen evidence and remaining work","Retained native development checks, not a new scientific run or a runtime comparison.")
    y = par(c,"<b>Study:</b> cayley-residual-transport-m31-v1. The panel used 128 public targets, 512 frames and 40 distinct deterministic child streams. Separate direct Fp² arithmetic and the retained symmetric-Semaev reference supplied the comparison. "+linked(2,"Receipt")+" · "+linked(3,"Frozen protocol"),113,size=8.9,leading=12.2)
    y += 10
    tab_bottom = table(c,y,["Input stratum","Cases","Empty","Ordered pairs"],[
        ["Random slopes",n("random_slope_cases"),n("matched_empty_fibers_random"),n("ordered_pair_occurrences_random")],
        ["Controlled slopes",n("controlled_slope_cases"),n("matched_empty_fibers_controlled"),n("ordered_pair_occurrences_controlled")],
        ["All cases",n("paired_transport_extraction_cases"),n("matched_empty_fibers_all_cases"),n("ordered_pair_occurrences_all_cases")],
    ],[105,53,53,81],font_size=8.2,leading=10.5,pad=5)
    chart_bottom = draw_scaled(c,chart,380,y,190)
    y = max(tab_bottom,chart_bottom)+9
    y = par(c,"<b>All gates passed:</b> 12,288 fiber comparisons; 1,024 withheld pair/doubling controls; 12,288 incompatible-label rejections before square roots; 1,536 supplied-set queries; 256 constructed prefix cases; 52 invalid-input rejections; 16 matrix sanity checks.",y,size=8.8,leading=12.0)
    y = par(c,"All prefix cases are constructed positives; supplied sets have designed overlaps. Empty and rejected outcomes remain recorded. Pair occurrences are not independent relations. Unequal deterministic strata support no natural-yield estimate or uncertainty interval.",y+7,size=8.6,leading=11.8)
    y = par(c,"The first build and both runs exited 0 with empty stderr. All five result files replayed byte-for-byte; source/executable manifests were unchanged. This is deterministic replay, not independent statistical replication.",y+7,size=8.6,leading=11.8)
    y = section(c,"Costs that must remain in a successor comparison",y+14)
    y = par(c,"Let a0,b0 be supplied list lengths; a,b their distinct-label counts; m=min(a,b); k the compatible outputs. The implementation uses deterministic BTreeSet membership.",y,size=8.5,leading=11.5)
    y = table(c,y+7,["Stage","Full accounting"],[
        ["Build supplied sets","O(a0 log(a0+1) + b0 log(b0+1)) comparisons; retain O(a+b) labels."],
        ["Query and normalize","m maps; O(m log(max(a,b)+1)) membership comparisons; at most m field inversions; O(k log(k+1)) output sorting."],
        ["Extract finite fibers","One coordinate square-root call per compatible finite label; 11,392 calls recorded in this panel."],
        ["Outside visit counters","Prefix setup and discovery, allowed-set generation, factor-base membership, duplicate processing and independent-rank work."],
    ],[109,419],font_size=8.1,leading=10.7,pad=4)
    y = par(c,"The panel's 8,192 set visits and 125,440 reference Cartesian checks are structural counts for designed sets. They are not priced arithmetic, timings or a speed ratio.",y+7,size=8.5,leading=11.5)
    y = section(c,"The remaining shared-index gap",y+10)
    y = par(c,"The remaining dependency is useful-prefix/label discovery that shares work across changing transforms T, or labels controlling different unresolved branches. A successor must charge setup, rejected candidates, membership, duplicates and rank. This publication supplies no natural-yield measurement or full solve.",y,size=8.7,leading=11.8)
    y += 10
    c.setStrokeColor(RULE); c.line(LEFT,H-y,RIGHT,H-y)
    y += 8
    y = par(c,"<b>Standing:</b> derived prose / PCC-unverified; producer correctness evidence only. No IDC1h record, Lean/kernel certificate or signed external validation is asserted. Historical evidence remains unchanged.",y,size=7.9,leading=10.5)
    y = par(c,f'<font name="ReportMono">{STUDY}/</font><br/>'+linked(1,"MATHEMATICS.md")+" · "+linked(2,"evidence/receipt.json")+" · "+linked(3,"PROTOCOL.md")+" · "+linked(6,"README.md")+"<br/>"+linked(4,"crypto@"+UPSTREAM)+" · "+linked(5,"Merged PR #5232: earlier x-Cayley component"),y+5,size=7.1,leading=10.0)
    check_bottom(y,3)
    c.showPage()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--study-root",type=Path,default=Path(__file__).resolve().parent.parent)
    parser.add_argument("--output-dir",type=Path,default=Path(__file__).resolve().parent)
    parser.add_argument("--font-dir",type=Path,default=Path("/usr/share/fonts/truetype/dejavu"))
    args = parser.parse_args()
    receipt = json.loads((args.study_root/"evidence/receipt.json").read_text())
    if receipt["study"] != "cayley-residual-transport-m31-v1":
        raise ValueError("This report is tied to the frozen residual-transport study")
    # Guard publication provenance, not scientific validity. All values are read
    # from the retained receipt; this program generates no experiment inputs.
    if receipt["binary_sha256"] != "f4a05b459d9c5889c6287759960333c5f191fc9f8354bf4fbc187207e51fa62c":
        raise ValueError("Unexpected historical executable identity")
    register_fonts(args.font_dir)
    args.output_dir.mkdir(parents=True,exist_ok=True)
    flow = diagram()
    chart = outcomes_chart(receipt["result"])
    save_svg(flow,args.output_dir/"residual_transport.svg")
    save_svg(chart,args.output_dir/"evidence_outcomes.svg")
    pdf = args.output_dir/"REPORT.pdf"
    c = canvas.Canvas(str(pdf),pagesize=(W,H),pageCompression=1,invariant=0)
    c.setTitle("M31 residual chord labels: exact transport and signed extraction")
    c.setAuthor("Isogeny Labs")
    c.setSubject("Publication of frozen native correctness evidence; derived/PCC-unverified")
    page_one(c,flow)
    page_two(c)
    page_three(c,receipt,chart)
    c.save()
    print(json.dumps({"pdf":str(pdf),"pages":3,"source_receipt":str(args.study_root/"evidence/receipt.json"),"research_execution":False},indent=2))


if __name__ == "__main__":
    main()
