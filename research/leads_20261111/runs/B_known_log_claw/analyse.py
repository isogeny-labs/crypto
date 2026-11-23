#!/usr/bin/env python3
"""Summarise the B_known_log_claw sessions into summary.json (BOUNDARY results)."""
import json, os, statistics as st
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "../../../.."))
board = {r["slug"]: r for r in json.load(open(os.path.join(ROOT, "docs/ic/leaderboard.json")))["board"]}
out = {"label": "BOUNDARY (BSGS-class known-log table arm; owner ruling 2026-11-08; outside the IC ratchet) [pcc:unverified]", "instances": []}
for c in ["f2m15", "f2m18", "f2m21", "f2m24", "f2m27"]:
    recs = [json.loads(l) for l in open(os.path.join(HERE, f"sessions/main-{c}/records.jsonl"))]
    recs = [r for r in recs if not r["warmup"]]
    slug = recs[0]["workload"]["curve"]["slug"]; r_ = recs[0]["workload"]["curve"]["r"]
    row = {"curve": c, "slug": slug, "r": r_, "arms": {}}
    for arm in ["rho-neg", "known-log", "known-log-aa"]:
        rs = [x for x in recs if x["arm"] == arm]
        a = {"runs": len(rs), "verified": sum(x["outcome"]["status"] == "verified" for x in rs),
             "matches_target": sum(bool(x["outcome"]["matches_target"]) for x in rs),
             "mean_S_w0": st.mean(x["cost"]["s"] for x in rs)}
        if arm != "rho-neg":
            ph = {}
            for x in rs:
                for p in x["phases"]:
                    ph[p["name"]] = ph.get(p["name"], 0) + p["gae"] / (r_ ** 0.5) / len(rs)
            a["phase_mean_S"] = {k: round(v, 4) for k, v in ph.items() if v}
            a["table_entries_T"] = rs[0]["counters"].get("pair_distinct_signed_classes")
            a["size"] = int(rs[0]["method"]["params"]["size"])
            a["mean_targets_tried"] = st.mean(x["counters"].get("targets_tried", 0) for x in rs)
        row["arms"][arm] = a
    for b in ["known-log", "known-log-aa"]:
        cmp_ = json.load(open(os.path.join(HERE, f"compare/{c}.rho-neg.{b}.json")))["ops"]
        row[f"{b}_over_rho"] = {"w": cmp_["band"]["weights"], "ratio": cmp_["band"]["ratio_b_over_a"],
                                "ci95_w0": cmp_["ci95"], "order": cmp_["band"]["order"]}
    aa = json.load(open(os.path.join(HERE, f"compare/{c}.known-log.known-log-aa.json")))["ops"]
    row["aa_ratio"] = {"ratio": aa["band"]["ratio_b_over_a"], "ci95_w0": aa["ci95"]}
    best = min(board[slug]["recipes"], key=lambda x: x["ratio_rho"])
    row["board"] = {"best_ratio_rho": best["ratio_rho"], "best_s": best["s"], "variant": best["variant"],
                    "reference_s": board[slug]["reference_s"]}
    out["instances"].append(row)
json.dump(out, open(os.path.join(HERE, "summary.json"), "w"), indent=1)
print("| curve | log2 r | s | T | rho-neg S | KL S | KL-aa S | KL/rho w=0 [CI] | w=0.1 | w=1 | w=1 drift | AA/KL | board best x |")
import math
for row in out["instances"]:
    k, aa, rh = row["arms"]["known-log"], row["arms"]["known-log-aa"], row["arms"]["rho-neg"]
    q = row["known-log_over_rho"]["ratio"]; ci = row["known-log_over_rho"]["ci95_w0"]
    print(f"| {row['curve']} | {math.log2(row['r']):.2f} | {k['size']} | {k['table_entries_T']} | {rh['mean_S_w0']:.3f} | {k['mean_S_w0']:.3f} | {aa['mean_S_w0']:.3f} | {q[0]:.3f} [{ci[0]:.2f}, {ci[1]:.2f}] | {q[1]:.3f} | {q[2]:.3f} | {q[2]/q[0]-1:+.1%} | {row['aa_ratio']['ratio'][0]:.3f} | {row['board']['best_ratio_rho']:.2f} |")
for row in out["instances"]:
    print(row["curve"], row["arms"]["known-log"]["phase_mean_S"], row["arms"]["known-log"]["mean_targets_tried"], "aa drift w1", round(row["known-log-aa_over_rho"]["ratio"][2]/row["known-log-aa_over_rho"]["ratio"][0]-1,4), "rho S board", round(row["board"]["reference_s"],3))
