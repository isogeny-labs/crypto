"""Render retained CSV observations and the report; performs no research runs."""
import csv
import json
import re
from pathlib import Path
from urllib.parse import urljoin

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst

HERE = Path(__file__).resolve().parent
FIG = HERE / "figures"
FIG.mkdir(exist_ok=True)
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 10,
                     "svg.fonttype": "none", "axes.spines.top": False,
                     "axes.spines.right": False})
BLUE, ORANGE, GREEN = "#235789", "#b04f15", "#287359"


def rows(name):
    with (HERE / name).open() as stream:
        return list(csv.DictReader(stream))


def save(fig, name):
    fig.savefig(FIG / f"{name}.svg", bbox_inches="tight")
    svg = FIG / f"{name}.svg"
    svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
    fig.savefig(FIG / f"{name}.png", dpi=150, bbox_inches="tight")
    plt.close(fig)


binary = rows("binary_results_v2.csv")
fig, ax = plt.subplots(figsize=(9, 4.5), constrained_layout=True)
for kind, marker, color, label in [
    ("generic", "o", BLUE, "b outside subfield: 7 curves per q"),
    ("descended_b_trace0", "s", GREEN, "descended b, trace(a)=0"),
    ("descended_b_trace1", "v", ORANGE, "descended b, trace(a)=1"),
]:
    selected = [r for r in binary if r["kind"] == kind]
    ax.scatter([2 ** int(r["m"]) for r in selected],
               [float(r["coverage"]) for r in selected],
               marker=marker, color=color, label=label, s=42, alpha=.8)
ax.axhline(.375, color="#333333", linestyle="--", linewidth=1,
           label="3/8: proposed generic limit")
ax.axhline(.39346934, color="#777777", linestyle=":", linewidth=1,
           label="1-exp(-1/2): occupancy approximation")
ax.set_xscale("log", base=2)
ax.set_xticks([8, 16, 32, 64, 128], ["8", "16", "32", "64", "128"])
ax.set(xlabel="Subfield size q (quadratic extension)",
       ylabel="Reachable points / full group", ylim=(-.02, .55),
       title="Same trace obstruction, different coverage")
ax.legend(loc="lower center", bbox_to_anchor=(.5, 1.02), ncol=2,
          fontsize=8, frameon=False)
save(fig, "binary_coverage")

odd = rows("odd_results_v2.csv")
fig, axes = plt.subplots(1, 2, figsize=(10, 4.5), constrained_layout=True)
for n, marker in [(2, "o"), (3, "s"), (4, "^"), (5, "D")]:
    selected = [r for r in odd if r["kind"] == "odd_subfield"
                and int(r["degree"]) == n]
    axes[0].scatter([float(r["null_mean_coverage"]) for r in selected],
                    [float(r["coverage"]) for r in selected],
                    label=f"n={n}", marker=marker, s=36)
for kind, marker, color, label in [
    ("prime_kummer", "o", BLUE, "Kummer"),
    ("prime_dickson_split", "s", ORANGE, "Split Dickson"),
]:
    selected = [r for r in odd if r["kind"] == kind]
    axes[1].scatter([float(r["null_mean_coverage"]) for r in selected],
                    [float(r["coverage"]) for r in selected],
                    marker=marker, color=color, label=label, s=36)
for ax, title in zip(axes, ["24 odd-characteristic subfield cases",
                            "18 prime-tower cases"]):
    ax.plot([0, .6], [0, .6], color="#777777", linestyle="--", linewidth=1)
    ax.set(xlim=(0, .6), ylim=(0, .6), xlabel="Mean coverage of 32 controls",
           ylabel="Structured-base coverage", title=title)
    ax.legend(frameon=False, fontsize=9)
fig.suptitle("Fresh small-instance diagnostics; not the original experiments", fontsize=12)
save(fig, "coverage_controls")

fig, axes = plt.subplots(1, 2, figsize=(10, 4.2), constrained_layout=True)
coordinates = [[(.15, .15), (.85, .15), (.5, .8)],
               [(.2, .2), (.8, .2), (.8, .8), (.2, .8)]]
for ax, vertices, names, title, caption in zip(
    axes, coordinates, ["xyz", "xyzw"],
    ["Triangle: no target constraint", "Square: usable when coefficient is nonzero"],
    ["Large-prime matrix rank 3 / 3\nAll 101 target values remain possible",
     "Large-prime matrix rank 3 / 4\nAlternating edge sum eliminates all four logs"],
):
    for i, (x, y) in enumerate(vertices):
        x2, y2 = vertices[(i + 1) % len(vertices)]
        ax.plot([x, x2], [y, y2], color=BLUE, linewidth=2)
        ax.scatter([x], [y], s=370, color="white", edgecolor=BLUE, zorder=3)
        ax.text(x, y, names[i], ha="center", va="center", zorder=4)
        ax.text((x+x2)/2, (y+y2)/2 + .045,
                f"{names[i]}+{names[(i+1)%len(vertices)]}",
                ha="center", fontsize=9, bbox=dict(facecolor="white", edgecolor="none", pad=1))
    ax.set(xlim=(0, 1), ylim=(-.15, 1), title=title)
    ax.text(.5, -.06, caption, ha="center", va="top", fontsize=9)
    ax.axis("off")
