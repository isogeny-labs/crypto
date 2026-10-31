#!/usr/bin/env python3
"""Independent validator for leaderboard Table B (prime and random-binary ladder).

Reads only committed raw records:
  - docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json         (IC variants, frozen rho)
  - research/ic_rho_reference_20260923/reprice/<same name>.json     (matched negation-map rho)
and compares with the board rows of docs/ic/leaderboard.json (the claims).
Does not import or call the builder, the sweep's jq scripts, or the ic binary.

Recomputes per instance: best recipe (argmin over recipes of the mean over repetitions of
total_gae), S_IC = mean total_gae / sqrt r, S_ref = mean matched-rho gae / sqrt r, x reference,
x floor, the decomposition check of every phase's gae, and the section 5a band at
w in {0, 0.1, 1} in two versions (repository formula; conservative extension).

usage: validate_B.py REPO_ROOT OUT.json [LEDGER|- REPRICE|- [EVIDENCE_DIR]]
raw_data paths are written relative to EVIDENCE_DIR (the gate resolves them there); repo-relative if omitted.
"""
import hashlib, json, math, sys
from collections import defaultdict
from pathlib import Path

PRICED = [("sqrt_solves", "ns_per_sqrt"), ("as_solves", "ns_per_as_solve"), ("s4_pairs", "ns_per_s4_pair"),
          ("lookups", "ns_per_lookup"), ("row_ops", "ns_per_row_op"), ("legendre_symbols", "ns_per_legendre"),
          ("inversions", "ns_per_inversion"), ("frobenius_maps", "ns_per_frobenius"),
          ("canonicalisations", "ns_per_canon"), ("target_guard_probes", "ns_per_lookup")]
PHASES = ("factor_base", "relations", "linear_algebra", "verify")
WS = (0.0, 0.1, 1.0)
FLOOR = math.sqrt(math.pi / 4)

def sha(p): return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def mean(xs): xs = list(xs); return sum(xs) / len(xs)

def analyse(inst, ref_inst):
    r = float(inst["r"]); q = math.sqrt(r)
    cal, pin = inst["calibration"], set(inst["calibration_pinned"]["pinned"])
    add_ns = cal["ns_per_add"]
    groups = defaultdict(list)
    for v in inst["variants"]: groups[v["name"]].append(v)
    best_name = min(groups, key=lambda n: mean(v["total_gae"] for v in groups[n]))
    best = groups[best_name]
    out = {"curve": inst["curve"]["name"], "best_recipe": best_name, "reps": len(best), "r": r}
    out["S_IC"] = mean(v["total_gae"] for v in best) / q
    out["all_verified_ic"] = all(v["verified"] for v in best)
    worst = 0.0
    per_rep = []
    for v in best:
        C = nC = 0.0
        unpriced_named = defaultdict(float)
        tot = 0.0
        for ph in PHASES:
            P = v[ph]; nat = P.get("native") or {}
            g = P["group_ops"]["adds"] + P["group_ops"]["doubles"]
            for c, u in PRICED:
                n = nat.get(c, 0)
                if not n: continue
                if u in pin and cal.get(u) is not None:
                    gae = n * cal[u] / add_ns
                    g += gae; C += gae; nC += n
                else:
                    unpriced_named[c] += n
            worst = max(worst, abs(g - P["gae"]))
            tot += P["gae"]
        worst = max(worst, abs(tot - v["total_gae"]) / max(1.0, v["total_gae"]))
        ins = v["factor_base"].get("native", {}).get("pair_table_entries", 0)
        per_rep.append({"G": v["total_gae"], "C": C, "nC": nC, "U": sum(unpriced_named.values()),
                        "U_named": dict(unpriced_named), "inserts": ins})
    out["phase_decomposition_max_abs_err"] = worst
    runs = ref_inst["matched_reference"]["per_run"]
    out["rho_runs"] = len(runs)
    out["S_ref"] = mean(x["gae"] for x in runs) / q
    out["S_ref_reported"] = ref_inst["matched_reference"]["mean_s"]
    out["rho_verified_all"] = all(x["verified"] for x in runs)
    out["x_reference"] = out["S_IC"] / out["S_ref"]
    out["x_floor"] = out["S_IC"] / FLOOR
    out["rho_unpriced_canonicalisations_mean"] = mean(x["counters"].get("canonicalisations_uncharged", 0) for x in runs)
    out["rho_walk_operations_mean"] = mean(x["counters"].get("walk_operations", 0) for x in runs)
    out["ic_priced_native_ops_mean"] = mean(p["nC"] for p in per_rep)
    out["ic_unpriced_named_ops_mean"] = mean(p["U"] for p in per_rep)
    out["ic_pair_table_inserts_mean"] = mean(p["inserts"] for p in per_rep)
    band = []
    for w in WS:
        ic = mean(p["G"] - p["C"] + w * (p["nC"] + p["U"]) for p in per_rep) / q
        rho = mean(x["gae"] + w * x["counters"].get("canonicalisations_uncharged", 0) for x in runs) / q
        ic_x = mean(p["G"] - p["C"] + w * (p["nC"] + p["U"] + p["inserts"]) for p in per_rep) / q
        rho_x = mean(x["gae"] + w * (x["counters"].get("canonicalisations_uncharged", 0)
                                     + x["counters"].get("walk_operations", 0)) for x in runs) / q
        band.append({"w": w, "ic": ic, "rho": rho, "ratio": ic / rho,
                     "ic_ext": ic_x, "rho_ext": rho_x, "ratio_ext": ic_x / rho_x})
    out["band"] = band
    out["order_holds_formula"] = all(b["ratio"] > 1 for b in band)
    out["order_holds_extended"] = all(b["ratio_ext"] > 1 for b in band)
    out["order_holds_across_band"] = out["order_holds_formula"] and out["order_holds_extended"]
    return out

