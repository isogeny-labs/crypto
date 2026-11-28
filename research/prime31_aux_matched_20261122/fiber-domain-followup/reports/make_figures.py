"""Exact SVG document illustrations; layout only, no research computation."""
from __future__ import annotations
from html import escape
from pathlib import Path

NAVY = "#192c42"
BLUE = "#345d98"
TEAL = "#087f82"
MUTED = "#53647a"
PALE = "#eff4f8"
GOLD = "#fff4df"
LINE = "#b9cbd9"


def text(x, y, words, size=21, fill=NAVY, weight="normal", anchor="start"):
    return f'<text x="{x}" y="{y}" font-size="{size}" fill="{fill}" font-weight="{weight}" text-anchor="{anchor}">{escape(words)}</text>'


def rect(x, y, width, height, fill=PALE, stroke="none", radius=12):
    return f'<rect x="{x}" y="{y}" width="{width}" height="{height}" rx="{radius}" fill="{fill}" stroke="{stroke}" stroke-width="2"/>'


def arrow(x1, y1, x2, y2, color=MUTED, dashed=False):
    dash = ' stroke-dasharray="7 6"' if dashed else ""
    return f'<path d="M{x1},{y1} L{x2},{y2}" fill="none" stroke="{color}" stroke-width="2.5" marker-end="url(#head)"{dash}/>'


def write(path: Path, height: int, title: str, parts: list[str]):
    body = "\n".join(parts)
    path.write_text(
        f'<svg xmlns="http://www.w3.org/2000/svg" width="960" height="{height}" viewBox="0 0 960 {height}" role="img">'
        f'<title>{escape(title)}</title><defs><marker id="head" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 Z" fill="{MUTED}"/></marker></defs>'
        f'<g font-family="DejaVu Sans, sans-serif">{body}</g></svg>\n', encoding="utf-8"
    )


