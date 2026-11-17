#!/usr/bin/env python3
"""Lay out recorded native results. No collection, statistics, or cost model."""
from pathlib import Path
import hashlib
import json
import shutil
import sys

import fitz
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst

root = Path(__file__).resolve().parent
source_json = Path(sys.argv[1]).resolve()
layout_out = Path(sys.argv[2]).resolve()
layout_out.mkdir(parents=True, exist_ok=False)
data = json.loads(source_json.read_text())
assert data["schema"] == "twist-zero-pipeline-accounting/v1"
assert data["mode"] == "snapshot"
assert data["source_summary_recorded_exits"] == 29
assert data["verified_configurations"] == 9
figures = root / "figures"
figures.mkdir(exist_ok=True)
assert not (figures / "systems.svg").exists()
assert not (root / "REPORT.pdf").exists()

arms = [("plain", "phi-l0-active", "plain φ"),
        ("plain", "double-l0-active", "plain [2]"),
        ("zero", "phi-l0-active", "zero φ"),
        ("zero", "double-l0-active", "zero [2]")]

def selected(seed):
    ordered = []
    for mode, arm, label in arms:
        matches = [r for r in data["rows"] if r["p"] == 37 and r["seed"] == seed
                   and r["mode"] == mode and r["arm"] == arm]
        assert len(matches) == 1 and matches[0]["status"] == "verified-first-aa"
        ordered.append((label, matches[0]))
    return ordered

plt.rcParams.update({"font.size": 10, "svg.fonttype": "none"})
fig, axes = plt.subplots(1, 2, figsize=(10.2, 3.5), sharey=True)
for seed, ax in enumerate(axes):
    records = selected(seed)
    zero = [r["metrics"]["zero_f4_systems"] for _, r in records]
    target = [r["metrics"]["target_f4_systems"] for _, r in records]
    positions = range(len(records))
    ax.bar(positions, zero, color="#d5a34f", label="zero precomputation")
    ax.bar(positions, target, bottom=zero, color="#37698a", label="target decomposition")
    for position, (_, record) in enumerate(records):
        m = record["metrics"]
        ax.annotate(str(m["total_f4_systems"]),
                    (position, m["total_f4_systems"]), xytext=(0, 4),
                    textcoords="offset points", ha="center")
    ax.set_xticks(list(positions), [label for label, _ in records])
    ax.set_title(f"p37, seed {seed}; one known-answer target")
    ax.set_ylim(0, 405)
    ax.grid(axis="y", alpha=0.2)
    ax.set_axisbelow(True)
axes[0].set_ylabel("Finite F4 systems counted")
axes[1].legend(loc="upper left", fontsize=8)
fig.suptitle("Historical partial panel: system counts, not complete cost", fontsize=12)
fig.tight_layout()
fig.savefig(figures / "systems.svg")
fig.savefig(figures / "systems.png", dpi=150)
plt.close(fig)
# Canonicalize incidental Matplotlib XML line whitespace. Coordinates,
# labels and the PNG are unchanged.
svg_path = figures / "systems.svg"
svg_path.write_text("\n".join(line.rstrip() for line in svg_path.read_text().splitlines()) + "\n")

rows = selected(1)
def fmt(v):
    return f"{v:,}" if isinstance(v, int) else str(v)

table1 = []
table2 = []
for label, r in rows:
    m = r["metrics"]
    table1.append([label, m["zero_f4_systems"], m["target_f4_systems"],
                   m["total_f4_systems"], m["target_queries"],
                   m["zero_factor_rank"], f'{m["rank"]}/{m["columns"]}'])
    table2.append([label, m["zero_unique_signed"], m["target_unique_signed"], m["unique_projective_rows"],
                   m["cold_f4_row_fp_muls"], m["online_native_fp_muls"],
                   m["native_fp_muls"]])

def typ_table(headers, body):
    cells = headers + [fmt(c) for row in body for c in row]
    return (f'#table(columns: {len(headers)}, inset: 5pt, align: left,\n'
            + ",\n".join(f'[{c}]' for c in cells) + ')\n')

