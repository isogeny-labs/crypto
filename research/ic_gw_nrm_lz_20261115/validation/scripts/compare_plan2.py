#!/usr/bin/env python3
"""Plan 2 comparison (validator): small-q sweep (REF vs BOTH vs ORACLE per (q, seed), every attempt; per-attempt
reconciliation of counted differences with the oracle) and negative controls (mutants vs REF must be flagged).
usage: compare_plan2.py REPLAY2_DIR REPLAY_DIR RUNS_TGZ OUT_JSON"""
import json, os, re, sys, tarfile
rd2, rd1, tgz, outp = sys.argv[1:5]
ARMS = ["D", "DL1", "DL"]
ALLOWED = {(a, f) for a in ARMS for f in ("relations.mul", "relations.add")} | \
          {(a, f) for a in ("D", "DL1") for f in ("linear_algebra.lmul", "linear_algebra.ladd")}
def load(d):
    U = {}
    for fn in sorted(os.listdir(os.path.join(d, "units"))):
        mm = re.fullmatch(r"(\d+)_(\w+?)_([A-Z0-9]+)_(\d+)_(\d+)_c([\d.]+)\.jsonl", fn)
        if mm:
            U[fn] = {"idx": int(mm.group(1)), "role": mm.group(2), "arm": mm.group(3), "q": int(mm.group(4)), "seed": int(mm.group(5)),
                     "recs": [json.loads(l) for l in open(os.path.join(d, "units", fn)) if l.strip()],
                     "err": open(os.path.join(d, "units", fn + ".err")).read()}
    return U
U2, U1 = load(rd2), load(rd1)
committed = {}
with tarfile.open(tgz) as tf:
    for m in tf.getmembers():
        mm = re.fullmatch(r"(?:\./)?units/(REF|BOTH)_(\d+)_(\d+)\.jsonl", m.name)
        if mm:
            committed[(mm.group(1), int(mm.group(2)), int(mm.group(3)))] = [json.loads(l) for l in tf.extractfile(m).read().decode().splitlines() if l.strip()]
ic = lambda recs: [r for r in recs if r.get("rec") == "ic"]
counted = lambda r: [k for k in r if not (k.endswith("wall") or k.endswith("cpu") or k == "load1" or k.startswith("s.controls"))]
def cmp(A, B, allowed=frozenset()):
    A, B = ic(A), ic(B)
    if [(r["arm"], r["attempt"]) for r in A] != [(r["arm"], r["attempt"]) for r in B]:
        return 0, ["record arms/attempts differ: %s vs %s" % ([(r["arm"], r["attempt"], r["ok"]) for r in A], [(r["arm"], r["attempt"], r["ok"]) for r in B])], set()
    n, bad, ch = 0, [], set()
    for a, b in zip(A, B):
        if counted(a) != counted(b):
            bad.append(f"{a['arm']}/{a['attempt']}: counted key list differs")
        for k in counted(a):
            n += 1
            if a[k] != b.get(k):
                if (a["arm"], k) in allowed:
                    ch.add(f"{a['arm']} {k}")
                else:
                    bad.append(f"{a['arm']}/{a['attempt']} {k}: {a[k]} vs {b.get(k)}")
    return n, bad, ch
wall = re.compile(r"wall=[0-9.]+s")
plain = lambda e: wall.sub("wall=*", "\n".join(l for l in e.splitlines() if not l.startswith("ORACLE_")))
def oracle_attempts(e):
    att = []
    for l in e.splitlines():
        if l.startswith("ORACLE_NRM"):
            d = dict(re.findall(r"(\w+)=(\[[^\]]*\]|\S+)", l))
            att.append({"nrm": {k: (json.loads(v) if v.startswith("[") else int(v)) for k, v in d.items()}, "lz": []})
        elif l.startswith("ORACLE_LZ"):
            d = dict(re.findall(r"(\w+)=(\S+)", l))
            att[-1]["lz"].append({k: (v if k == "result" else int(v)) for k, v in d.items()})
    return att
by = {(u["role"], u["arm"], u["q"], u["seed"]): fn for fn, u in U2.items()}
small, tot = [], {"units": 0, "fields": 0, "mismatches": 0, "attempts": 0, "multi_attempt_units": 0, "failed_attempt_records": 0,
                  "b0zero_pairs": 0, "p0zero_resid": 0, "nrm_bad": 0, "full_value_bad": 0, "lz_calls": 0, "vvp_used": 0, "wy_used": 0,
                  "lz_bad": 0, "reconciled_records": 0, "reconcile_fail": [], "rc_nonzero": 0, "final_not_ok": [], "log_diff": []}
