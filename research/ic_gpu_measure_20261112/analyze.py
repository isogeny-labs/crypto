#!/usr/bin/env python3
"""Tabulate the pod outputs (out/, out_rf/, out_mac/ extracted under DIR) into results.json and
RESULTS.md. Computes medians, minima and ratios only; every input number is a value the binaries
printed. A ratio is reported against the A/A ratio of the same metric (same arm built twice).
    analyze.py DIR
"""
import json, re, statistics, sys
from pathlib import Path

D = Path(sys.argv[1])
res = {"cards": {}, "rootfind_variants": {}, "macaulay_variants": {}}
md = []


def med(xs):
    return statistics.median(xs) if xs else None


def stat(xs):
    return {"n": len(xs), "median": med(xs), "min": min(xs) if xs else None, "max": max(xs) if xs else None}


def mac_rows(text):
    """batch -> ms from a macaulay `throughput` log."""
    out = {}
    for m in re.finditer(r"^\s+(\d+)\s+([\d.]+)\s+(\d+)\s+([\d.]+)\s*$", text, re.M):
        out[int(m.group(1))] = float(m.group(2))
    return out


def rf_rows(text):
    """(lookup, chunk) -> ms from a rootfind `throughput` log."""
    out = {}
    for m in re.finditer(r"cuda (\w+)\s+chunk=(\d+)\s*: ([\d.]+) ms", text):
        out[f"{m.group(1)}/C{m.group(2)}"] = float(m.group(3))
    return out


def ratio_table(title, metric_unit, arms_by_metric, base, cand, aa=None, lower_better=True):
    md.append(f"\n### {title}\n")
    hdr = f"| metric | {base} median | {cand} median | {cand}/{base} |"
    sep = "|---|---:|---:|---:|"
    if aa:
        hdr += f" A/A ({aa}/{base}) |"
        sep += "---:|"
    md.append(hdr)
    md.append(sep)
    rows = {}
    for metric, arms in arms_by_metric.items():
        b, c = med(arms.get(base, [])), med(arms.get(cand, []))
        if b is None or c is None:
            continue
        line = f"| {metric} ({metric_unit}) | {b:.4g} | {c:.4g} | {c / b:.4g} |"
        row = {"base": stat(arms[base]), "cand": stat(arms[cand]), "ratio_cand_over_base": c / b}
        if aa and arms.get(aa):
            a = med(arms[aa])
            line += f" {a / b:.4f} |"
            row["aa"] = stat(arms[aa])
            row["ratio_aa_over_base"] = a / b
        md.append(line)
        rows[metric] = row
    return rows


# ---------- cards (out/) ----------
o = D / "out"
if o.exists():
    md.append("## Cards S/F/M/R (base e9d5dbdccd = A, A2 second base build; cand = B)\n")
    md.append("GPU: " + (o / "gpu.csv").read_text().strip().splitlines()[-1])
    km = {}
    for line in (o / "timing/kernel_ms.txt").read_text().splitlines():
        w, arm, rnd, _, v = line.split()
        km.setdefault(w, {}).setdefault(arm, []).append(float(v))
    aa = {w: {k: v for k, v in a.items() if k in ("A", "A2")} for w, a in km.items()}
    # A appears in both phases; the A/B comparison uses the A runs of the A/B phase (rounds after A/A).
    ab = {}
    lines = (o / "timing/kernel_ms.txt").read_text().splitlines()
    for w in km:
        seq = [l.split() for l in lines if l.split()[0] == w]
        a_runs = [float(x[4]) for x in seq if x[1] == "A"]
        n_aa = len([x for x in seq if x[1] == "A2"])
        ab[w] = {"A": a_runs[n_aa:], "B": [float(x[4]) for x in seq if x[1] == "B"],
                 "A2": [float(x[4]) for x in seq if x[1] == "A2"]}
        aa[w]["A"] = a_runs[:n_aa]
    res["cards"]["kernel_ms_ab"] = ratio_table("Worker kernels (launch to sync, ms)", "ms", ab, "A", "B")
    res["cards"]["kernel_ms_aa"] = ratio_table("A/A control (base vs second base build)", "ms", aa, "A", "A2")
    for name, parse, key in (("macaulay", mac_rows, "batch"), ("rootfind", rf_rows, "cfg")):
        arms = {}
        for f in sorted((o / "timing").glob(f"{name}-*-*.log")):
            _, arm, rnd = f.stem.split("-")
            for k, v in parse(f.read_text()).items():
                arms.setdefault(f"{key} {k}", {}).setdefault(f"{arm}#{rnd}", v)
        # job.sh wrote <name>-A-<r>.log in both phases, so the A/A-phase A logs were overwritten:
        # A here is the A/B phase, and the A/A control is A2 (A/A phase) against it (same code).
        flat = {m: {} for m in arms}
        for m, d in arms.items():
            for k, v in d.items():
                flat[m].setdefault(k.split("#")[0], []).append(v)
        if flat:
            res["cards"][name] = ratio_table(f"{name} throughput (kernel ms, lower is better)", "ms", flat, "A", "B", aa="A2")
    ident = o / "identity/digests.txt"
    if ident.exists():
        md.append("\n### Worker output digests (sorted solution lists, sha256)\n\n```\n" + ident.read_text().strip() + "\n```")
        res["cards"]["digests"] = ident.read_text().splitlines()

