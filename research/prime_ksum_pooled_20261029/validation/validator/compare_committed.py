#!/usr/bin/env python3
"""Step 4: compare the validator's own recompute (recompute.json) with the
committed analysis outputs and with the planner's preflight files. Written
after recompute.json existed; reads those files only as comparison targets."""
import json, math, re, sys
from collections import defaultdict
from pathlib import Path

ME = json.loads(Path(sys.argv[1]).read_text())
AN = Path(sys.argv[2])                  # study analysis/ dir
PF = Path(sys.argv[3])                  # scratchpad ksum dir (preflight files)
OUT = Path(sys.argv[4])
W = ["0", "0.1", "1"]

def rel(a, b):
    return abs(a - b) / max(abs(a), abs(b), 1e-300)

res = {}

# ---------------------------------------------------------------- compare-*.json
cmp_rows = []
for f in sorted(AN.glob("compare-P*-*__*-w*.json")):
    m = re.match(r"compare-P(\d+)-(.+?)__(.+?)-w([0-9.]+)\.json", f.name)
    size, A, B, w = m.group(1), m.group(2), m.group(3), m.group(4)
    w = {"0": "0", "0.1": "0.1", "1": "1"}[w]
    d = json.loads(f.read_text())
    ops = d["ops"]
    mine_pre = ME["ratios"][f"pre|P{size}|{A}__{B}|w{w}"]
    mine_post = ME["ratios"][f"post|P{size}|{A}__{B}|w{w}"]
    row = {"file": f.name, "size": int(size), "a": A, "b": B, "w": w,
           "committed_ratio": ops["ratio_b_over_a"], "mine_pre": mine_pre["ratio"], "mine_post": mine_post["ratio"],
           "rel_pre": rel(ops["ratio_b_over_a"], mine_pre["ratio"]),
           "rel_post": rel(ops["ratio_b_over_a"], mine_post["ratio"]),
           "pairs_committed": ops["pairs"], "pairs_mine": mine_pre["pairs"],
           "unpaired_committed": ops["unpaired_runs"], "same_seeds_committed": ops["same_seeds"],
           "same_seeds_mine": mine_pre["same_seeds"],
           "pricing_weight": d.get("pricing_weight"),
           "ci95_committed": ops.get("ci95"), "ci_method": ops.get("ci_method"),
           "ci95_mine_linearised": mine_pre["ci95_linearised_own"],
           "ci95_bootstrap_committed": ops.get("ci95_bootstrap"),
           "ci95_bootstrap_mine_own_rng": mine_pre.get("ci95_cluster_bootstrap_own_rng"),
           "interval_scope": ops.get("interval_scope"),
           "band_committed": ops.get("band", {}).get("ratio_b_over_a"),
           "band_order_committed": ops.get("band", {}).get("order"),
           "has_priced_table_work_key": any("table" in k for k in d.keys()) or any("table" in k for k in ops.get("band", {}).keys()),
           }
    row["ci_rel_lin"] = max(rel(a, b) for a, b in zip(row["ci95_committed"], row["ci95_mine_linearised"])) if row["ci95_committed"] else None
    if row["band_committed"]:
        row["band_rel_pre"] = max(rel(row["band_committed"][i], ME["ratios"][f"pre|P{size}|{A}__{B}|w{W[i]}"]["ratio"]) for i in range(3))
    # per-curve ratios inside the file vs per-curve sums recomputed from my band sums is not
    # identical (pairs vs means are the same here: every run paired) -> compare via mean_s
    cmp_rows.append(row)
res["compare_files"] = len(cmp_rows)
res["compare_max_rel_pre"] = max(r["rel_pre"] for r in cmp_rows)
res["compare_max_rel_post"] = max(r["rel_post"] for r in cmp_rows)
res["compare_max_rel_band_pre"] = max(r.get("band_rel_pre", 0) for r in cmp_rows)
res["compare_max_rel_ci_linearised"] = max(r["ci_rel_lin"] for r in cmp_rows if r["ci_rel_lin"] is not None)
res["compare_pairs_all_equal"] = all(r["pairs_committed"] == r["pairs_mine"] == 192 for r in cmp_rows)
res["compare_unpaired_all_zero"] = all(r["unpaired_committed"] == 0 for r in cmp_rows)
res["compare_same_seeds_all"] = all(r["same_seeds_committed"] and r["same_seeds_mine"] for r in cmp_rows)
res["compare_interval_scope"] = sorted({r["interval_scope"] for r in cmp_rows})
res["compare_ci_methods"] = sorted({r["ci_method"] for r in cmp_rows})
res["compare_any_table_work_key"] = any(r["has_priced_table_work_key"] for r in cmp_rows)
boot_dev = [max(abs(a - b) for a, b in zip(r["ci95_bootstrap_committed"], r["ci95_bootstrap_mine_own_rng"]))
            for r in cmp_rows if r["ci95_bootstrap_committed"] and r["ci95_bootstrap_mine_own_rng"]]
res["compare_bootstrap_max_abs_dev_own_rng"] = max(boot_dev) if boot_dev else None
res["compare_rows"] = cmp_rows

