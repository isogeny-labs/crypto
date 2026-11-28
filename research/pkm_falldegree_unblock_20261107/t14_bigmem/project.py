#!/usr/bin/env python3
"""Re-derive the numbers of MEMORY.md and HOSTS.md from committed files (t14_bigmem design, 2026-11-21).

Reads only committed files: the 2026-11-17 snapshot archive of the cap-7 112Gi pods (read in memory, nothing is
extracted), the committed RPX and KUMMER logs, the t = 10 and t = 11 rows of earlier studies, and the catalog
responses saved in catalog/. Run from anywhere: python3 -I project.py. Every number it prints is derived and
unverified (no pcc claim exists).
"""
import json
import re
import tarfile
from math import comb
from pathlib import Path

HERE = Path(__file__).resolve().parent
STUDY = HERE.parent
RESEARCH = STUDY.parent
GIB = 2**30
STEP = re.compile(
    r"(?:step (\d+)|stop in step (\d+) \([^)]*\)): degree (\d+), (\d+) critical \+ (\d+) tower pairs, (\d+) S-rows \+ (\d+)"
    r" reducers \+ (\d+) promoted x (\d+) cols, nnz (\d+), residue (\d+) x (\d+), fresh (\d+) \(lowest degree (\d+)\),"
    r" basis (\d+), pairs left (\d+), ([\d.]+) ms \(rows (\d+), A (\d+), B (\d+), update (\d+)\), B' (\d+) entries,"
    r" kept (\d+) elements with (\d+) entries(?:, muladds (\d+))?, memory (\d+) MB \(peak (\d+) MB\)")
KEYS = ("deg crit tow S red pro cols nnz rres nq fresh low basis left ms rows A B upd bp kept kent mul rss peak").split()


def steps(text):
    out = []
    for n, line in enumerate(text.splitlines(), 1):
        m = STEP.search(line)
        if m:
            g = m.groups()
            d = {"line": n, "step": int(g[0] or g[1]), "stop": g[1] is not None}
            d.update({k: None if v is None else float(v) if k == "ms" else int(v) for k, v in zip(KEYS, g[2:])})
            out.append(d)
    return out


def snap_log(pod, cell):
    with tarfile.open(STUDY / f"results/SNAPSHOT_20261117/{pod}/results.tar.gz") as t:
        return t.extractfile(f"results/runs/{cell}.log").read().decode()


c7 = steps(snap_log("pfd-x-t14-k0-c7-m112-rp", "X-m2-t14-k0-c7"))
t13 = steps((STUDY / "results/RPX/pfd-x-t13-k0-c6-rp/runs/X-m2-t13-k0-c6.log").read_text())
t12 = steps((STUDY / "results/KUMMER/pfd-x-t13-k0-c6-rp/fetch-20261115T0720Z/runs/X-m2-t12-k0-c6.log").read_text())
t10 = steps((RESEARCH / "pkm_tower_round4_20260926/runs/T2-kummer-m2-p1-N20.log").read_text())[:24]
t11 = json.loads((RESEARCH / "prime_fall_degree_t14_20261101/runs/R-t11-k0-c6.jsonl").read_text().splitlines()[0])

s35 = c7[-1]
s34 = c7[-2]
assert s35["stop"] and s35["step"] == 35 and s34["step"] == 34
rows35 = s35["S"] + s35["red"] + s35["pro"]
piv35 = s35["red"] + s35["pro"]
print(f"step 35 (log line {s35['line']}): B' {s35['bp']}, nnz {s35['nnz']}, rows {rows35}, cols {s35['cols']}, "
      f"nq {s35['nq']}, piv {piv35} (= cols - nq: {s35['cols'] - s35['nq']}), B'/(piv*nq) {s35['bp'] / (piv35 * s35['nq']):.4f}")
print(f"RSS before step 35 (line {s34['line']}): {s34['rss']} MiB; peak so far {s34['peak']} MiB")

# Ratio of the largest B' at the refuting degree to the first all-pairs step at that degree (step 23).
first = {T: [s for s in tr if s["step"] == 23][0]["bp"] for T, tr in (("t10", t10), ("t12", t12), ("t13", t13))}
ratios = {
    "t10 (step 23 is the max, refutes at step 24)": max(s["bp"] for s in t10 if s["deg"] == 6) / first["t10"],
    "t11 (row max_dense_entries / t>=12 step 23, assumed equal at t=11)": t11["max_dense_entries"] / first["t12"],
    "t12": max(s["bp"] for s in t12 if s["deg"] == 6) / first["t12"],
    "t13": max(s["bp"] for s in t13 if s["deg"] == 6) / first["t13"],
    "t14 degree 6, not refuting": max(s["bp"] for s in c7 if s["deg"] == 6 and not s["stop"]) / first["t13"],
}
for k, r in ratios.items():
    print(f"ratio {k}: {r:.3f} -> degree-7 B' {r * s35['bp'] / 1e9:.2f}e9, 4*B' {4 * r * s35['bp'] / GIB:.1f} GiB")

