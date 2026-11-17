#!/usr/bin/env python3
"""EXP-PKM-IDEAS-001 analysis: one verdict per idea against the frozen prediction table (PROTOCOL.md).

Reads runs/*.jsonl (frozen run), runs/ext/*.jsonl (Amendment 1 extension) and the committed EXP-PKM-ODD-001
tower rows. Writes results.json. Counted field_ops only; no clock enters a verdict.
"""
import glob
import json
import os
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
ODD = os.path.join(HERE, "..", "pkm_odd_tower_f4_20261114", "runs")
P1 = 1399680001


def rows(pattern):
    out = []
    for f in sorted(glob.glob(os.path.join(HERE, pattern))):
        for line in open(f):
            if line.strip():
                d = json.loads(line)
                d["_cell"] = os.path.basename(f)[:-6]
                out.append(d)
    return out


def tower_counts():
    """Committed EXP-PKM-ODD-001 degree-2 Kummer tower refutation counts at P1, keyed (m, S)."""
    t = defaultdict(list)
    for f in glob.glob(os.path.join(ODD, "M[23]-A2-t*-tower.jsonl")):
        for line in open(f):
            d = json.loads(line)
            if d.get("kind") == "refute" and d["p"] == P1 and not d["censored"] and d["field_ops"]:
                t[(d["m"], d["S"])].append(d["field_ops"])
    return {k: min(v) for k, v in t.items()}, {k: max(v) for k, v in t.items()}


def idea1():
    tmin, tmax = tower_counts()
    out = []
    for src, pat in (("frozen", "runs/dense-*.jsonl"), ("ext (Amendment 1, dmax 1024)", "runs/ext/dense-*.jsonl")):
        cells = defaultdict(list)
        for d in rows(pat):
            cells[(d["m"], d["S"], d["vkind"])].append(d)
        for (m, s, v), rs in sorted(cells.items()):
            cens = [r for r in rs if r["censored"]]
            ok = [r for r in rs if not r["censored"]]
            fo = [r["field_ops"] for r in ok]
            rec = {"source": src, "m": m, "S": s, "vkind": v, "rows": len(rs), "censored": len(cens),
                   "agree_all": all(r["agree"] for r in ok) if ok else None,
                   "field_ops_min": min(fo) if fo else None, "field_ops_max": max(fo) if fo else None,
                   "tower_field_ops": tmin.get((m, s)), "tower_field_ops_max": tmax.get((m, s))}
            if fo and (m, s) in tmin:
                rec["dense_over_tower_max"] = max(fo) / tmin[(m, s)]
                rec["dense_over_tower_min"] = min(fo) / tmax[(m, s)]
            out.append(rec)
    rnd = [r for r in out if r["vkind"] == "random" and "dense_over_tower_max" in r]
    falsified = [r for r in rnd if r["dense_over_tower_max"] < 2]
    verdict = ("falsified" if falsified else ("held" if rnd else "no completed S"))
    return {"cells": out, "prediction": "random dense / tower >= 2 at every completed S, growing with S",
            "verdict": verdict,
            "completed_S_random": [(r["source"], r["m"], r["S"]) for r in rnd],
            "falsifying_S": [(r["source"], r["m"], r["S"], r["dense_over_tower_max"]) for r in falsified]}


def idea2():
    out = []
    for d in rows("runs/aut-*.jsonl"):
        checks = {"omega_closed": d["omega_closed"], "eigen_relation_holds": d["eigen_relation_holds"],
                  "decomp_count_invariant_all": d["decomp_count_invariant"] == d["targets"],
                  "columns_F_over_6": d["orbits_neg_omega"] * 6 == d["F"],
                  "negation_only_F_over_2": d["columns_negation_only"] * 2 == d["F"],
                  "random_set_not_closed": d["random_set_omega_closed_elements"] < d["F"]}
        out.append({"S": d["S"], "F": d["F"], "p": d["p"], "n": d["n"], "targets": d["targets"],
                    "targets_with_relation": d["targets_with_relation"], "columns": d["orbits_neg_omega"],
                    "columns_negation_only": d["columns_negation_only"],
                    "random_set_omega_closed_elements": d["random_set_omega_closed_elements"], "checks": checks})
    ok = bool(out) and all(all(c["checks"].values()) for c in out)
    return {"cells": out, "prediction": "all checks hold, columns = |F|/6, random sets not closed",
            "verdict": "held" if ok else "falsified"}


