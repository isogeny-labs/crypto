#!/usr/bin/env python3
"""Lay out the recorded values of raw/ as results.json and three SVG graphs.

Reads only files under raw/.  The few derived quantities (projections of the
expected rho work onto a measured step rate, iteration-count ratios) are
products of recorded numbers; each is written to results.json beside the
formula that produced it.  Nothing is re-measured here.
"""
import glob
import json
import math
import re
import statistics as st
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = sys.path[0]
RAW = f"{HERE}/raw"
SEC_PER_YEAR = 365.25 * 86400
PAPER_LOG2 = 60.9  # Bernstein et al., "Breaking ECC2K-130", iteration function penalty included

out = {"unit_notes": {}, "derived": {}}

# ---- exact expected work (counted, from the audit of the exact group orders) ----
audit = {}
for c in ("ECCp-131", "ECC2K-130", "ECC2-131"):
    a = json.load(open(f"{RAW}/audit_{c}.json"))["audit"]
    audit[c] = {k: a[k] for k in ("order_bits", "rho_class_size", "rho_log2_expected", "rho_log2_expected_unfolded")}
out["audit"] = audit
base = audit["ECC2K-130"]["rho_log2_expected"]
out["derived"]["iteration_ratio_vs_ECC2K-130"] = {
    c: {
        "formula": "2^(rho_log2_expected[c] - rho_log2_expected[ECC2K-130])",
        "audit_formula_base": 2 ** (audit[c]["rho_log2_expected"] - base),
        "vs_paper_2^60.9": 2 ** (audit[c]["rho_log2_expected"] - PAPER_LOG2),
    }
    for c in audit
}

# ---- isolation audit ----
iso = []
for f in sorted(glob.glob(f"{RAW}/iso/*.jsonl")):
    for line in open(f):
        r = json.loads(line)
        iso.append((f.split("/")[-1], r["run"]["contended"], r["run"]["exit_status"]))
out["isolation"] = {
    "records": len(iso),
    "contended": sum(1 for _, c, _ in iso if c),
    "nonzero_exit": [n for n, _, e in iso if e],
}
out["isolation"]["contended_records"] = [n for n, c, _ in iso if c]
out["isolation"]["refused_by_gate"] = [l.split()[0] for l in open(f"{RAW}/exit_codes.txt") if "exit=1" in l]

CONTENDED = {n.replace(".jsonl", "") for n, c, _ in iso if c}  # never pooled with uncontended runs

# ---- generic Rust arithmetic: affine additions per second ----
generic = {}
for c, name in (("eccp-131", "ECCp-131"), ("ecc2k-130", "ECC2K-130"), ("ecc2-131", "ECC2-131")):
    v = [json.load(open(f))["additions_per_second"] for f in sorted(glob.glob(f"{RAW}/nist_bench_{c}_r*.stdout"))
         if f.split("/")[-1].replace(".stdout", "") not in CONTENDED]
    generic[name] = {"samples": v, "median": st.median(v), "min": min(v), "max": max(v)}
out["generic_rust_adds_per_s"] = generic

# ---- production ECC2K-130 CPU client ----
k = []
for f in sorted(glob.glob(f"{RAW}/k130cpu_1t_r*.stdout")):
    if f.split("/")[-1].replace(".stdout", "") in CONTENDED:
        continue
    m = re.findall(r"finished: ([0-9.]+) M it/s", open(f).read())
    if m:
        k.append(float(m[-1]) * 1e6)
out["ecc2k130_cpu_client_1thread_it_per_s"] = {"samples": k, "median": st.median(k), "min": min(k), "max": max(k)}
m4 = re.findall(r"finished: ([0-9.]+) M it/s", open(f"{RAW}/k130cpu_4t_unisolated.stdout").read())
out["ecc2k130_cpu_client_4threads_unisolated_it_per_s"] = float(m4[-1]) * 1e6

# ---- native certicom131 batched-affine walk ----
def parse_bench(path, curve_name):
    rows = {"field": None, "step": {}, "gates": []}
    for line in open(path):
        j = json.loads(line)
        if j.get("curve") != curve_name:
            continue
        if j["kind"] == "field":
            rows["field"] = j
        elif j["kind"] == "gate":
            rows["gates"].append(j)
        elif j["kind"] == "step" and j["steps"] > 1:
            rows["step"][j["batch"]] = j
    return rows

arms = {
    "ECCp-131 (32-bit Montgomery, header)": parse_bench(f"{RAW}/bench131_prime.stdout", "ECCp-131"),
    "ECC2-131 (header shift-xor mul)": parse_bench(f"{RAW}/bench131_binary_header.stdout", "ECC2-131"),
    "ECC2-131 (pclmulqdq field)": parse_bench(f"{RAW}/bench131_binary_clmul.stdout", "ECC2-131/clmul"),
}
out["native_walk"] = arms
out["derived"]["batching_gain_batch1_over_batch128"] = {
    n: a["step"][1]["ns_per_step_median"] / a["step"][128]["ns_per_step_median"] for n, a in arms.items()
}