# Memory model: peak ~ RSS before the step + 4 B' + 4 fresh nq + c0; check it on steps 27-30 of the cap-7 log.
for s in c7:
    if s["step"] in (27, 28, 29, 30):
        before = [x for x in c7 if x["step"] == s["step"] - 1][0]["rss"]
        model = before + 4 * s["bp"] / 2**20 + 4 * s["fresh"] * s["nq"] / 2**20
        print(f"memory model step {s['step']}: {model:.0f} MiB vs logged peak {s['peak']} MiB (residual {s['peak'] - model:+.0f})")


def peak_gib(bp, rss_mib, fresh, nq, c0_mib=1024):
    return (rss_mib + 4 * bp / 2**20 + 4 * fresh * nq / 2**20 + c0_mib) / 1024


nq = s35["nq"]
bands = [("L", 1.0, s34["rss"], nq / 2), ("L+", 1.0, s34["rss"], nq),
         ("C", ratios["t11 (row max_dense_entries / t>=12 step 23, assumed equal at t=11)"], 16 * 1024, nq / 2),
         ("C+", ratios["t11 (row max_dense_entries / t>=12 step 23, assumed equal at t=11)"], 24 * 1024, nq),
         ("H", ratios["t12"], 30 * 1024, nq)]
for name, r, rss, fresh in bands:
    print(f"band {name}: B' {r * s35['bp'] / 1e9:.2f}e9, peak {peak_gib(r * s35['bp'], rss, fresh, nq if name != 'H' else 1.5 * nq):.1f} GiB")

M = {d: sum(comb(28, k) for k in range(d + 1)) for d in (6, 7, 8)}
print(f"monomials of degree <= d in 28 squarefree tower variables: {M}; cols35/M7 {s35['cols'] / M[7]:.3f}; "
      f"ceiling M7^2/4 {M[7] ** 2 / 4:.3e}, M8^2/4 {M[8] ** 2 / 4:.3e}")
print(f"first all-pairs step growth: degree 5->6 {first['t13'] / [s for s in c7 if s['step'] == 12][0]['bp']:.0f}x, "
      f"6->7 {s35['bp'] / first['t13']:.1f}x")

pod = 256e9 / GIB
for head in (8, 36):
    dense = int(((pod - head) * GIB / 4) // 10**9 * 10**9)
    print(f"pod limit 256e9 B = {pod:.2f} GiB; headroom {head} GiB -> DENSE {dense} ({dense / s35['bp']:.3f} x step 35)")
D = 54_000_000_000
print(f"DENSE {D}: 4*DENSE {4 * D / GIB:.1f} GiB; with RSS 24 GiB + full echelon {4 * nq * nq / 2 / GIB:.1f} GiB + 1 GiB: "
      f"{4 * D / GIB + 24 + 4 * nq * nq / 2 / GIB + 1:.1f} GiB")
print(f"112Gi counterfactual: 4*27.9e9 = {4 * 27.9e9 / GIB:.1f} GiB + RSS {s34['rss'] / 1024:.1f} GiB = "
      f"{4 * 27.9e9 / GIB + s34['rss'] / 1024:.1f} GiB > 112 GiB")

# Time: A (B' formation) per unit (nnz/row * nq * piv) and B (S-rows) per unit (S * nnz/row * nq), steps 28-34.
nr35 = s35["nnz"] / rows35
uA, uB = nr35 * nq * piv35, s35["S"] * nr35 * nq
for s in c7:
    if 28 <= s["step"] <= 34:
        n = s["S"] + s["red"] + s["pro"]
        kA = s["A"] / 1e3 / ((s["nnz"] / n) * s["nq"] * (s["red"] + s["pro"]))
        kB = s["B"] / 1e3 / (s["S"] * (s["nnz"] / n) * s["nq"]) if s["S"] > 10000 else float("nan")
        print(f"step {s['step']}: A unit {kA:.2e} s -> step-35 A {kA * uA:.0f} s; B unit {kB:.2e} s -> step-35 B {kB * uB:.0f} s")
start35 = sum(s["ms"] for s in c7[:-1]) / 1e3
print(f"step 35 starts about {start35:.0f} s after the F4 start (k0 host); its symbolic phase took {s35['ms'] / 1e3:.0f} s")

cat = json.loads((HERE / "catalog/cpus_20261121T2045Z.json").read_text())
for c in cat["cpus"]:
    gb = c["ramGbPerVcpu"] * c["vcpu"]["max"]
    if gb >= 256:
        h = c["price"]["securePerVcpu"] * c["vcpu"]["max"]
        print(f"{c['id']} {c['vcpu']['max']} vCPU {gb} GB: US${h:.2f}/h, 30 h US${30 * h:.2f}, ledger 465.8424 + 30 h = "
              f"US${465.8424 + 30 * h:.2f}")