def main(repo, out_path, led_path=None, ref_path=None, evid=None):
    import os
    led_path = None if led_path in (None, "-") else led_path
    ref_path = None if ref_path in (None, "-") else ref_path
    repo = Path(repo).resolve()
    evid = Path(evid).resolve() if evid else None
    led_p = repo / "docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json"
    ref_p = repo / "research/ic_rho_reference_20260923/reprice/ic-boundary-ledger-round5-2026-09-22.json"
    board_p = repo / "docs/ic/leaderboard.json"
    led = json.load(open(led_path or led_p)); ref = json.load(open(ref_path or ref_p))
    board = json.load(open(board_p))["board"]
    legacy = {b["legacy"]: b for b in board if b["family"] == "B"}
    refd = {i["instance"]: i for i in ref["instances"]}
    rows, heads = [], []
    for inst in led["ledger"]["instances"]:
        if inst["regime"] not in ("prime", "char2"): continue
        name = inst["curve"]["name"]
        b = legacy[name]
        a = analyse(inst, refd[name])
        bb = b.get("best") or {}
        ok = (bb.get("variant", a["best_recipe"]) == a["best_recipe"])
        best_board = next(x for x in b["recipes"] if x["variant"] == a["best_recipe"])
        claimed = {"S_IC": best_board["s"], "S_ref": b["reference_s"], "x_reference": best_board["ratio_rho"],
                   "x_floor": best_board["ratio_floor"]}
        row = {"slug": b["slug"], "legacy": name, "regime": b["regime"], "recipe": a["best_recipe"],
               "board_best_recipe_agrees": ok, "claimed": claimed, "analysis": a, "agree": {}}
        for k in claimed:
            rc = a[k]
            row["agree"][k] = abs(rc - claimed[k]) <= 1e-9 * max(abs(rc), abs(claimed[k]))
            heads.append({"name": f"{b['slug']} {k}", "claimed": claimed[k], "recomputed": rc, "rel_tol": 1e-9})
        row["pass_recompute"] = all(row["agree"].values()) and ok and a["all_verified_ic"] and a["rho_verified_all"] \
            and a["phase_decomposition_max_abs_err"] < 1e-6
        rows.append(row)
    r0 = rows[0]
    pert = r0["claimed"]["S_IC"] * 1.001
    neg_flagged = abs(pert - r0["analysis"]["S_IC"]) > 1e-9 * pert
    rep = {"schema": "validator-report/v1", "independent": True,
           "status": "pass" if all(r["pass_recompute"] for r in rows) and neg_flagged else "fail",
           "table": "B", "method": "stdlib python recompute from committed raw ledger and re-priced rho; no builder, no sweep script, no ic binary",
           "raw_data": [dict(path=(os.path.relpath(p, evid) if evid else str(p.relative_to(repo))), **({"sha256": sha(p)} if p != board_p else {})) for p in (led_p, ref_p, board_p)],
           "note_raw_data": "leaderboard.json is listed as the source of the claimed numbers and is not pinned: the builder rewrites it",
           "negative_control": {"perturbed_S_IC_x1.001_flagged": neg_flagged},
           "headline": heads, "rows": rows}
    json.dump(rep, open(out_path, "w"), indent=1)
    bad = [r["slug"] for r in rows if not r["pass_recompute"]]
    print("status", rep["status"], "rows", len(rows), "headline", len(heads), "failing", bad)
    for r in rows:
        a = r["analysis"]
        print(f'{r["slug"]:30s} S_IC {a["S_IC"]:.4f} S_ref {a["S_ref"]:.4f} x {a["x_reference"]:.3f} '
              f'band {[round(b["ratio"],2) for b in a["band"]]} ext {[round(b["ratio_ext"],2) for b in a["band"]]} '
              f'decomp {a["phase_decomposition_max_abs_err"]:.1e} unpricedIC {a["ic_unpriced_named_ops_mean"]:.0f} agree {all(r["agree"].values())}')

if __name__ == "__main__":
    main(*sys.argv[1:])
