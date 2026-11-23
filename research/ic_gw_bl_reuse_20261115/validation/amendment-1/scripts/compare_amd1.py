#!/usr/bin/env python3
"""Validator replay comparison for amendment 1 of ic_gw_bl_reuse_20261115 (compare.py with arms BLR1/AUD1; PROD_RUNS_DIR = the amendment tarball, whose BLR_* files are BLR1 records).
usage: compare.py REPLAY_DIR PROD_RUNS_DIR FASTHASH_RUNS_DIR SUMMARY_JSON > replay-compare.json
REPLAY_DIR/units/<idx>_<role>_<arm>_<q>_<seed>_b<block>.jsonl (+ .err with BLAUDIT lines for AUD/MUT* builds)."""
import json, os, re, sys, glob, collections

rep, prod, fh, summ = sys.argv[1:5]
PH = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"]
LA = ("linear_algebra.lmul", "linear_algebra.ladd")
C = {}
for line in open(summ):
    r = json.loads(line)
    if r.get("kind") == "ic_size" and r.get("arm") == "DL":
        C[r["q"]] = (r["r_inv"], r["r_lmul"], r["gae"])

def counted_keys(r):
    return [k for k in r if not (k.endswith("wall") or k.endswith("cpu")) and k != "load1" and not k.startswith("s.controls")]

def load(p):
    return [json.loads(l) for l in open(p) if l.strip()]

def ics(rs):
    return [r for r in rs if r.get("rec") == "ic"]

def diff(a, b, allow_la=False):
    """counted-field differences between two record lists; allow_la: DL linear_algebra.lmul/ladd may differ."""
    a, b = ics(a), ics(b)
    if [x["arm"] for x in a] != [x["arm"] for x in b]:
        return ["record arms/count differ: %s vs %s" % ([x["arm"] for x in a], [x["arm"] for x in b])], 0
    d, n = [], 0
    for x, z in zip(a, b):
        kx, kz = counted_keys(x), counted_keys(z)
        if kx != kz:
            d.append(f"{x['arm']}: key list differs"); continue
        for k in kx:
            n += 1
            if x[k] != z[k] and not (allow_la and x["arm"] == "DL" and k in LA):
                d.append(f"{x['arm']}/{k}: {x[k]} vs {z[k]}")
    return d, n

def meq(rs, wa, wh=0.0):
    ri, rl, gae = C[rs[0]["q"]]
    return sum(r[p + ".mul"] + ri * r[p + ".inv"] + rl * r[p + ".lmul"] + wa * (r[p + ".add"] + r[p + ".ladd"])
               + wh * gae * (r[p + ".hash"] + r[p + ".cmp"]) for r in rs for p in PH)

runs = {}
for f in sorted(glob.glob(os.path.join(rep, "units", "*.jsonl"))):
    m = re.match(r"(\d+)_(\w+?)_(\w+?)_(\d+)_(\d+)_b(\d+)\.jsonl$", os.path.basename(f))
    idx, role, arm, q, s, b = int(m[1]), m[2], m[3], int(m[4]), int(m[5]), int(m[6])
    rs = load(f)
    err = open(f + ".err").read().splitlines() if os.path.exists(f + ".err") else []
    aud = [json.loads(l[8:]) for l in err if l.startswith("BLAUDIT ")]
    runs[idx] = dict(file=os.path.basename(f), idx=idx, role=role, arm=arm, q=q, seed=s, block=b, rs=rs, aud=aud)

key = lambda r: (r["q"], r["seed"], r["block"])
by = collections.defaultdict(dict)
for r in runs.values():
    by[key(r)].setdefault(r["arm"], r)
