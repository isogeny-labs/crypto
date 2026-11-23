#!/usr/bin/env python3
"""Validator comparison of the replay (scratch replay/units) with itself and with the committed raw tarball.
usage: compare_replay.py REPLAY_DIR RUNS_TGZ SUMMARY_JSONL OUT_JSON"""
import json, re, sys, tarfile
rd, tgz, summ, outp = sys.argv[1:5]
import os
PH = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"]
ARMS = ["D", "DL1", "DL"]
const = {}
for line in open(summ):
    if line.strip():
        r = json.loads(line)
        if r.get("kind") == "ic_size" and r.get("arm") == "DL":
            const[r["q"]] = (r["r_inv"], r["r_lmul"], r["gae"])
committed, cerr = {}, {}
with tarfile.open(tgz) as tf:
    for m in tf.getmembers():
        mm = re.fullmatch(r"(?:\./)?units/(REF|A0|NRM|LZ|BOTH)_(\d+)_(\d+)\.jsonl(\.err)?", m.name)
        if mm and m.isfile():
            d = tf.extractfile(m).read().decode()
            key = (mm.group(1), int(mm.group(2)), int(mm.group(3)))
            if mm.group(4):
                cerr[key] = d
            else:
                committed[key] = [json.loads(l) for l in d.splitlines() if l.strip()]
units = {}
for fn in sorted(os.listdir(os.path.join(rd, "units"))):
    mm = re.fullmatch(r"(\d+)_(\w+?)_(REF|BOTH|ORACLE|A0)_(\d+)_(\d+)_c([\d.]+)\.jsonl", fn)
    if not mm:
        continue
    recs = [json.loads(l) for l in open(os.path.join(rd, "units", fn)) if l.strip()]
    err = open(os.path.join(rd, "units", fn + ".err")).read()
    units[fn] = {"idx": int(mm.group(1)), "role": mm.group(2), "arm": mm.group(3), "q": int(mm.group(4)),
                 "seed": int(mm.group(5)), "c": mm.group(6), "recs": recs, "err": err}

def ic(recs):
    return [r for r in recs if r.get("rec") == "ic"]
def counted(r):
    return [k for k in r if not (k.endswith("wall") or k.endswith("cpu") or k == "load1" or k.startswith("s.controls"))]
ALLOWED_BOTH = {(a, f) for a in ARMS for f in ("relations.mul", "relations.add")} | \
               {(a, f) for a in ("D", "DL1") for f in ("linear_algebra.lmul", "linear_algebra.ladd")}
def cmp(A, B, allowed=frozenset()):
    """returns (fields_compared, mismatches outside allowed, allowed fields that differ)"""
    A, B = ic(A), ic(B)
    if [(r["arm"], r["attempt"]) for r in A] != [(r["arm"], r["attempt"]) for r in B]:
        return 0, ["record arms/attempts differ"], []
    n, bad, ch = 0, [], []
    for a, b in zip(A, B):
        if counted(a) != counted(b):
            bad.append(f"{a['arm']}: counted key list differs")
        for k in counted(a):
            n += 1
            if a[k] != b.get(k):
                if (a["arm"], k) in allowed:
                    ch.append(f"{a['arm']} {k}")
                else:
                    bad.append(f"{a['arm']} {k}: {a[k]} vs {b.get(k)}")
    return n, bad, ch
def meq(recs, arm, q, wa, wh):
    ri, rl, gae = const[q]
    return sum(r[p + ".mul"] + ri * r[p + ".inv"] + rl * r[p + ".lmul"] + wa * (r[p + ".add"] + r[p + ".ladd"])
               + wh * gae * (r[p + ".hash"] + r[p + ".cmp"]) for r in ic(recs) if r["arm"] == arm for p in PH)
wall = re.compile(r"wall=[0-9.]+s")
def plainlog(e):
    return wall.sub("wall=*", "\n".join(l for l in e.splitlines() if not l.startswith("ORACLE_")))
def oracle_parse(e):
    nrm = [l for l in e.splitlines() if l.startswith("ORACLE_NRM")]
    lz = [l for l in e.splitlines() if l.startswith("ORACLE_LZ")]
    P = []
    for l in nrm:
        d = dict(re.findall(r"(\w+)=(\[[^\]]*\]|\S+)", l))
        P.append({k: (json.loads(v) if v.startswith("[") else int(v)) for k, v in d.items()})
    L = []
    for l in lz:
        d = dict(re.findall(r"(\w+)=(\S+)", l))
        L.append({k: (v if k == "result" else int(v)) for k, v in d.items()})
    return P, L

