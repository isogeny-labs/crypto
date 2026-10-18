#!/usr/bin/env python3
"""Lay out the recorded results as a figure, RESULT.md's tables and PDFs.

Layout only (AGENTS.md "Implementation language: no Python"; research-visuals skill): every
number is read from `tables.json` (written by `examples/volcano_tall_analyze.rs`) or from the
sessions' own audit logs.  Nothing is computed here except axis limits and text formatting.
Needs matplotlib and typst (a scratch virtualenv outside the repository).
"""
import glob, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
T = json.load(open(f"{HERE}/tables.json"))
RUN = sys.argv[1] if len(sys.argv) > 1 else "run"
OUT = f"{HERE}/reports/{RUN}"
os.makedirs(OUT, exist_ok=True)

import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt
plt.rcParams.update({"svg.fonttype": "none", "font.size": 8.5})

M = T["margin"]
vols = T["volcanoes"]
pooled = {e["scope"]: e for e in T["pooled"]}
colors = ["#1f6fb2", "#c4561a", "#2a9d5c", "#8e44ad", "#b8860b", "#d6336c", "#17a2b8", "#555555", "#000000", "#7f7f7f"]

fig, ax = plt.subplots(1, 3, figsize=(11, 3.7), gridspec_kw={"width_ratios": [1.35, 1.25, 0.6]})
# (a) per-level mean S / volcano mean against level / depth
for i, v in enumerate(vols):
    xs = [l["level"] / v["depth"] for l in v["levels"]]
    ys = [l["mean_s"] / v["mean_s"] for l in v["levels"]]
    ax[0].plot(xs, ys, "o-", ms=2.5, lw=0.8, color=colors[i % len(colors)], label=f'{v["id"]} (ℓ={v["ell"]}, D={v["d"]}, k={v["depth"]})', alpha=0.85)
ax[0].axhspan(1 - M, 1 / (1 - M), color="#999", alpha=0.15, lw=0)
ax[0].axhline(1, color="#444", lw=0.7, ls="--")
ax[0].set_xlabel("level / depth  (0 = crater, 1 = floor)")
ax[0].set_ylabel("mean S at the level / volcano mean")
ax[0].set_title("(a) every level of every volcano")
ax[0].legend(fontsize=6, frameon=False, ncol=2, loc="upper center")
ax[0].set_ylim(0.6, 1.6)
# (b) forest plot of pooled and per-volcano ratios
rows = []
for scope in [s for s in pooled if not s.startswith("volcano")] + [s for s in pooled if s.startswith("volcano")]:
    for r in pooled[scope]["rows"]:
        if r["comparison"] == "floor / crater":
            rows.append((scope, r))
for k, (scope, r) in enumerate(rows):
    y = len(rows) - k
    bold = not scope.startswith("volcano")
    ax[1].errorbar(r["estimate"], y, xerr=[[r["estimate"] - r["lo"]], [r["hi"] - r["estimate"]]], fmt="s" if bold else "o", color="#1f6fb2" if bold else "#666", ms=5 if bold else 3, capsize=2)
ax[1].set_yticks([len(rows) - k for k in range(len(rows))], [s for s, _ in rows])
ax[1].axvspan(1 - M, 1 / (1 - M), color="#999", alpha=0.15, lw=0)
ax[1].axvline(1, color="#444", lw=0.7, ls="--")
ax[1].set_xlabel("floor / crater, mean normalised S (95% interval)")
ax[1].set_title(f"(b) floor against crater; band = ±{100*M:.0f}% margin")
# (c) positive control
for k, c in enumerate(T["control"]):
    ax[2].errorbar(k, c["ratio"], yerr=[[c["ratio"] - c["lo"]], [c["hi"] - c["ratio"]]], fmt="D", color="#c4561a", capsize=3)
ax[2].axhline(2 ** 0.5, color="#c4561a", lw=0.7, ls=":", label="√2")
ax[2].axhline(1, color="#444", lw=0.7, ls="--")
ax[2].axhspan(1 - M, 1 / (1 - M), color="#999", alpha=0.15, lw=0)
ax[2].set_xticks(range(len(T["control"])), [f'tier {c["tier"]}' for c in T["control"]])
ax[2].set_xlim(-0.6, len(T["control"]) - 0.4)
ax[2].set_ylim(0.8, 1.7)
ax[2].set_title("(c) control: plain / neg")
ax[2].legend(frameon=False, fontsize=7)
fig.tight_layout()
for d in (OUT, HERE):
    fig.savefig(f"{d}/fig_levels.svg")

# ---------------------------------------------------------------- run report (descriptive)
def audit(name):
    p = f"{HERE}/sessions/{name}.verify.log"
    return open(p).read().strip().splitlines()[-1] if os.path.exists(p) else "(no audit log)"
sess = sorted(os.path.basename(d) for d in glob.glob(f"{HERE}/sessions/*__*") if os.path.isdir(d))
srows = []
for s in sess:
    j = json.load(open(f"{HERE}/sessions/{s}/session.json"))
    n = sum(1 for _ in open(f"{HERE}/sessions/{s}/records.jsonl"))
    srows.append((s, j.get("session_id", "?"), j.get("spec_id", "?"), str(n), j.get("status", "?"), audit(s)))

def md_table(head, body):
    return "| " + " | ".join(head) + " |\n|" + "---|" * len(head) + "\n" + "".join("| " + " | ".join(r) + " |\n" for r in body)
