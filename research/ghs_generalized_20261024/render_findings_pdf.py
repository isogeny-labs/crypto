"""Render the concise reader-facing GHS report from frozen research receipts.

The raw per-phase vectors remain in REPORT.md and results/*.json. This renderer
checks the facts it places in the PDF before formatting them.
"""

from __future__ import annotations

import json
from pathlib import Path

from reportlab import rl_config
from reportlab.lib import colors
from reportlab.lib.enums import TA_CENTER, TA_LEFT, TA_RIGHT
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.lib.units import inch
from reportlab.platypus import (
    HRFlowable,
    KeepTogether,
    PageBreak,
    Paragraph,
    SimpleDocTemplate,
    Spacer,
    Table,
    TableStyle,
)

rl_config.useA85 = 0  # Keep compressed PDF streams binary in Git.


ROOT = Path(__file__).resolve().parent
RESULTS = ROOT / "results"
OUTPUT = ROOT / "output" / "pdf" / "GHS_FINDINGS_REPORT.pdf"
SPEC = [
    (6, 0, 32, ["direct", "two_isogeny"]),
    (8, 0, 32, ["direct", "two_isogeny"]),
    (10, 0, 32, ["direct", "two_isogeny", "three_isogeny"]),
    (12, 1, 512, ["direct", "two_isogeny", "five_isogeny"]),
]

NAVY = colors.HexColor("#173349")
TEAL = colors.HexColor("#087C7A")
INK = colors.HexColor("#243745")
MUTED = colors.HexColor("#5B6C78")
PALE = colors.HexColor("#EDF5F6")
LIGHT = colors.HexColor("#F5F8FA")
RULE = colors.HexColor("#CCD9DF")
WHITE = colors.white


def load_json(name: str) -> dict:
    return json.loads((RESULTS / name).read_text())


def pt(value: list) -> str:
    return "O" if value[2] else f"({value[0]}, {value[1]})"


def vec(values: list[str]) -> str:
    return "[" + ", ".join(values) + "]"


def millisecond(ns: int) -> str:
    return f"{ns / 1_000_000:.3f}"


def measured_costs(receipt: dict) -> tuple[str, str, str, str]:
    setup = sum(
        phase["wall_ns"]
        for phase in receipt["setup_phases"].values()
        if isinstance(phase, dict) and "wall_ns" in phase
    )
    if receipt["isogeny"]:
        setup += receipt["isogeny"]["construction_and_validation_cost"]["wall_ns"]
    phases = receipt["ghs"]["phases"]
    precompute = sum(phases[p]["wall_ns"] for p in ("factor_base", "relations_and_linear_algebra"))
    online = sum(phases[p]["wall_ns"] for p in ("map", "target_descent_and_replay"))
    rho = receipt["rho"]["cost"]["wall_ns"]
    return tuple(map(millisecond, (setup, precompute, online, rho)))


def evidence() -> tuple[list[dict], dict, dict]:
    rows = []
    for n, a, b, edges in SPEC:
        for edge in edges:
            name = f"exact-n{n}-a{a}-b{b}-{edge}.json"
            receipt = load_json(name)
            source = receipt["source"]
            assert source["absolute_degree"] == n
            assert int(source["a"], 0) == a and int(source["b"], 0) == b
            assert receipt["ghs_status"] == "verified"
            assert receipt["ghs"]["status"] == "verified"
            assert receipt["setup_phases"]["full_subgroup_checked_scalars"] == source["subgroup_order"]
            scalar = source["target_scalar_for_replay"]
            assert receipt["ghs"]["target_descent"]["scalar"] == scalar
            assert receipt["rho"]["result"]["scalar"] == scalar
            if receipt["isogeny"]:
                assert receipt["isogeny"]["subgroup_map_verified"]
                assert receipt["isogeny"]["subgroup_map_checked_scalars"] == source["subgroup_order"]
            rows.append({"n": n, "edge": edge, "name": name, "receipt": receipt, "costs": measured_costs(receipt)})
    assert len(rows) == 10
    bound = load_json("subgroup-weil-audit.json")
    equivalence = load_json("c-equivalence-audit.json")
    assert bound["excluded_profiles"] == 21 and bound["remaining_profiles"] == 232
    assert equivalence["total_profiles"] == 232
    assert [c["genus_two_profiles"] for c in equivalence["cases"]] == [14, 30, 62, 126]
    assert sum(c["checked_nonzero_subgroup_points"] for c in equivalence["cases"]) == 324
    audit = (RESULTS / "lean-audit.txt").read_text()
    assert "audited 13 theorems" in audit and "0 failing" in audit and "0 trusted" in audit
    return rows, bound, equivalence