# ---- projections of the expected rho work (derived) ----
proj = []
def add(curve, arm, rate, source):
    lg = audit[curve]["rho_log2_expected"]
    secs = 2 ** lg / rate
    proj.append(
        {
            "curve": curve,
            "arm": arm,
            "rate_per_s": rate,
            "log2_expected_iterations": lg,
            "core_years": secs / SEC_PER_YEAR,
            "formula": "2^log2_expected_iterations / rate / (365.25*86400)",
            "rate_source": source,
        }
    )
add("ECC2K-130", "production CPU client (Frobenius orbit, bitsliced)", out["ecc2k130_cpu_client_1thread_it_per_s"]["median"], "k130cpu_1t_r*")
add("ECC2K-130", "generic Rust arithmetic", generic["ECC2K-130"]["median"], "nist_bench_ecc2k-130_r*")
add("ECC2-131", "pclmulqdq batched affine (B=128)", 1e9 / arms["ECC2-131 (pclmulqdq field)"]["step"][128]["ns_per_step_median"], "bench131_binary_clmul")
add("ECC2-131", "generic Rust arithmetic", generic["ECC2-131"]["median"], "nist_bench_ecc2-131_r*")
add("ECCp-131", "32-bit Montgomery batched affine (B=128)", 1e9 / arms["ECCp-131 (32-bit Montgomery, header)"]["step"][128]["ns_per_step_median"], "bench131_prime")
add("ECCp-131", "generic Rust arithmetic", generic["ECCp-131"]["median"], "nist_bench_eccp-131_r*")
out["derived"]["projections"] = proj

# repo-recorded GPU rates (not measured here)
gpu = {}
for name, rate, src in (
    ("RTX PRO 4500 (g7.2xlarge)", 12.499203e9, "ecc2k130/README.md, G7 x-only doubling and bridge campaign"),
    ("RTX PRO 6000", 17.090e9, "ecc2k130/RTX-PRO6000.md, measured 17.090, 17.100, 17.089 B/s"),
):
    gpu[name] = {
        "recorded_rate_per_s": rate,
        "source": src + "; recorded in the repository, not measured in this study",
        "gpu_years_at_audit_iterations": 2 ** audit["ECC2K-130"]["rho_log2_expected"] / rate / SEC_PER_YEAR,
        "gpu_years_at_paper_2^60.9": 2 ** PAPER_LOG2 / rate / SEC_PER_YEAR,
    }
out["derived"]["ecc2k130_gpu_years_from_recorded_rates"] = gpu

# ---- ECCp-131 exact-curve relation probe ----
probe = {}
for f in sorted(glob.glob(f"{RAW}/ic_eccp131_*_r*.stdout")):
    arm = re.sub(r"_r\d+\.stdout$", "", f.split("/")[-1])
    p = json.load(open(f))["ic_probe"]
    e = probe.setdefault(arm, {"elapsed_ms": [], "natural_relations": set(), "rank": set(), "pair_sums": p.get("distinct_pair_sum_abscissae"), "B": p["factor_base_abscissae"], "arity": p["relation_arity"]})
    e["elapsed_ms"].append(p["elapsed_ms"])
    e["natural_relations"].add(len(p["relations"]))
    e["rank"].add(p["combined_rank"])
for e in probe.values():
    e["natural_relations"] = sorted(e["natural_relations"])
    e["rank"] = sorted(e["rank"])
    e["median_ms"] = st.median(e["elapsed_ms"])
out["eccp131_exact_probe"] = probe
n_minus_1_log2 = audit["ECCp-131"]["order_bits"]  # 2^130.17 per problem record; use exact n below
N = 0x048E1D43F293469E317F7ED728F6B8E6F1
for e in probe.values():
    B = e["B"]
    cover = (2 * B * B) if e["arity"] == 2 else (8 * B**3)
    e["support_bound_log2_hit_probability"] = math.log2(cover) - math.log2(N - 1)
    e["support_bound_formula"] = "log2(2B^2 or 8B^3) - log2(n-1)"

# ---- IC ladders (ic boundary, seed 131) ----
d = json.load(open(f"{RAW}/ic_boundary_default.json"))
ladders = {"prime": [], "char2": [], "koblitz": []}
fixed = {}
for inst in d["ledger"]["instances"]:
    reg = inst["regime"]
    rho = inst["rho_s_mean"]
    vs = [(v["name"], v["s"], v["verified"]) for v in inst["variants"] if v.get("s") is not None]
    best = min(vs, key=lambda t: t[1])
    ladders[reg].append({"log2_r": inst["log2_r"], "rho_s": rho, "best_variant": best[0], "best_s": best[1], "ratio_best_to_rho": best[1] / rho, "verified": best[2], "curve": inst["curve"].get("slug") if isinstance(inst["curve"], dict) else None})
    for name, s, ok in vs:
        fixed.setdefault((reg, name), []).append((inst["log2_r"], s / rho, ok))
for reg in ladders:
    ladders[reg].sort(key=lambda r: r["log2_r"])
