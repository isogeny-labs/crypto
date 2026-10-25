#!/usr/bin/env python3
"""Render recorded native summaries only; performs no research statistics."""
import json
import os
from pathlib import Path
import subprocess
import sys

os.environ.setdefault("MPLCONFIGDIR", "/tmp/conductor-gap-matplotlib")
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parent
REPO = ROOT.parents[1]
load = lambda p: json.loads(p.read_text())

def save(fig, path):
    fig.savefig(str(path) + ".svg", bbox_inches="tight")
    fig.savefig(str(path) + ".png", dpi=150, bbox_inches="tight")
    fig.savefig(str(path) + ".pdf", bbox_inches="tight")
    plt.close(fig)

def pdf(md):
    subprocess.run(["pandoc", str(md), "--pdf-engine=pdflatex", "-V", "geometry:margin=0.7in",
                    "-V", "fontsize=10pt", "-o", str(md.with_suffix(".pdf"))],
                   cwd=md.parent, check=True, stdout=subprocess.DEVNULL)

def run_reports():
    for source in sorted((ROOT / "analysis").glob("diagnostics-*.json")):
        for row in load(source)["sessions"]:
            session = REPO / row["session"]
            meta = load(session / "session.json")
            name = row["name"]
            dest = ROOT / "reports" / name
            dest.mkdir(parents=True, exist_ok=True)
            band = row["band"]
            fig, ax = plt.subplots(figsize=(8, 3.3))
            for arm in sorted({x["arm"] for x in band["rows"]}):
                points = [x for x in band["rows"] if x["arm"] == arm]
                valid = [(i, x["mean_s"][0]) for i, x in enumerate(points) if x["mean_s"][0] is not None]
                if valid:
                    ax.scatter([x[0] for x in valid], [x[1] for x in valid], s=12, label=arm)
            ax.set(xlabel="Curve ordinal within arm (full IDs in source JSON)",
                   ylabel="Mean cold S, native price 0 (cost floor)", title=name)
            ax.legend(fontsize=8)
            ax.grid(alpha=.2)
            save(fig, dest / "recorded-costs")
            text = f"# Recorded run: {name}\n\n"
            text += f"Session `{meta['session_id']}`; spec `{meta['spec_id']}`. "
            text += f"Source commit `{meta['git_commit']}`. Public synthetic, independent one-target solves.\n\n"
            text += f"Requested {meta['executions_planned']}; recorded {meta['records_written']}. "
            text += f"Statuses: `{meta['status_counts']}`. Required isolation L0; wall time is descriptive only.\n\n"
            text += "| Arm | Attempts | Verified | Charged GAE sum (lower bound) | Peak RSS KiB |\n|---|---:|---:|---:|---:|\n"
            for arm in row["resources"]["arms"]:
                text += f"| {arm['arm']} | {arm['measured']} | {arm['verified']} | {arm['total_gae_sum']} | {arm['peak_rss_kib']} |\n"
            text += "\n![Native recorded means; no timing or speed ranking.](recorded-costs.pdf)\n\n"
            text += f"Raw records: `{row['session']}/records.jsonl`; spec and seeds: that session's `spec.json`. "
            text += f"Host: `host.json`; exact replay: `{row['session']}.audit.json`. "
            text += f"All per-curve 0/0.1/1 GAE bands, native counters, resource and failure totals: `{source.relative_to(ROOT)}`. "
            text += "The plotted means are native ecbench summaries of verified records. Missing prices remain unknown. "
            text += "These observations alone admit no full-cost method ranking. Calibration sessions are excluded from primary inference.\n"
            (dest / "REPORT.md").write_text(text)
            pdf(dest / "REPORT.md")

