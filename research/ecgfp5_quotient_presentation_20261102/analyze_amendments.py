#!/usr/bin/env python3
"""Frozen analysis of amendments A1-A4 (PROTOCOL.md §9). Reads results/ and the merged bw-study
analysis.json only. Writes analysis_amendments.json, analysis_amendments.md and
figures/amendments.{svg,png}. The decision rules applied are A1-A4's, verbatim:
  - F4 stage improvement of the u ring: U1/U1F F4 <= A0 F4 / 1.5 at every prime;
  - whole-solve improvement at a prime: c <= bw baseline / 1.5, every gate passed;
  - pipeline improvement (ratchet): a whole-solve improvement at all three primes;
  - cap stops and failed gates are not_measured, never extrapolated;
  - A4 (post-restart A/A) changes no rule; it gives drift-corrected ratios beside the measured ones.
"""
import json
import math
import re
from pathlib import Path

HERE = Path(__file__).resolve().parent
RES = HERE / "results"
BW = json.loads((HERE.parent / "ecgfp5_quotient_bw_finish_20261102" / "analysis.json").read_text())
BASE = {c["p"]: c for c in BW["cells"]}
PRIMES = [4111, 262231, 33554371]
BAR = 1.5
CLOCK = 2.8e9
CLOSE, PROMOTE = 46.0, 38.0
F4_RE = re.compile(r"^overall\(cpu\)\s+([\d.]+)", re.M)
HCPU_RE = re.compile(r"\[hnfsolve(?:_u)?\] (?:parse_gb|staircase|dump_P): [\d.]+ s wall, ([\d.]+) s cpu")


def lg(cpu):
    return math.log2(cpu * CLOCK)


def g1(c):
    """The bw study's G1 labelling (its analyze.py g1(), verbatim logic): point and [c/4, 4c] band."""
    lo, hi = lg(c / 4), lg(4 * c)
    point = "CLOSE" if lg(c) >= CLOSE else ("PROMOTE" if lg(c) <= PROMOTE else "not_measured")
    if lo >= CLOSE:
        band = "CLOSE"
    elif hi <= PROMOTE:
        band = "PROMOTE"
    elif lo > PROMOTE and hi < CLOSE:
        band = "not_measured"
    else:
        band = "straddling"
    return point, band


def kv(path):
    return dict(l.split("=", 1) for l in path.read_text().split()) if path.exists() else {}


def run_rec(d):
    r = json.loads((d / "msolve.run.json").read_text())
    capped = bool(r["killed_by_rss_cap"] or r["killed_by_wall_cap"])
    return r, capped


def f4_cell(d):
    """F4-only or whole-solve cell: msolve F4 CPU (TIMINGS overall(cpu)) when measured."""
    if not (d / "msolve.run.json").exists():
        return {"dir": str(d.relative_to(HERE)), "measured": False, "reason": "absent"}
    r, capped = run_rec(d)
    log = (d / "msolve.log").read_text()
    m = F4_RE.search(log)
    ok = r["exit_code"] == 0 and not capped and m is not None
    out = {"dir": str(d.relative_to(HERE)), "measured": ok, "cap_stop": capped,
           "msolve_cpu_s": round(r["user_seconds"] + r["sys_seconds"], 1),
           "msolve_maxrss_gib": round(r["maxrss_kib"] / 2**20, 2),
           "f4_cpu_s": float(m.group(1)) if ok else None}
    if not ok:
        out["reason"] = "rss cap stop" if r["killed_by_rss_cap"] else ("wall cap stop" if r["killed_by_wall_cap"] else "no TIMINGS / nonzero exit")
    gj = d / "gen.json"
    if gj.exists():
        g = json.loads(re.search(r"\{.*\}", gj.read_text(), re.S).group(0))
        out["generator_gate"] = (g.get("uring_fresh_decompositions") == "24/24" and g.get("uring_zero_on_R_plus_N") == "0/8"
                                 and g.get("uring_zero_on_random") == "0/8" and g["systems"]["uring"]["planted_satisfies"] is True)
    return out


