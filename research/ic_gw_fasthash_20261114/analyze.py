#!/usr/bin/env python3
"""Scores PROTOCOL.md. usage: analyze.py RESULTS_DIR   (RESULTS_DIR/units/p{pass}_{arm}_{q}_{seed}.jsonl)
Writes analysis.json and tables.md next to this file. Every number is [pcc:unverified]."""
import json, math, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
res = sys.argv[1]
QS = [262111, 524287, 1048507, 2097091, 4194271]
SEEDS = (1, 2, 3)
PASSES = (1, 2)
PH = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"]
AA_Q = (2097091, 4194271)
gm = lambda v: math.exp(sum(map(math.log, v)) / len(v))


def load(p, arm, q, s):
    fn = os.path.join(res, "units", f"p{p}_{arm}_{q}_{s}.jsonl")
    if not os.path.exists(fn):
        return None
    return [json.loads(l) for l in open(fn) if l.strip()]


def ic(u, y=None):
    return [r for r in u if r.get("rec") == "ic" and (y is None or r["arm"] == y)]


def counted(r):
    return {k: v for k, v in r.items() if not (k.endswith("wall") or k.endswith("cpu")) and k != "load1" and not k.startswith("s.controls")}


def cpu(u, phases, y="DL"):
    return sum(r.get(p + ".cpu", 0.0) for r in ic(u, y) for p in phases)


def rss(u):
    return [r["vmhwm_kb"] for r in u if r.get("rec") == "rss"][0]


# ---- gate: counted identity (full-line compare of every ic record, in order) ----
gate = {"units_compared": 0, "records_compared": 0, "mismatches": [], "missing": [], "verify_failures": [], "retried": [], "rc_nonzero": []}
for q in QS:
    for s in SEEDS:
        base = None
        for p in PASSES:
            for arm in ("REF", "HASH", "COMBO", "REF2"):
                u = load(p, arm, q, s)
                if u is None:
                    if arm != "REF2" or (s == 1 and q in AA_Q):
                        gate["missing"].append(f"p{p}_{arm}_{q}_{s}")
                    continue
                un = [r for r in u if r.get("rec") == "unit"][0]
                if un["rc"] != 0 or [r for r in u if r.get("rec") == "rss"][0]["rc"] != 0:
                    gate["rc_nonzero"].append(f"p{p}_{arm}_{q}_{s}")
                recs = [counted(r) for r in ic(u)]
                for r in ic(u):
                    if not r["ok"] or r["k_found"] != r["k"]:
                        gate["verify_failures"].append(f"p{p}_{arm}_{q}_{s}/{r['arm']}")
                if len(ic(u, "DL")) > 1:
                    gate["retried"].append(f"p{p}_{arm}_{q}_{s}")
                if base is None:
                    base = recs
                    continue
                gate["units_compared"] += 1
                gate["records_compared"] += len(recs)
                if recs != base:
                    d = [k for a, b in zip(recs, base) for k in set(a) | set(b) if a.get(k) != b.get(k)]
                    gate["mismatches"].append(f"p{p}_{arm}_{q}_{s}: {len(recs)} vs {len(base)} records, keys {sorted(set(d))[:6]}")
gate["pass"] = not (gate["mismatches"] or gate["missing"] or gate["verify_failures"] or gate["rc_nonzero"])

# ---- primary: relations.cpu ratio (DL record, summed over attempts) per (q, seed) ----
def ratio(num, den, q, s, p, phases):
    return cpu(load(p, num, q, s), phases) / cpu(load(p, den, q, s), phases)


prim = {}
for X in ("HASH", "COMBO"):
    for q in QS:
        for s in SEEDS:
            per = [ratio(X, "REF", q, s, p, ["relations"]) for p in PASSES]
            prim[f"{X}/{q}/{s}"] = {"pass1": per[0], "pass2": per[1], "mean": sum(per) / 2}
aa = {}
for q in AA_Q:
    per = [ratio("REF2", "REF", q, 1, p, ["relations"]) for p in PASSES]
    aa[str(q)] = {"pass1": per[0], "pass2": per[1], "mean": sum(per) / 2}
floor = max([abs(v[k] - 1) for v in aa.values() for k in ("pass1", "pass2", "mean")])
verdict = {}
for X in ("HASH", "COMBO"):
    cells = [prim[f"{X}/{q}/{s}"] for q in QS for s in SEEDS]
    below = all(c["mean"] < 1 and c["pass1"] < 1 and c["pass2"] < 1 for c in cells)
    beyond = all(c["mean"] < 1 - floor and c["pass1"] < 1 - floor and c["pass2"] < 1 - floor for c in cells)
    verdict[X] = {"below_1_every_unit_both_passes": below, "below_1_minus_AA_floor_every_unit_both_passes": beyond,
                  "max_mean_ratio": max(c["mean"] for c in cells), "min_mean_ratio": min(c["mean"] for c in cells),
                  "decision": "SUPPORTED" if beyond else ("below 1 but not beyond the A/A floor" if below else "NOT SUPPORTED")}

