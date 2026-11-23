#!/usr/bin/env python3
"""Validator: where the S change between two arms comes from (own code, stdlib).

For a pair of arms (a = baseline, b = candidate) over the 288 matched
(workload, round) pairs, writes the exact additive split

    sum S_b(w) / sum S_a(w) - 1 = sum_c  sum_pairs (c_b - c_a) / sqrt(r)  /  sum_pairs S_a(w)

over the components c:
  charged GAE  : factor_base, oracle_setup (= N (N + 1) additions),
                 relations_steps (one addition per trial step: stride_steps or
                 walk_steps), relations_other (the rest of the relations phase:
                 scalar multiplications, R0, stride renewals, walk pool
                 offsets, the oracle's confirming additions), linear_algebra,
                 verify;
  native counts: each name of the record's `cost.unpriced`, weighted by w.
The components add to the ratio exactly (checked).  Also reports per-pair
means of trials, relations, repeats, guard probes, lookups and memory, the
trial-count ratio against the section 2 model T ~ 1/N, own two-stage
bootstrap intervals (seed 29102026) for the count ratios, and the factorial
telescoping of the decision ratio through the control arms.

Usage: python3 -I mechanism.py <session_dir> <baseline_arm>   (prints JSON)
"""
import json
import math
import random
import sys
from collections import defaultdict

WEIGHTS = (0.0, 0.1, 1.0)
SEED = 29102026


def phase(rec, name):
    return next(p for p in rec["phases"] if p["name"] == name)


def unpriced(rec):
    out = {}
    for name in rec["cost"]["unpriced"]:
        base = name[:-len("_uncharged")] if name.endswith("_uncharged") else name
        if name in rec.get("counters", {}):
            out[base] = rec["counters"][name]
            continue
        tot, found = 0, False
        for p in rec["phases"]:
            n = p.get("native") or {}
            if name in n:
                tot += n[name]
                found = True
            elif base in n:
                tot += n[base]
                found = True
        assert found, (rec["record_id"], name)
        out[base] = tot
    return out


def components(rec):
    rel = phase(rec, "relations")
    n = rel["native"]
    steps = n.get("stride_steps", n.get("walk_steps", 0))
    c = {
        "factor_base": phase(rec, "factor_base")["gae"],
        "oracle_setup": phase(rec, "oracle_setup")["gae"],
        "relations_steps": float(steps),
        "relations_other": rel["gae"] - steps,
        "linear_algebra": phase(rec, "linear_algebra")["gae"],
        "verify": phase(rec, "verify")["gae"],
    }
    assert abs(sum(c.values()) - rec["cost"]["total_gae"]) < 1e-9, rec["record_id"]
    return c


def boot(cl, seed=SEED, resamples=4000):
    rng = random.Random(seed)
    ids = sorted(cl)
    vals = []
    for _ in range(resamples):
        a = b = 0.0
        for _ in ids:
            mem = cl[ids[rng.randrange(len(ids))]]
            for _ in mem:
                x, y = mem[rng.randrange(len(mem))]
                a += x
                b += y
        vals.append(b / a if a else float("nan"))
    vals.sort()
    return [vals[int(0.025 * resamples)], vals[int(0.975 * resamples) - 1]]


