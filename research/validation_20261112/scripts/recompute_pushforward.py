#!/usr/bin/env python3
"""Recompute PUSHFORWARD.md headline numbers from pushforward.json / pushforward_15seeds.json."""
import json, sys, statistics as st
M = (1 << 64) - 1
prim = json.load(open(sys.argv[1])); sup = json.load(open(sys.argv[2]))
out = {"rows": [], "errors": []}
def med(v): return st.median(v)
tot = {"relations": 0, "e1_invalid": 0, "e2_invalid": 0, "e2_rows_carrying_target": 0, "fibre_type_e1": 0}
for name, doc, nseeds in (("primary", prim, 3), ("15seeds", sup, 15)):
    T = dict.fromkeys(tot, 0)
    for row in doc["rows"]:
        p, r = row["p"], row["r"]
        assert len(row["seeds"]) == nseeds, (p, len(row["seeds"]))
        for k, s in enumerate(row["seeds"]):
            assert s["seed"] == row["seed0"] ^ (k * 0x1000)
            x = ((s["seed"] * 2246822519 + 7) & M) % r
            if s["x"] != x: out["errors"].append(f"{name} p={p} seed {k}: x {s['x']} != formula {x}")
            logs = [s["ic_jc"]["recovered"], s["ic_e1_fibre"]["run"]["recovered"], s["ic_e1_two_term"]["run"]["recovered"],
                    s["birthday_e1"]["log"], s["ic_jc"]["first_collision"]["log"], s["ic_e1_fibre"]["first_collision"]["log"]]
            if any(l != x for l in logs): out["errors"].append(f"{name} p={p} seed {k}: logs {logs} != x {x}")
            if not s["rho"]["recovered_ok"]: out["errors"].append(f"{name} p={p} seed {k}: rho not ok")
            if not all(a["certified"] for a in s["arms"].values()): out["errors"].append(f"{name} p={p} seed {k}: arm uncertified")
            if not s["transfer_ok"]: out["errors"].append(f"{name} p={p} seed {k}: transfer not ok")
            if not s["ic_e1_fibre"]["accepted_trials_equal_jc"]: out["errors"].append(f"{name} p={p} seed {k}: P7 fails")
            if s["ic_e1_fibre"]["run"]["group_ops"] != s["ic_jc"]["group_ops"]: out["errors"].append(f"{name} p={p} seed {k}: A1/A4 ops differ")
            if s["ic_jc"]["distinct_columns_used"] != 2 * s["ic_e1_fibre"]["run"]["cols"]: out["errors"].append(f"{name} p={p} seed {k}: cols not 2x")
            pu = s["pushed"]
            for key in T: T[key] += pu[key]
            # totals consistency
            for an, a in s["arms"].items():
                for w, kk in ((0, "w0"), (0.1, "w0.1"), (1, "w1")):
                    v = a["group_ops"] + w * a["la_mulmods"]
                    if abs(v - a["totals"][kk]) > 1e-6 * max(1, v): out["errors"].append(f"{name} p={p} {an} total {kk}")
                    rr = v / s["rho"]["ops"]
                    if abs(rr - a["totals"]["ratio_" + kk]) > 1e-9 * max(1, rr): out["errors"].append(f"{name} p={p} {an} ratio {kk}")
        if name == "primary":
            # median ops and median ratio
            ops = {"rho_e1": [s["rho"]["ops"] for s in row["seeds"]], "ic_jc": [s["ic_jc"]["group_ops"] for s in row["seeds"]],
                   "ic_e1_two_term": [s["ic_e1_two_term"]["run"]["group_ops"] for s in row["seeds"]]}
            for k2, v in ops.items():
                if med(v) != row["median_ops"][k2]: out["errors"].append(f"p={p} median_ops {k2} {med(v)} != {row['median_ops'][k2]}")
            rat = {}
            for arm in ("ic_jc", "ic_e1_fibre_full", "ic_e1_two_term"):
                rat[arm] = [round(med([s["arms"][arm]["totals"]["ratio_" + kk] for s in row["seeds"]]), 2) for kk in ("w0", "w0.1", "w1")]
            b = row["b"]
            out["rows"].append({"p": p, "r": r, "e1": row["e1_order"], "B": b["b_size"], "B_from_e1": b["b_size_from_e1"],
                                "B/E1": round(b["b_size"] / row["e1_order"], 4), "Bsub/r": round(row["b_in_subgroup"] / r, 4),
                                "fibre": row["fibre_hist"], "median_rho": med(ops["rho_e1"]), "median_icjc": med(ops["ic_jc"]),
                                "median_2term": med(ops["ic_e1_two_term"]), "gap": rat})
            if b["b_size"] != b["b_size_from_e1"]: out["errors"].append(f"p={p} |B| mismatch")
        else:
            fc = []
            for row2 in [row]:
                rho = [s["rho"]["ops"] for s in row2["seeds"]]
                jc = [s["ic_jc"]["first_collision"]["ops"] for s in row2["seeds"]]
                bd = [s["birthday_e1"]["ops"] for s in row2["seeds"]]
                below_jc = sum(1 for a, b2 in zip(jc, rho) if a < b2); below_bd = sum(1 for a, b2 in zip(bd, rho) if a < b2)
                two = [s["ic_e1_two_term"]["run"]["group_ops"] for s in row2["seeds"]]
                below_two = sum(1 for a, b2 in zip(two, rho) if a < b2)
                full_below = sum(1 for s in row2["seeds"] if s["ic_jc"]["group_ops"] < s["rho"]["ops"])
                out.setdefault("firstcol", []).append({"p": p, "rho_med": med(rho), "rho_rng": (min(rho), max(rho)),
                    "jc_med": med(jc), "jc_rng": (min(jc), max(jc)), "jc_below": below_jc, "bd_med": med(bd), "bd_rng": (min(bd), max(bd)),
                    "bd_below": below_bd, "two_term_below_same_seed": below_two, "full_jc_below": full_below,
                    "cover_ge_control_all_seeds": all(a >= b2 for a, b2 in zip(jc, bd))})
    out["totals_" + name] = T
json.dump(out, open(sys.argv[3], "w"), indent=1, default=str)
for r in out["rows"]: print(r)
for r in out["firstcol"]: print(r)
print("totals", out["totals_primary"], out["totals_15seeds"])
print("errors", len(out["errors"]), out["errors"][:10])
