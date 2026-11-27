#!/usr/bin/env python3
"""Step-3 validation run (PROTOCOL.md). Usage: python run_validation.py OUT.jsonl

Population: ../g3_affine_twist_20261121/data/main.jsonl, every protocol prime
(17,19,29,31,37,41,43,47), curves 1..20 in file order (curve 0 of every prime skipped:
p = 17 curve 0 was used in development), both stable affine kernels.
"""
import json
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import build as BD  # noqa: E402

TW = os.path.join(HERE, "..", "g3_affine_twist_20261121")
PRIMES = [17, 19, 29, 31, 37, 41, 43, 47]
CURVES = range(1, 21)
REPLAY_CURVES = range(1, 4)


def leg(x, p):
    x %= p
    return 0 if x == 0 else (1 if pow(x, (p - 1) // 2, p) == 1 else -1)


def main(outpath):
    rows = [json.loads(l) for l in open(os.path.join(TW, "data", "main.jsonl"))]
    byp = {}
    for r in rows:
        byp.setdefault(r["p"], [])
        cur = byp[r["p"]]
        if not cur or cur[-1][0]["f"] != r["f"]:
            cur.append([r])
        else:
            cur[-1].append(r)
    done = set()
    if os.path.exists(outpath):  # resume after an external kill (deviation D1)
        for l in open(outpath):
            d = json.loads(l)
            done.add((d["p"], d["curve"], d["L"]))
    with open(outpath, "a") as out:
        for p in PRIMES:
            for ci in CURVES:
                for r in byp[p][ci]:
                    if (p, ci, r["L"]) in done:
                        continue
                    e0 = r["evals"][0]
                    s = 0 if e0["val"] == 0 else leg(-1, p) * leg(e0["c"], p) * leg(e0["val"], p)
                    label = {1: "jacobian", -1: "twist", 0: "degenerate"}[s]
                    t0 = time.time()
                    res = BD.construct(p, r["f"], r["L_subsets"], e0["t"], seed=1000 * p + ci)
                    P = r["P"]
                    tw = [c * (-1) ** (6 - i) for i, c in enumerate(P)]
                    PX = res.get("P_X")
                    rec = {"p": p, "curve": ci, "L": r["L"], "f": r["f"], "label": label,
                           "P_J": P, "P_twist": tw, "informative": P != tw,
                           "status": res["status"], "P_X": PX,
                           "match": None if PX is None else ("J" if PX == P else ("twist" if PX == tw else "neither")),
                           "diag": {k: v for k, v in res.items() if k not in ("G", "P_X", "f")},
                           "G": res.get("G")}
                    if ci in REPLAY_CURVES and res["status"] == "ok":
                        r2 = BD.construct(p, r["f"], r["L_subsets"], (e0["t"] + 1) % p if
                                          sum(cf * pow((e0["t"] + 1) % p, i, p) for i, cf in enumerate(r["f"])) % p
                                          else (e0["t"] + 2) % p,
                                          seed=7 + 1000 * p + ci, sig_index=1)
                        rec["replay"] = {"status": r2["status"], "N": r2.get("N"), "same_N": r2.get("N") == res.get("N"),
                                         "t": r2["t"], "G": r2.get("G")}
                    rec["sec"] = round(time.time() - t0, 2)
                    out.write(json.dumps(rec) + "\n")
                    out.flush()
                    print(p, ci, r["L"], label, rec["status"], rec["match"], rec["sec"], flush=True)


if __name__ == "__main__":
    main(sys.argv[1])