def main():
    sdir, baseline = sys.argv[1], sys.argv[2]
    M = defaultdict(dict)
    for line in open(sdir + "/records.jsonl"):
        x = json.loads(line)
        if x["warmup"] or x["method"]["id"] != "ic.pipeline":
            continue
        rel = phase(x, "relations")
        n = rel["native"]
        M[x["arm"]][(x["workload"]["workload_id"], x["round"])] = {
            "rec": x, "r": x["workload"]["curve"]["r"], "G": x["cost"]["total_gae"],
            "comp": components(x),
            "U": unpriced(x),
            "N": int(x["method"]["params"]["factor_base"].split("size=")[1]),
            "trials": n.get("trials"), "relations": n.get("relations"),
            "repeats": n.get("repeated_targets_skipped"), "probes": n.get("target_guard_probes"),
            "lookups": n.get("lookups"), "rel_adds": rel["adds"], "rel_doubles": rel["doubles"],
            "rel_scalar_mults": rel["scalar_mults"], "rss": x["time"]["max_rss_kib"],
        }
    keys = sorted(M[baseline])

    def split(a, b):
        out = {}
        for w in WEIGHTS:
            Sa = sum((M[a][k]["G"] + w * sum(M[a][k]["U"].values())) / math.sqrt(M[a][k]["r"]) for k in keys)
            Sb = sum((M[b][k]["G"] + w * sum(M[b][k]["U"].values())) / math.sqrt(M[b][k]["r"]) for k in keys)
            contrib = defaultdict(float)
            for k in keys:
                sq = math.sqrt(M[a][k]["r"])
                for c in M[a][k]["comp"]:
                    contrib["charged:" + c] += (M[b][k]["comp"][c] - M[a][k]["comp"][c]) / sq
                for u in set(M[a][k]["U"]) | set(M[b][k]["U"]):
                    contrib["native:" + u] += w * (M[b][k]["U"].get(u, 0) - M[a][k]["U"].get(u, 0)) / sq
            rel = {c: v / Sa for c, v in sorted(contrib.items())}
            out[str(w)] = {"ratio": Sb / Sa, "ratio_minus_1": Sb / Sa - 1,
                           "sum_of_contributions": sum(rel.values()),
                           "split_closes": abs(sum(rel.values()) - (Sb / Sa - 1)) < 1e-12,
                           "contribution_to_ratio_minus_1": rel,
                           "charged_total": sum(v for c, v in rel.items() if c.startswith("charged:")),
                           "native_total": sum(v for c, v in rel.items() if c.startswith("native:"))}
        return out

    def means(arm):
        d = M[arm]
        m = {f: sum(d[k][f] for k in keys) / len(keys) for f in
             ("G", "trials", "relations", "repeats", "probes", "lookups", "rel_adds", "rel_doubles",
              "rel_scalar_mults", "rss")}
        for c in d[keys[0]]["comp"]:
            m["gae:" + c] = sum(d[k]["comp"][c] for k in keys) / len(keys)
        for u in d[keys[0]]["U"]:
            m["native:" + u] = sum(d[k]["U"][u] for k in keys) / len(keys)
        m["N"] = d[keys[0]]["N"]
        m["oracle_setup_eq_N_N_plus_1_all"] = all(d[k]["comp"]["oracle_setup"] == d[k]["N"] * (d[k]["N"] + 1) for k in keys)
        m["native_per_trial"] = sum(sum(d[k]["U"].values()) for k in keys) / sum(d[k]["trials"] for k in keys)
        return m

    def count_ratio(a, b, f):
        cl = defaultdict(list)
        for k in keys:
            cl[k[0]].append((M[a][k][f], M[b][k][f]))
        ta = sum(M[a][k][f] for k in keys)
        tb = sum(M[b][k][f] for k in keys)
        return {"ratio": tb / ta if ta else None, "ci95_own_two_stage": boot(cl) if ta else None}

    ic_arms = sorted(M)  # IC arms only (rho records are skipped above)
    out = {"baseline": baseline, "pairs": len(keys), "means": {a: means(a) for a in ic_arms}}
    out["split"] = {f"{baseline}__dg-small": split(baseline, "dg-small"),
                    "srenew__dg": split("srenew", "dg"),
                    "srenew__srenew-small": split("srenew", "srenew-small"),
                    "srenew-small__dg-small": split("srenew-small", "dg-small")}
    if baseline != "srenew":
        out["split"][f"{baseline}__srenew"] = split(baseline, "srenew")
    out["count_ratios"] = {}
    for a, b in ((baseline, "dg-small"), ("srenew", "srenew-small"), ("srenew", "dg"), ("srenew-small", "dg-small")):
        out["count_ratios"][f"{a}__{b}"] = {f: count_ratio(a, b, f) for f in ("trials", "relations", "repeats", "G")}
    Ni, Nb = M["srenew"][keys[0]]["N"], M["srenew-small"][keys[0]]["N"]
    out["trial_model"] = {"N_inc": Ni, "N_b": Nb, "model_trials_ratio_N_inc_over_N_b": Ni / Nb,
                          "measured_trials_ratio_srenew_small_over_srenew": out["count_ratios"]["srenew__srenew-small"]["trials"],
                          "measured_relations_ratio_srenew_small_over_srenew": out["count_ratios"]["srenew__srenew-small"]["relations"],
                          "setup_ratio_N_b_over_N_inc": Nb * (Nb + 1) / (Ni * (Ni + 1))}
    # telescoping through the control arms (ratio of totals over the same pairs)
    tel = {}
    for w in WEIGHTS:
        def tot(arm):
            return sum((M[arm][k]["G"] + w * sum(M[arm][k]["U"].values())) / math.sqrt(M[arm][k]["r"]) for k in keys)
        T = {a: tot(a) for a in ic_arms}
        path1 = {"targets_at_N_inc(srenew/" + baseline + ")": T["srenew"] / T[baseline],
                 "N_alone(srenew-small/srenew)": T["srenew-small"] / T["srenew"],
                 "guard_at_N_b(dg-small/srenew-small)": T["dg-small"] / T["srenew-small"]}
        path2 = {"targets_at_N_inc(srenew/" + baseline + ")": T["srenew"] / T[baseline],
                 "guard_at_N_inc(dg/srenew)": T["dg"] / T["srenew"],
                 "N_with_deferred_guard(dg-small/dg)": T["dg-small"] / T["dg"]}
        tel[str(w)] = {"decision_ratio": T["dg-small"] / T[baseline],
                       "path_N_then_guard": path1, "product_path1": math.prod(path1.values()),
                       "path_guard_then_N": path2, "product_path2": math.prod(path2.values())}
    out["telescoping"] = tel
    print(json.dumps(out, indent=1, sort_keys=True))


if __name__ == "__main__":
    main()
