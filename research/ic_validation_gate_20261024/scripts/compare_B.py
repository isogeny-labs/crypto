#!/usr/bin/env python3
"""Compare a native replay of Table B with the archive, field by field (stdlib only).

usage: compare_B.py REPO REPLAY_DIR OUT.json
Exact comparison of every field of every IC variant and every frozen-rho run except the clocks
(wall_ns, total_wall_ns, s_wall, elapsed). Then the matched-rho re-pricing of the replay against the
archived re-pricing (per-run gae, counters, recovered, verified). Instances are matched by position
within the regime (the replay names them by ICV1 slug, the archive by legacy name) and checked by
subgroup order r, generator and seeds.
"""
import json, sys
from pathlib import Path

CLOCK = {"wall_ns", "total_wall_ns", "s_wall", "elapsed_seconds"}
# `ratio_to_rho` changed definition between the archive (against the frozen plain walk, A = 1) and the current
# binary (against the matched negation walk of the same repetition); it is derived and is checked separately.
DERIVED = {"ratio_to_rho"}
REL = 1e-9   # the archive rounds some phase gae to 6 decimals; ulp-level differences are not mismatches
additive = set()   # counters present only in the replay, with value 0

def diff(a, b, path="", out=None, limit=20):
    out = [] if out is None else out
    if len(out) >= limit: return out
    if isinstance(a, dict) and isinstance(b, dict):
        for k in sorted(set(a) | set(b)):
            if k in CLOCK or k in DERIVED: continue
            if k not in a or k not in b:
                if k not in a and b[k] == 0 and "/native" in path:
                    additive.add(k)
                else:
                    out.append(f"{path}/{k}: only in {'archive' if k in a else 'replay'}")
            else:
                diff(a[k], b[k], f"{path}/{k}", out, limit)
    elif isinstance(a, list) and isinstance(b, list):
        if len(a) != len(b): out.append(f"{path}: length {len(a)} vs {len(b)}")
        else:
            for i, (x, y) in enumerate(zip(a, b)): diff(x, y, f"{path}[{i}]", out, limit)
    elif isinstance(a, float) or isinstance(b, float):
        try:
            if abs(a - b) > REL * max(1.0, abs(a), abs(b)): out.append(f"{path}: archive {a!r} replay {b!r}")
        except TypeError:
            out.append(f"{path}: archive {a!r} replay {b!r}")
    elif a != b:
        out.append(f"{path}: archive {a!r} replay {b!r}")
    return out

def main(repo, rdir, out_path):
    repo, rdir = Path(repo), Path(rdir)
    arch = json.load(open(repo / "docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json"))
    arch_ref = json.load(open(repo / "research/ic_rho_reference_20260923/reprice/ic-boundary-ledger-round5-2026-09-22.json"))
    arch_ref_by = {i["instance"]: i for i in arch_ref["instances"]}
    cur_ref = json.load(open(rdir / "archive.reprice.json"))
    cur_ref_by = {i["instance"]: i for i in cur_ref["instances"]}
    KEYS = ("gae", "counters", "recovered", "s", "s_walk", "verified")
    res = {"instances": [], "mismatches": 0, "runs_compared": 0,
           "archive_reprice_on_current_binary": {"identity": cur_ref["identity"], "binary_blake3": cur_ref["software"]["binary_blake3"]}}
    for reg in ("prime", "char2"):
        new = json.load(open(rdir / f"{reg}.json"))
        old_inst = [i for i in arch["ledger"]["instances"] if i["regime"] == reg]
        new_inst = [i for i in new["ledger"]["instances"] if i["regime"] == reg]
        assert len(old_inst) == len(new_inst), (reg, len(old_inst), len(new_inst))
        for o, n in zip(old_inst, new_inst):
            rec = {"archive_name": o["curve"]["name"], "replay_name": n["curve"]["name"], "regime": reg,
                   "r": o["r"], "same_r": o["r"] == n["r"], "same_generator": o["curve"]["generator"] == n["curve"]["generator"],
                   "same_seeds_targets": o["seeds"] == n["seeds"] and o["targets"] == n["targets"]}
            d_var = diff(o["variants"], n["variants"], "variants")
            frozen_new = [x for x in n["rho"] if x["automorphisms"] == 1]
            matched_new = [x for x in n["rho"] if x["automorphisms"] == 2]
            d_rho = diff(o["rho"], frozen_new, "frozen_rho")
            om = arch_ref_by[o["curve"]["name"]]["matched_reference"]
            d_m = diff([{k: x[k] for k in KEYS} for x in om["per_run"]],
                       [{k: x[k] for k in KEYS} for x in matched_new], "matched_rho")
            cm = cur_ref_by[o["curve"]["name"]]["matched_reference"]
            d_c = diff(om["per_run"], cm["per_run"], "archive_reprice_on_current_binary")
            nv = len(n["variants"]) // len(n["targets"]); nm = len(matched_new) // len(n["targets"])
            bad_ratio = 0
            for idx, v in enumerate(n["variants"]):
                blk = idx // nv
                mean_s = sum(x["s"] for x in matched_new[blk * nm:(blk + 1) * nm]) / nm
                if abs(v["ratio_to_rho"] - v["s"] / mean_s) > 1e-9 * v["ratio_to_rho"]: bad_ratio += 1
            if bad_ratio: d_var.append(f"ratio_to_rho != s / matched mean of its repetition on {bad_ratio} variants")
            rec.update({"variants": len(o["variants"]), "frozen_rho_runs": len(o["rho"]), "matched_rho_runs": len(om["per_run"]),
                        "variant_diffs": d_var, "frozen_rho_diffs": d_rho, "matched_rho_diffs": d_m,
                        "reprice_on_current_binary_diffs": d_c,
                        "matched_mean_s": {"archive": om["mean_s"], "replay_ledger": n["rho_s_mean"],
                                           "reprice_on_current_binary": cm["mean_s"],
                                           "equal": om["mean_s"] == n["rho_s_mean"] == cm["mean_s"]}})
            rec["identical"] = not (d_var or d_rho or d_m or d_c) and rec["same_r"] and rec["same_generator"] \
                and rec["same_seeds_targets"] and rec["matched_mean_s"]["equal"] and len(frozen_new) == len(o["rho"]) \
                and len(matched_new) == len(om["per_run"])
            res["runs_compared"] += len(o["variants"]) + len(o["rho"]) + 2 * len(om["per_run"])
            res["mismatches"] += (0 if rec["identical"] else 1)
            res["instances"].append(rec)
        res[f"{reg}_replay_binary_blake3"] = new["software"]["binary_blake3"]
    res["additive_zero_counters_only_in_replay"] = sorted(additive)
    res["all_identical"] = res["mismatches"] == 0 and cur_ref["identity"].get("all_reproduced") is True
    json.dump(res, open(out_path, "w"), indent=1)
    print("instances", len(res["instances"]), "identical", sum(r["identical"] for r in res["instances"]),
          "runs", res["runs_compared"], "reprice identity", cur_ref["identity"].get("all_reproduced"))
    for r in res["instances"]:
        if not r["identical"]:
            print(r["archive_name"], r["variant_diffs"][:3], r["frozen_rho_diffs"][:3], r["matched_rho_diffs"][:3], r["reprice_on_current_binary_diffs"][:3])

if __name__ == "__main__":
    main(*sys.argv[1:])
