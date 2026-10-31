#!/usr/bin/env python3
"""E34 (§9.38/§9.39) tables: identity against E29's records, per-step costs,
the §5a band, rankings.  Reads experiments/38_twist24_cheap_rep/*.json and
E29's experiments/34_twist24_stage_r/stage_r_*.json; writes tables.md and
summary.json next to this script.  Standard library only."""
import glob
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
E29 = os.path.join(ROOT, "experiments", "34_twist24_stage_r")
WEIGHTS = [0.0, 0.1, 1.0]
GAE = 97
GAE_KARATSUBA_OWN = 79
ARMS = ["Order6Built", "Order6Cheap", "Order24Built", "Order24Cheap", "Order24CheapMulLean"]
KINDS = ["cmp", "neg", "modn", "hash", "table"]
WALK_FIELDS = ["steps", "group_ops", "cycle_escapes", "recovered", "correct", "visit_digest"]


def load(path):
    with open(path) as fh:
        return json.load(fh)


def e29_records():
    out = {}
    for path in sorted(glob.glob(os.path.join(E29, "stage_r_*.json"))):
        for row in load(path):
            for run in row["runs"]:
                for arm, key in (("Order6Built", "order6"), ("Order24Built", "order24")):
                    for w in run[key]:
                        out[(row["p"], run["karatsuba"], arm, w["seed"])] = w
    return out