def bw_cell(d, gen_gate=True):
    """Whole solve through hnfsolve(_u) + bwfinish(_u)."""
    out = f4_cell(d)
    ex = kv(d / "exit_codes.txt")
    out["exit_codes"] = ex
    dl = (d / "dump.log").read_text() if (d / "dump.log").exists() else ""
    hcpu = sum(float(x) for x in HCPU_RE.findall(dl))
    st = re.search(r"stab_fail=(\d+)", dl)
    out["stab_fail"] = int(st.group(1)) if st else None
    ur = re.search(r"unit_rows=(\d+) dense_rows=(\d+)", dl)
    if ur:
        out["unit_rows"], out["dense_rows"] = int(ur.group(1)), int(ur.group(2))
    bwp = d / "bw.json"
    if not (bwp.exists() and ex.get("bwfinish_exit") == "0" and out["measured"]):
        out.update({"c_cpu_s": None, "whole": "not_measured"})
        return out
    bw = json.loads(bwp.read_text())
    cp = d / "check_points.json"
    pts_ok = json.loads(cp.read_text())["all_satisfy"] if cp.exists() else None
    gates = (gen_gate and out.get("generator_gate", True) and out["stab_fail"] == 0 and bw["elim_degree"] == 65536
             and bw["D"] == 65536 and bw["planted"] == "recovered" and pts_ok is not False
             and all(v == "0" for v in ex.values()))
    c = out["msolve_cpu_s"] + hcpu + bw["total_cpu_s"]
    ph = {k: v["cpu_s"] for k, v in bw["phases"].items()}
    out.update({"hnfsolve_cpu_s": round(hcpu, 1), "finish_cpu_s": bw["total_cpu_s"], "finish_phases_cpu_s": ph,
                "finish_maxrss_gib": round(bw["maxrss_kib"] / 2**20, 2), "limbs": bw["limbs"], "t": bw["t"],
                "D": bw["D"], "elim_degree": bw["elim_degree"], "rational_roots": bw["rational_roots"],
                "planted": bw["planted"], "points_satisfy": pts_ok, "gates": bool(gates), "c_cpu_s": round(c, 1),
                "log2_c_cyc": round(lg(c), 3), "band_log2": [round(lg(c / 4), 2), round(lg(4 * c), 2)]})
    return out


cells = {}
# A0 and U1 F4 at every prime (frozen arms at 4111, A1 replication at the others, A4 post-restart)
f4 = {"A0": {4111: f4_cell(RES / "A0")}, "U1": {4111: f4_cell(RES / "U1")}}
for p in PRIMES[1:]:
    f4["A0"][p] = f4_cell(RES / "A1" / f"A0_{p}")
    f4["U1"][p] = f4_cell(RES / "A1" / f"U1F_{p}")
aa = {"A0_post_restart": f4_cell(RES / "A1" / "A0_4111"), "U1_post_restart": f4_cell(RES / "A1" / "U1F_4111")}
f4_rows = []
for p in PRIMES:
    a, u = f4["A0"][p], f4["U1"][p]
    gate = u.get("generator_gate", True) if p != 4111 else True
    r = (a["f4_cpu_s"] / u["f4_cpu_s"]) if (a["f4_cpu_s"] and u["f4_cpu_s"]) else None
    f4_rows.append({"p": p, "A0_f4_cpu_s": a["f4_cpu_s"], "U1_f4_cpu_s": u["f4_cpu_s"], "ratio": round(r, 3) if r else None,
                    "generator_gate": gate, "f4_improvement": bool(r and gate and r >= BAR)})
f4_stage = all(r["f4_improvement"] for r in f4_rows)

# other F4-only / whole-solve arms at 4111
others = {"U2_4111 (s5 := w^2, F4 only)": f4_cell(RES / "A1" / "U2_4111"),
          "U3_4111 (sheared w, bw finish)": bw_cell(RES / "A1" / "U3_4111"),
          "U4_4111 (s5 eliminating, bw finish)": bw_cell(RES / "A2" / "U4_4111")}
u1p = RES / "A1" / "U1P_4111"
if (u1p / "msolve.run.json").exists():
    o = f4_cell(u1p)
    ck = json.loads((u1p / "check.json").read_text()) if (u1p / "check.json").exists() else {}
    o["check"] = ck
    ok = o["measured"] and ck.get("elim_degree") == 65536 and ck.get("planted_root_of_eliminant") and ck.get("planted_coordinates_recovered")
    o["c_cpu_s"] = o["msolve_cpu_s"] if ok else None
    o["gates"] = bool(ok)
    others["U1P_4111 (u ring, msolve's own finish)"] = o
else:
    others["U1P_4111 (u ring, msolve's own finish)"] = {"measured": False, "reason": "not run yet"}

# A3: U5 whole solve at three primes
u5 = []
drift_aa = None
a0r = aa["A0_post_restart"].get("f4_cpu_s")
if a0r:
    drift_aa = f4["A0"][4111]["f4_cpu_s"] / a0r   # > 1: post-restart host is faster