# ---- secondary: total CPU (DL record, all phases), RSS ----
tot, rssr = {}, {}
for X in ("HASH", "COMBO"):
    for q in QS:
        v = [ratio(X, "REF", q, s, p, PH) for s in SEEDS for p in PASSES]
        tot[f"{X}/{q}"] = {"geomean": gm(v), "min": min(v), "max": max(v)}
        v = [rss(load(p, X, q, s)) / rss(load(p, "REF", q, s)) for s in SEEDS for p in PASSES]
        rssr[f"{X}/{q}"] = {"geomean": gm(v), "min": min(v), "max": max(v)}
aa_tot = {str(q): [ratio("REF2", "REF", q, 1, p, PH) for p in PASSES] for q in AA_Q}
rss_abs = {str(q): {a: gm([rss(load(p, a, q, s)) for s in SEEDS for p in PASSES]) for a in ("REF", "HASH", "COMBO")} for q in QS}
rel_by_q = {f"{X}/{q}": gm([prim[f"{X}/{q}/{s}"]["mean"] for s in SEEDS]) for X in ("HASH", "COMBO") for q in QS}
loads = [float(r["load1"]) for q in QS for s in SEEDS for p in PASSES for a in ("REF", "HASH", "COMBO")
         for r in load(p, a, q, s) if r.get("rec") == "unit"]
out = {"gate": gate, "primary_relations_cpu_ratio": prim, "aa": aa, "aa_floor": floor, "verdict": verdict,
       "relations_ratio_geomean_over_seeds": rel_by_q, "total_cpu_ratio": tot, "aa_total_cpu": aa_tot,
       "rss_ratio": rssr, "rss_kb_geomean": rss_abs, "load1": {"min": min(loads), "max": max(loads)}}
json.dump(out, open(os.path.join(HERE, "analysis.json"), "w"), indent=1)

L = ["# Fast hash / combined binary: tables (analyze.py)  [pcc:unverified]", "",
     f"Gate (counted fields identical, full-line, every ic record, REF/HASH/COMBO/REF2 and both passes): "
     f"{gate['units_compared']} unit comparisons, {gate['records_compared']} records, {len(gate['mismatches'])} mismatches, "
     f"{len(gate['missing'])} missing, {len(gate['verify_failures'])} verification failures, {len(gate['retried'])} retried units: "
     f"**{'PASS' if gate['pass'] else 'FAIL'}**.", "",
     f"A/A floor on relations.cpu (REF2/REF, 4 single-pass pairs and 2 pass-means): {floor*100:.2f}%. "
     + "; ".join(f"q={q}: pass1 {v['pass1']:.3f}, pass2 {v['pass2']:.3f}" for q, v in aa.items()), "",
     "Decision: " + "; ".join(f"{X}: {v['decision']} (mean ratio {v['min_mean_ratio']:.3f}..{v['max_mean_ratio']:.3f})" for X, v in verdict.items()), "",
     "## relations.cpu ratio to REF (DL record), mean of the two passes; seeds 1 / 2 / 3", "",
     "| arm | q=262111 | 524287 | 1048507 | 2097091 | 4194271 |", "|:--|:--|:--|:--|:--|:--|"]
for X in ("HASH", "COMBO"):
    L.append(f"| {X} | " + " | ".join(" / ".join(f"{prim[f'{X}/{q}/{s}']['mean']:.3f}" for s in SEEDS) for q in QS) + " |")
L += ["", "Geometric mean over seeds: " + "; ".join(f"{k} {v:.3f}" for k, v in rel_by_q.items()), "",
      "## Total CPU ratio to REF (all phases, DL record; geomean, min..max over 3 seeds x 2 passes)", "",
      "| arm | q=262111 | 524287 | 1048507 | 2097091 | 4194271 |", "|:--|:--|:--|:--|:--|:--|"]
for X in ("HASH", "COMBO"):
    L.append(f"| {X} | " + " | ".join(f"{tot[f'{X}/{q}']['geomean']:.3f} ({tot[f'{X}/{q}']['min']:.2f}..{tot[f'{X}/{q}']['max']:.2f})" for q in QS) + " |")
L += ["", "A/A total CPU (REF2/REF, pass 1, pass 2): " + "; ".join(f"q={q}: " + ", ".join(f"{x:.3f}" for x in v) for q, v in aa_tot.items()), "",
      "## Peak RSS (VmHWM) ratio to REF (geomean, min..max)", "",
      "| arm | q=262111 | 524287 | 1048507 | 2097091 | 4194271 |", "|:--|:--|:--|:--|:--|:--|"]
for X in ("HASH", "COMBO"):
    L.append(f"| {X} | " + " | ".join(f"{rssr[f'{X}/{q}']['geomean']:.3f} ({rssr[f'{X}/{q}']['min']:.2f}..{rssr[f'{X}/{q}']['max']:.2f})" for q in QS) + " |")
L += ["", "REF peak RSS (kB, geomean): " + "; ".join(f"q={q}: {v['REF']:.0f}" for q, v in rss_abs.items()),
      f"load1 at unit start (inside the lock): {out['load1']['min']:.2f}..{out['load1']['max']:.2f}"]
open(os.path.join(HERE, "tables.md"), "w").write("\n".join(L) + "\n")
print("\n".join(L))
