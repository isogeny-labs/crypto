# Validator replay comparison + timing table. usage: python3 -I compare.py RUNS_DIR COMMITTED_RAW_DIR OUT_JSON
import json, os, sys, copy
from collections import defaultdict

runs, committed, out_json = sys.argv[1], sys.argv[2], sys.argv[3]
PH = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"]


def counted(rec):
    return {k: v for k, v in rec.items()
            if not (k.endswith("wall") or k.endswith("cpu") or k == "load1" or k.startswith("s.controls"))}


def load(path):
    return [json.loads(l) for l in open(path) if l.strip()]


def ics(recs):
    return [r for r in recs if r["rec"] == "ic"]


def ratios_counted(recs):
    return [{k: v for k, v in r.items() if k not in ("search_s", "load1")} for r in recs if r["rec"] == "ratios"]


def cmp(a, b):
    """full-line, in-order compare of counted ic records; returns list of mismatches"""
    A, B = [counted(r) for r in ics(a)], [counted(r) for r in ics(b)]
    mm = []
    if len(A) != len(B):
        mm.append({"what": "record count", "a": len(A), "b": len(B)})
    for i, (x, y) in enumerate(zip(A, B)):
        if json.dumps(x) != json.dumps(y):
            mm.append({"record": i, "arm": x.get("arm"), "fields": sorted(k for k in set(x) | set(y) if x.get(k) != y.get(k)),
                       "key_order_equal": list(x) == list(y)})
    return mm


meta = [json.loads(l) for l in open(os.path.join(runs, "replay-log.jsonl"))]
recs = {}
for m in meta:
    recs[m["idx"]] = load(os.path.join(runs, f"{m['idx']}_{m['tag']}_{m['arm']}_{m['q']}_{m['seed']}.jsonl"))

by_unit = defaultdict(list)
for m in meta:
    by_unit[(m["q"], m["seed"])].append(m)

checks = []
fields_compared = None
for (q, s), ms in by_unit.items():
    ref0 = [m for m in ms if m["arm"] == "REF"][0]
    R = recs[ref0["idx"]]
    if fields_compared is None:
        fields_compared = {r["arm"]: list(counted(r)) for r in ics(R)}
    for m in ms:
        x = recs[m["idx"]]
        ic = ics(x)
        okk = all(r["ok"] is True and r["k_found"] == r["k"] and r["fail"] == "" for r in ic)
        entry = {"idx": m["idx"], "arm": m["arm"], "q": q, "seed": s, "rc": m["rc"], "n_ic": len(ic),
                 "ic_arms": [(r["arm"], r["attempt"]) for r in ic], "ok_and_k_found_eq_k": okk,
                 "vs_replay_REF_idx": ref0["idx"], "mismatches_vs_replay_REF": cmp(R, x) if m["idx"] != ref0["idx"] else [],
                 "ratios_record_equal_vs_replay_REF": ratios_counted(R) == ratios_counted(x)}
        if s in (1, 2, 3):
            cm = {}
            for p in (1, 2):
                for arm in ("REF", "COMBO"):
                    f = os.path.join(committed, "units", f"p{p}_{arm}_{q}_{s}.jsonl")
                    if os.path.exists(f):
                        cm[f"p{p}_{arm}"] = cmp(load(f), x)
            entry["mismatches_vs_committed"] = cm
        checks.append(entry)

# negative control on the comparator: perturb one counted field of one replayed COMBO DL record
neg = []
for (q, s) in sorted(by_unit)[:1] + sorted(by_unit)[-1:]:
    ms = by_unit[(q, s)]
    R = recs[[m for m in ms if m["arm"] == "REF"][0]["idx"]]
    C = copy.deepcopy(recs[[m for m in ms if m["arm"] == "COMBO"][0]["idx"]])
    for r in C:
        if r["rec"] == "ic" and r["arm"] == "DL":
            r["relations.mul"] += 1
            break
    C2 = copy.deepcopy(recs[[m for m in ms if m["arm"] == "COMBO"][0]["idx"]])
    for r in C2:
        if r["rec"] == "ic" and r["arm"] == "DL":
            r["relations.cpu"] *= 0.5  # timing field: must NOT be flagged
            break
    neg.append({"q": q, "seed": s, "perturbed_relations.mul+1_flags": cmp(R, C), "perturbed_relations.cpu_x0.5_flags": cmp(R, C2)})


def dlsum(x, f):
    return sum(r[f] for r in ics(x) if r["arm"] == "DL")


