"""Lay out one `ic sweep run` store as a run report (report.md) and a graph.

Layout only (AGENTS.md, "Implementation language: no Python"; research-visuals
skill): every number below is read from the store, none is computed here
beyond counting records.  Usage:

    python3 make_report.py STORE.jsonl OUT_DIR "command that produced it" HOST_NOTE
"""
import collections
import json
import pathlib
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

store, out, command, host = sys.argv[1], pathlib.Path(sys.argv[2]), sys.argv[3], sys.argv[4]
out.mkdir(parents=True, exist_ok=True)
lines = [json.loads(l) for l in open(store) if l.strip()]
header = lines[0]
baselines = [l for l in lines if l["kind"] == "baseline"]
trials = [l for l in lines if l["kind"] == "trial"]
usable = [b["scale"] for b in baselines if b["record"]["class"] == "ok"]


def ratio_pts(t):
    return [(r["log2_r"], r["log2_ratio"]) for r in t["rungs"] if r.get("log2_ratio") is not None and r.get("log2_r")]


fig, (a, b) = plt.subplots(1, 2, figsize=(11, 4.4), gridspec_kw={"width_ratios": [3, 2]})
colour = {"complete": "#2ca02c", "pruned": "#7f7f7f", "failed": "#d62728"}
seen = set()
for t in trials:
    pts = ratio_pts(t)
    if len(pts) >= 1:
        label = t["state"] if t["state"] not in seen else None
        seen.add(t["state"])
        a.plot(*zip(*pts), "o-", color=colour[t["state"]], alpha=0.6, lw=1, ms=3, label=label)
a.axhline(0, color="black", lw=1, ls="--", label="frozen baseline (ratio 1)")
a.set_xlabel("log2 r of the rung")
a.set_ylabel("log2 of worst-case S ratio to the baseline over the band")
a.set_title("A. Each trial's measured ratio per rung")
a.legend(fontsize=7)
a.grid(alpha=0.3)

reasons = collections.Counter()
for t in trials:
    if t["state"] == "failed":
        last = t["rungs"][-1] if t["rungs"] else {}
        d = (last.get("detail") or t.get("note") or "")[:34]
        reasons[f"failed: {last.get('class', '?')}: {d}"] += 1
    else:
        reasons[t["state"]] += 1
labels, counts = zip(*reasons.most_common())
b.barh(range(len(labels)), counts, color="#1f77b4")
b.set_yticks(range(len(labels)))
b.set_yticklabels(labels, fontsize=6)
b.invert_yaxis()
b.set_xlabel("trials")
b.set_title("B. How the trials ended")
fig.suptitle(f"{header['space']} on {header['instance']}: {len(trials)} trials, rungs {header['rungs']}", fontsize=9)
fig.tight_layout()
fig.savefig(out / "ratios_by_rung.svg")
fig.savefig(out / "ratios_by_rung.png", dpi=150)

md = []
md.append(f"# Run report: `{header['space']}` on {header['instance']}\n")
md.append("Observations only. A smoke run of the sweeper on a small space; it is not a result about index calculus.\n")
md.append("## What ran\n")
md.append(f"- Command: `{command}`")
commit = header.get("git_commit") or "not recorded (the build could not read it from a git worktree; see the Host line)"
md.append(f"- Commit: {commit}; binary BLAKE3 `{header.get('binary_blake3')}`")
md.append(f"- Host: {host}. Isolation level: L0 (shared host, no CPU pinning, 3 trials at a time). Counts are unaffected; wall time is not used by the score.")
md.append(f"- Track: **{header['track']}** ({'counted' if header['counted'] else 'reference only'}). Rungs requested: {header['rungs']}; usable (baseline scored): {usable}.")
md.append(f"- Target for the projection: log2 r = {header['target_log2_r']}; `min_fit` {header['min_fit']}; repeats {header['repeats']}; seed {header['seed']}; counted-trial cap {header['max_trials']}.")
md.append("- Raw records: [`store.jsonl`](store.jsonl) (every baseline and trial line, as written).\n")
md.append("## Baseline (frozen before the search)\n")
md.append("| rung n | class | S(w=0) | S(w=0.1) | S(w=1) | detail |\n|--:|:--|--:|--:|--:|:--|")
for bl in baselines:
    r = bl["record"]
    s = r["band"]["s"] if r.get("band") else [None] * 3
    f = lambda x: "—" if x is None else f"{x:.4g}"
    md.append(f"| {bl['scale']} | {r['class']} | {f(s[0])} | {f(s[1])} | {f(s[2])} | {(r['detail'] or '')[:60]} |")
md.append("\n## Trials\n")
md.append("Ratio columns are log2 of the worst-case (over w) `S_trial / S_baseline`; negative would be better than the baseline.\n")
md.append("| id | state | solver | m | dim_frac | " + " | ".join(f"n={s}" for s in header["rungs"]) + " | objective | note |")
md.append("|--:|:--|:--|:--|--:|" + "--:|" * len(header["rungs"]) + "--:|:--|")
for t in trials:
    by = {r["scale"]: r for r in t["rungs"]}
    cells = []
    for s in header["rungs"]:
        r = by.get(s)
        cells.append("" if r is None else ("—" if r.get("log2_ratio") is None else f"{r['log2_ratio']:+.2f}"))
    obj = "—" if t["objective"] is None else f"{t['objective']:+.2f}{'' if t['projected'] else ' (last rung)'}"
    a_ = t["assignment"]
    md.append(f"| {t['id']} | {t['state']} | {a_.get('solver')} | {a_.get('m')} | {a_.get('dim_frac', 0):.2f} | " + " | ".join(cells) + f" | {obj} | {(t['note'] or '')[:70]} |")
md.append("\n## How the trials ended\n")
md.append("| outcome | trials |\n|:--|--:|")
for k, v in reasons.most_common():
    md.append(f"| {k} | {v} |")
md.append("\n![ratios by rung](ratios_by_rung.svg)\n")
md.append("*Figure: recorded values from `store.jsonl`; no value is computed in the plot. Left: each trial's log2 worst-case ratio at the rungs it completed. Right: trial outcomes.*\n")
md.append("## Limits\n")
md.append("- Few trials on a small space; this says nothing about which configuration is best.")
md.append("- Rungs below `min_fit` are not projected: the objective is then the last rung's measured ratio.")
md.append("- K_0 at n = 15 has a larger cofactor than the cofactor-4 rungs, so the rungs mix cofactors.")
md.append("- Solver rows in a different unit than the baseline's are recorded and not rankable; see the failed rows' notes.")
(out / "report.md").write_text("\n".join(md) + "\n")
print("wrote", out)