for p in PRIMES:
    d = RES / "A3" / f"U5_{p}"
    o = bw_cell(d) if d.exists() else {"c_cpu_s": None, "whole": "absent"}
    base = BASE[p]["c_new_cpu_s"]
    o["p"] = p
    o["baseline_c_cpu_s"] = base
    o["bar_cpu_s"] = round(base / BAR, 1)
    if o.get("c_cpu_s"):
        o["speedup_vs_baseline"] = round(base / o["c_cpu_s"], 3)
        o["improvement"] = bool(o["gates"] and o["c_cpu_s"] <= base / BAR)
        o["distance_to_promote_bits"] = round(o["log2_c_cyc"] - PROMOTE, 2)
        o["g1_point"], o["g1_band"] = g1(o["c_cpu_s"])
        if drift_aa:
            o["speedup_drift_corrected"] = round(base / (o["c_cpu_s"] * drift_aa), 3)
    else:
        o["improvement"] = False
    u5.append(o)
pipeline = all(o["improvement"] for o in u5)

out = {"unit": "CPU-s; c_cyc = CPU-s x 2.8e9", "bar": BAR, "f4_rows": f4_rows, "f4_stage_improvement_all_primes": f4_stage,
       "post_restart_aa": aa, "drift_factor_A0_pre_over_post": round(drift_aa, 4) if drift_aa else None,
       "other_arms_4111": others, "u5": u5, "pipeline_improvement": pipeline}
(HERE / "analysis_amendments.json").write_text(json.dumps(out, indent=1, default=str) + "\n")

fmt = lambda x, d=0: "–" if x is None else f"{x:,.{d}f}"
L = ["# Amendments A1–A4: analysis (generated by analyze_amendments.py; do not edit)", "",
     "## F4 CPU-s, A0 (W0 system) against the u ring (U1/U1F)", "",
     "| p | A0 | U1 | A0 / U1 | gates | F4 improvement (≥ 1.5×) |", "|---|---:|---:|---:|---|---|"]
for r in f4_rows:
    L.append(f"| {r['p']} | {fmt(r['A0_f4_cpu_s'])} | {fmt(r['U1_f4_cpu_s'])} | {fmt(r['ratio'], 2)}× | {r['generator_gate']} | {r['f4_improvement']} |")
L += ["", f"F4 stage improvement at all three primes: **{f4_stage}**", "",
      "## Other arms at p = 4111", "", "| arm | F4 CPU-s | c CPU-s | gates / status |", "|---|---:|---:|---|"]
for k, o in others.items():
    if not o.get("measured"):
        st = f"not_measured ({o.get('reason')}; {fmt(o.get('msolve_cpu_s'))} CPU-s at stop)" if o.get("msolve_cpu_s") else f"not_measured ({o.get('reason')})"
    else:
        st = f"gates {o['gates']}" if "gates" in o else "measured (F4 only)"
    L.append(f"| {k} | {fmt(o.get('f4_cpu_s'))} | {fmt(o.get('c_cpu_s'))} | {st} |")
L += ["", "## U5: u ring, whole solve (msolve F4 + hnfsolve_u + bwfinish_u)", "",
      "| p | msolve | hnfsolve_u | finish | c | baseline | bar | baseline / c | drift-corrected | log2 c_cyc | band | gates | improvement |",
      "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|---|"]
for o in u5:
    L.append(f"| {o['p']} | {fmt(o.get('msolve_cpu_s'))} | {fmt(o.get('hnfsolve_cpu_s'))} | {fmt(o.get('finish_cpu_s'))} | **{fmt(o.get('c_cpu_s'))}** | "
             f"{fmt(o['baseline_c_cpu_s'])} | {fmt(o['bar_cpu_s'])} | **{fmt(o.get('speedup_vs_baseline'), 2)}×** | "
             f"{fmt(o.get('speedup_drift_corrected'), 2)}× | {o.get('log2_c_cyc', '–')} | {o.get('band_log2', '–')} point {o.get('g1_point', '–')}, band {o.get('g1_band', '–')} | {o.get('gates')} | {o['improvement']} |")
L += ["", f"Post-restart A/A: A0 F4 {fmt(aa["A0_post_restart"].get("f4_cpu_s"))} (pre-restart {fmt(f4['A0'][4111]['f4_cpu_s'])}), "
      f"U1 F4 {fmt(aa["U1_post_restart"].get("f4_cpu_s"))} (pre-restart {fmt(f4['U1'][4111]['f4_cpu_s'])}); drift factor {out['drift_factor_A0_pre_over_post']}.",
      "", f"**Pipeline improvement (whole-solve improvement at all three primes, every gate passed): {pipeline}**"]
(HERE / "analysis_amendments.md").write_text("\n".join(L) + "\n")

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