# ---------------------------------------------------------------- band.json
band = json.loads((AN / "band.json").read_text())
res["band_json_keys"] = list(band.keys())
res["band_json_priced_table_work_key"] = "priced_table_work" in band
bm = []
for row in band["rows"]:
    k = f"pre|{row['curve']}|{row['arm']}"
    mine = ME["band"].get(k)
    if mine is None:
        bm.append({"row": k, "missing_in_mine": True}); continue
    kp = f"post|{row['curve']}|{row['arm']}"
    minep = ME["band"][kp]
    bm.append({"row": k, "runs": (row["runs"], mine["runs"]), "verified": (row["verified"], mine["verified"]),
               "rel_mean_s_pre": max(rel(row["mean_s"][i], mine["mean_s"][W[i]]) for i in range(3)),
               "rel_mean_s_post": max(rel(row["mean_s"][i], minep["mean_s"][W[i]]) for i in range(3)),
               "charged_sum_equal": row.get("charged_sum") == mine["charged_sum"],
               "native_sum_equal_pre": row.get("native_sum") == mine["native_sum"],
               "native_sum_equal_post": row.get("native_sum") == minep["native_sum"]})
res["band_rows_committed"] = len(band["rows"])
res["band_rows_mine_pre"] = sum(1 for k in ME["band"] if k.startswith("pre|"))
res["band_max_rel_mean_s_pre"] = max(r["rel_mean_s_pre"] for r in bm if "rel_mean_s_pre" in r)
res["band_max_rel_mean_s_post"] = max(r["rel_mean_s_post"] for r in bm if "rel_mean_s_post" in r)
res["band_charged_sum_all_equal"] = all(r.get("charged_sum_equal") for r in bm)
res["band_native_sum_all_equal_pre"] = all(r.get("native_sum_equal_pre") for r in bm)
res["band_native_sum_equal_post_count"] = sum(1 for r in bm if r.get("native_sum_equal_post"))
res["band_runs_verified_equal"] = all(r["runs"][0] == r["runs"][1] and r["verified"][0] == r["verified"][1] for r in bm if "runs" in r)
# ranks: recompute from my pre band means, same rule (order holds at every weight)
my_rows = defaultdict(dict)
for k, v in ME["band"].items():
    reading, curve, arm = k.split("|")
    if reading == "pre":
        my_rows[curve][arm] = v
rank_mismatch = []
for rk in band["ranks"]:
    a, b = my_rows[rk["curve"]][rk["a"]], my_rows[rk["curve"]][rk["b"]]
    lt = all(a["mean_s"][w] < b["mean_s"][w] for w in W)
    gt = all(a["mean_s"][w] > b["mean_s"][w] for w in W)
    if a["verified"] < a["runs"] or b["verified"] < b["runs"]:
        v = "incomplete"
    elif lt:
        v = f"{rk['a']} < {rk['b']}"
    elif gt:
        v = f"{rk['b']} < {rk['a']}"
    else:
        v = "unresolved inside the band"
    if v != rk["verdict"]:
        rank_mismatch.append({"curve": rk["curve"], "a": rk["a"], "b": rk["b"], "committed": rk["verdict"], "mine": v})
res["band_ranks_committed"] = len(band["ranks"])
res["band_rank_mismatches"] = rank_mismatch
res["band_aa_verdicts"] = sorted({rk["verdict"] for rk in band["ranks"]
                                  if {rk["a"], rk["b"]} == {"rho-neg", "rho-neg-aa"}})

# ---------------------------------------------------------------- preflight files
def pf_compare(path, reading):
    d = json.loads(path.read_text())
    out = {"band_max_rel": 0.0, "pairs_max_rel": 0.0, "n_band": 0, "n_pairs": 0, "other": {}}
    for size, s in d["sizes"].items():
        for curve, c in s["band_per_curve"].items():
            for arm, v in c["arms"].items():
                mine = ME["band"][f"{reading}|{curve}|{arm}"]
                for w in W:
                    out["band_max_rel"] = max(out["band_max_rel"], rel(v[w], mine["mean_s"][w])); out["n_band"] += 1
        for key, v in s["pairs"].items():
            B, A = key.split("/")
            mk = f"{reading}|P{size}|{A}__{B}"
            if f"{mk}|w0" not in ME["ratios"]:
                out["other"][key] = "pair not in validator set"; continue
            for w in W:
                out["pairs_max_rel"] = max(out["pairs_max_rel"], rel(v[w], ME["ratios"][f"{mk}|w{w}"]["ratio"])); out["n_pairs"] += 1
        out["other"][f"P{size}"] = {k: s[k] for k in ("calibration_pinned", "measured", "records", "statuses", "unpriced_missing")}
    return out
res["preflight_pre"] = pf_compare(PF / "preflight.json", "pre")
res["preflight_tablework"] = pf_compare(PF / "preflight-tablework.json", "post")
g = json.loads((PF / "gae_band_per_curve.json").read_text())
gm = {"pre_2026_11_08": "pre", "table_work_default": "post"}
gres = {}
for key, reading in gm.items():
    mx = 0.0; order_agree = True
    for curve, v in g[key].items():
        for w in W:
            mx = max(mx, rel(v[w]["ic"], ME["band"][f"{reading}|{curve}|ksum5-sel"]["mean_s"][w]),
                     rel(v[w]["rho"], ME["band"][f"{reading}|{curve}|rho-neg"]["mean_s"][w]),
                     rel(v[w]["inc_sel"], ME["band"][f"{reading}|{curve}|inc-sel"]["mean_s"][w]))
        order_agree &= v["order_holds_across_band"] == ME["orders"][reading][curve]["holds_all_weights"]
    gres[key] = {"max_rel": mx, "order_flags_agree": order_agree, "curves": len(g[key])}
res["gae_band_per_curve"] = gres

OUT.write_text(json.dumps(res, indent=1))
for k, v in res.items():
    if k not in ("compare_rows",):
        print(k, ":", json.dumps(v)[:400])