for q in sorted({u["q"] for u in U2.values() if u["role"] == "small"}):
    for s in range(1, 6):
        R, B, O = (U2[by[("small", a, q, s)]] for a in ("REF", "BOTH", "ORACLE"))
        ent = {"q": q, "seed": s}
        for name, X, Y, al in (("BOTH vs REF", R, B, ALLOWED), ("ORACLE vs BOTH", B, O, frozenset())):
            n, bad, ch = cmp(X["recs"], Y["recs"], al)
            ent[name] = {"fields": n, "mismatches": bad[:10], "changed": sorted(ch)}
            tot["fields"] += n; tot["mismatches"] += len(bad)
        for X in (R, B, O):
            if [r for r in X["recs"] if r.get("rec") == "vunit"][0]["rc"] != 0:
                tot["rc_nonzero"] += 1
            if plain(X["err"]) != plain(R["err"]):
                tot["log_diff"].append(f"{X['arm']} {q} {s}")
        recs = ic(R["recs"])
        atts = sorted({r["attempt"] for r in recs})
        tot["attempts"] += len(atts); tot["multi_attempt_units"] += len(atts) > 1
        tot["failed_attempt_records"] += sum(1 for r in recs if not r["ok"])
        last = [r for r in recs if r["attempt"] == max(atts)]
        if not all(r["ok"] and r["k_found"] == r["k"] for r in last):
            tot["final_not_ok"].append(f"{q} {s}: " + ", ".join(f"{r['arm']}:{r['ok']}:{r['fail'][:40]}" for r in last))
        OA = oracle_attempts(O["err"])
        ent["oracle_attempts"] = len(OA)
        for a in OA:
            p = a["nrm"]
            tot["b0zero_pairs"] += p["b0zero_pairs"]; tot["p0zero_resid"] += p["p0zero_resid"]
            tot["nrm_bad"] += sum(p["bad"]); tot["full_value_bad"] += p["full_value_bad"]
            for l in a["lz"]:
                tot["lz_calls"] += 1; tot["vvp_used"] += l["vvp_used"]; tot["wy_used"] += l["wy_used"]
                tot["lz_bad"] += l["vvp_bad"] + l["wy_bad"] + l["wA0_nonzero"]
        Rr, Or = ic(R["recs"]), ic(O["recs"])
        for r, o in zip(Rr, Or):
            if r["attempt"] > len(OA):
                tot["reconcile_fail"].append(f"{q} {s} {r['arm']}/{r['attempt']}: no oracle line"); continue
            a = OA[r["attempt"] - 1]
            ok = (r.get("relations.mul", 0) - o.get("relations.mul", 0) == a["nrm"]["skipped_mul"]
                  and r.get("relations.add", 0) - o.get("relations.add", 0) == a["nrm"]["add_delta_ref_minus_cand"])
            if "linear_algebra.lmul" in r:
                pm = {"D": 0, "DL1": 1}.get(r["arm"])
                pred = sum(l["pred_lmul_saving"] for l in a["lz"] if l["pm1"] == pm) if pm is not None else 0
                ok = ok and r["linear_algebra.lmul"] - o["linear_algebra.lmul"] == pred and r["linear_algebra.ladd"] - o["linear_algebra.ladd"] == pred
            if "relations.mul" not in r:
                ok = ok and a["nrm"]["skipped_mul"] == 0
            if ok:
                tot["reconciled_records"] += 1
            else:
                tot["reconcile_fail"].append(f"{q} {s} {r['arm']}/{r['attempt']}")
        tot["units"] += 3
        small.append(ent)
neg = []
for fn, u in sorted(U2.items(), key=lambda kv: kv[1]["idx"]):
    if u["role"] != "neg":
        continue
    q, s = u["q"], u["seed"]
    ref = committed[("REF", q, s)] if s <= 6 and q == 262111 else [x for x in U1.values() if x["arm"] == "REF" and x["q"] == q and x["seed"] == s and x["role"] == "cnt"][0]["recs"]
    both = committed[("BOTH", q, s)]
    n, bad, ch = cmp(ref, u["recs"], ALLOWED)
    n2, bad2, _ = cmp(both, u["recs"])
    recs = ic(u["recs"])
    neg.append({"unit": fn, "arm": u["arm"], "q": q, "seed": s,
                "rc": [r for r in u["recs"] if r.get("rec") == "vunit"][0]["rc"],
                "records": [[r["arm"], r["attempt"], r["ok"], r["fail"][:50]] for r in recs],
                "vs_REF_outside_allowed": len(bad), "first": bad[:3], "vs_committed_BOTH": len(bad2), "first_vs_BOTH": bad2[:3],
                "detected": len(bad) > 0 or len(bad2) > 0})
out = {"small_q": {"totals": tot, "cells": small}, "negative_controls": neg}
json.dump(out, open(outp, "w"), indent=1)
print(json.dumps(tot, indent=1))
for x in neg:
    print(x["arm"], x["q"], x["seed"], "rc", x["rc"], "detected", x["detected"], x["vs_REF_outside_allowed"], x["vs_committed_BOTH"], x["records"][:4], x["first"][:2])
