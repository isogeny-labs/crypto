#!/usr/bin/env python3
"""Lay out the recorded values of raw/ as results.json and the parity graphs.

Reads only files under raw/ (and the audit JSON of the 2026-10-22 baseline study for the counted
expected work).  Derived quantities (ratios between recorded medians, core-years) are products of
recorded numbers and are written to results.json beside the formula that produced them.
"""
import glob
import json
import re
import statistics as st
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = sys.path[0]
RAW = f"{HERE}/raw"
AUDIT = f"{HERE}/../certicom131_rho_ic_bench_20261022/raw"
SEC_PER_YEAR = 365.25 * 86400
ROUNDS = range(1, 8)

out = {}

# ---- isolation audit ----
iso = []
for f in sorted(glob.glob(f"{RAW}/iso/*.jsonl")):
    for line in open(f):
        r = json.loads(line)["run"]
        iso.append((f.split("/")[-1].replace(".jsonl", ""), r["contended"], r["exit_status"]))
out["isolation"] = {
    "records": len(iso),
    "contended": [n for n, c, _ in iso if c],
    "nonzero_exit": [n for n, _, e in iso if e],
    "refused_by_gate": [l.split()[0] for l in open(f"{RAW}/exit_codes.txt") if "exit=1" in l],
}

def jl(path):
    rows = []
    for line in open(path):
        line = line.strip()
        if line.startswith("{"):
            rows.append(json.loads(line))
    return rows

def k_rate(path):
    m = re.findall(r"finished: ([0-9.]+) M it/s", open(path).read())
    return float(m[-1]) * 1e6

contended = set(out["isolation"]["contended"])
def ok(name):
    return name not in contended

samples = {k: [] for k in ("k130_a", "k130_b", "old_prime", "old_clmul", "new_prime64", "new_clmul3", "pv131", "bs131")}
for r in ROUNDS:
    if ok(f"k130_a_r{r}"): samples["k130_a"].append(k_rate(f"{RAW}/k130_a_r{r}.stdout"))
    if ok(f"k130_b_r{r}"): samples["k130_b"].append(k_rate(f"{RAW}/k130_b_r{r}.stdout"))
    if ok(f"old_prime_r{r}"):
        for x in jl(f"{RAW}/old_prime_r{r}.stdout"):
            if x["kind"] == "step" and x.get("curve") == "ECCp-131" and x["batch"] == 128:
                samples["old_prime"].append(x["steps_per_s_median"])
    if ok(f"old_clmul_r{r}"):
        for x in jl(f"{RAW}/old_clmul_r{r}.stdout"):
            if x["kind"] == "step" and x.get("curve") == "ECC2-131/clmul" and x["batch"] == 128 and x["steps"] == 500:
                samples["old_clmul"].append(x["steps_per_s_median"])
    if ok(f"new_prime64_r{r}"):
        for x in jl(f"{RAW}/new_prime64_r{r}.stdout"):
            if x["kind"] == "step" and x["batch"] == 256 and x["chains"] == 8:
                samples["new_prime64"].append(x["steps_per_s_median"])
    if ok(f"new_clmul3_r{r}"):
        for x in jl(f"{RAW}/new_clmul3_r{r}.stdout"):
            if x["kind"] == "step" and x["batch"] == 512 and x["chains"] == 8:
                samples["new_clmul3"].append(x["steps_per_s_median"])
    if ok(f"pv131_r{r}"):
        for x in jl(f"{RAW}/pv131_r{r}.stdout"):
            if x["kind"] == "step":
                samples["pv131"].append(x["steps_per_s_median"])
    if ok(f"bs131_r{r}"):
        for x in jl(f"{RAW}/bs131_r{r}.stdout"):
            if x["kind"] == "step":
                samples["bs131"].append(x["steps_per_s_median"])

arms = {}
for k, v in samples.items():
    arms[k] = {"samples": v, "n": len(v), "median": st.median(v), "min": min(v), "max": max(v)}
out["arms_steps_per_s"] = arms
kref = st.median(samples["k130_a"] + samples["k130_b"])
out["derived"] = {
    "k130_reference_median_both_positions": kref,
    "aa_ratio_k130_b_over_a": arms["k130_b"]["median"] / arms["k130_a"]["median"],
    "ratio_to_k130": {k: arms[k]["median"] / kref for k in arms},
    "speedup_vs_old_same_session": {
        "ECCp-131 mont64 / old 32-bit": arms["new_prime64"]["median"] / arms["old_prime"]["median"],
        "ECCp-131 avx512x8 / old 32-bit": arms["pv131"]["median"] / arms["old_prime"]["median"],
        "ECC2-131 clmul3 / old clmul": arms["new_clmul3"]["median"] / arms["old_clmul"]["median"],
        "ECC2-131 bitsliced / old clmul": arms["bs131"]["median"] / arms["old_clmul"]["median"],
    },
    "formula": "ratio = median(arm samples) / median(ECC2K-130 client samples, both positions); speedup = median(new) / median(old)",
}

