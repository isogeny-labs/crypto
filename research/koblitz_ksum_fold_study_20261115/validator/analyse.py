#!/usr/bin/env python3
"""Validator's own numbers: band means, H1/H3 ratios, per-replicate rows, A/A,
fresh-session readings and the extra-arm fairness checks. Uses recompute.py."""
import json, sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import recompute as rc

SCR = pathlib.Path("/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/kf")
W = ["0", "0.1", "1"]


def study(curve, incumbents):
    labels = [f"{curve}-r{i}" for i in (1, 2, 3)]
    recs = []
    for l in labels:
        recs += rc.load(l, SCR / "sessions" / l)
    out = {"band_pooled": rc.band(recs)}
    out["band_per_replicate"] = {l: rc.band([r for r in recs if r["session"] == l]) for l in labels}
    rows = {}
    for wi, w in enumerate(W):
        pooled = {a: out["band_pooled"][a]["mean_S"][wi] for a in incumbents}
        strict = min(incumbents, key=lambda a: pooled[a])
        row = {"strictest_incumbent": strict, "incumbent_mean_S": pooled}
        row["kf3_over_strictest_pooled"] = rc.ratio(recs, strict, "kf3-c1g-sr", wi=wi, seed=11 + wi)
        row["kf3_over_strictest_per_replicate"] = {
            l: rc.ratio(recs, strict, "kf3-c1g-sr", sessions={l}, wi=wi, seed=21 + wi) for l in labels}
        row["kf3_over_each_incumbent"] = {a: rc.ratio(recs, a, "kf3-c1g-sr", wi=wi, seed=31 + wi) for a in incumbents}
        row["aa"] = rc.ratio(recs, "rho-strong", "rho-strong-aa", wi=wi, seed=41 + wi)
        for arm in ["kf3-c1g-sr", strict, "rho-neg"] + [a for a in out["band_pooled"] if a.startswith("k1-")]:
            row[f"{arm}_over_rho-strong"] = rc.ratio(recs, "rho-strong", arm, wi=wi, seed=51 + wi)
        row["kf3w_over_strictest"] = rc.ratio(recs, strict, "kf3-c1g-w", wi=wi, seed=61 + wi)
        if any(a.startswith("k1-") for a in out["band_pooled"]):
            k1 = [a for a in out["band_pooled"] if a.startswith("k1-")][0]
            row["kf3_over_k1"] = rc.ratio(recs, k1, "kf3-c1g-sr", wi=wi, seed=71 + wi)
        rows[w] = row
    out["rows"] = rows
    # same-seed check and candidate soundness
    out["candidate_records"] = {
        "verified": sum(1 for r in recs if r["arm"] == "kf3-c1g-sr" and r["status"] == "verified"),
        "all": sum(1 for r in recs if r["arm"] == "kf3-c1g-sr"),
        "frobfold_mismatches": sum(r["frobfold_mismatches"] for r in recs if r["arm"] == "kf3-c1g-sr"),
        "relations_per_run": sorted({r["relations"] for r in recs if r["arm"] == "kf3-c1g-sr"}),
        "table_entries": sorted({r["table_entries"] for r in recs if r["arm"] == "kf3-c1g-sr"}),
        "matches_target_all": all(r["matches_target"] for r in recs if r["arm"] == "kf3-c1g-sr" and r["status"] == "verified"),
    }
    out["errors"] = sorted({(r["arm"], (r["error"] or "")[:160]) for r in recs if r["status"] != "verified"})
    return out


def fresh():
    labels = ["F1", "F2", "F3"]
    recs = []
    for l in labels:
        d = SCR / "fresh" / l
        if (d / "records.jsonl").exists():
            recs += rc.load(l, d)
    out = {"band_pooled": rc.band(recs)}
    out["band_per_session"] = {l: rc.band([r for r in recs if r["session"] == l]) for l in labels}
    rows = {}
    ics = [a for a in out["band_pooled"] if not a.startswith("rho") and not a.startswith("kf3")]
    rhos = [a for a in out["band_pooled"] if a.startswith("rho") and a != "rho-strong-aa"]
    for wi, w in enumerate(W):
        reg = {a: out["band_pooled"][a]["mean_S"][wi] for a in ("lazy-c3", "lazy-c4")}
        strict = min(reg, key=reg.get)
        allic = {a: out["band_pooled"][a]["mean_S"][wi] for a in ics}
        best = min(allic, key=allic.get)
        rr = {a: out["band_pooled"][a]["mean_S"][wi] for a in rhos}
        bestrho = min(rr, key=rr.get)
        row = {"registered_strictest": strict,
               "kf3_over_registered_strictest": rc.ratio(recs, strict, "kf3-c1g-sr", wi=wi, seed=101 + wi),
               "kf3_over_registered_per_session": {l: rc.ratio(recs, strict, "kf3-c1g-sr", sessions={l}, wi=wi, seed=111 + wi) for l in labels},
               "best_existing_ic_arm": best, "ic_mean_S": allic,
               "kf3_over_best_existing_ic": rc.ratio(recs, best, "kf3-c1g-sr", wi=wi, seed=121 + wi),
               "kf3_over_best_existing_ic_per_session": {l: rc.ratio(recs, best, "kf3-c1g-sr", sessions={l}, wi=wi, seed=131 + wi) for l in labels},
               "kf3_over_rho-strong": rc.ratio(recs, "rho-strong", "kf3-c1g-sr", wi=wi, seed=141 + wi),
               "rho_mean_S": rr, "cheapest_rho": bestrho,
               "kf3_over_cheapest_rho": rc.ratio(recs, bestrho, "kf3-c1g-sr", wi=wi, seed=151 + wi),
               "cheapest_rho_over_rho-strong": rc.ratio(recs, "rho-strong", bestrho, wi=wi, seed=161 + wi),
               "kf3sg_over_kf3sr": rc.ratio(recs, "kf3-c1g-sr", "kf3-c1g-sg", wi=wi, seed=171 + wi),
               "aa": rc.ratio(recs, "rho-strong", "rho-strong-aa", wi=wi, seed=181 + wi)}
        for a in ics:
            row[f"kf3_over_{a}"] = rc.ratio(recs, a, "kf3-c1g-sr", wi=wi, seed=191 + wi)
        for a in rhos:
            row[f"kf3_over_{a}"] = rc.ratio(recs, a, "kf3-c1g-sr", wi=wi, seed=201 + wi)
        rows[w] = row
    out["rows"] = rows
    return out


if __name__ == "__main__":
    what = sys.argv[1]
    if what == "study":
        res = {"n23": study("n23", ["lazy-c3", "lazy-c4"]), "n29": study("n29", ["lazy-c1", "lazy-c2"])}
    else:
        res = fresh()
    pathlib.Path(sys.argv[2]).write_text(json.dumps(res, indent=1, default=list))