out["ic_ladders"] = ladders
out["ic_fits_total"] = [
    {k: f[k] for k in ("regime", "variant", "alpha", "r_squared", "points")} for f in d["ledger"]["fits"] if f["phase"] == "total"
]
out["ic_all_verified"] = d["all_verified"]
json.dump(out, open(f"{HERE}/results.json", "w"), indent=1, default=lambda o: sorted(o) if isinstance(o, set) else str(o))

# ================= graphs =================
plt.rcParams.update({"font.size": 9, "axes.spines.top": False, "axes.spines.right": False})
C = {"ECCp-131": "#b5532b", "ECC2K-130": "#1f6f8b", "ECC2-131": "#6a4c93"}

# Fig 1: ns per rho step against batch size, native arms; plus ECC2K-130 client as a reference line
fig, ax = plt.subplots(figsize=(6.4, 3.8))
for name, a in arms.items():
    bs = sorted(a["step"])
    col = C["ECCp-131"] if name.startswith("ECCp") else C["ECC2-131"]
    ls = "--" if "header shift" in name else "-"
    ax.errorbar(bs, [a["step"][b]["ns_per_step_median"] for b in bs],
                yerr=[[a["step"][b]["ns_per_step_median"] - a["step"][b]["ns_per_step_min"] for b in bs],
                      [a["step"][b]["ns_per_step_max"] - a["step"][b]["ns_per_step_median"] for b in bs]],
                marker="o", ms=4, lw=1.4, ls=ls, color=col, label=name, capsize=2)
ax.axhline(1e9 / out["ecc2k130_cpu_client_1thread_it_per_s"]["median"], color=C["ECC2K-130"], lw=1.2, ls=":",
           label="ECC2K-130 production client (Frobenius orbit)")
ax.set_xscale("log", base=2)
ax.set_yscale("log")
ax.set_xticks([1, 8, 32, 128])
ax.set_xticklabels(["1", "8", "32", "128"])
ax.set_xlabel("inversion batch size (lanes sharing one inversion)")
ax.set_ylabel("ns per rho step, one pinned core")
ax.set_title("Rho step cost on the exact 131-bit curves (median, min-max bars)", fontsize=9)
ax.legend(fontsize=7, frameon=False)
fig.tight_layout()
fig.savefig(f"{HERE}/fig1_rho_step.svg")
fig.savefig(f"{HERE}/fig1_rho_step.png", dpi=150)
plt.close(fig)

# Fig 2: single-core-years to the expected work
fig, ax = plt.subplots(figsize=(6.4, 3.6))
short = {"production CPU client": "bitsliced\nclient", "generic Rust arithmetic": "generic\nRust", "pclmulqdq batched affine": "pclmul\nbatch 128", "32-bit Montgomery batched affine": "32-bit Mont.\nbatch 128"}
labels = [f"{p['curve']}\n{short[p['arm'].split(' (')[0]]}" for p in proj]
vals = [p["core_years"] for p in proj]
cols = [C[p["curve"]] for p in proj]
hatch = ["//" if "generic" in p["arm"] else "" for p in proj]
bars = ax.bar(range(len(proj)), vals, color=cols)
for b, h in zip(bars, hatch):
    b.set_hatch(h)
    b.set_edgecolor("white")
ax.set_yscale("log")
ax.set_xticks(range(len(proj)))
ax.set_xticklabels(labels, fontsize=6.5)
for i, v in enumerate(vals):
    ax.text(i, v * 1.15, f"{v:,.0f}", ha="center", fontsize=7)
ax.set_ylabel("core-years to expected rho work (derived)")
ax.set_title("Expected rho work / measured step rate (this host, 1 core)", fontsize=9)
fig.tight_layout()
fig.savefig(f"{HERE}/fig2_projection.svg")
fig.savefig(f"{HERE}/fig2_projection.png", dpi=150)
plt.close(fig)

# Fig 3: IC S / rho S against log2 r, per regime (best variant per instance, chosen after the fact)
fig, axs = plt.subplots(1, 3, figsize=(8.4, 3.2), sharey=True)
for ax, (reg, nm) in zip(axs, (("prime", "ECCp-131 family (prime)"), ("char2", "ECC2-131 family (random binary)"), ("koblitz", "ECC2K-130 family (Koblitz)"))):
    L = ladders[reg]
    ax.plot([r["log2_r"] for r in L], [r["ratio_best_to_rho"] for r in L], "o", ms=4.5, color="#333")
    ax.axhline(1, color="#b00", lw=1)
    ax.set_yscale("log")
    ax.set_xlabel("log2 of subgroup order r")
    ax.set_title(nm, fontsize=8)
    ax.set_xlim(8, 42)
    ax.text(41, 0.3, "131-bit target is\n90+ bits to the right", fontsize=6, ha="right", color="#555")
axs[0].set_ylabel("S(index calculus) / S(matched rho)")
axs[0].set_ylim(0.2, 300)
fig.suptitle("Best IC variant per instance (post-hoc minimum) against the matched rho; red line = parity", fontsize=8.5)
fig.tight_layout()
fig.savefig(f"{HERE}/fig3_ic_vs_rho.svg")
fig.savefig(f"{HERE}/fig3_ic_vs_rho.png", dpi=150)
plt.close(fig)
print("wrote results.json and 3 graphs")
