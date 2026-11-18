#!/usr/bin/env python3
"""L06 analysis (SPEC.md section 12). Usage: python3 -I analyze.py raw > raw/summary.json"""
import json, math, os, sys
from collections import Counter

RAW = sys.argv[1] if len(sys.argv) > 1 else "raw"
PS = [257, 521, 1031, 2053, 4099]
LS = [3, 5, 7]
Z = 1.959964


def wilson(k, n):
    if n == 0:
        return [None, None]
    ph = k / n
    d = 1 + Z * Z / n
    center = (ph + Z * Z / (2 * n)) / d
    half = Z * math.sqrt(ph * (1 - ph) / n + Z * Z / (4 * n * n)) / d
    return [center - half, center + half]


def load_jsonl(path):
    rows, summ = [], None
    if not os.path.exists(path):
        return rows, summ
    for line in open(path):
        line = line.strip()
        if not line:
            continue
        o = json.loads(line)
        if o.get("type") == "run_summary":
            summ = o
        else:
            rows.append(o)
    return rows, summ


def rd(name):
    path = os.path.join(RAW, name)
    return json.load(open(path)) if os.path.exists(path) else None


census = {}
for p in PS:
    census[p] = load_jsonl(os.path.join(RAW, f"census_p{p}.jsonl"))

gates = {}
# G0
tl = os.path.join(RAW, "test.log")
if os.path.exists(tl):
    txt = open(tl).read()
    bad = [l for l in txt.splitlines() if "FAILED" in l or "panicked" in l]
    gates["G0"] = {"pass": ("test result: ok." in txt) and not bad, "failed_lines": len(bad)}
else:
    gates["G0"] = {"pass": False, "note": "test.log missing"}

# G1
g1 = {"files": {}, "total": 0, "ok": 0, "agree": 0, "ambiguous": 0, "other": 0}
for p in (101, 257):
    rows, _ = load_jsonl(os.path.join(RAW, f"gate_enum_p{p}.jsonl"))
    c = Counter(r["status"] for r in rows)
    ag = sum(1 for r in rows if r["agree"] is True)
    dis = sum(1 for r in rows if r["agree"] is False)
    g1["files"][str(p)] = {"draws": len(rows), "status": dict(c), "agree": ag, "disagree": dis}
    g1["total"] += len(rows)
    g1["ok"] += c.get("ok", 0)
    g1["agree"] += ag
    g1["ambiguous"] += c.get("ambiguous", 0)
    g1["other"] += len(rows) - c.get("ok", 0) - c.get("ambiguous", 0)
g1["ambiguous_fraction"] = g1["ambiguous"] / g1["total"] if g1["total"] else None
g1["pass"] = (g1["total"] == 240 and g1["agree"] == g1["ok"] and g1["other"] == 0
              and g1["ambiguous"] <= 0.02 * g1["total"])
gates["G1"] = g1

# census-level gates
chi_rows = {p: [r for r in census[p][0] if r.get("status") in ("ok", "ambiguous", "error")] for p in PS}
ok_rows = {p: [r for r in census[p][0] if r.get("status") == "ok"] for p in PS}
allok = [r for p in PS for r in ok_rows[p]]
gates["G2"] = {"checked": len(allok), "failed": sum(1 for r in allok if not r["gate_annihilation"])}
gates["G2"]["pass"] = len(allok) > 0 and gates["G2"]["failed"] == 0
gates["G3"] = {"checked": len(allok), "failed": sum(1 for r in allok if not r["gate_weil"])}
gates["G3"]["pass"] = len(allok) > 0 and gates["G3"]["failed"] == 0
g4_fail = 0
g4_n = 0
for r in allok:
    for l in ("2", "3", "5", "7"):
        c = r["crit"][l]
        g4_n += 1
        good = (c["fire"] == c["fire_fac"]) and ((not c["fire"]) or c["n_h"] == 2 ** c["n_pairs"])
        if not good:
            g4_fail += 1
gsp = rd("gsp_reference.json")
gsp_g4 = {"samples_checked": 0, "failed": 0}
if gsp:
    for row in gsp["g3"]:
        gsp_g4["samples_checked"] += row["samples"]
        gsp_g4["failed"] += row["samples"] - row["g4_agree"]