def campaign():
    a = load(ROOT / "analysis/panel-A.json")
    fig, ax = plt.subplots(figsize=(8.2, 4.5))
    ax.axhspan(.95, 1.05, color="#dcebdd", alpha=.6, label="Preregistered +/-5% band")
    ax.axhline(1, color="#333333", lw=.8)
    for row in a["levels"]:
        x = row["level"]
        ax.vlines(x, *row["ci98"], color="#a7b2c6", lw=7)
        ax.vlines(x, *row["ci90"], color="#254f87", lw=3)
        ax.scatter([x], [row["ratio"]], c="#254f87", zorder=4)
    ax.set(xticks=range(5), xlabel="Depth in each 2-isogeny volcano", ylabel="S2 / S1, equal-class geometric mean",
           title="Eight classes; three S2 volcanoes per class; 192 solves per level/volcano")
    ax.text(.01, .02, "Dark: 90% CI. Light: 98% CI. Whole-class bootstrap.\n30,720 primary solves; controls excluded.", transform=ax.transAxes, fontsize=9)
    ax.grid(axis="y", alpha=.2)
    save(fig, ROOT / "figures/level-ratios")
    b = load(ROOT / "analysis/panel-B.json")
    fig, axes = plt.subplots(1, 2, figsize=(10, 4), sharey=True)
    for ax, ci in zip(axes, [0, 2]):
        for arm in ["rho-neg", "ic-scalar", "ic-recurrence", "ic-velu"]:
            cells = [x for x in b["cells"] if x["class"] == ci and x["model"] == 0 and x["arm"] == arm]
            ax.plot([x["volcano"] for x in cells], [x["mean_s"] for x in cells], "o-", label=arm)
        ax.set(xticks=range(4), xlabel="Crater (0), three sampled floors (1-3)", title=f"B class {ci}; model 0; 16 targets/cell")
        ax.grid(alpha=.2)
    axes[0].set_ylabel("Mean cold S at native price 0 (cost floor)")
    axes[1].legend(fontsize=8)
    save(fig, ROOT / "figures/cm-cost-floors")

def volcanoes():
    sys.path.insert(0, str(REPO / "tools/volcano-diagram"))
    from volcano_diagram import render
    identities = {tuple(x["params"][k] for k in ["p", "a", "b"]): x
                  for x in load(ROOT / "prepared-a/identities.json")}
    diagnostics = {x["name"]: x for x in load(ROOT / "analysis/diagnostics-A.json")["sessions"]}
    for ci in [0, 7]:
        means = {}
        for level in range(5):
            for row in diagnostics[f"a-c{ci}-level{level}"]["band"]["rows"]:
                means[row["curve"]] = row["mean_s"][0]
        for vi in [0, 1]:
            source = ROOT / f"prepared-a/a-class{ci}/volcano{vi}/curves.jsonl"
            vertices = []
            names = []
            for row in [json.loads(x) for x in source.read_text().splitlines()]:
                if row["model"] != 0:
                    continue
                identity = identities[tuple(str(row[k]) for k in ["p", "a", "b"])]
                names.append({"vertex": row["vertex"], "slug": identity["slug"],
                              "ec1": identity["representations"][0]["ec1"]})
                vertices.append({"id": row["vertex"], "label": f"v{row['vertex']}", "level": row["level"],
                                 "nbrs": row["nbrs"], "values": {"s": means[identity["slug"]]}})
            spec = {"schema": "volcano-diagram/v2", "ell": 2, "depth": 4,
                    "title": f"Balanced rho sampling: class {ci}, volcano {vi}",
                    "subtitle": f"conductor-gap-replication-20261025; a-c{ci}-level0..4; model 0",
                    "footnote": "192/64/32/16/8 targets per vertex at depths 0..4. Cost floor; native prices omitted.",
                    "metrics": [{"key": "s", "title": "Mean cold S, recorded group-addition cost", "label": "GAE / sqrt(r)"}],
                    "vertices": vertices}
            stem = ROOT / f"figures/volcano-c{ci}-v{vi}"
            stem.with_suffix(".diagram.json").write_text(json.dumps(spec, indent=2) + "\n")
            stem.with_suffix(".identities.json").write_text(json.dumps(names, indent=2) + "\n")
            render(spec, str(stem))

if __name__ == "__main__":
    (ROOT / "figures").mkdir(exist_ok=True)
    if sys.argv[1:] == ["runs"]:
        run_reports()
    elif sys.argv[1:] == ["campaign"]:
        campaign()
    elif sys.argv[1:] == ["volcanoes"]:
        volcanoes()
    else:
        raise SystemExit("runs | campaign")