styles = getSampleStyleSheet()
styles.add(ParagraphStyle(name="ReportTitle", fontName="Helvetica-Bold", fontSize=22, leading=25,
                          textColor=NAVY, spaceAfter=10))
styles.add(ParagraphStyle(name="Deck", fontName="Helvetica", fontSize=10.5, leading=15,
                          textColor=MUTED, spaceAfter=15))
styles.add(ParagraphStyle(name="Section", fontName="Helvetica-Bold", fontSize=13.2, leading=17,
                          textColor=NAVY, spaceBefore=12, spaceAfter=7))
styles.add(ParagraphStyle(name="Subsection", fontName="Helvetica-Bold", fontSize=10.2, leading=13,
                          textColor=NAVY, spaceBefore=8, spaceAfter=5))
styles.add(ParagraphStyle(name="BodyText2", fontName="Helvetica", fontSize=9.2, leading=13.2,
                          textColor=INK, spaceAfter=8))
styles.add(ParagraphStyle(name="SmallText2", fontName="Helvetica", fontSize=8.1, leading=11.3,
                          textColor=INK, spaceAfter=5))
styles.add(ParagraphStyle(name="TinyText2", fontName="Helvetica", fontSize=7.4, leading=10,
                          textColor=INK, spaceAfter=3))
styles.add(ParagraphStyle(name="TableHead2", fontName="Helvetica-Bold", fontSize=7.5, leading=9.6,
                          textColor=WHITE, alignment=TA_LEFT))
styles.add(ParagraphStyle(name="TableCell2", fontName="Helvetica", fontSize=7.6, leading=10.2,
                          textColor=INK))
styles.add(ParagraphStyle(name="TableCellRight2", parent=styles["TableCell2"], alignment=TA_RIGHT))
styles.add(ParagraphStyle(name="MetricNumber", fontName="Helvetica-Bold", fontSize=18, leading=20,
                          textColor=TEAL, alignment=TA_CENTER))
styles.add(ParagraphStyle(name="MetricLabel", fontName="Helvetica", fontSize=7.8, leading=10.5,
                          textColor=MUTED, alignment=TA_CENTER))
styles.add(ParagraphStyle(name="NoteText2", fontName="Helvetica", fontSize=8.2, leading=11.4,
                          textColor=NAVY))


def P(text: str, style: str = "BodyText2") -> Paragraph:
    return Paragraph(text, styles[style])


def section(label: str) -> Paragraph:
    return P(label, "Section")


def table(data: list[list], widths: list[float], right_cols: tuple[int, ...] = ()) -> Table:
    cooked = []
    for ri, row in enumerate(data):
        cooked.append([
            P(str(cell), "TableHead2" if ri == 0 else ("TableCellRight2" if ci in right_cols else "TableCell2"))
            for ci, cell in enumerate(row)
        ])
    result = Table(cooked, colWidths=widths, repeatRows=1, hAlign="LEFT")
    result.setStyle(TableStyle([
        ("BACKGROUND", (0, 0), (-1, 0), NAVY),
        ("VALIGN", (0, 0), (-1, -1), "MIDDLE"),
        ("ROWBACKGROUNDS", (0, 1), (-1, -1), [WHITE, LIGHT]),
        ("LINEBELOW", (0, 0), (-1, 0), 0.8, TEAL),
        ("LINEBELOW", (0, -1), (-1, -1), 0.5, RULE),
        ("LEFTPADDING", (0, 0), (-1, -1), 7),
        ("RIGHTPADDING", (0, 0), (-1, -1), 7),
        ("TOPPADDING", (0, 0), (-1, -1), 6),
        ("BOTTOMPADDING", (0, 0), (-1, -1), 6),
    ]))
    return result


def note(text: str) -> Table:
    t = Table([[P(text, "NoteText2")]], colWidths=[7.04 * inch])
    t.setStyle(TableStyle([
        ("BACKGROUND", (0, 0), (-1, -1), PALE),
        ("LINEBEFORE", (0, 0), (0, -1), 3, TEAL),
        ("LEFTPADDING", (0, 0), (-1, -1), 11),
        ("RIGHTPADDING", (0, 0), (-1, -1), 11),
        ("TOPPADDING", (0, 0), (-1, -1), 9),
        ("BOTTOMPADDING", (0, 0), (-1, -1), 9),
    ]))
    return t