def make_figures(directory: Path):
    directory.mkdir(parents=True, exist_ok=True)
    parts = [
        rect(0, 0, 452, 152), rect(508, 0, 452, 152),
        text(22, 32, "AUXILIARY CURVE", 19, TEAL, "bold"),
        text(22, 66, "A− : y² = x³ − x", 28, NAVY, "bold"),
        text(22, 103, "H₂ chooses the kernel"), text(22, 134, "S + H₂ chooses one fiber"),
        text(530, 32, "QUOTIENT CURVE", 19, BLUE, "bold"),
        text(530, 66, "C = A− / H₂", 28, NAVY, "bold"),
        text(530, 103, "Every S + h has the same image"), text(530, 134, "φ(S + h) = φ(S)"),
        arrow(456, 77, 504, 77), text(480, 53, "φ", 24, TEAL, "bold", "middle"),
        rect(0, 197, 452, 99, "#e8f5f2"), text(22, 233, "V: 1,024 auxiliary x-values", 24, TEAL, "bold"),
        text(22, 270, "Forget auxiliary y; keep x as a label"),
        rect(508, 197, 452, 99, "#e9effb"), text(530, 233, "F: 515 usable target x-values", 24, BLUE, "bold"),
        text(530, 270, "Lift (x, ±y) on the target E"),
        arrow(226, 155, 226, 192), arrow(456, 246, 504, 246, dashed=True),
        text(480, 333, "The dashed step is a coordinate filter, not a group homomorphism A− → E.", 20, MUTED, anchor="middle"),
    ]
    write(directory / "roles.svg", 350, "An auxiliary isogeny creates coordinates; the target curve supplies signed points", parts)

    parts = [text(0, 25, "TEN DEGREE-2 MAPS, GROUPED INTO FIVE DEGREE-4 BLOCKS", 21, TEAL, "bold")]
    for block in range(5):
        x = block * 194
        parts.extend([
            rect(x, 49, 176, 100),
            text(x + 88, 78, f"Block {block + 1}", 21, NAVY, "bold", "middle"),
            text(x + 40, 119, "2", 27, BLUE, "bold", "middle"),
            text(x + 136, 119, "2", 27, BLUE, "bold", "middle"),
            arrow(x + 62, 110, x + 113, 110),
        ])
        if block < 4:
            parts.append(arrow(x + 179, 99, x + 190, 99))
    parts.extend([
        rect(0, 178, 952, 58, "#e8f5f2"),
        text(476, 215, "Same full map: 2¹⁰ = 4⁵ = 1,024   |   Same kernel and same full fiber V", 23, TEAL, "bold", "middle"),
    ])
    write(directory / "degrees.svg", 246, "Small-degree and grouped presentations of the same degree-1024 map", parts)

    parts = [text(0, 25, "SYMBOLIC ACCOUNTING FOR q QUERIES; THESE ARE NOT MEASURED DURATIONS", 20, MUTED)]
    rows = [
        (58, "Legacy", "q × fixed validation", "qC", PALE),
        (132, "Prepared", "one fixed validation", "C", "#e8f5f2"),
        (206, "Fiber driver", "same validation; reuse its paths", "C", GOLD),
    ]
    for y, label, work, formula, fill in rows:
        parts.extend([rect(0, y, 952, 60, fill), text(20, y + 38, label, 23, NAVY, "bold"),
                      text(242, y + 38, work, 23), text(917, y + 39, formula, 27, TEAL, "bold", "end")])
    parts.append(text(476, 306, "C = 624,640 counted multiplications in the retained full-tower validation", 23, NAVY, "bold", "middle"))
    parts.append(text(476, 338, "Index assembly and temporary path storage remain charged cold costs", 20, MUTED, "normal", "middle"))
    write(directory / "validation.svg", 354, "One validation pass, reused forward paths and separately charged index construction", parts)

    parts = [
        text(0, 25, "SMALL ALGEBRA EXAMPLE IN F₃₁; NOT A PRIME31 PERFORMANCE RUN", 20, MUTED),
        rect(300, 45, 360, 60, "#e8f5f2"), text(480, 83, "Fixed parent z = 5", 25, TEAL, "bold", "middle"),
    ]
    positions = [93, 336, 579, 822]
    for x, label in zip(positions, ["8", "13", "19", "27"]):
        parts.extend([arrow(480, 108, x + 31, 143), rect(x - 35, 149, 132, 64), text(x + 31, 190, label, 29, NAVY, "bold", "middle")])
    parts.extend([
        text(480, 252, "A coupled consequence h(x) = (x − 8)(x − 13) leaves only 8 and 13.", 22, NAVY, "normal", "middle"),
        rect(0, 283, 460, 91, "#e8f5f2"), text(20, 317, "Two candidates remain", 25, TEAL, "bold"),
        text(20, 353, "Check all original equations and signs"),
        rect(492, 283, 460, 91, GOLD), text(512, 317, "All four remain?", 25, NAVY, "bold"),
        text(512, 353, "No new information: retain Unknown"),
    ])
    write(directory / "branching.svg", 388, "Exact conditional candidate filtering with an explicit unknown outcome", parts)

    parts = [
        rect(0, 0, 470, 130), rect(523, 0, 429, 130, "#e8f5f2"),
        text(22, 34, "KEEP ONE AUGMENTED MATRIX", 19, BLUE, "bold"),
        text(235, 82, "B = [ A | −β ]", 34, NAVY, "bold", "middle"),
        text(235, 115, "Unknowns: base logs ℓ and target log s", 20, MUTED, anchor="middle"),
        arrow(476, 68, 516, 68),
        text(544, 34, "PROJECT AWAY THE LAST COLUMN", 19, TEAL, "bold"),
        text(737, 80, "rank(A) = rank(B) − ε", 27, NAVY, "bold", "middle"),
        text(737, 115, "ε = 1 if the target column is pinned", 20, MUTED, anchor="middle"),
        text(476, 173, "A row (0, …, 0, 1) determines s even before every base logarithm is known.", 22, NAVY, anchor="middle"),
    ]
    write(directory / "rank.svg", 190, "Rank of the base matrix follows from a projection of the retained augmented row space", parts)


if __name__ == "__main__":
    make_figures(Path(__file__).resolve().parent / "figures")
