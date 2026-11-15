#!/usr/bin/env python3
"""EXP-PKM-DENSE-DEC-001 analysis (PROTOCOL.md): validity V1-V3, then P1-P4 per completed S. Writes results.json."""
import glob
import json
import os
from statistics import median

HERE = os.path.dirname(os.path.abspath(__file__))
ODD_B = os.path.join(HERE, "..", "pkm_odd_tower_f4_20261114", "runs")


def load():
    rows = []
    for f in sorted(glob.glob(os.path.join(HERE, "runs", "*.jsonl"))):
        rows += [json.loads(l) for l in open(f) if l.strip()]
    return rows


def decided(side):
    return side is not None and not side["undetermined"]


def main():
    rows = load()
    # V1, V2
    v1 = {"checked": 0, "disagree": []}
    v2 = {"planted": 0, "without_relation": []}
    for r in rows:
        for arm in ("tower", "dense"):
            s = r.get(arm)
            if decided(s):
                v1["checked"] += 1
                if not s["agree"]:
                    v1["disagree"].append((r["cell"], r["index"], arm))
        if r["kind"] == "planted":
            v2["planted"] += 1
            if r["enum_relations"] == 0:
                v2["without_relation"].append((r["cell"], r["index"]))
    # V3: tower counts equal ODD-001 part B (P1, same seed/targets)
    v3 = []
    for m, t in ((2, 4), (2, 6), (2, 8), (3, 2), (3, 3), (3, 4)):
        f = os.path.join(ODD_B, f"B-M{m}-A2-t{t}.jsonl")
        mine = [r for r in rows if r["cell"] == f"m{m}-t{t}-both"]
        if not os.path.exists(f) or not mine:
            continue
        old = [json.loads(l) for l in open(f)]
        key = lambda r: (r["kind"], r["draw"])  # noqa: E731
        oldmap = {key(r): r["alg_field_ops"] for r in old}
        pairs = [(oldmap.get(key(r)), r["tower"]["field_ops"]) for r in mine if key(r) in oldmap]
        v3.append({"cell": f"m{m}-t{t}", "pairs": len(pairs), "equal": all(a == b for a, b in pairs)})
    # per-cell summaries
    cells = {}
    for r in rows:
        c = cells.setdefault(r["cell"], {"cell": r["cell"], "p": r["p"], "m": r["m"], "S": r["S"],
                                         "vkind": r["vkind"], "membership": r["membership"], "rows": []})
        c["rows"].append(r)
    out = []
    for c in cells.values():
        rs = c.pop("rows")
        pl = [r for r in rs if r["kind"] == "planted"]
        rec = dict(c)
        rec["censored"] = sum(1 for r in rs for a in ("tower", "dense") if r.get(a) and r[a]["undetermined"])
        for arm in ("tower", "dense"):
            ok = [r for r in pl if decided(r.get(arm))]
            if ok:
                rec[f"{arm}_planted_ops_median"] = median(r[arm]["field_ops"] for r in ok)
                rec[f"{arm}_planted_over_enum_median"] = median(r[arm]["ratio_over_enum"] for r in ok)
                rec[f"{arm}_staircase_max"] = max(r[arm]["staircase"] for r in ok)
                rec[f"{arm}_linear_stage_share_median"] = median(
                    r[arm]["linear_stage_ops"] / max(1, r[arm]["field_ops"]) for r in ok)
            rnd = [r for r in rs if r["kind"] == "random" and decided(r.get(arm))]
            if rnd:
                rec[f"{arm}_random_ops_median"] = median(r[arm]["field_ops"] for r in rnd)
        pr = [r["ratio_dense_over_tower"] for r in pl if r["ratio_dense_over_tower"] is not None]
        if pr:
            rec["dense_over_tower_planted_median"] = median(pr)
            rec["dense_over_tower_planted_range"] = [min(pr), max(pr)]
        rec["enum_ops"] = rs[0]["enum_field_ops"]
        out.append(rec)
    out.sort(key=lambda c: (c["p"], c["m"], c["S"], c["vkind"], c["membership"]))
    P1 = 1399680001
    paired = [c for c in out if c["p"] == P1 and "dense_over_tower_planted_median" in c]
    p1 = {"cells": [(c["m"], c["S"], c["dense_over_tower_planted_median"]) for c in paired]}
    p1["verdict"] = "falsified" if any(x[2] >= 1 for x in p1["cells"]) else ("held" if paired else "no data")
    p2 = {"verdict": "falsified" if any(x[2] >= 0.5 for x in p1["cells"]) else ("held" if paired else "no data")}
    p3 = {"cells": []}
    for c in out:
        if c["p"] == P1 and c["vkind"] == "random" and "dense_planted_ops_median" in c:
            co = [d for d in out if d["p"] == P1 and d["m"] == c["m"] and d["S"] == c["S"] and d["vkind"] == "coset"
                  and "dense_planted_ops_median" in d]
            if co:
                p3["cells"].append((c["m"], c["S"], c["dense_planted_ops_median"] / co[0]["dense_planted_ops_median"]))
    p3["verdict"] = "falsified" if any(not (0.5 < x[2] < 2) for x in p3["cells"]) else ("held" if p3["cells"] else "no data")
    p4 = {"cells": [(c["m"], c["S"], c["vkind"], c["dense_planted_over_enum_median"]) for c in out
                    if c["p"] == P1 and c["vkind"] == "coset" and "dense_planted_over_enum_median" in c]}
    p4["verdict"] = "falsified" if any(x[3] <= 1 for x in p4["cells"]) else ("held" if p4["cells"] else "no data")
    rep = [(c["cell"], c.get("dense_over_tower_planted_median")) for c in out if c["p"] != P1]
    res = {"study": "EXP-PKM-DENSE-DEC-001", "metric": "field_ops (counted F_p multiplications)",
           "validity": {"V1": v1, "V2": v2, "V3": v3,
                        "valid": not v1["disagree"] and not v2["without_relation"] and all(x["equal"] for x in v3)},
           "P1": p1, "P2": p2, "P3": p3, "P4": p4, "replication_P2": rep, "cells": out,
           "note": "No claim about cryptographic sizes or Pollard rho [pcc:unverified]."}
    json.dump(res, open(os.path.join(HERE, "results.json"), "w"), indent=1)
    print("valid", res["validity"]["valid"], "V1", v1["checked"], len(v1["disagree"]), "V3", v3)
    for k in ("P1", "P2", "P3", "P4"):
        print(k, res[k]["verdict"], res[k].get("cells"))
    print("rep", rep)


if __name__ == "__main__":
    main()