def metric_cards() -> Table:
    numbers = ["10", "232", "21", "0"]
    labels = ["completed attack edges", "equivalent genus-two profiles", "zero-image profiles", "new vulnerabilities established"]
    content = [[P(n, "MetricNumber"), P(l, "MetricLabel")] for n, l in zip(numbers, labels)]
    t = Table([[Table([[n], [l]], colWidths=[1.7 * inch]) for n, l in content]],
              colWidths=[1.76 * inch] * 4)
    t.setStyle(TableStyle([
        ("BACKGROUND", (0, 0), (-1, -1), PALE),
        ("BOX", (0, 0), (-1, -1), 0.5, RULE),
        ("INNERGRID", (0, 0), (-1, -1), 0.5, RULE),
        ("TOPPADDING", (0, 0), (-1, -1), 10),
        ("BOTTOMPADDING", (0, 0), (-1, -1), 10),
        ("LEFTPADDING", (0, 0), (-1, -1), 2),
        ("RIGHTPADDING", (0, 0), (-1, -1), 2),
    ]))
    return t


def footer(canvas, doc):
    canvas.saveState()
    canvas.setStrokeColor(RULE)
    canvas.setLineWidth(0.5)
    canvas.line(0.73 * inch, 0.57 * inch, 7.77 * inch, 0.57 * inch)
    canvas.setFont("Helvetica", 7.5)
    canvas.setFillColor(MUTED)
    canvas.drawString(0.73 * inch, 0.39 * inch, "GENERALIZED GHS  /  25 OCT 2026")
    canvas.drawRightString(7.77 * inch, 0.39 * inch, f"{doc.page}")
    canvas.restoreState()