out_runs, mism_total, fields_total = [], 0, 0
ratios = []
for idx in sorted(runs):
    r = runs[idx]
    rs = r["rs"]
    unit = [x for x in rs if x.get("rec") == "vunit"]
    rec = dict(unit=r["file"], idx=idx, role=r["role"], arm=r["arm"], q=r["q"], seed=r["seed"], block=r["block"],
               command=unit[0]["cmd"] if unit else None, exit_code=unit[0]["rc"] if unit else None,
               start_utc=unit[0]["start_utc"] if unit else None, loadavg_before=unit[0]["loadavg_before"] if unit else None,
               ic_records=len(ics(rs)), ic_arms_attempts=[[x["arm"], x["attempt"]] for x in ics(rs)],
               k_found_equals_k_and_ok=all(x["ok"] is True and x["k_found"] == x["k"] for x in ics(rs)) and len(ics(rs)) > 0)
    k = key(r); refr = by[k].get("REF")
    checks = []
    if r["arm"] in ("BLR1", "AUD1") and refr:
        d, n = diff(refr["rs"], rs, allow_la=True); checks.append(("vs replayed REF (all counted fields except DL LA lmul/ladd)", d, n))
        dl_a = [x for x in ics(refr["rs"]) if x["arm"] == "DL"]; dl_b = [x for x in ics(rs) if x["arm"] == "DL"]
        for kk in ("s.bl_iters", "s.bl_narrowed"):
            if [x.get(kk) for x in dl_a] != [x.get(kk) for x in dl_b]:
                checks.append((f"{kk} vs REF", [f"{kk} differs"], 1))
        if r["arm"] == "BLR1":
            ratios.append({"q": r["q"], "seed": r["seed"], "block": r["block"],
                           "DL_meq_ratio": {str(w): meq(dl_b, w) / meq(dl_a, w) for w in (0.0, 0.5, 1.0)},
                           "DL_meq_ratio_wh": {str(w): meq(dl_b, 0.0, w) / meq(dl_a, 0.0, w) for w in (0.1, 1.0)},
                           "D_DL1_ratio_all_1": all(meq([x for x in ics(rs) if x["arm"] == y], w) == meq([x for x in ics(refr["rs"]) if x["arm"] == y], w)
                                                    for y in ("D", "DL1") for w in (0.0, 1.0)),
                           "la_lmul_ratio": sum(x["linear_algebra.lmul"] for x in dl_b) / sum(x["linear_algebra.lmul"] for x in dl_a),
                           "bl_iters": [x.get("s.bl_iters") for x in dl_b], "bl_narrowed": [x.get("s.bl_narrowed") for x in dl_b]})
    if r["arm"] == "AUD1":
        blr = by[k].get("BLR1")
        if blr:
            d, n = diff(blr["rs"], rs); checks.append(("vs replayed BLR1 (every counted field)", d, n))
        if r["block"] == 4 and r["seed"] <= 6:
            d, n = diff(load(os.path.join(prod, "units", f"BLR_{r['q']}_{r['seed']}.jsonl")), rs)
            checks.append(("vs committed amendment BLR1 (every counted field)", d, n))
        bad1 = sum(a["id1_bad"] for a in r["aud"]); bad2 = sum(a["id2_bad"] for a in r["aud"])
        rec["oracle"] = {"calls": len(r["aud"]), "id1_used": sum(a["id1_used"] for a in r["aud"]), "id1_bad": bad1,
                         "id1_direct_fallback": sum(a["id1_direct"] for a in r["aud"]),
                         "id2_used": sum(a["id2_used"] for a in r["aud"]), "id2_bad": bad2,
                         "id2_direct": sum(a["id2_direct"] for a in r["aud"]),
                         "narrowing": [[a["iters"], a["narrowed0"], a["narrow_at"], a["narrow_k"]] for a in r["aud"]],
                         "id2_reuse_after_a_narrowing": sum(1 for a in r["aud"] if a["narrow_at"] and min(a["narrow_at"]) < a["iters"] - 1)}
        # exact reconciliation of the LA counts: REF - AUD(=BLR) == skipped direct grams - recurrence
        if refr or (r["block"] == 4 and r["seed"] <= 6):
            refrs = refr["rs"] if refr else load(os.path.join(prod, "units", f"REF_{r['q']}_{r['seed']}.jsonl"))
            dl_a = [x for x in ics(refrs) if x["arm"] == "DL"]; dl_b = [x for x in ics(rs) if x["arm"] == "DL"]
            dm = sum(x["linear_algebra.lmul"] for x in dl_a) - sum(x["linear_algebra.lmul"] for x in dl_b)
            da = sum(x["linear_algebra.ladd"] for x in dl_a) - sum(x["linear_algebra.ladd"] for x in dl_b)
            sm = sum(a["skip_lmul"] - a["rec_lmul"] for a in r["aud"]); sa = sum(a["skip_ladd"] - a["rec_ladd"] for a in r["aud"])
            rec["reconciliation"] = {"ref_minus_blr_lmul": dm, "skipped_minus_recurrence_lmul": sm,
                                     "ref_minus_blr_ladd": da, "skipped_minus_recurrence_ladd": sa, "exact": dm == sm and da == sa,
                                     "ref_source": "replayed REF" if refr else "committed REF"}
    if r["arm"] in ("REF", "BLR1") and r["block"] == 4 and r["seed"] <= 6:
        fa = "BLR" if r["arm"] == "BLR1" else "REF"
        d, n = diff(load(os.path.join(prod, "units", f"{fa}_{r['q']}_{r['seed']}.jsonl")), rs)
        checks.append((f"vs committed amendment {r['arm']} (every counted field)", d, n))
    if r["arm"] == "REF" and r["block"] == 4 and r["seed"] <= 3:
        for p in ("p1", "p2"):
            d, n = diff(load(os.path.join(fh, "units", f"{p}_COMBO_{r['q']}_{r['seed']}.jsonl")), rs)
            checks.append((f"vs committed fasthash {p}_COMBO (every counted field)", d, n))
    if r["arm"] in ("MUTC2", "MUTPM"):
        d, n = diff(refr["rs"], rs, allow_la=True) if refr else (["no REF"], 0)
        rec["negative_control"] = {"diffs_vs_REF_outside_DL_LA": len(d), "first": d[:6],
                                   "DL_records": [[x["attempt"], x["ok"], x["fail"], x.get("s.bl_iters"), x.get("s.bl_narrowed")] for x in ics(rs) if x["arm"] == "DL"],
                                   "REF_DL_records": [[x["attempt"], x["ok"], x["fail"], x.get("s.bl_iters"), x.get("s.bl_narrowed")] for x in ics(refr["rs"]) if x["arm"] == "DL"] if refr else None,
                                   "oracle_id1_bad": sum(a["id1_bad"] for a in r["aud"]), "oracle_id2_bad": sum(a["id2_bad"] for a in r["aud"]),
                                   "detected": len(d) > 0}
    rec["checks"] = [{"against": c, "fields_compared": n, "mismatches": d[:10], "n_mismatches": len(d)} for c, d, n in checks]
    if r["role"] != "neg":
        mism_total += sum(len(d) for _, d, _ in checks); fields_total += sum(n for _, _, n in checks)
        rec["verified"] = rec["exit_code"] == 0 and rec["k_found_equals_k_and_ok"] and all(not d for _, d, _ in checks) \
            and (r["arm"] != "AUD1" or (rec["oracle"]["id1_bad"] == 0 and rec["oracle"]["id2_bad"] == 0 and rec.get("reconciliation", {}).get("exact", True)))
    out_runs.append(rec)
json.dump({"runs": out_runs, "total_mismatches_non_negative_roles": mism_total, "fields_compared_non_negative_roles": fields_total,
           "replayed_ratios": ratios}, sys.stdout, indent=1)
print()