def replay_cells(pattern):
    out = []
    cells = defaultdict(list)
    for d in rows(pattern):
        if d["schema"] == "pkm-ideas.replay/v1":
            cells[d["_cell"]].append(d)
    for c, rs in sorted(cells.items()):
        r = [x["ratio_replay_over_fresh"] for x in rs]
        out.append({"cell": c, "m": rs[0]["m"], "degrees": rs[0]["degrees"], "S": _prod(rs[0]["degrees"]),
                    "record_p": rs[0]["record_p"], "p": rs[0]["p"], "replays": len(rs),
                    "mismatches": sum(x["trace_mismatch"] for x in rs),
                    "disagreements": sum(not x["agree"] for x in rs),
                    "fresh_field_ops": sorted({x["fresh_field_ops"] for x in rs}),
                    "replay_field_ops": sorted({x["replay_field_ops"] for x in rs}),
                    "ratio_min": min(r), "ratio_max": max(r)})
    return out


def _prod(xs):
    p = 1
    for x in xs:
        p *= x
    return p


def idea3():
    cells = replay_cells("runs/replay-*.jsonl")
    bad = [c for c in cells if c["mismatches"] or c["disagreements"] or c["ratio_max"] > 0.5]
    return {"cells": cells, "prediction": "no mismatch, replay agrees, replay/fresh <= 0.5",
            "verdict": "falsified" if bad else ("held" if cells else "no rows"),
            "falsifying_cells": [c["cell"] for c in bad]}


def idea4():
    cells = replay_cells("runs/xcurve-*.jsonl") + replay_cells("runs/xprime-*.jsonl")
    bad = [c for c in cells if c["mismatches"] or c["disagreements"]]
    over = [c["cell"] for c in cells if c["ratio_max"] > 0.5]
    return {"cells": cells, "prediction": "no mismatch across curve or prime; ratios as in idea 3",
            "verdict": "falsified" if bad else ("held" if cells else "no rows"),
            "falsifying_cells": [c["cell"] for c in bad], "ratio_above_0_5": over}


def idea5():
    a = defaultdict(lambda: defaultdict(list))
    for d in rows("runs/cosets-*.jsonl"):
        a[d["S"]][d["layout"]].append(d)
    acells = []
    for s, lay in sorted(a.items()):
        sh = [x["field_ops"] for x in lay["shared"] if not x.get("censored")]
        pp = [x["field_ops"] for x in lay["per_point"] if not x.get("censored")]
        rec = {"S": s, "shared": sorted(set(sh)), "per_point": sorted(set(pp)),
               "agree_all": all(x["agree"] for v in lay.values() for x in v)}
        if sh and pp:
            rec["ratio_min"] = min(pp) / max(sh)
            rec["ratio_max"] = max(pp) / min(sh)
        acells.append(rec)
    bcells = []
    for d in rows("runs/coverage-*.jsonl"):
        bcells.append({"S": d["S"], "p": d["p"], "n": d["n"], "F_shared": d["F_shared"], "F_other": d["F_other"],
                       "covered_shared": d["covered_shared"], "covered_per_point": d["covered_per_point"],
                       "covered_random_shared": d["covered_random_shared"],
                       "covered_random_pair": d["covered_random_pair"],
                       "per_point_over_shared": d["covered_per_point"] / d["covered_shared"],
                       "random_pair_over_random_shared": d["covered_random_pair"] / d["covered_random_shared"]})
    a_ok = all(0.5 < c["ratio_min"] and c["ratio_max"] < 2 for c in acells if "ratio_min" in c)
    b_ok = all(c["per_point_over_shared"] >= 1.5 for c in bcells)
    return {"a": acells, "b": bcells,
            "prediction": "(a) per-point/shared in (0.5, 2); (b) per-point covers about 2x shared (falsified < 1.5)",
            "verdict_a": "held" if a_ok and acells else "falsified",
            "verdict_b": "held" if b_ok and bcells else "falsified"}


def main():
    res = {"study": "EXP-PKM-IDEAS-001", "metric": "field_ops (counted F_p multiplications)",
           "idea1": idea1(), "idea2": idea2(), "idea3": idea3(), "idea4": idea4(), "idea5": idea5(),
           "note": "No claim about cryptographic sizes or Pollard rho [pcc:unverified]."}
    json.dump(res, open(os.path.join(HERE, "results.json"), "w"), indent=1)
    for k in ("idea1", "idea2", "idea3", "idea4"):
        print(k, res[k]["verdict"])
    print("idea5a", res["idea5"]["verdict_a"], "idea5b", res["idea5"]["verdict_b"])


if __name__ == "__main__":
    main()
