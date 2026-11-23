#!/usr/bin/env python3
"""Tie the producer's instances and recovered logarithms to the validator's
independent arithmetic.

Inputs: certs.json (validator: f, G, T, t from its own Cantor code and its own
re-implementation of the seeding), one or more dumper outputs (JSON lines from
val_g4dump, built from the a99e7267f1 tree), and the committed run.jsonl files.

Checks, per instance line: the producer's f, G, T, t equal the validator's.
Per dumped run: recovered == t; [recovered]G == T by the validator's own
arithmetic; the dumped counted fields equal the committed row of the same
(arm, p, curve_seed, target_seed, seed) (which ties the recovered log to the
row).  For the validator's window-free plain walk Q: same recovered log and
same group operations and steps as the registered plain walk P, and P's walk
counters show no short-cycle hit (cycles_2/4/other all zero).  [] indexing."""
import json, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from jac import Jac, random_f, trim

ROOT, CERTS, OUT = sys.argv[1], sys.argv[2], sys.argv[3]
DUMPS = sys.argv[4:]
certs = {(c["p"], c["curve_seed"], c["target_seed"]): c for c in json.load(open(CERTS))}
S1 = [json.loads(l) for l in open(f"{ROOT}/research/genus4_positive_control_20261116/run.jsonl")]
S2 = [json.loads(l) for l in open(f"{ROOT}/research/genus4_rho_refs_20261116/run.jsonl")]
ARM = {"c": "C_double_lp+pre", "v": "C_double_lp+pre+coord", "w": "C_double_lp+pre+coord+unitT",
       "R": "R_rho_neg", "P": "R_rho_plain"}
committed = {}
for rows in (S1, S2):
    for r in rows:
        if r["kind"] == "run":
            ts = r["target_seed"] if "target_seed" in r else r["curve_seed"]
            committed.setdefault((r["arm"], r["p"], r["curve_seed"], ts, r["seed"]), []).append(r)

res = {"instances": 0, "instance_fail": [], "runs": 0, "run_fail": [], "Q_vs_P": [], "P_counters": []}
jac_cache = {}
dumped = {}
for fn in DUMPS:
    for line in open(fn):
        d = json.loads(line)
        key3 = (d["p"], d["curve_seed"], d["target_seed"])
        c = certs[key3]
        if d["kind"] == "instance":
            res["instances"] += 1
            G = [tuple(d["G"][0]), trim(d["G"][1])]
            T = [tuple(d["T"][0]), trim(d["T"][1])]
            ok = (d["f"] == random_f(d["p"], 4, d["curve_seed"]) and d["t"] == c["t"]
                  and G == [tuple(c["G"][0]), tuple(c["G"][1])] and T == [tuple(c["T"][0]), tuple(c["T"][1])])
            if not ok:
                res["instance_fail"].append(key3)
            continue
        res["runs"] += 1
        p = d["p"]
        if p not in jac_cache or jac_cache[p][0] != d["curve_seed"]:
            jac_cache[p] = (d["curve_seed"], Jac(p, random_f(p, 4, d["curve_seed"])))
        J = jac_cache[p][1]
        Gd = (tuple(c["G"][0]), tuple(c["G"][1]))
        Td = (tuple(c["T"][0]), tuple(c["T"][1]))
        rec = d["recovered"]
        fails = []
        if rec != c["t"]:
            fails.append(("recovered != t", rec, c["t"]))
        if rec is None or J.mul(Gd, rec) != Td:
            fails.append("[recovered]G != T (validator arithmetic)")
        if d["certified"] is not True:
            fails.append("not certified")
        dumped[(d["arm"], key3, d["seed"])] = d
        if d["arm"] in ARM:
            k5 = (ARM[d["arm"]], p, d["curve_seed"], d["target_seed"], d["seed"])
            if k5 in committed:
                for cr in committed[k5]:
                    for f in ("group_ops", "rel_mul", "rel_add", "la_mul", "la_add", "trials", "relations",
                              "cantor_mul", "cantor_inv", "native", "steps"):
                        if f in cr and (f not in d or d[f] != cr[f]):
                            fails.append(("field != committed", f, d.get(f), cr[f]))
                    if "compares_uncharged" in cr and cr["compares_uncharged"] != d["compares"]:
                        fails.append(("compares != committed", d["compares"], cr["compares_uncharged"]))
            else:
                fails.append(("no committed row", k5)) if d["arm"] in "cvwR" and p != 8209 else None
        if d["arm"] in ("R", "P", "Q"):
            if d["direct_steps"] != d["steps"] or d["direct_recovered"] != rec:
                fails.append("direct walk call differs from run_rho_walk")
        if d["arm"] == "P":
            wc = d["walk_counters"]
            res["P_counters"].append({"key": [list(key3), d["seed"]],
                                      "cycles_2": wc["cycles_length_2"], "cycles_4": wc["cycles_length_4"],
                                      "cycles_other": wc["cycles_other_length"], "escapes": wc["cycle_escape_doublings"], "walks_capped": wc["walks_capped"], "abandoned": wc["walks_abandoned_in_a_cycle"],
                                      "work_native": d["walk_work_native"]})
        if fails:
            res["run_fail"].append({"run": [d["arm"], list(key3), d["seed"]], "fails": fails})
for (arm, key3, seed), d in dumped.items():
    if arm == "Q" and ("P", key3, seed) in dumped:
        P = dumped[("P", key3, seed)]
        res["Q_vs_P"].append({"key": [list(key3), seed], "same_recovered": d["recovered"] == P["recovered"],
                              "same_group_ops": d["group_ops"] == P["group_ops"], "same_steps": d["steps"] == P["steps"],
                              "Q_compares": d["compares"], "P_compares": P["compares"],
                              "Q_group_ops": d["group_ops"], "P_group_ops": P["group_ops"]})
json.dump(res, open(OUT, "w"), indent=1)
print("instances checked", res["instances"], "fails", res["instance_fail"])
print("runs checked", res["runs"], "fails", len(res["run_fail"]))
for f in res["run_fail"][:20]:
    print("  ", f)
for q in res["Q_vs_P"]:
    print("  Q vs P", q)
for c in res["P_counters"]:
    print("  P walk counters", c)
