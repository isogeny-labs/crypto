#!/usr/bin/env python3
"""Lay out the recorded values of raw/ as results.json and fig1_ptxas.svg/png.  Reads only raw/."""
import glob
import json
import re
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = sys.path[0]
RAW = f"{HERE}/raw"
out = {"exit_codes": dict(l.split()[0:2] for l in open(f"{RAW}/exit_codes.txt") if "exit=" in l)}

def jl(path):
    return [json.loads(l) for l in open(path) if l.strip().startswith("{")]

gates = {}
for name in ("host_emul", "host_emul_long"):
    rows = jl(f"{RAW}/{name}.stdout")
    gates[name] = rows
out["host_gates"] = gates
out["host_gate_summary"] = {
    n: {
        "all_pass": rows[-1]["pass"],
        "g1_mismatches": sum(r.get("mismatches", 0) for r in rows if r.get("gate") == "G1"),
        "g2_mismatches": sum(r.get("mismatches", 0) for r in rows if r.get("gate") == "G2"),
        "off_curve_total": sum(r.get("off_curve", 0) for r in rows if r.get("gate") == "G3G4"),
        "split_mismatches": sum(1 for r in rows if r.get("gate") == "G3G4" and not r["matches_reference_split"]),
        "exceptional_total": sum(r.get("exceptional", 0) for r in rows),
    }
    for n, rows in gates.items()
}

stats = {}
FILES = {"sm_80": "sm_80", "sm_86": "sm_86", "sm_89": "sm_89", "sm_90": "sm_90", "sm_120*": "sm_120_retarget"}
for a, fn in FILES.items():
    txt = open(f"{RAW}/ptxas_{fn}.txt").read()
    for m in re.finditer(r"Compiling entry function '(\S+)' for '(\S+)'\n(?:.*\n)*?ptxas info\s+: Function properties for \S+\n\s+(\d+) bytes stack frame, (\d+) bytes spill stores, (\d+) bytes spill loads\nptxas info\s+: Used (\d+) registers", txt):
        k = "prime" if "prime_kernel" in m.group(1) else "binary"
        stats.setdefault(k, {})[a] = {"stack_bytes": int(m.group(3)), "spill_stores": int(m.group(4)), "spill_loads": int(m.group(5)), "registers": int(m.group(6))}
    # the called thread function's own spill (reported as a separate "Function properties" record)
    for m in re.finditer(r"Function properties for (_ZN2gw\d+(prime|bin)_thread\S*)\n\s+(\d+) bytes stack frame, (\d+) bytes spill stores, (\d+) bytes spill loads", txt):
        k = "prime" if m.group(2) == "prime" else "binary"
        stats[k][a]["thread_fn_spill_stores"] = int(m.group(4))
        stats[k][a]["thread_fn_spill_loads"] = int(m.group(5))
out["ptxas"] = stats
out["tool_versions"] = {"ptxas": open(f"{RAW}/ptxas_version.txt").read().strip().splitlines()[-2], "clang": open(f"{RAW}/clang_version.txt").read().splitlines()[0]}
json.dump(out, open(f"{HERE}/results.json", "w"), indent=1)

plt.rcParams.update({"font.size": 9, "axes.spines.top": False, "axes.spines.right": False})
archs = list(FILES)
fig, axs = plt.subplots(1, 2, figsize=(7.4, 3.2))
w = 0.38
for ax, key, lab in ((axs[0], "registers", "registers per thread"), (axs[1], "stack_bytes", "stack (local memory) bytes per thread")):
    for i, (k, col) in enumerate((("prime", "#b5532b"), ("binary", "#6a4c93"))):
        vals = [stats[k][a][key] for a in archs]
        xs = [j + (i - 0.5) * w for j in range(len(archs))]
        ax.bar(xs, vals, w, color=col, label=f"{'ECCp-131' if k == 'prime' else 'ECC2-131'} kernel")
        for x, v in zip(xs, vals):
            ax.text(x, v * 1.02, f"{v:,}", ha="center", fontsize=6.5)
    ax.set_xticks(range(len(archs)))
    ax.set_xticklabels(archs)
    ax.set_ylabel(lab)
    ax.set_yscale("log" if key == "stack_bytes" else "linear")
axs[0].set_ylim(0, 300)
fig.legend(*axs[0].get_legend_handles_labels(), fontsize=7.5, frameon=False, loc="lower center", ncol=2)
fig.suptitle("ptxas -v on the clang-built kernels (static compile result; no GPU run; * = sm_90 PTX retargeted)", fontsize=8)
fig.tight_layout(rect=(0, 0.06, 1, 1))
fig.savefig(f"{HERE}/fig1_ptxas.svg")
fig.savefig(f"{HERE}/fig1_ptxas.png", dpi=150)
print("wrote results.json and fig1_ptxas")