gates["G4"] = {"census_checked": g4_n, "census_failed": g4_fail, "gsp": gsp_g4,
               "pass": g4_n > 0 and g4_fail == 0 and gsp is not None and gsp_g4["failed"] == 0}
g5_fail = sum(1 for r in allok if not r["gate_mod2"])
gates["G5"] = {"checked": len(allok), "failed": g5_fail, "pass": len(allok) > 0 and g5_fail == 0}
l2 = rd("l2_lagrangians.json")
if l2:
    ok6 = (l2["total"], l2["pair"], l2["other"]) == (135, 105, 30) and all(
        t["pair_rule"] == (t["stable_pair"] > 0) for t in l2["types"]) and len(l2["types"]) == 15
    gates["G6"] = {"total": l2["total"], "pair": l2["pair"], "other": l2["other"],
                   "pair_rule_mismatch": sum(1 for t in l2["types"] if t["pair_rule"] != (t["stable_pair"] > 0)),
                   "pass": ok6}
else:
    gates["G6"] = {"pass": False, "note": "missing"}
if gsp:
    zmax = max(abs(r["z"]) for r in gsp["g1"])
    bad_exact = sum(1 for r in gsp["g1"] if r["exact"] == 0 and r["fire"] > 0)
    gates["G7"] = {"similitude_failures": gsp["similitude_failures"], "max_abs_z": zmax,
                   "cells": len(gsp["g1"]), "exact0_fire_pos": bad_exact,
                   "pass": gsp["similitude_failures"] == 0 and zmax <= 4 and bad_exact == 0}
else:
    gates["G7"] = {"pass": False, "note": "missing"}
g8 = {}
for p in PS:
    n = len(chi_rows[p])
    amb = sum(1 for r in chi_rows[p] if r["status"] == "ambiguous")
    err = sum(1 for r in chi_rows[p] if r["status"] == "error")
    g8[str(p)] = {"chi_requested": n, "ambiguous": amb, "error": err,
                  "ambiguous_fraction": (amb / n if n else None), "pass": n > 0 and amb <= 0.02 * n}
gates["G8"] = {"per_p": g8, "pass": all(v["pass"] for v in g8.values())}
g8_dropped = [p for p in PS if not g8[str(p)]["pass"]]

# populations
USE = [p for p in PS if g8[str(p)]["pass"]]


def pops(p):
    ok = ok_rows[p]
    U = [r for r in ok if r["block"] == "U"]
    return {
        "ALL": U,
        "PRIME": [r for r in ok if r["prime"]],
        "C2": [r for r in U if r["c2"]],
        "ODD": [r for r in U if r["nJ"] % 2 == 1],
    }


P = {p: pops(p) for p in PS}
pooled = {k: [r for p in USE for r in P[p][k]] for k in ("ALL", "PRIME", "C2", "ODD")}
populations = {"per_p": {}, "pooled_over_p_passing_G8": {k: len(v) for k, v in pooled.items()}, "used_p": USE}
for p in PS:
    s = census[p][1] or {}
    populations["per_p"][str(p)] = {
        "draws": s.get("draws"), "stop_reason": s.get("stop_reason"),
        "u_block_squarefree_seen": s.get("u_count"), "status_counts": s.get("status_counts"),
        **{k: len(v) for k, v in P[p].items()},
    }


def stats(rows, l):
    n = len(rows)
    k = sum(1 for r in rows if r["crit"][str(l)]["fire"])
    cls = Counter(r["crit"][str(l)]["class"] for r in rows)
    fr = [r for r in rows if r["crit"][str(l)]["fire"]]
    nh = Counter(r["crit"][str(l)]["n_h"] for r in fr)
    ed = Counter(r["crit"][str(l)]["ext_deg"] for r in fr)
    return {"n": n, "fire": k, "fraction": (k / n if n else None), "wilson95": wilson(k, n),
            "class": {c: cls.get(c, 0) for c in ("fire", "none", "undetermined")},
            "n_h_hist_on_firing": {str(a): b for a, b in sorted(nh.items())},
            "ext_deg_hist_on_firing": {str(a): b for a, b in sorted(ed.items())}}


crit_out = {}
for l in LS:
    crit_out[str(l)] = {}
    for k in ("ALL", "PRIME", "C2"):
        crit_out[str(l)][k] = {"per_p": {str(p): stats(P[p][k], l) for p in PS},
                                "pooled": stats(pooled[k], l)}