def main():
    e29 = e29_records()
    rows = []
    for path in sorted(glob.glob(os.path.join(HERE, "stage_r_p*.json"))):
        rows.extend(load(path))
    rows.sort(key=lambda r: r["p"])
    walks = {}
    curves = {}
    for row in rows:
        curves[row["p"]] = row
        for w in row["walks"]:
            walks[(row["p"], w["karatsuba"], w["arm"], w["run"]["seed"])] = w

    # (a) the counted copies of E29's walks against E29's records.
    replay = {"records": 0, "identical": 0, "differences": []}
    for key, rec in sorted(e29.items(), key=lambda kv: str(kv[0])):
        if key not in walks:
            continue
        mine = dict(walks[key]["run"])
        theirs = dict(rec)
        if key[2] == "Order24Built":
            mine["visit_digest"] = theirs.get("visit_digest", 0)
        replay["records"] += 1
        if mine == theirs:
            replay["identical"] += 1
        else:
            replay["differences"].append({"key": list(key), "fields": sorted(k for k in theirs if mine.get(k) != theirs[k])})

    # (c) cheap walks against the built walk of their order.
    identity = {"compared": 0, "identical": 0, "differences": []}
    for (p, kar, arm, seed), w in walks.items():
        ref = {"Order6Cheap": "Order6Built", "Order24Cheap": "Order24Built",
               "Order24CheapMulLean": "Order24Built"}.get(arm)
        if ref is None:
            continue
        r = walks[(p, kar, ref, seed)]["run"]
        identity["compared"] += 1
        same = all(w["run"][f] == r[f] for f in WALK_FIELDS)
        if same:
            identity["identical"] += 1
        else:
            identity["differences"].append([p, kar, arm, seed])
    # Visit digests also equal across arithmetics.
    digests_across_arith = all(
        walks[(p, False, a, s)]["run"]["visit_digest"] == walks[(p, True, a, s)]["run"]["visit_digest"]
        for (p, kar, a, s) in walks if not kar and (p, True, a, s) in walks)

    # Per curve, arithmetic, arm.
    per = {}
    for (p, kar, arm, seed), w in walks.items():
        per.setdefault((p, kar, arm), []).append(w)
    stats = {}
    for key, ws in per.items():
        m = len(ws)
        mean = lambda f: sum(f(w) for w in ws) / m
        steps = mean(lambda w: w["run"]["steps"])
        calls = mean(lambda w: w["rep_calls"])
        s = {
            "walks": m,
            "all_correct": all(w["run"]["correct"] for w in ws),
            "steps": steps,
            "muls": mean(lambda w: w["run"]["fp_muls"] - w["run"]["verify_fp_muls"]),
            "native": mean(lambda w: sum(w["ops"][k] for k in KINDS)),
            "native_by_kind": {k: mean(lambda w, k=k: w["ops"][k]) for k in KINDS},
            "rep_calls": calls,
            "rep_muls_per_call": mean(lambda w: w["rep_fp_muls"]) / calls,
            "rep_ops_per_call": {k: mean(lambda w, k=k: w["rep_ops"][k]) / calls for k in ("cmp", "neg", "modn")},
            "table_fp_muls": mean(lambda w: w["table_fp_muls"]),
        }
        s["muls_per_step"] = s["muls"] / steps
        s["native_per_step"] = s["native"] / steps
        s["native_per_step_by_kind"] = {k: v / steps for k, v in s["native_by_kind"].items()}
        stats[key] = s

    def cost(s, w, gae, convention):
        native = s["native"] if convention == "full" else s["steps"]
        return s["muls"] + w * gae * native

    ps = sorted(curves)
    sqrtn = {p: math.sqrt(curves[p]["n"]) for p in ps}

    def ratios(a6, a24, kar, convention="full", gae=GAE):
        out = []
        for w in WEIGHTS:
            per_curve = [cost(stats[(p, kar, a6)], w, gae, convention) / cost(stats[(p, kar, a24)], w, gae, convention) for p in ps]
            pooled = sum(cost(stats[(p, kar, a6)], w, gae, convention) / sqrtn[p] for p in ps) / \
                sum(cost(stats[(p, kar, a24)], w, gae, convention) / sqrtn[p] for p in ps)
            out.append({"weight": w, "pooled": pooled, "per_curve": dict(zip(map(str, ps), per_curve)),
                        "curves_order24_cheaper": sum(r > 1 for r in per_curve)})
        return out

    def holds(rs):
        side = [r["pooled"] > 1 for r in rs]
        return all(side) or not any(side)

    pairs = [("Order6Built", "Order24Built"), ("Order6Built", "Order24Cheap"), ("Order6Cheap", "Order24Cheap"),
             ("Order6Built", "Order24CheapMulLean"), ("Order6Cheap", "Order24CheapMulLean")]
    band = {}
    for a6, a24 in pairs:
        for kar in (False, True):
            for conv in ("full", "one_per_step"):
                gaes = [GAE] + ([GAE_KARATSUBA_OWN] if kar and conv == "full" else [])
                for gae in gaes:
                    rs = ratios(a6, a24, kar, conv, gae)
                    k = f"{a6}/{a24}|{'karatsuba' if kar else 'schoolbook'}|{conv}|gae{gae}"
                    band[k] = {"rows": rs, "order_holds_across_band": holds(rs),
                               "per_curve_holds": {str(p): (all(r["per_curve"][str(p)] > 1 for r in rs) or
                                                            all(r["per_curve"][str(p)] < 1 for r in rs)) for p in ps},
                               "per_curve_order24_cheaper_everywhere": sum(all(r["per_curve"][str(p)] > 1 for r in rs) for p in ps)}

    # Strongest walk of each order at each weight (family minimum, pooled).
    strongest = {}
    for kar in (False, True):
        lab = "karatsuba" if kar else "schoolbook"
        rows_s = []
        for w in WEIGHTS:
            def pooled(arm):
                return sum(cost(stats[(p, kar, arm)], w, GAE, "full") / sqrtn[p] for p in ps)
            best6 = min(("Order6Built", "Order6Cheap"), key=pooled)
            best24 = min(("Order24Built", "Order24Cheap", "Order24CheapMulLean"), key=pooled)
            r = pooled(best6) / pooled(best24)
            rows_s.append({"weight": w, "order6": best6, "order24": best24, "pooled": r,
                           "e18_distance_factor": r if r > 1 else 1.0,
                           "e18_handover_shift_bits": 4 * math.log2(r) if r > 1 else 0.0})
        strongest[lab] = rows_s

    # Negative control walks.
    neg = {"walks": 0, "digest_differs": 0}
    npath = os.path.join(HERE, "negative_walks.json")
    if os.path.exists(npath):
        for row in load(npath):
            for w in row["walks"]:
                ref = walks.get((row["p"], w["karatsuba"], "Order24Built", w["run"]["seed"]))
                if ref is None:
                    continue
                neg["walks"] += 1
                neg["digest_differs"] += w["run"]["visit_digest"] != ref["run"]["visit_digest"]
                neg.setdefault("correct", 0)
                neg["correct"] += w["run"]["correct"]
    ident = {}
    ipath = os.path.join(HERE, "identity.json")
    if os.path.exists(ipath):
        for row in load(ipath):
            for c in row["checks"]:
                ident.setdefault(c["arm"], []).append({"p": row["p"], "members": c["members"], "mismatches": c["mismatches"],
                                                       "forced_x0_zero": c["forced_x0_zero"], "x_zero_point": c["x_zero_point"],
                                                       "scalar_checks": c["scalar_checks"], "scalar_failures": c["scalar_failures"]})

    summary = {
        "experiment": "E34", "note": "§9.38/§9.39", "curves": ps,
        "gae": GAE, "weights": WEIGHTS,
        "band_definition": "cost(w) = mean F_p multiplications (fp_muls - verify_fp_muls) + w * GAE * mean native operations "
                           "(cmp+neg+modn+hash+table); 'one_per_step' replaces native operations by steps (the E29 validator's "
                           "convention); pooled = sum over curves of cost/sqrt(n), order 6 over order 24",
        "e29_replay": replay, "walk_identity": identity, "digests_equal_across_arithmetics": digests_across_arith,
        "identity_checks": ident, "negative_control_walks": neg,
        "per_arm": {f"{p}|{'karatsuba' if kar else 'schoolbook'}|{arm}": s for (p, kar, arm), s in sorted(stats.items(), key=lambda kv: str(kv[0]))},
        "band": band, "strongest": strongest,
    }
    with open(os.path.join(HERE, "summary.json"), "w") as fh:
        json.dump(summary, fh, indent=1, sort_keys=True)

    L = ["# E34 tables (§9.39)", "", "Generated by `tables.py` from this directory's records and E29's Stage R records.", ""]
    L.append(f"E29 replay: {replay['identical']}/{replay['records']} records identical. "
             f"Walk identity: {identity['identical']}/{identity['compared']} cheap walks equal their built walk "
             f"({', '.join(WALK_FIELDS)}). Digests equal across arithmetics: {digests_across_arith}. "
             f"Negative-control walks: digest differs on {neg['digest_differs']}/{neg['walks']}.")
    L.append("")
    L.append("## Identity checks (every class member of each point)")
    L.append("")
    L.append("| arm | curves | members | mismatches | forced x₀=0 points | scalar checks (failures) |")
    L.append("|:--|--:|--:|--:|--:|--:|")
    for arm, cs in ident.items():
        L.append(f"| {arm} | {len(cs)} | {sum(c['members'] for c in cs)} | {sum(c['mismatches'] for c in cs)} | "
                 f"{sum(c['forced_x0_zero'] for c in cs)} | {sum(c['scalar_checks'] for c in cs)} ({sum(c['scalar_failures'] for c in cs)}) |")
    for kar in (False, True):
        lab = "karatsuba" if kar else "schoolbook"
        L += ["", f"## Per-step costs, {lab} (means over 16 walks)", "",
              "| p | arm | steps | muls/step | rep muls/call | native/step | cmp | neg | modn | hash | table | rep cmp/call |",
              "|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|"]
        for p in ps:
            for arm in ARMS:
                s = stats[(p, kar, arm)]
                k = s["native_per_step_by_kind"]
                L.append(f"| {p} | {arm} | {s['steps']:.0f} | {s['muls_per_step']:.2f} | {s['rep_muls_per_call']:.2f} | "
                         f"{s['native_per_step']:.2f} | {k['cmp']:.2f} | {k['neg']:.2f} | {k['modn']:.2f} | {k['hash']:.2f} | "
                         f"{k['table']:.2f} | {s['rep_ops_per_call']['cmp']:.2f} |")
    L += ["", "## The §5a band: order 6 / order 24 (> 1: order 24 cheaper), pooled and per curve", ""]
    for k, v in band.items():
        L.append(f"**{k}** — order holds across the band: {v['order_holds_across_band']}; curves where order 24 is cheaper "
                 f"at every weight: {v['per_curve_order24_cheaper_everywhere']}/8")
        L.append("")
        L.append("| GAE | pooled | " + " | ".join(str(p) for p in ps) + " |")
        L.append("|--:|--:|" + "--:|" * len(ps))
        for r in v["rows"]:
            L.append(f"| {r['weight']} | {r['pooled']:.3f} | " + " | ".join(f"{r['per_curve'][str(p)]:.3f}" for p in ps) + " |")
        L.append("")
    L += ["## Strongest walk of each order (pooled, full emission, GAE 97)", "",
          "| arithmetic | GAE | order 6 | order 24 | ratio | E18 distance factor | handover shift (bits) |", "|:--|--:|:--|:--|--:|--:|--:|"]
    for lab, rs in strongest.items():
        for r in rs:
            L.append(f"| {lab} | {r['weight']} | {r['order6']} | {r['order24']} | {r['pooled']:.3f} | {r['e18_distance_factor']:.3f} | {r['e18_handover_shift_bits']:.2f} |")
    with open(os.path.join(HERE, "tables.md"), "w") as fh:
        fh.write("\n".join(L) + "\n")
    print("\n".join(L[:6]))


if __name__ == "__main__":
    main()