fig.suptitle("Graph cycles require a signed linear-dependence check (modulo 101)", fontsize=12)
save(fig, "cycle_dependence")

# Typeset the entire Markdown report. This adapter only changes presentation;
# it does not compute results, statistical summaries, field arithmetic or costs.
base = "https://github.com/aburan28/crypto/blob/codex/three-leads-audit-20261018/research/three_leads_audit_20261018/"


def quoted(value):
    return json.dumps(value, ensure_ascii=False)


def inline(value):
    tokens = re.split(r"(`[^`]+`|\*\*[^*]+\*\*|\[[^\]]+\]\([^)]+\))", value)
    output = []
    for token in tokens:
        if not token:
            continue
        if token.startswith("`"):
            output.append(f'#text(font: "DejaVu Sans Mono", size: 8.1pt, {quoted(token[1:-1])})')
        elif token.startswith("**"):
            output.append(f'#strong[{inline(token[2:-2])}]')
        elif token.startswith("["):
            label, target = re.match(r"\[([^\]]+)\]\(([^)]+)\)", token).groups()
            target = target if "://" in target else urljoin(base, target)
            output.append(f'#link({quoted(target)})[{inline(label)}]')
        else:
            output.append(f'#text({quoted(token)})')
    return "".join(output)


def figure(name, caption):
    return (f'#figure(image("figures/{name}.svg", width: 100%), '
            f'caption: [{inline(caption)}])\n')


source = [
    '#set page(paper: "a4", margin: (x: 17mm, y: 17mm), '
    'numbering: "1", header: align(right, text(size: 8pt, fill: rgb("666666"))[Three ECDLP leads · 2026-10-18]))',
    '#set text(font: "DejaVu Sans", size: 10.3pt)',
    '#set par(leading: .6em, spacing: .85em)',
    '#set heading(numbering: none)',
    '#show link: set text(fill: rgb("235789"))',
    '#show heading.where(level: 1): set text(size: 19pt)',
    '#show heading.where(level: 2): set text(size: 13pt)',
    '#show heading.where(level: 3): set text(size: 10.5pt)',
    '#show table.cell: set text(size: 8.7pt)',
]
lines = (HERE / "README.md").read_text().splitlines()
i = 0
while i < len(lines):
    line = lines[i]
    if not line.strip():
        i += 1
        continue
    if line.startswith("#"):
        if line == "### Fresh odd-characteristic checks":
            source.append(figure("binary_coverage", "Complete two-sum enumeration on 39 quadratic-extension binary curves. The two degree-four controls are excluded. Descended-b controls show why trace alone does not fix coverage."))
        elif line.startswith("## 3."):
            source.append(figure("coverage_controls", "Structured coverage versus matched-control mean. Fields are deliberately small; each point is a diagnostic, not a significance test or a replay of the original census."))
        elif line == "### Counts, costs and ranking":
            source.append(figure("cycle_dependence", "Exact signed-cycle control: a triangle can constrain only the large-prime variables, while a square can eliminate them. Extraction still requires a nonzero target coefficient."))
        level = len(line) - len(line.lstrip("#"))
        source.append(f'#heading(level: {level})[{inline(line[level:].strip())}]\n')
        i += 1
    elif line.startswith("|"):
        table = []
        while i < len(lines) and lines[i].startswith("|"):
            cells = [cell.strip() for cell in lines[i].strip("|").split("|")]
            if not all(re.fullmatch(r"[-: ]+", cell) for cell in cells):
                table.append(cells)
            i += 1
        n = len(table[0])
        widths = "(1fr, 2fr)" if n == 2 else "(" + ", ".join("1fr" for _ in range(n)) + ")"
        body = ",\n".join("[" + inline(cell) + "]" for row in table[1:] for cell in row)
        header = ", ".join("[" + inline(cell) + "]" for cell in table[0])
        source.append(f'#table(columns: {widths}, inset: 4pt, stroke: .3pt + rgb("cccccc"), table.header({header}), {body})\n')
    else:
        paragraph = []
        while i < len(lines) and lines[i].strip() and not lines[i].startswith(("#", "|")):
            paragraph.append(lines[i].strip())
            i += 1
        source.append(inline(" ".join(paragraph)) + "\n\n")

(HERE / "report.typ").write_text("\n".join(source).rstrip() + "\n")
typst.compile(str(HERE / "report.typ"), output=str(HERE / "report.pdf"), root=str(HERE))
print("Rendered three SVG/150-dpi PNG figures and report.pdf from retained observations.")
