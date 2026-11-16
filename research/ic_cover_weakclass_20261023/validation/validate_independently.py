#!/usr/bin/env python3
"""Independent recompute of the study's headline numbers from `raw/*.json`.

Shares no code with `examples/jv_cover_weakclass_report.rs` (the aggregator that
wrote `summary.json`).  Reads only committed raw records and the frozen summary
(for the claimed values); writes `validator-report.json`.
"""
from __future__ import annotations

import hashlib
import json
import math
import statistics
import sys
from pathlib import Path

STUDY = Path(__file__).resolve().parent.parent
PW = (0.0, 0.1, 1.0)


def load(rel: str) -> dict:
    return json.loads((STUDY / rel).read_text())[0]


def sha256(rel: str) -> str:
    return hashlib.sha256((STUDY / rel).read_bytes()).hexdigest()


def s(rec: dict) -> float:
    return rec["total_muls"] / (rec["c_add_e"] * math.sqrt(rec["l"]))


def s_over_rho(rec: dict, rho_s: float) -> float:
    return rec["total_muls"] / (rho_s * math.sqrt(rec["l"]) * rec["c_add_e"])


def slope(points: list[tuple[float, float]]) -> float:
    xs = [math.log10(p) for p, _ in points]
    ys = [math.log10(v) for _, v in points]
    mx, my = statistics.fmean(xs), statistics.fmean(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)


def main() -> int:
    summary = json.loads((STUDY / "summary.json").read_text())
    claimed_p = {row["p"]: row for row in summary["per_p"]}
    raw: dict[tuple[int, int, str], dict] = {}
    raw_data = []
    for f in sorted((STUDY / "raw").glob("p*_s*.json")):
        rec = json.loads(f.read_text())[0]
        raw[(rec["p"], rec["seed"], f.stem.rsplit("_", 1)[1])] = rec
        rel = f"raw/{f.name}"
        raw_data.append({"path": rel, "sha256": sha256(rel)})

    p_sorted = sorted({p for p, _, _ in raw})
    pooled: dict[int, float] = {}
    for p in p_sorted:
        runs = [r["s"] for (pp, _, arm), c in raw.items()
                if pp == p and arm == "A" for r in c["rho"]]
        if runs:
            pooled[p] = statistics.fmean(runs)
    all_runs = [r["s"] for (_, _, arm), c in raw.items() if arm == "A" for r in c["rho"]]
    pooled_default = statistics.fmean(all_runs)

    headline = []
    errors = []

    def add(name: str, claimed: float, recomputed: float, rel_tol: float = 1e-9) -> None:
        headline.append({"name": name, "claimed": claimed, "recomputed": recomputed,
                         "rel_tol": rel_tol})
        scale = max(abs(claimed), abs(recomputed), 1e-300)
        if abs(claimed - recomputed) > rel_tol * scale:
            errors.append(f"{name}: {claimed} != {recomputed}")

    means = {"A": [], "B": []}
    for p in p_sorted:
        rho = pooled.get(p, pooled_default)
        for arm in ("A", "B"):
            recs = [c for (pp, _, a), c in raw.items() if pp == p and a == arm]
            mean_s = statistics.fmean(s(c) for c in recs)
            mean_sr = statistics.fmean(s_over_rho(c, rho) for c in recs)
            claimed_sr = claimed_p[p][f"S_over_rho_{arm}"]["mean"]
            claimed_s = statistics.fmean(c[arm]["S"] for c in summary["cells"] if c["p"] == p)
            add(f"mean S ({arm}) at p={p}", claimed_s, mean_s)
            add(f"mean S/rho ({arm}) at p={p}", claimed_sr, mean_sr)
            means[arm].append((p, mean_sr))
        ratios = []
        for seed in sorted({sd for (pp, sd, _) in raw if pp == p}):
            ratios.append(s(raw[(p, seed, "A")]) / s(raw[(p, seed, "B")]))
        add(f"mean S_A/S_B at p={p}", claimed_p[p]["ratio_S_A_over_S_B"]["mean"],
            statistics.fmean(ratios))

    add("slope of log S/rho (A) over p", summary["slope_A"][0], slope(means["A"]))
    add("slope of log S/rho (B) over p", summary["slope_B"][0], slope(means["B"]))

    checked = 0
    for (p, seed, arm), c in sorted(raw.items()):
        if arm not in ("A", "B"):
            continue
        a = raw[(p, seed, "A")]
        b = raw[(p, seed, "B")]
        ok = (a["solved"] and b["solved"] and a["correct"] and b["correct"]
              and a["mismatches"] == 0 and b["mismatches"] == 0
              and a["residuals"] == b["residuals"] and a["relations"] == b["relations"]
              and a["decompositions"] == b["decompositions"]
              and a["relation_digest"] == b["relation_digest"])
        checked += 1
        if not ok:
            errors.append(f"cell p={p} seed={seed}: equal-relations/verification check failed")
    add("fraction of A/B cells with equal relations and verified logarithms",
        1.0, (checked - len([e for e in errors if e.startswith("cell ")])) / checked)

    for (p, seed, arm), c in sorted(raw.items()):
        if arm == "A2":
            add(f"A/A total_muls ratio at p={p} seed={seed}", 1.0, c["total_muls"] / raw[(p, seed, "A")]["total_muls"])
        if arm == "B0":
            a = raw[(p, seed, "A")]
            cell = [x for x in summary["cells"] if x["p"] == p and x["seed"] == seed][0]["B0_vs_A"]
            add(f"B0 relabelling S_A/S_B0 at p={p} seed={seed}",
                cell["ratio_S_A_over_S_B"], a["s"] / c["s"])
            add(f"B0 native ops / A at p={p} seed={seed}",
                cell["native_B_over_A"], c["f4_native_ops"] / a["f4_native_ops"])

    for stem in ("p53_s2_B", "p101_s4_B"):
        rec = raw[(int(stem.split("_")[0][1:]), int(stem.split("_s")[1].split("_")[0]), "B")]
        rep = json.loads((STUDY / f"replay/{stem}_replay.json").read_text())[0]
        add(f"{stem} replay total_muls", float(rec["total_muls"]), float(rep["total_muls"]), 0.0)
        add(f"{stem} replay relation_digest", float(rec["relation_digest"]), float(rep["relation_digest"]), 0.0)

    report = {
        "schema": "validator-report/v1",
        "status": "pass" if not errors else "failed",
        "independent": True,
        "method": ("validate_independently.py recomputes S = total_muls/(c_add_E*sqrt(l)), the "
                   "pooled plain rho S per p, per-p means of S/rho, the paired S_A/S_B, the log-log "
                   "slope, the A/A identity, the B0 relabelling ratios and the replay scalars from "
                   "raw/*.json; claimed values are read from the frozen summary.json"),
        "raw_data": raw_data,
        "headline": headline,
        "errors": errors,
    }
    (STUDY / "validation/validator-report.json").write_text(json.dumps(report, indent=1) + "\n")
    print(f"{len(headline)} headline numbers, {len(errors)} errors")
    for e in errors:
        print(" ", e)
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
