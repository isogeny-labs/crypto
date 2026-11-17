#!/usr/bin/env python3
"""Tabulate round 2 (out2/ under DIR) into results2.json and RESULTS2.md. Medians, min/max and
ratios only; every input number is one the binaries printed. Each table gives the A/A ratio of the
same metric next to the candidate ratio.
    analyze2.py DIR
"""
import json, re, statistics, sys
from pathlib import Path

D = Path(sys.argv[1]); o = D / "out2"
res, md = {}, []
med = statistics.median


def table(title, data, base, arms, aa, unit="ms"):
    md.append(f"\n### {title}\n")
    md.append(f"| metric | {base} | " + " | ".join(arms) + f" | A/A {aa}/{base} |")
    md.append("|---|---:|" + "---:|" * len(arms) + "---:|")
    rows = {}
    for metric, by in data.items():
        if base not in by:
            continue
        b = med(by[base])
        cells, row = [], {"base": {"median": b, "min": min(by[base]), "max": max(by[base]), "n": len(by[base])}}
        for a in arms:
            if a in by:
                m = med(by[a]); cells.append(f"{m:.4g} ({m / b:.3f})")
                row[a] = {"median": m, "min": min(by[a]), "max": max(by[a]), "n": len(by[a]), "ratio": m / b}
            else:
                cells.append("—")
        aa_r = med(by[aa]) / b if aa in by else None
        row["aa_ratio"] = aa_r
        md.append(f"| {metric} ({unit}) | {b:.4g} | " + " | ".join(cells) + (f" | {aa_r:.4f} |" if aa_r else " | — |"))
        rows[metric] = row
    return rows


md.append("## Round 2 (A100 80GB PCIe; main c5678a98ae vs this branch); cells: median (ratio to base)")
md.append("GPU: " + (o / "gpu.csv").read_text().strip().splitlines()[-1])

# FES
fes = {}
for line in (o / "fes-kernel_ms.txt").read_text().splitlines():
    w, a, r, _, v = line.split()
    fes.setdefault(w, {}).setdefault(a, []).append(float(v))
ident = (o / "fes-identity.txt").read_text()
md.append("\n### FES identity\n\n```\n" + ident.strip() + "\n```")
res["fes_identity"] = ident.splitlines()
res["fes"] = table("FES kernel time, M = main Gray walk; G = this tree Gray (A/A); B<K> = block walk",
                   fes, "M", ["B4", "B5", "B6", "B7", "B8"], "G")

# macaulay
def mac_rows(t):
    return {int(m.group(1)): float(m.group(2)) for m in re.finditer(r"^\s+(\d+)\s+([\d.]+)\s+(\d+)\s+([\d.]+)\s*$", t, re.M)}
mac = {}
for f in sorted(o.glob("mac-tp-*-*.log")):
    _, _, a, r = f.stem.split("-")
    for k, v in mac_rows(f.read_text()).items():
        mac.setdefault(f"batch {k}", {}).setdefault(a, []).append(v)
st = {f.stem: ("ok" if "selftest ok" in f.read_text() else "FAILED") for f in sorted(o.glob("mac-selftest*.log"))}
md.append("\nmacaulay selftests: " + ", ".join(f"{k} {v}" for k, v in st.items()))
res["mac_selftests"] = st
res["mac"] = table("macaulay ms per batch, M = main (shared u16, 1024 thr); D = MAC_UPDATE_2D", mac, "M", ["D1024", "D512"], "MA")

# rootfind
def rf_rows(t):
    return {f"{m.group(1)}/C{m.group(2)}": float(m.group(3)) for m in re.finditer(r"cuda (\w+)\s+chunk=(\d+)\s*: ([\d.]+) ms", t)}
st = {}
for f in sorted(o.glob("rf-selftest-*.log")):
    st[f.stem] = f.read_text().count("SELFTEST PASSED")
md.append("\nrootfind selftests passed (of 2): " + ", ".join(f"{k} {v}" for k, v in st.items()))
res["rf_selftests"] = st
for pref, label in (("rf-tp", "|F|=65536, 65536 targets"), ("rf-tpsmall", "|F|=1024, 262144 targets")):
    data = {}
    for f in sorted(o.glob(f"{pref}-*-*.log")):
        parts = f.stem.split("-")
        a, r = parts[-2], parts[-1]
        for k, v in rf_rows(f.read_text()).items():
            data.setdefault(k, {}).setdefault(a, []).append(v)
    res[pref] = table(f"rootfind {label}, launch knobs RF_BLOCK_MINBLOCKS (base R128_0 = main default)",
                      data, "R128_0", ["R64_0", "R256_0", "R128_2"], "R128_0b")
ncu = (o / "rf-ncu.txt").read_text()
md.append("\nncu: " + ("counters available" if "Speed Of Light" in ncu or "SpeedOfLight" in ncu else "not available: " + ncu.strip().splitlines()[-2:][0][:200] if ncu.strip() else "empty"))
(D / "results2.json").write_text(json.dumps(res, indent=1))
(D / "RESULTS2.md").write_text("\n".join(md) + "\n")
print("\n".join(md))
