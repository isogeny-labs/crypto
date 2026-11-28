#!/usr/bin/env python3
"""Analyse the sweep-window session against the frozen PROTOCOL.md.

usage: analyze.py <records.jsonl> [<band_default.json> <band_notable.json>] > results.json

Reads every key with [] so a missing field fails loudly (lesson L-99ee952350b2).
"""
import json
import math
import statistics
import sys
from collections import defaultdict

PRED = {  # PROTOCOL.md prediction 2: (curve bits label, a) -> {f: ratio}
    ("37.4", 2048): {2: 0.59, 4: 0.46, 8: 0.41, 16: 0.38},
    ("37.4", 4096): {2: 0.71, 4: 0.61, 8: 0.57, 16: 0.55},
    ("39.3", 2048): {2: 0.58, 4: 0.44, 8: 0.39, 16: 0.36},
    ("39.3", 4096): {2: 0.61, 4: 0.48, 8: 0.43, 16: 0.41},
}


def phase(r, name):
    return [p for p in r["phases"] if p["name"] == name][0]


def main():
    recs = [json.loads(l) for l in open(sys.argv[1])]
    rows = []
    for r in recs:
        if r["warmup"]:
            continue
        w = r["workload"]
        p = w["curve_spec"]["p"]
        rr = w["curve_spec"]["r"]
        base = dict(arm=r["arm"], round=r["round"], wid=w["workload_id"],
                    bits=f"{math.log2(rr):.1f}", p=p,
                    status=r["outcome"]["status"],
                    matches_planted=r["outcome"]["matches_planted"],
                    S=r["cost"]["s"], gae=r["cost"]["total_gae"])
        if r["arm"].startswith("rf3-"):
            lab, fa = r["arm"].split("-")[1], r["arm"].split("-")[2]
            rel = phase(r, "relations")
            la = phase(r, "linear_algebra")
            base.update(f=int(lab[1:]), a=int(fa[1:]),
                        trials=rel["native"]["trials"], relations=rel["native"]["relations"],
                        inversions=rel["native"]["inversions"], lookups=rel["native"]["lookups"],
                        sweep_adds=rel["adds"], setup_adds=phase(r, "oracle_setup")["adds"],
                        rows=la["native"]["rows"], row_ops=la["native"]["row_ops"])
        rows.append(base)

    out = {"records": len(rows),
           "verified": sum(1 for x in rows if x["status"] == "verified" and x["matches_planted"]),
           "rows": rows}

    # The algorithm seed is shared by every arm of a (workload, round), so arms are
    # paired on (workload, round); the A/A copy must equal the reference exactly.
    by = {(x["arm"], x["wid"], x["round"]): x for x in rows}
    aa = [(by[k]["S"], by[("rho-neg-aa",) + k[1:]]["S"]) for k in by if k[0] == "rho-neg"]
    out["aa_pairs"] = len(aa)
    out["aa_identity"] = all(a == b for a, b in aa)

    # Per (bits, a, f): mean S, ratio to f = 1, rows / a, yield check, rho gap.
    cells = defaultdict(list)
    rho = defaultdict(list)
    for x in rows:
        if x["arm"] == "rho-neg":
            rho[x["bits"]].append(x["S"])
        elif "f" in x:
            cells[(x["bits"], x["a"], x["f"])].append(x)
    table = []
    for (bits, a, f), v in sorted(cells.items()):
        b1 = cells[(bits, a, 1)]
        mS, mS1 = statistics.mean(y["S"] for y in v), statistics.mean(y["S"] for y in b1)
        per_wl = {(y["wid"], y["round"]): y["S"] for y in v}
        per_wl1 = {(y["wid"], y["round"]): y["S"] for y in b1}
        p1 = sum(y["relations"] for y in b1) / sum(y["trials"] for y in b1)
        mu = -math.log(1 - p1)
        frac = 1.0 / f
        pred_yield = 1 - math.exp(-mu * (1 - (1 - frac) ** 3))
        obs_yield = sum(y["relations"] for y in v) / sum(y["trials"] for y in v)
        table.append(dict(
            bits=bits, a=a, f=f, n=len(v), mean_S=mS, ratio_to_f1=mS / mS1,
            lower_on_every_workload=all(per_wl[w] < per_wl1[w] for w in per_wl) if f != 1 else None,
            predicted_ratio=PRED.get((bits, a), {}).get(f),
            S_over_rho=mS / statistics.mean(rho[bits]),
            max_rows_over_a=max(y["rows"] / a for y in v),
            mean_rows_over_a=statistics.mean(y["rows"] / a for y in v),
            trials=statistics.mean(y["trials"] for y in v),
            inversions=statistics.mean(y["inversions"] for y in v),
            row_ops=statistics.mean(y["row_ops"] for y in v),
            yield_observed=obs_yield, yield_predicted=pred_yield,
            yield_rel_err=obs_yield / pred_yield - 1))
    out["cells"] = table
    out["rho_mean_S"] = {b: statistics.mean(v) for b, v in rho.items()}

    f8 = [c for c in table if c["a"] == 4096 and c["f"] == 8]
    out["falsifiers"] = {
        "f8_a4096_ratio_above_0_75_on_both_curves": all(c["ratio_to_f1"] > 0.75 for c in f8),
        "rows_above_1_05a_any_candidate_record": any(
            x["rows"] / x["a"] > 1.05 for x in rows if x.get("f", 1) != 1),
        "any_verification_failure": out["verified"] != len(rows),
    }
    out["yield_within_5pct_all"] = all(abs(c["yield_rel_err"]) <= 0.05 for c in table)
    json.dump(out, sys.stdout, indent=1)


if __name__ == "__main__":
    main()
