#!/usr/bin/env python3
"""Tabulate out_lk/ (job.sh) into RESULTS.md and results.json: per workload and (lookup, chunk), the
median kernel ms of each arm over the rounds, its ratio to main (M), the A/A ratio (M2/M), and per
workload the best configuration of each arm against main's best. Values are the bench's own printed
kernel times; medians and ratios are derived here.
    analyze.py DIR
"""
import json, re, statistics, sys
from pathlib import Path

D = Path(sys.argv[1]); o = D / "out_lk"
ARMS = ["M", "M2", "E", "H", "P", "EP", "HP", "EHP"]
NAMES = {"M": "main", "M2": "main (A/A)", "E": "Eytzinger", "H": "packed hash", "P": "paired",
         "EP": "Eytzinger + paired", "HP": "packed + paired", "EHP": "all three"}
med = statistics.median
md, res = [], {}
md.append("GPU: " + next((l.split(":", 1)[1].strip() for l in (o / "nvidia-smi-q.txt").read_text().splitlines() if "Product Name" in l), "?"))
st = {a: (o / f"selftest-{a}.log").read_text().count("SELFTEST PASSED") for a in ARMS if (o / f"selftest-{a}.log").exists()}
md.append("Selftests passed (of 3): " + ", ".join(f"{a} {n}" for a, n in st.items()))
res["selftests"] = st
for pref, label in (("tp", "|F| = 65536, 65536 targets"), ("tpsmall", "|F| = 1024, 262144 targets")):
    data = {}
    for f in o.glob(f"{pref}-*-*.log"):
        _, a, r = f.stem.rsplit("-", 2)
        for m in re.finditer(r"cuda (\w+)\s+chunk=(\d+)\s*: ([\d.]+) ms", f.read_text()):
            data.setdefault(f"{m.group(1)}/C{m.group(2)}", {}).setdefault(a, []).append(float(m.group(3)))
    arms = [a for a in ARMS if any(a in v for v in data.values())]
    md.append(f"\n## {label}\n")
    md.append("| config | " + " | ".join(NAMES[a] for a in arms) + " |")
    md.append("|---|" + "---:|" * len(arms))
    out = {}
    for cfg in sorted(data):
        base = med(data[cfg]["M"])
        cells = []
        for a in arms:
            v = data[cfg].get(a)
            if not v:
                cells.append("—"); continue
            m = med(v)
            cells.append(f"{m:.2f}" if a == "M" else f"{m:.2f} ({m / base:.3f})")
        md.append(f"| {cfg} | " + " | ".join(cells) + " |")
        out[cfg] = {a: {"median": med(v), "min": min(v), "max": max(v), "n": len(v)} for a, v in data[cfg].items()}
    best = {}
    for a in arms:
        cfgs = {c: med(v[a]) for c, v in data.items() if a in v}
        c = min(cfgs, key=cfgs.get)
        best[a] = (c, cfgs[c])
    mb = best["M"][1]
    md.append("\nBest configuration per arm (the comparison that counts):\n")
    for a in arms:
        c, v = best[a]
        md.append(f"- {NAMES[a]}: {c} {v:.2f} ms ({mb / v:.3f}× main's best)")
    aa = max(max(med(v["M2"]) / med(v["M"]), med(v["M"]) / med(v["M2"])) for v in data.values() if "M2" in v)
    spread = max(max(v[a]) / min(v[a]) for v in data.values() for a in v)
    md.append(f"\nA/A: largest |M2/M − 1| over configurations {aa - 1:.4f}; largest max/min of one arm's rounds {spread:.4f}.")
    res[pref] = {"configs": out, "best": {a: {"config": c, "ms": v} for a, (c, v) in best.items()}, "aa_max": aa, "round_spread_max": spread}
(D / "results.json").write_text(json.dumps(res, indent=1))
(D / "RESULTS.md").write_text("\n".join(md) + "\n")
print("\n".join(md))