def build() -> Path:
    rows, bound, equivalence = evidence()
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    doc = SimpleDocTemplate(
        str(OUTPUT), pagesize=(8.5 * inch, 11 * inch),
        leftMargin=0.73 * inch, rightMargin=0.73 * inch,
        topMargin=0.72 * inch, bottomMargin=0.77 * inch,
        title="Generalized GHS: checked descent and attack costs",
        author="Codex research report", subject="Exact binary GHS subgroup maps and matched rho costs",
    )
    story = []

    # Page 1: decision and exact source identity.
    story += [P("Generalized GHS: checked descent and attack costs", "ReportTitle"),
              P("Exact binary curves, subgroup-preserving maps, prior-work classification, and matched target costs | 25 October 2026", "Deck"),
              metric_cards(), Spacer(1, 13),
              note("<b>Finding.</b> In the four completed screens, every surviving low-genus Hess parameter reduces to one of four canonical quadratic covers. The executed curves lie in known GHS/isogeny families. The recorded one-sample costs do not support a calibrated advantage over rho."),
              section("Exact source curves")]
    curve_data = [["GF(2^n)", "modulus", "a, b", "#E / r", "P", "Q"]]
    for n, a, b, _ in SPEC:
        source = next(row["receipt"]["source"] for row in rows if row["n"] == n)
        curve_data.append([f"n={n}", source["modulus"], f"{a}, {b}",
                           f"{source['group_order']} / {source['subgroup_order']}",
                           pt(source["generator"]), pt(source["target"])])
    story += [table(curve_data, [0.72*inch, 0.88*inch, 0.84*inch, 0.92*inch, 1.60*inch, 1.80*inch]),
              Spacer(1, 8),
              P("Each curve has equation <font face='Courier'>y^2 + x*y = x^3 + a*x^2 + b</font> in the stated polynomial-basis field. The source subgroup generator and target coordinates are part of the exact instance identity; the target scalar is 18 for n=6 and 37 for the other three curves.", "SmallText2"),
              section("Classification"),
              P("The screen found 253 genus-at-most-two profiles. A Jacobian-order bound excludes 21 because the selected order-199 subgroup cannot have a nonzero image. The other 232 are parameter presentations of four direct covers already executed end to end. Ten direct or isogeny attack edges are complete. No exact instance is classified as a new vulnerable curve; absence of a publication match for its hexadecimal coefficients does not establish novelty."),
              P("Scope is deliberately exact: four source curves, their listed subfields and descent parameters, and six small rational-kernel isogeny edges. It is not a census of every binary elliptic curve or every isogeny chain.", "SmallText2"),
              PageBreak()]

    # Page 2: canonicalization and map evidence.
    story += [section("Why 232 parameters collapse"),
              P("Write <font face='Courier'>beta = sqrt(b)</font> and <font face='Courier'>d = beta/c</font>. The documented Artin-Schreier presentation is <font face='Courier'>v^2 + v = c/z + a + d*z</font>, with <font face='Courier'>c*d = beta</font>. For all 232 index-two genus-two profiles in these exact screens, exactly one of <font face='Courier'>c</font> and <font face='Courier'>d</font> lies in the subfield. If <font face='Courier'>d</font> does, put <font face='Courier'>w=d*z</font>; otherwise put <font face='Courier'>w=c/z</font>. Both give <font face='Courier'>v^2 + v = beta/w + a + w</font> by a subfield-rational change."),
              P("In the second branch the source point is presented as <font face='Courier'>T-P</font>, where <font face='Courier'>T=(0,beta)</font> is rational two-torsion. Subtracting the transformed origin makes the induced degree-zero transfer the negative of the canonical transfer, preserving the discrete-log relation."),
              table([["n", "subfield degree", "canonical c", "genus-two c values", "subgroup points checked"]] +
                    [[str(c["absolute_degree"]), str(c["base_degree"]), c["canonical_c"],
                      str(c["genus_two_profiles"]), str(c["checked_nonzero_subgroup_points"])]
                     for c in equivalence["cases"]],
                    [0.52*inch, 1.18*inch, 1.18*inch, 1.45*inch, 2.71*inch], right_cols=(3, 4)),
              Spacer(1, 7),
              note("<b>Map check.</b> Each of the ten executed composite maps was checked on every scalar of its selected prime-order subgroup, with nonzero Jacobian image and preserved target relation. Sage independently checked the 232 parameter identities and all 324 nonzero subgroup-point identities. The normalized presentations were not separately timed as 232 Jacobian implementations."),
              section("Six rational-kernel isogeny edges")]
    iso_data = [["source n", "degree", "kernel generator", "quotient (a, b)"]]
    for row in rows:
        iso = row["receipt"]["isogeny"]
        if iso:
            iso_data.append([str(row["n"]), str(iso["degree"]), pt(iso["kernel_generator"]),
                             f"({iso['codomain_a']}, {iso['codomain_b']})"])
    story += [table(iso_data, [0.90*inch, 0.82*inch, 2.10*inch, 3.22*inch]),
              Spacer(1, 8),
              P("The full curve-to-cover formulas, quotient maps, and exact reduced Mumford images of <font face='Courier'>P</font> and <font face='Courier'>Q</font> are preserved per edge in the raw JSON receipts and the detailed REPORT.md. Quotient equations were checked on all rational source points; the selected subgroup was checked exhaustively.", "SmallText2"),
              PageBreak()]

    # Page 3: complete cost matrix.
    story += [section("Measured GHS versus rho work"),
              P("All elapsed values below are milliseconds from one unisolated sample per edge. Setup includes cover construction, source-order enumeration, and exhaustive subgroup-map verification; isogeny rows also include kernel and quotient validation. GHS precompute covers the factor base, relation collection, and matrix work. GHS online covers target mapping, descent, and replay. Rho is the complete negation-folded run on the same source <font face='Courier'>(P,Q)</font>."),
              table([["n", "edge", "FB / rank", "smooth / tries", "setup", "GHS pre", "GHS on", "rho on"]] +
                    [[str(row["n"]), row["edge"].replace("_isogeny", "-isog"),
                      f"{row['receipt']['ghs']['factor_base_size']} / {row['receipt']['ghs']['matrix_rank']}",
                      f"{row['receipt']['ghs']['relation_counts']['smooth']} / {row['receipt']['ghs']['relation_counts']['attempts']}",
                      *row["costs"]] for row in rows],
                    [0.39*inch, 0.91*inch, 0.84*inch, 0.96*inch, 0.98*inch, 0.98*inch, 0.98*inch, 1.00*inch],
                    right_cols=(2, 3, 4, 5, 6, 7)),
              Spacer(1, 10),
              note("<b>Cost interpretation.</b> The n=10 degree-3 edge has a lower recorded GHS online time (0.237 ms) than this rho run (0.313 ms), but it also incurs 33.400 ms of setup and 16.653 ms of GHS precomputation. These samples are too small and unisolated for a speedup claim."),
              section("What the counters include"),
              P("The receipts separately tally finite-field additions, multiplications, squarings, and informational inverse calls; they also record matrix-entry operations, polynomial-root tests, hash work, and rho additions, doublings, steps, and restarts. Inverse-call tallies overlap with counted field operations and must not be added as an independent cost. Source-field rho, smaller-field Jacobian arithmetic, and matrix work use different units, so the elapsed columns are not an algorithm-independent operation ratio."),
              P("No calibrated GAE band or comparison against the fastest external rho implementation was completed. The report preserves the original single-sample receipts rather than reselecting favorable timings after the fact.", "SmallText2"),
              PageBreak()]

    # Page 4: proofs, literature, and reproducibility.
    story += [section("Proof and replay status"),
              table([["check", "result", "boundary"] ,
                     ["Lean axiom audit", "13 declarations; 0 failing; 0 native_decide trusted", "Abstract scalar transfer, push-pull injectivity, zero-image criterion, normalization, sign"],
                     ["Executable maps", "10/10 full subgroup checks", "Concrete trace-conorm and Velu formulas checked on these finite groups"],
                     ["Independent Sage", "4 curve orders; 10 receipt replays; 232 c identities", "324 nonzero selected subgroup-point identities"],
                     ["Rust focused tests", "26 library + 4 attack executable passed", "Integrated PR head 16f786d3d26"]],
                    [1.25*inch, 2.25*inch, 3.54*inch]),
              Spacer(1, 9),
              P("Lean does not yet prove the concrete trace-conorm or Velu coordinate maps, and the Weil bound arithmetic is audited outside Lean. The 21 excluded profiles have zero image for the selected order-199 subgroup by the stated bound; that result does not classify arbitrary subgroups on other curves.", "SmallText2"),
              section("Canonical direct-cover data"),
              P("For the four direct edges, <font face='Courier'>C: W^2 + hW = f</font> with <font face='Courier'>h=[0x0,0x1,0x1]</font>. Coefficient arrays run from constant term upward. The basis embeds the smaller field in the source field; the last column gives the reduced Mumford images of the exact source generator and target.", "TinyText2")]
    cover_data = [["n", "f coefficients", "subfield basis", "J(P) / J(Q) as (u; v)"]]
    for row in rows:
        if row["edge"] != "direct":
            continue
        ghs = row["receipt"]["ghs"]
        image_p, image_q = ghs["image_generator"], ghs["image_target"]
        fmt = lambda image: f"u={vec(image['u'])}; v={vec(image['v'])}"
        cover_data.append([
            str(row["n"]), vec(ghs["map"]["f"]), vec(ghs["map"]["subfield_embedding_basis"]),
            f"P: {fmt(image_p)}<br/>Q: {fmt(image_q)}",
        ])
    story += [table(cover_data, [0.37*inch, 1.72*inch, 1.63*inch, 3.32*inch]),
              section("Prior work and candidate status"),
              P("The descent model and its parameter <font face='Courier'>c</font> are documented by <link href='https://docs.magma-maths.org/ArithmeticGeometry/EllipticCurvesOverFiniteFields/ec_weil_desc.html' color='#087C7A'>Magma's Weil Descent handbook</link>. Moving to isogenous curves before GHS is covered by <link href='https://www.iacr.org/archive/eurocrypt2002/23320028/smart.pdf' color='#087C7A'>Galbraith-Hess-Smart (EUROCRYPT 2002)</link>; <link href='https://www.cambridge.org/core/journals/lms-journal-of-computation-and-mathematics/article/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem/315278965D8D277A85812D9498236A88' color='#087C7A'>Hess (2004)</link> generalizes the construction. The per-edge coefficient searches and exact-instance uncertainty are recorded in PRIOR_WORK.md. All ten edges are classified as known-family examples, with exact publication matches unresolved. No new vulnerability claim survives this exact set."),
              section("Reproduce and inspect")]
    story += [P("Research directory: <font face='Courier'>research/ghs_generalized_20261024/</font>. The detailed <font face='Courier'>REPORT.md</font> and <font face='Courier'>results/exact-*.json</font> contain every image and raw cost vector. <font face='Courier'>results/MANIFEST.json</font> binds the measured executable and receipt hashes. Run <font face='Courier'>verify_orders.sage</font>, <font face='Courier'>verify_receipts.sage</font>, and <font face='Courier'>verify_c_equivalence.sage</font> with Sage; run the Lean axiom audit from <font face='Courier'>lean/</font>. The review branch is <link href='https://github.com/aburan28/crypto/pull/2539' color='#087C7A'>PR #2539</link>.", "SmallText2"),
              HRFlowable(width="100%", thickness=0.5, color=RULE, spaceBefore=4, spaceAfter=8),
              P("<b>Research limit.</b> This report covers the quadratic genus-two executable branch on four specified curves. It does not establish an exhaustive classification across all fields, higher-genus index calculus, long isogeny walks, or a calibrated advantage over rho.", "SmallText2")]

    doc.build(story, onFirstPage=footer, onLaterPages=footer)
    return OUTPUT


if __name__ == "__main__":
    print(build())