# ---- gates ----
gates = []
for g in ("gate_walk131", "gate_bs131", "gate_pv131"):
    for x in jl(f"{RAW}/{g}.stdout"):
        if x["kind"] in ("gate", "setup", "exceptional", "profile"):
            gates.append({"source": g, **x})
out["gates"] = gates

# ---- expected work (counted, from the baseline study's audit of the exact group orders) ----
audit = {}
for c in ("ECCp-131", "ECC2K-130", "ECC2-131"):
    audit[c] = json.load(open(f"{AUDIT}/audit_{c}.json"))["audit"]["rho_log2_expected"]
out["expected_log2_steps"] = audit
proj = {
    "ECC2K-130 client": ("ECC2K-130", kref),
    "ECC2-131 old clmul": ("ECC2-131", arms["old_clmul"]["median"]),
    "ECC2-131 bitsliced": ("ECC2-131", arms["bs131"]["median"]),
    "ECCp-131 old 32-bit": ("ECCp-131", arms["old_prime"]["median"]),
    "ECCp-131 avx512x8": ("ECCp-131", arms["pv131"]["median"]),
}
out["derived"]["core_years"] = {
    k: {"log2_expected_steps": audit[c], "rate": r, "core_years": 2 ** audit[c] / r / SEC_PER_YEAR}
    for k, (c, r) in proj.items()
}
out["derived"]["core_years_formula"] = "2^log2_expected_steps / rate / (365.25*86400)"
json.dump(out, open(f"{HERE}/results.json", "w"), indent=1)

# ================= graphs =================
plt.rcParams.update({"font.size": 9, "axes.spines.top": False, "axes.spines.right": False})
C = {"ECCp-131": "#b5532b", "ECC2K-130": "#1f6f8b", "ECC2-131": "#6a4c93"}
order = [
    ("old_prime", "ECCp-131\nold 32-bit\nbatch 128", "ECCp-131"),
    ("new_prime64", "ECCp-131\n3x64 Montgomery\nbatch 256", "ECCp-131"),
    ("pv131", "ECCp-131\nAVX-512 x8\n64 slots", "ECCp-131"),
    ("old_clmul", "ECC2-131\nold clmul\nbatch 128", "ECC2-131"),
    ("new_clmul3", "ECC2-131\nclmul, 3 products\nbatch 512", "ECC2-131"),
    ("bs131", "ECC2-131\nbitsliced 512\n16 slots", "ECC2-131"),
]
fig, ax = plt.subplots(figsize=(7.4, 3.9))
for i, (k, lab, c) in enumerate(order):
    a = arms[k]
    hatch = "//" if k.startswith("old") else ""
    b = ax.bar(i, a["median"] / 1e6, color=C[c], hatch=hatch, edgecolor="white")
    ax.errorbar(i, a["median"] / 1e6, yerr=[[(a["median"] - a["min"]) / 1e6], [(a["max"] - a["median"]) / 1e6]], color="#222", capsize=3, lw=1)
    ax.text(i, a["max"] / 1e6 + 0.35, f"{a['median']/1e6:.2f}", ha="center", fontsize=7.5)
ax.axhline(kref / 1e6, color=C["ECC2K-130"], ls="--", lw=1.4, label=f"ECC2K-130 client, median {kref/1e6:.2f} M/s (A/A {out['derived']['aa_ratio_k130_b_over_a']:.3f})")
ax.set_xticks(range(len(order)))
ax.set_xticklabels([o[1] for o in order], fontsize=7)
ax.set_ylabel("rho steps per second, one pinned core (M/s)")
ax.set_title("Step rate on the exact 131-bit curves: baseline arms (hatched) and parity arms; bars = median, whiskers = min to max of 7 rounds", fontsize=8)
ax.legend(fontsize=7.5, frameon=False, loc="upper left")
fig.tight_layout()
fig.savefig(f"{HERE}/fig1_step_rates.svg")
fig.savefig(f"{HERE}/fig1_step_rates.png", dpi=150)
plt.close(fig)

fig, ax = plt.subplots(figsize=(6.6, 3.5))
names = list(proj)
vals = [out["derived"]["core_years"][n]["core_years"] for n in names]
cols = [C[proj[n][0]] for n in names]
bars = ax.bar(range(len(names)), vals, color=cols)
for b, n in zip(bars, names):
    if "old" in n:
        b.set_hatch("//")
        b.set_edgecolor("white")
for i, v in enumerate(vals):
    ax.text(i, v * 1.15, f"{v:,.0f}", ha="center", fontsize=7.5)
ax.set_yscale("log")
ax.set_xticks(range(len(names)))
ax.set_xticklabels([n.replace(" ", "\n", 1) for n in names], fontsize=7)
ax.set_ylabel("core-years to expected rho work (derived)")
ax.set_title("Expected rho work / measured step rate (this host class, 1 core)", fontsize=8.5)
fig.tight_layout()
fig.savefig(f"{HERE}/fig2_core_years.svg")
fig.savefig(f"{HERE}/fig2_core_years.png", dpi=150)
plt.close(fig)
print("wrote results.json and 2 graphs")
