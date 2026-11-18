#!/usr/bin/env python3
"""PRESET section 5 checks on finished runs: (i) unit iff 0 grid zeros; (ii) planted GB vanishes at planted point.
Usage: check_m8.py OUTDIR  -> OUTDIR/checks.jsonl"""
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import gen_mixed as GM  # noqa: E402
G, MP = GM.G, __import__("msparse")


def main():
    out = sys.argv[1]
    rd, sd = os.path.join(out, "runs"), os.path.join(out, "systems")
    recs = []
    for f in sorted(os.listdir(rd)):
        if not f.endswith(".res.json"):
            continue
        res = json.load(open(os.path.join(rd, f)))
        name = res["name"]
        meta = json.load(open(os.path.join(sd, name + ".json")))
        rec = dict(name=name, stop=res["stop"], unit=res.get("unit"))
        if res["stop"] is not None or res["exit"] != 0:
            rec["check"] = "not_applicable_unfinished"
            recs.append(rec)
            continue
        p = meta["p"]
        zeros = GM.enumerate_mixed(p, meta["A"], meta["B"], meta["Vs"], meta["xR"], meta["yR"])
        rec["grid_zeros"] = len(zeros)
        rec["check_i_unit_iff_no_zero"] = (res.get("unit") is True) == (len(zeros) == 0)
        if meta["kind"] == "planted":
            names = [f"y{i}_{j}" for i in range(meta["m"]) for j in range(meta["depths"][i])] + \
                    [f"u{k}" for k in range(meta["m"] - 2)]
            gb = MP.read_gb(os.path.join(rd, name + ".out"), names, p) if not res.get("unit") else None
            F = G.Fp2(p)
            pt = [(meta["planted"][nm] % p, 0) for nm in names]
            rec["check_ii_planted"] = bool(gb) and all(MP.eval_poly(P, pt, F) == (0, 0) for P in gb)
        recs.append(rec)
    with open(os.path.join(out, "checks.jsonl"), "w") as fh:
        for r in recs:
            fh.write(json.dumps(r) + "\n")
    fin = [r for r in recs if "grid_zeros" in r]
    bad = [r["name"] for r in fin if not r["check_i_unit_iff_no_zero"] or r.get("check_ii_planted") is False]
    print("checked", len(fin), "unfinished", len(recs) - len(fin), "failures", bad)


if __name__ == "__main__":
    main()