def esc(t):
    return re.sub(r"([#$\[\]*_@<>`\\])", r"\\\1", t)
def typ_table(head, body):
    cells = ", ".join(f"[{esc(c)}]" for c in head) + ", " + ", ".join(f"[{esc(c)}]" for r in body for c in r)
    return f"#table(columns: {len(head)}, inset: 3pt, stroke: 0.4pt + gray, {cells})\n"

def fmt_rows(pred):
    out = []
    for scope, e in pooled.items():
        if not pred(scope):
            continue
        for r in e["rows"]:
            out.append((scope, r["comparison"], f'{r["estimate"]:.4f}', f'[{r["lo"]:.4f}, {r["hi"]:.4f}]', r["reading"]))
        if "ks" in e:
            k = e["ks"]
            out.append((scope, "KS crater vs floor", f'D = {k["d"]:.4f}', f'p = {k["p"]:.3f}', "H2 rejected" if k["p"] < 0.05 else "H2 not rejected"))
        if "raw_s" in e:
            r = e["raw_s"]
            out.append((scope, "mean raw S (floor 0.8862)", f'{r["estimate"]:.4f}', f'[{r["lo"]:.4f}, {r["hi"]:.4f}]', "descriptive"))
    return out
pooled_rows = fmt_rows(lambda s: not s.startswith("volcano"))
vol_rows = fmt_rows(lambda s: s.startswith("volcano"))
ctrl_rows = [(f'tier {c["tier"]}', str(c["curves"]), str(c["pairs"]), f'{c["ratio"]:.4f}', f'[{c["lo"]:.4f}, {c["hi"]:.4f}]', c["reading"]) for c in T["control"]]
vrows = [(v["id"], str(v["ell"]), str(v["d"]), str(v["depth"]), str(v["m"]), str(v["p"]), f'{v["r"]}', str(v["solves"]), f'{v["mean_s"]:.4f}') for v in vols]
H_P = ["scope", "comparison", "estimate", "95% interval / p", "reading"]
H_C = ["tier", "curves", "pairs", "Σ plain / Σ neg", "95% interval", "reading"]
H_V = ["id", "ℓ", "D", "depth", "m", "p", "r", "solves", "mean S"]
H_S = ["session", "session id", "spec id", "records", "status", "audit"]

md = f"# Run report {RUN}\n\nDescriptive; the interpretation is in RESULT.md. Every number comes from `tables.json` (`examples/volcano_tall_analyze.rs`).\n\n"
md += "## Sessions\n\n" + md_table(H_S, srows) + "\n## Volcanoes\n\n" + md_table(H_V, vrows)
md += "\n## Pooled\n\n" + md_table(H_P, pooled_rows) + "\n## Positive control\n\n" + md_table(H_C, ctrl_rows)
md += "\n## Per volcano\n\n" + md_table(H_P, vol_rows) + "\n![levels](fig_levels.svg)\n"
md += f"\nUnverified measured records: {len(T['unverified'])}.\n"
open(f"{OUT}/report.md", "w").write(md)
open(f"{HERE}/tables_pooled.md", "w").write(md_table(H_P, pooled_rows) + "\n" + md_table(H_C, ctrl_rows))

if "--no-pdf" not in sys.argv:
    import typst
    pre = '#set page(margin: 1.3cm, flipped: true)\n#set text(size: 7.5pt)\n#set par(justify: true)\n'
    s = pre + f"= Run report {esc(RUN)}\n\n" + esc("Descriptive; interpretation in RESULT.pdf. Numbers from tables.json.") + "\n\n"
    s += '#figure(image("fig_levels.svg", width: 100%), caption: [(a) mean S per level over the volcano mean; (b) floor/crater with 95% intervals; (c) positive control. Grey band: the pre-registered margin.])\n\n'
    s += "== Pooled\n\n" + typ_table(H_P, pooled_rows) + "\n== Positive control\n\n" + typ_table(H_C, ctrl_rows)
    s += "\n== Volcanoes\n\n" + typ_table(H_V, vrows) + "\n== Per volcano\n\n" + typ_table(H_P, vol_rows)
    s += "\n== Sessions\n\n" + typ_table(H_S, srows)
    open(f"{OUT}/report.typ", "w").write(s)
    typst.compile(f"{OUT}/report.typ", output=f"{OUT}/report.pdf")
    print("wrote", f"{OUT}/report.pdf")
    if os.path.exists(f"{HERE}/RESULT.md"):
        r = pre.replace("flipped: true", "flipped: false")
        for line in open(f"{HERE}/RESULT.md").read().splitlines():
            if line.startswith("# "):
                r += f"= {esc(line[2:])}\n"
            elif line.startswith("## "):
                r += f"== {esc(line[3:])}\n"
            elif line.startswith("|"):
                if not re.match(r"^\|[-:| ]+\|$", line):
                    r += esc(line) + " \\\n"
            elif line.startswith("!["):
                r += '#figure(image("fig_levels.svg", width: 100%))\n'
            elif line:
                r += esc(line) + "\n\n"
        open(f"{HERE}/RESULT.typ", "w").write(r)
        typst.compile(f"{HERE}/RESULT.typ", output=f"{HERE}/RESULT.pdf")
        print("wrote", f"{HERE}/RESULT.pdf")
print("wrote", f"{OUT}/report.md", f"{OUT}/fig_levels.svg")