def dltot(x):
    return sum(r[p + ".cpu"] for r in ics(x) if r["arm"] == "DL" for p in PH)


timing = []
for (q, s), ms in by_unit.items():
    tags = {m["tag"] for m in ms}
    rows = []
    for m in ms:
        x = recs[m["idx"]]
        rows.append({"idx": m["idx"], "arm": m["arm"], "relations_cpu": dlsum(x, "relations.cpu"), "dl_total_cpu": dltot(x),
                     "D_total_cpu": sum(r[p + ".cpu"] for r in ics(x) if r["arm"] == "D" for p in PH),
                     "proc_cpu_s": m["utime_s"] + m["stime_s"], "wall_s": m["wall_s"], "maxrss_kb": m["maxrss_kb"],
                     "load1_before": float(m["loadavg_before"].split()[0]), "loadavg_before": m["loadavg_before"]})
    t = {"q": q, "seed": s, "tags": sorted(tags), "runs": rows}
    if "abba" in tags:
        a = [r for r in rows if r["idx"] in [m["idx"] for m in ms if m["tag"] == "abba"]]
        R1, C1, C2, R2 = a
        assert [R1["arm"], C1["arm"], C2["arm"], R2["arm"]] == ["REF", "COMBO", "COMBO", "REF"]
        for key in ("relations_cpu", "dl_total_cpu", "proc_cpu_s", "wall_s"):
            t[key] = {"pair1_C1/R1": C1[key] / R1[key], "pair2_C2/R2": C2[key] / R2[key],
                      "mean_of_pairs": (C1[key] / R1[key] + C2[key] / R2[key]) / 2,
                      "sum_ratio": (C1[key] + C2[key]) / (R1[key] + R2[key])}
        t["REF_R2/R1_relations_cpu"] = R2["relations_cpu"] / R1["relations_cpu"]
        t["COMBO_C2/C1_relations_cpu"] = C2["relations_cpu"] / C1["relations_cpu"]
        # internal-timer consistency: drop in process CPU vs drop in relations.cpu (seconds, mean over the pairs)
        t["delta_relations_cpu_s"] = ((R1["relations_cpu"] - C1["relations_cpu"]) + (R2["relations_cpu"] - C2["relations_cpu"])) / 2
        t["delta_proc_cpu_s"] = ((R1["proc_cpu_s"] - C1["proc_cpu_s"]) + (R2["proc_cpu_s"] - C2["proc_cpu_s"])) / 2
    if "aa" in tags:
        a = [r for r in rows if r["idx"] in [m["idx"] for m in ms if m["tag"] == "aa"]]
        t["aa_relations_cpu_REF2/REF"] = a[1]["relations_cpu"] / a[0]["relations_cpu"]
        t["aa_dl_total_cpu_REF2/REF"] = a[1]["dl_total_cpu"] / a[0]["dl_total_cpu"]
        t["aa_proc_cpu_REF2/REF"] = a[1]["proc_cpu_s"] / a[0]["proc_cpu_s"]
        refs = [r for r in rows if r["arm"] in ("REF", "REF2")]
        rc = [r["relations_cpu"] for r in refs]
        t["all_REF_relations_cpu_spread_max/min"] = max(rc) / min(rc)
    timing.append(t)

total_mm = sum(len(c["mismatches_vs_replay_REF"]) + sum(len(v) for v in c.get("mismatches_vs_committed", {}).values()) for c in checks)
res = {"checks": checks, "fields_compared_per_ic_arm": fields_compared, "negative_control": neg, "timing": timing,
       "total_mismatches": total_mm, "all_rc0": all(c["rc"] == 0 for c in checks),
       "all_ok_k_found": all(c["ok_and_k_found_eq_k"] for c in checks),
       "all_ratios_records_equal": all(c["ratios_record_equal_vs_replay_REF"] for c in checks)}
json.dump(res, open(out_json, "w"), indent=1)
print(json.dumps({k: res[k] for k in ("total_mismatches", "all_rc0", "all_ok_k_found", "all_ratios_records_equal")}))
print(json.dumps([{k: c[k] for k in ("idx", "arm", "q", "seed", "rc", "n_ic", "ok_and_k_found_eq_k")} | {"mm": len(c["mismatches_vs_replay_REF"]), "mm_committed": {k: len(v) for k, v in c.get("mismatches_vs_committed", {}).items()}} for c in checks]))
print(json.dumps(neg))
for t in timing:
    print(json.dumps({k: v for k, v in t.items() if k != "runs"}))