FIG = HERE / "figures"
FIG.mkdir(exist_ok=True)
fig, axs = plt.subplots(1, 3, figsize=(15, 4.8))
x = range(len(PRIMES))
lab = [str(p) for p in PRIMES]
ax = axs[0]
w = 0.38
ax.bar([i - w / 2 for i in x], [r["A0_f4_cpu_s"] or 0 for r in f4_rows], w, color="#777777", label="A0 (W0 system)")
ax.bar([i + w / 2 for i in x], [r["U1_f4_cpu_s"] or 0 for r in f4_rows], w, color="#e6550d", label="U1 (u ring, w last)")
for i, r in enumerate(f4_rows):
    if r["ratio"]:
        ax.text(i, max(r["A0_f4_cpu_s"], r["U1_f4_cpu_s"]) * 1.02, f"{r['ratio']:.2f}×", ha="center", fontsize=9)
ax.set_xticks(list(x), lab)
ax.set_xlabel("p")
ax.set_ylabel("msolve F4 CPU-s")
ax.set_title("(a) F4 by presentation", fontsize=10)
ax.legend(fontsize=8, frameon=False)
ax = axs[1]
cols = {"F4 + basis print (msolve)": "#3182bd", "dump (hnfsolve)": "#9ecae1", "bw sequence": "#e6550d", "generator + det + roots + points": "#fdae6b"}
for j, (who, xs) in enumerate([("bw baseline", [i - w / 2 for i in x]), ("U5", [i + w / 2 for i in x])]):
    bottoms = [0.0] * 3
    for i, p in enumerate(PRIMES):
        if who == "bw baseline":
            b = BASE[p]
            ph = b["finish_phases_cpu_s"]
            parts = [b["msolve_cpu_s"], b["hnfsolve_cpu_s"], ph.get("sequence", 0), b["finish_cpu_s"] - ph.get("sequence", 0)]
        else:
            o = u5[i]
            if not o.get("c_cpu_s"):
                continue
            ph = o["finish_phases_cpu_s"]
            parts = [o["msolve_cpu_s"], o["hnfsolve_cpu_s"], ph.get("sequence", 0), o["finish_cpu_s"] - ph.get("sequence", 0)]
        for k, (name, col) in enumerate(cols.items()):
            ax.bar(xs[i], parts[k], w, bottom=bottoms[i], color=col, edgecolor="white", linewidth=0.5,
                   label=name if (j == 0 and i == 0) else None, hatch="//" if who == "U5" else None)
            bottoms[i] += parts[k]
        ax.text(xs[i], bottoms[i] * 1.01, f"{bottoms[i]:,.0f}", ha="center", fontsize=7)
for i, p in enumerate(PRIMES):
    ax.plot([i - 0.45, i + 0.45], [BASE[p]["c_new_cpu_s"] / BAR] * 2, "k--", lw=1)
ax.set_xticks(list(x), lab)
ax.set_xlabel("p   (left: bw baseline; right, hatched: U5; dashed: bar = baseline / 1.5)")
ax.set_ylabel("whole-solve CPU-s")
ax.set_title("(b) whole-solve c by phase", fontsize=10)
ax.legend(fontsize=7, frameon=False, loc="upper left")
ax = axs[2]
for i, p in enumerate(PRIMES):
    b = BASE[p]
    ax.errorbar(i - 0.12, b["log2_c_cyc"], yerr=[[2], [2]], fmt="o", color="#777777", capsize=3, label="bw baseline" if i == 0 else None)
    o = u5[i]
    if o.get("c_cpu_s"):
        ax.errorbar(i + 0.12, o["log2_c_cyc"], yerr=[[2], [2]], fmt="s", color="#e6550d", capsize=3, label="U5" if i == 0 else None)
ax.axhline(CLOSE, color="k", ls="--", lw=1)
ax.text(2.45, CLOSE + 0.1, "CLOSE 2^46", ha="right", fontsize=8)
ax.axhline(PROMOTE, color="#31a354", ls="--", lw=1)
ax.text(2.45, PROMOTE + 0.1, "PROMOTE 2^38", ha="right", fontsize=8, color="#31a354")
ax.set_xticks(list(x), lab)
ax.set_xlabel("p   (bars: frozen [c/4, 4c] band)")
ax.set_ylabel("log2 c_cyc")
ax.set_title("(c) G1 position", fontsize=10)
ax.legend(fontsize=8, frameon=False, loc="center right")
fig.suptitle("EcGFp5 m = 5 quotient: u-ring presentation (A1–A4)", fontsize=11)
fig.tight_layout()
fig.savefig(FIG / "amendments.svg")
fig.savefig(FIG / "amendments.png", dpi=150)
print((HERE / "analysis_amendments.md").read_text())