by = {}
for fn, u in units.items():
    by[(u["role"], u["arm"], u["q"], u["seed"], u["c"])] = fn
runs, total_bad, total_fields = [], 0, 0
oracle_tot = {"nrm_used": [0] * 6, "nrm_bad": [0] * 6, "b0zero_pairs": 0, "p0zero_resid": 0, "full_value_bad": 0,
              "lz_calls": 0, "vvp_used": 0, "vvp_bad": 0, "wy_used": 0, "wy_bad": 0, "wA0_nonzero": 0,
              "reconciled_units": 0, "reconcile_fail": []}
ratios = []
for fn, u in sorted(units.items(), key=lambda kv: kv[1]["idx"]):
    q, s, c = u["q"], u["seed"], u["c"]
    meta = [r for r in u["recs"] if r.get("rec") == "vunit"][0]
    ics = ic(u["recs"])
    run = {"unit": fn, "idx": u["idx"], "role": u["role"], "arm": u["arm"], "q": q, "seed": s, "c_D": float(c),
           "command": meta["cmd"], "env": meta["env"], "exit_code": meta["rc"], "start_utc": meta["start_utc"],
           "loadavg_before": meta["loadavg_before"], "ic_records": len(ics),
           "ic_arms_attempts": [[r["arm"], r["attempt"]] for r in ics],
           "k_found_equals_k_and_ok": all(r["ok"] is True and r["k_found"] == r["k"] for r in ics) and len(ics) == 3,
           "checks": []}
    def chk(against, B, allowed=frozenset(), expect_changes=None):
        n, bad, ch = cmp(B, u["recs"], allowed)
        e = {"against": against, "fields_compared": n, "mismatches": bad[:20], "n_mismatches": len(bad)}
        if allowed:
            e["allowed_fields_changed"] = sorted(set(ch))
            if expect_changes is not None and sorted(set(ch)) != sorted(expect_changes):
                e["n_mismatches"] += 1
                e["mismatches"].append(f"allowed-change set {sorted(set(ch))} != expected {sorted(expect_changes)}")
        run["checks"].append(e)
    refkey = by.get(("cnt" if u["role"] in ("cnt", "orc", "aa") else u["role"], "REF", q, s, c))
    if u["role"] == "orcp":
        refrecs = committed[("REF", q, s)]
    else:
        refrecs = units[refkey]["recs"] if refkey else None
    expect = sorted({f"{a} {f}" for a, f in ALLOWED_BOTH})
    if u["arm"] == "REF":
        if (u["role"] == "cnt") and s <= 6:
            chk("committed REF (every counted field)", committed[("REF", q, s)])
        run["log_equal_committed_mod_wall"] = (plainlog(u["err"]) == plainlog(cerr[("REF", q, s)])) if s <= 6 and c == "1.7" else None
    if u["arm"] == "A0":
        chk("replayed REF (every counted field)", refrecs)
        chk("committed A0 (every counted field)", committed[("A0", q, s)])
        run["log_equal_ref_mod_wall"] = plainlog(u["err"]) == plainlog(units[refkey]["err"])
    if u["arm"] == "BOTH":
        chk("replayed REF (only NRM+LZ allowed fields may differ; all must)", refrecs, ALLOWED_BOTH, expect)
        if s <= 6 and c == "1.7":
            chk("committed BOTH (every counted field)", committed[("BOTH", q, s)])
        run["log_equal_ref_mod_wall"] = plainlog(u["err"]) == plainlog(units[refkey]["err"])
        rr = {}
        for a in ARMS:
            rr[a] = {str(wa): meq(u["recs"], a, q, wa, 0.0) / meq(refrecs, a, q, wa, 0.0) for wa in (0.0, 0.5, 1.0)}
            rr[a + "_wh"] = {str(wh): meq(u["recs"], a, q, 0.0, wh) / meq(refrecs, a, q, 0.0, wh) for wh in (0.1, 1.0)}
        ratios.append({"q": q, "seed": s, "c_D": float(c), "BOTH_over_REF": rr,
                       "all_below_1": all(v < 1 for a in rr for v in rr[a].values())})
    if u["arm"] == "ORACLE":
        both = by.get((u["role"] if u["role"] == "cd" else "cnt", "BOTH", q, s, c))
        if both:
            chk("replayed BOTH (every counted field)", units[both]["recs"])
        if s <= 6 and c == "1.7":
            chk("committed BOTH (every counted field)", committed[("BOTH", q, s)])
        chk("REF (replayed, or committed for producer cells; only NRM+LZ allowed fields)", refrecs, ALLOWED_BOTH, expect)
        P, L = oracle_parse(u["err"])
        run["oracle"] = {"nrm": P, "lz": L}
        for p in P:
            for i in range(6):
                oracle_tot["nrm_used"][i] += p["used"][i]; oracle_tot["nrm_bad"][i] += p["bad"][i]
            for k in ("b0zero_pairs", "p0zero_resid", "full_value_bad"):
                oracle_tot[k] += p[k]
        for l in L:
            oracle_tot["lz_calls"] += 1
            for k in ("vvp_used", "vvp_bad", "wy_used", "wy_bad"):
                oracle_tot[k] += l[k]
            oracle_tot["wA0_nonzero"] += l["wA0_nonzero"]
        # reconciliation of counted differences with the oracle's skip / reuse counts
        R = ic(refrecs); O = ic(u["recs"])
        rec_ok = len(P) == 1
        detail = {}
        for r, o in zip(R, O):
            a = r["arm"]
            dm = r["relations.mul"] - o["relations.mul"]; da = r["relations.add"] - o["relations.add"]
            pm = 0 if a == "D" else 1
            pred = sum(l["pred_lmul_saving"] for l in L if l["pm1"] == pm) if a in ("D", "DL1") else 0
            dl = r["linear_algebra.lmul"] - o["linear_algebra.lmul"]; dla = r["linear_algebra.ladd"] - o["linear_algebra.ladd"]
            ok = (len(P) == 1 and dm == P[0]["skipped_mul"] and da == P[0]["add_delta_ref_minus_cand"] and dl == pred and dla == pred)
            detail[a] = {"d_rel_mul": dm, "oracle_skipped_mul": P[0]["skipped_mul"] if P else None, "d_rel_add": da,
                         "oracle_add_delta": P[0]["add_delta_ref_minus_cand"] if P else None,
                         "d_la_lmul": dl, "d_la_ladd": dla, "oracle_pred_lmul_saving": pred, "exact": ok}
            rec_ok = rec_ok and ok
        run["reconciliation"] = detail
        if rec_ok:
            oracle_tot["reconciled_units"] += 1
        else:
            oracle_tot["reconcile_fail"].append(fn)
        run["oracle_clean"] = (all(sum(p["bad"]) == 0 and p["full_value_bad"] == 0 for p in P)
                               and all(l["vvp_bad"] == 0 and l["wy_bad"] == 0 and l["wA0_nonzero"] == 0 for l in L))
        run["log_equal_ref_mod_wall"] = plainlog(u["err"]) == plainlog(units[refkey]["err"] if u["role"] != "orcp" else cerr[("REF", q, s)])
    nb = sum(c_["n_mismatches"] for c_ in run["checks"])
    run["verified"] = (nb == 0 and run["exit_code"] == 0 and run["k_found_equals_k_and_ok"]
                       and run.get("oracle_clean", True) and run.get("log_equal_ref_mod_wall", True) is not False
                       and run.get("log_equal_committed_mod_wall", True) is not False
                       and (u["arm"] != "ORACLE" or fn not in oracle_tot["reconcile_fail"]))
    total_bad += nb + (0 if run["verified"] else 1 if nb == 0 else 0)
    total_fields += sum(c_["fields_compared"] for c_ in run["checks"])
    runs.append(run)
out = {"runs": runs, "n_runs": len(runs), "all_verified": all(r["verified"] for r in runs), "total_mismatches": total_bad,
       "fields_compared": total_fields, "all_exit_codes_zero": all(r["exit_code"] == 0 for r in runs),
       "all_ok_and_k_found_equals_k": all(r["k_found_equals_k_and_ok"] for r in runs),
       "oracle_totals": oracle_tot, "replayed_ratios": ratios}
json.dump(out, open(outp, "w"), indent=1)
print(json.dumps({k: v for k, v in out.items() if k not in ("runs", "replayed_ratios")}, indent=1))
for r in runs:
    print(r["idx"], r["role"], r["arm"], r["q"], r["seed"], r["c_D"], "rc", r["exit_code"], "verified", r["verified"],
          [(c_["against"][:30], c_["fields_compared"], c_["n_mismatches"]) for c_ in r["checks"]])
for x in ratios:
    print(x["q"], x["seed"], x["c_D"], {a: {k: round(v, 4) for k, v in x["BOTH_over_REF"][a].items()} for a in ARMS}, x["all_below_1"])