sha = hashlib.sha256(source_json.read_bytes()).hexdigest()
public_sha = rows[0][1]["public_input_sha256"]
intro = (
    "The native accounting tool re-read the already committed 29-exit snapshot. "
    "Nine configurations in that historical snapshot have first-run audit and "
    "two deterministic A/A checks. The comparison below uses the two complete "
    "four-arm groups at p37, seeds 0 and 1. The remaining p37 seed-2 zero-phi "
    "configuration is retained in the JSON, without inventing its missing controls. "
    "Later panel completions are excluded from this historical analysis."
)
interpretation = (
    "At seed 1, zero-phi reaches full factor rank before the target and uses one "
    "target query where plain phi uses two. Its recorded zero plus target F4 "
    "system count is 154 versus 144 for plain phi. At seed 0, all four arms "
    "already recover after one target query. This separates an observed online "
    "rank effect from the preprocessing charge. Neither observation determines "
    "complete end-to-end cost."
)
report_typ = '''#set page(paper: "a4", margin: (x: 17mm, y: 16mm))
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false, leading: 0.55em)
#set heading(numbering: none)
= Twist index calculus: checked accounting
*Historical partial panel; 25 October 2026. No new solve or timing admission.*

''' + intro + '''

#image("figures/systems.svg", width: 100%)

== Seed 1: preprocessing, online work and rank
''' + typ_table(["Arm", "Zero F4", "Target F4", "All F4", "Queries", "Zero rank", "Final rank"], table1) + '''

''' + interpretation + '''

*Scope:* these are public synthetic known-answer workloads and presentations,
not independent secrets. A finite-system count is a diagnostic, not a common
operation price. Every full-panel and complete-price obligation remains open.

#pagebreak()
= Relation yield and arithmetic coverage
Seed 1, same verified configurations. Signed target tuples and projective
matrix rows count different objects; they are not interchangeable relation yields.

''' + typ_table(["Arm", "Zero signed", "Target signed", "Projective rows", "F4 row muls", "Online Fp muls", "Cold Fp muls"], table2) + '''

F4 row multiplications and native Fp multiplications cover different code.
Group calls also contain lower-level arithmetic; their counters cannot simply
be added to field counters as another independent primitive price. The JSON
retains these vectors, allocation observations, RSS, exact count ratios and
all three raw diagnostic intervals for every verified configuration.

== What was checked
The tool binds records to independent audit and parity hashes, requires all
three registered executions to have exit 0, checks exact deterministic A/A
equality, recomputes counts and cross-checks the frozen panel reducer and
independent rank/point checks. Three native tests cover journal allocation,
missing/non-telescoping intervals, duplicate rows and undefined ratios. The
actual partial-as-complete control must reject before creating output.

The eight phases telescope to the charged cold interval. The reusable and
online intervals also telescope. The two journal portions are derived from
that recorded cut; all journal time is not assigned to the online interval.
The clocks remain raw shared-host diagnostics and are not used for speedups.

== What remains unresolved
Complete end-to-end prices remain null. Missing items include non-row F4
work, unmetered primitives, memory/hashing/journal costs, and calibrated scalar
and group prices. Full-panel completion, final input/source seals, native
reduction replay, archive publication, admitted timing and strongest matched
IC/rho controls remain separate obligations. This re-analysis does not modify
the two frozen producers, their workload or managed owner.

== Reproducibility
Native tool: `postprocess/native`; raw check: `runs/phase2-accounting-v1`.
Compact results: `results/phase2-accounting-v1/accounting.json`.
Source snapshot: `results/phase2-live-progress-v1/summary.json`.

Accounting JSON SHA-256:\\
#text(size: 7pt)[''' + sha + ''']

Source summary SHA-256:\\
#text(size: 7pt)[''' + data["source_summary_sha256"] + ''']

p37 seed-1 public-input SHA-256:\\
#text(size: 7pt)[''' + public_sha + ''']

Canonical IC/rho charts and benchmark index stay unchanged: complete price,
matched rho and VALIDATION artifacts have not been established here.
'''
(root / "REPORT.typ").write_text(report_typ)
md = "# Twist index calculus: checked accounting\n\n" + intro + "\n\n" + interpretation + "\n\n"
md += "## Seed 1\n\n| Arm | Zero F4 | Target F4 | Total F4 | Queries | Zero rank | Final rank |\n|---|---:|---:|---:|---:|---:|---:|\n"
for row in table1:
    md += "| " + " | ".join(fmt(v) for v in row) + " |\n"
md += "\n## Yield and counted arithmetic, seed 1\n\n| Arm | Zero signed | Target signed | Projective rows | F4 row muls | Online Fp muls | Cold Fp muls |\n|---|---:|---:|---:|---:|---:|---:|\n"
for row in table2:
    md += "| " + " | ".join(fmt(v) for v in row) + " |\n"
md += ("\nAll complete end-to-end prices and admitted speedup fields remain null. "
       "Full panel, final seals, complete primitive prices and strongest matched "
       "IC/rho remain open. Raw clocks are diagnostic only.\n\n"
       "Three native tests and the actual partial-as-complete rejection are "
       "implementation controls; see the raw command/exit receipts. No new solve "
       "or new live snapshot was executed.\n\n"
       "![Recorded finite F4 systems](figures/systems.svg)\n\n"
       "[Native accounting](../results/phase2-accounting-v1/accounting.json); "
       "[source snapshot](../results/phase2-live-progress-v1/summary.json).\n\n"
       "Canonical IC/rho charts and benchmark index stay unchanged because the "
       "complete-price and VALIDATION gates are unresolved.\n\n"
       f"Accounting JSON SHA-256: `{sha}`.\n")
(root / "REPORT.md").write_text(md)

snapshot = layout_out / "source"
(snapshot / "figures").mkdir(parents=True)
for name in ["REPORT.typ", "REPORT.md", "render-report.py"]:
    shutil.copy2(root / name, snapshot / name)
for name in ["systems.svg", "systems.png"]:
    shutil.copy2(figures / name, snapshot / "figures" / name)
shutil.copy2(source_json, snapshot / "accounting.json")
typst.compile(str(snapshot / "REPORT.typ"), output=str(root / "REPORT.pdf"))
pages = []
with fitz.open(root / "REPORT.pdf") as document:
    for i, page in enumerate(document):
        destination = layout_out / f"page-{i+1}.png"
        page.get_pixmap(matrix=fitz.Matrix(1.5, 1.5)).save(destination)
        pages.append({"page": i+1, "image": str(destination), "characters": len(page.get_text())})
(layout_out / "layout.json").write_text(json.dumps({
    "scope": "Layout/plotting of recorded native values only",
    "accounting_sha256": sha,
    "pdf_sha256": hashlib.sha256((root / "REPORT.pdf").read_bytes()).hexdigest(),
    "pages": pages,
}, indent=2) + "\n")