a2 = {}
for k in ("ALL", "ODD"):
    def a2stats(rows):
        n = len(rows)
        return {"n": n,
                "fire_2": sum(1 for r in rows if r["crit"]["2"]["fire"]),
                "stable_pair_exists": sum(1 for r in rows if r["l2"]["stable_pair"] > 0),
                "pair_rule": sum(1 for r in rows if r["l2"]["pair_rule"]),
                "stable_other_ge2": sum(1 for r in rows if r["l2"]["stable_other"] >= 2),
                "stable_other_min": min((r["l2"]["stable_other"] for r in rows), default=None),
                "stable_other_hist": {str(a): b for a, b in sorted(Counter(r["l2"]["stable_other"] for r in rows).items())},
                "stable_pair_hist": {str(a): b for a, b in sorted(Counter(r["l2"]["stable_pair"] for r in rows).items())}}
    a2[k] = {"per_p": {str(p): a2stats(P[p][k]) for p in PS}, "pooled": a2stats(pooled[k])}
o = a2["ODD"]["pooled"]
a2["predictions_on_ODD_pooled"] = {
    "a_fire_2_all": o["fire_2"] == o["n"],
    "b_no_pair_type": o["stable_pair_exists"] == 0,
    "c_other_ge2_all": o["stable_other_ge2"] == o["n"],
}

# S4
ref = {}
if gsp:
    for row in gsp["g3"]:
        ref[(row["l"], row["m"])] = row
s4 = {"cells": [], "n_abs_z_gt_3": 0, "n_cells": 0}
for p in PS:
    for l in LS:
        m = p % l
        r = ref.get((l, m))
        for pk, rows, fkey, nkey in (("ALL", P[p]["ALL"], "fire", "samples"), ("PRIME", P[p]["PRIME"], "fire_nz1", "nz1")):
            n = len(rows)
            if n == 0 or r is None:
                continue
            if pk == "PRIME":
                assert all(rr["crit"][str(l)] is not None for rr in rows)
            k = sum(1 for rr in rows if rr["crit"][str(l)]["fire"])
            fo = k / n
            fr = r[fkey] / r[nkey]
            seo = math.sqrt(fo * (1 - fo) / n)
            ser = math.sqrt(fr * (1 - fr) / r[nkey])
            den = math.sqrt(seo ** 2 + ser ** 2)
            z = 0.0 if den == 0 else (fo - fr) / den
            s4["cells"].append({"p": p, "l": l, "m": m, "pop": pk, "n": n, "fire": k,
                                "F_obs": fo, "F_ref": fr, "z": z})
            s4["n_cells"] += 1
            if abs(z) > 3:
                s4["n_abs_z_gt_3"] += 1

# verdict (PROTOCOL.md section 6)
fl = {}
for l in LS:
    st = crit_out[str(l)]["PRIME"]["pooled"]
    fl[str(l)] = {"n": st["n"], "fire": st["fire"], "F": st["fraction"], "lo": st["wilson95"][0], "hi": st["wilson95"][1]}
n_prime = len(pooled["PRIME"])
core = ["G0", "G1", "G2", "G3", "G4", "G5", "G6", "G7"]
failed = [g for g in core if not gates[g]["pass"]]


def rules_3_5():
    if n_prime < 300:
        return "inconclusive (sample)"
    if all(fl[str(l)]["F"] is not None and fl[str(l)]["F"] < 0.01 for l in LS):
        return "killed"
    if any(fl[str(l)]["lo"] is not None and fl[str(l)]["lo"] >= 0.01 for l in LS):
        return "supported"
    return "inconclusive"


if failed:
    verdict = "void (instrument)"
else:
    verdict = rules_3_5()

out = {"gates": gates, "gate_failures_G0_G7": failed, "g8_dropped_p": g8_dropped,
       "populations": populations, "criterion": crit_out, "A2": a2, "S4": s4,
       "pooled_prime_F_l": fl, "n_prime_pooled": n_prime,
       "outcome_of_rules_2_to_5_regardless_of_gates_NOT_THE_VERDICT": rules_3_5(),
       "verdict": verdict}
json.dump(out, sys.stdout, indent=1)
print()