# ---------- rootfind variants (out_rf/) ----------
o = D / "out_rf"
if o.exists():
    md.append("\n## Rootfind sweep variants (B = base, R0 = main, R0b = second R0 build, R1/R2 fused)\n")
    st = {}
    for f in sorted(o.glob("selftest*-*.log")) + sorted(o.glob("p256-*.log")):
        t = f.read_text()
        st[f.stem] = "PASSED" if "SELFTEST PASSED" in t else ("FAILED" if "FAIL" in t else "missing")
    res["rootfind_variants"]["selftests"] = st
    md.append("Selftests: " + ", ".join(f"{k} {v}" for k, v in sorted(st.items())))
    for pref, label in (("tp", "|F|=65536, 65536 targets"), ("tpsmall", "|F|=1024, 262144 targets")):
        arms = {}
        for f in sorted(o.glob(f"{pref}-*-*.log")):
            _, arm, rnd = f.stem.split("-")
            for k, v in rf_rows(f.read_text()).items():
                arms.setdefault(k, {}).setdefault(arm, []).append(v)
        for cand in ("R0", "R1", "R2"):
            res["rootfind_variants"][f"{pref}:{cand}"] = ratio_table(f"rootfind {label}: {cand} vs B (ms)", "ms", arms, "B", cand, aa=None)
        res["rootfind_variants"][f"{pref}:aa"] = ratio_table(f"rootfind {label}: A/A R0b vs R0 (ms)", "ms", arms, "R0", "R0b")
    for a in ("B", "R0", "R1", "R2"):
        f = o / f"build-{a}.log"
        if f.exists():
            regs = re.findall(r"Compiling entry function '(\S+)'.*?\n.*?(\d+) bytes stack frame, (\d+) bytes spill stores, (\d+) bytes spill loads\n.*?Used (\d+) registers", f.read_text(), re.S)
            res["rootfind_variants"][f"ptxas:{a}"] = [{"kernel": k, "stack": int(s), "spill_st": int(ss), "spill_ld": int(sl), "regs": int(r)} for k, s, ss, sl, r in regs]

# ---------- macaulay variants (out_mac/) ----------
o = D / "out_mac"
if o.exists():
    md.append("\n## Macaulay shared-resident u16 (G = main binary, G2 = new binary global path, S* = shared path)\n")
    st = {f.stem: ("ok" if "selftest ok" in f.read_text() else "FAILED") for f in sorted(o.glob("selftest*.log"))}
    res["macaulay_variants"]["selftests"] = st
    md.append("Selftests: " + ", ".join(f"{k} {v}" for k, v in sorted(st.items())))
    arms = {}
    for f in sorted(o.glob("tp-*-*.log")):
        _, arm, rnd = f.stem.split("-")
        for k, v in mac_rows(f.read_text()).items():
            arms.setdefault(f"batch {k}", {}).setdefault(arm, []).append(v)
    for cand in ("S256", "S512", "S1024"):
        res["macaulay_variants"][cand] = ratio_table(f"macaulay {cand} vs G (ms per batch)", "ms", arms, "G", cand, aa="G2")

(D / "results.json").write_text(json.dumps(res, indent=1))
(D / "RESULTS.md").write_text("\n".join(md) + "\n")
print("\n".join(md))
