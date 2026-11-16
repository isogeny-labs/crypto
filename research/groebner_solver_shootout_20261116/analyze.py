#!/usr/bin/env python3
"""Frozen analysis for GB-SOLVER-SHOOTOUT-20261116 (see PROTOCOL.md, protocol.json).

Pure standard library. Runs blind: arm labels in rows.jsonl may be masked
(arm-00, arm-01, ...). Phase 1 (always) computes every per-label metric and
every unordered label pair; phase 2 (only with --key, after phase 1 output is
written) maps labels back, applies the A/A noise-floor criterion and the
matched-rho band verdicts.

Inputs
  rows.jsonl   one row per (cell, population, target_index, arm). Fields:
               cell_id, population in {natural, planted, tamper}, target_index,
               arm, order_position, status in {verified, no_solution, censored,
               error, unavailable, invalid}, solver_cpu_s, extract_cpu_s,
               verify_cpu_s, wall_s, peak_rss_bytes, max_step_degree,
               max_matrix_rows, max_matrix_cols, relations_verified,
               has_relation, oracle_count, complete, planted_recovered,
               wall_valid
  cells.json   {cell_id: {m, n, l, a, variant, fb_abscissae, curve_order,
               r_largest_prime, t_ga_seconds, cap_cpu_seconds,
               rho: {expected_steps, unpriced_per_step: {counter: count},
                     measured_cpu_s_per_dlp}}}
Outputs (write-once): summary.json, pairs.tsv, arms.tsv

Decision rules implemented here are the ones frozen in protocol.json
decision_rule; changing this file after the freeze is a protocol amendment.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import math
import random
import sys
from collections import defaultdict
from pathlib import Path

ALPHA = 0.05
TESTS_IN_FAMILY = 315          # 21 label pairs x 15 cell-variants, preregistered
BOOTSTRAP = 20000
BAND = (0.0, 0.1, 1.0)         # GAE per unpriced native operation (AGENTS.md 5a)
RELATIONS_MARGIN = 10          # projected relations needed = fb_abscissae + 10


def load_rows(path: Path) -> list[dict]:
    rows = []
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.strip():
            rows.append(json.loads(line))
    return rows


def charged_cpu(row: dict, cap: float) -> float:
    """Solver + in-house extraction CPU; censored calls charged at the cap."""
    if row["status"] == "censored":
        return cap
    return float(row.get("solver_cpu_s") or 0.0) + float(row.get("extract_cpu_s") or 0.0)


def wilcoxon_exact(d: list[float]) -> tuple[float, int]:
    """Two-sided signed-rank p. Zeros dropped; exact DP without ties, normal
    approximation with tie correction when ties exist. Returns (p, n_eff)."""
    d = [x for x in d if x != 0.0]
    n = len(d)
    if n == 0:
        return 1.0, 0
    order = sorted(range(n), key=lambda i: abs(d[i]))
    ranks = [0.0] * n
    i = 0
    ties = False
    while i < n:
        j = i
        while j + 1 < n and abs(d[order[j + 1]]) == abs(d[order[i]]):
            j += 1
        if j > i:
            ties = True
        for k in range(i, j + 1):
            ranks[order[k]] = (i + j) / 2 + 1
        i = j + 1
    w_plus = sum(r for r, x in zip(ranks, d) if x > 0)
    total = n * (n + 1) / 2
    if not ties:
        counts = [0] * (int(total) + 1)
        counts[0] = 1
        for r in range(1, n + 1):
            for s in range(int(total), r - 1, -1):
                counts[s] += counts[s - r]
        denom = 2 ** n
        w = min(w_plus, total - w_plus)
        tail = sum(counts[: int(math.floor(w)) + 1]) / denom
        return min(1.0, 2 * tail), n
    mean = total / 2
    tie_term = 0.0
    seen = defaultdict(int)
    for r in ranks:
        seen[r] += 1
    for t in seen.values():
        tie_term += t ** 3 - t
    var = n * (n + 1) * (2 * n + 1) / 24 - tie_term / 48
    if var <= 0:
        return 1.0, n
    z = (abs(w_plus - mean) - 0.5) / math.sqrt(var)
    return min(1.0, math.erfc(max(z, 0.0) / math.sqrt(2))), n


def median(xs: list[float]) -> float | None:
    if not xs:
        return None
    s = sorted(xs)
    k = len(s)
    return s[k // 2] if k % 2 else (s[k // 2 - 1] + s[k // 2]) / 2


def quantile(xs: list[float], q: float) -> float:
    s = sorted(xs)
    pos = q * (len(s) - 1)
    lo, hi = math.floor(pos), math.ceil(pos)
    return s[lo] + (s[hi] - s[lo]) * (pos - lo)


def seeded_rng(protocol_sha: str, *parts: str) -> random.Random:
    h = hashlib.sha256(":".join((protocol_sha,) + parts).encode()).digest()
    return random.Random(int.from_bytes(h[:8], "big"))


def arm_cell_metrics(rows: list[dict], cap: float, cell: dict) -> dict:
    nat = [r for r in rows if r["population"] == "natural"]
    planted = [r for r in rows if r["population"] == "planted"]
    tamper = [r for r in rows if r["population"] == "tamper"]
    statuses = defaultdict(int)
    for r in nat:
        statuses[r["status"]] += 1
    if nat and all(r["status"] == "unavailable" for r in nat + planted):
        return {"availability": "UNAVAILABLE"}
    cpu = sum(charged_cpu(r, cap) for r in nat)
    rel = sum(1 for r in nat if r.get("has_relation") and r["status"] == "verified")
    oracle_pos = [r for r in nat if (r.get("oracle_count") or 0) > 0]
    oracle_zero = [r for r in nat if r.get("oracle_count") == 0]
    out = {
        "availability": "available",
        "natural_targets": len(nat),
        "status_counts": dict(statuses),
        "verified_relations": rel,
        "charged_cpu_s": cpu,
        "relations_per_cpu_s": rel / cpu if cpu > 0 else None,
        "relation_recall_vs_oracle": (sum(1 for r in oracle_pos if r.get("has_relation")) / len(oracle_pos)
                                      if oracle_pos else None),
        "oracle_zero_targets": len(oracle_zero),
        "false_relations_on_oracle_zero": sum(1 for r in oracle_zero if r.get("has_relation")),
        "positive_control": {
            "planted": len(planted),
            "verified": sum(1 for r in planted if r["status"] == "verified" and r.get("has_relation")),
            "planted_recovered": sum(1 for r in planted if r.get("planted_recovered")),
            "censored": sum(1 for r in planted if r["status"] == "censored"),
        },
        "tamper_rejected": (sum(1 for r in tamper if r["status"] == "invalid"), len(tamper)),
        "invalid_rows": sum(1 for r in nat + planted if r["status"] == "invalid"),
        "median_cpu_s": median([charged_cpu(r, cap) for r in nat]),
        "max_step_degree": max([r["max_step_degree"] for r in nat + planted
                                if r.get("max_step_degree") is not None], default=None),
        "max_matrix": max([(r["max_matrix_rows"] * r["max_matrix_cols"], r["max_matrix_rows"], r["max_matrix_cols"])
                           for r in nat + planted if r.get("max_matrix_rows")], default=None),
        "max_peak_rss_bytes": max([r["peak_rss_bytes"] for r in nat + planted
                                   if r.get("peak_rss_bytes") is not None], default=None),
        "median_wall_s": median([r["wall_s"] for r in nat if r.get("wall_s") is not None
                                 and r.get("wall_valid", True)]),
    }
    if out["max_matrix"]:
        out["max_matrix"] = f"{out['max_matrix'][1]}x{out['max_matrix'][2]}"
    # cost per relation and matched rho, every reading of the band plus the clock
    if rel > 0 and cell.get("t_ga_seconds") and cell.get("rho"):
        rho = cell["rho"]
        cpu_per_rel = cpu / rel
        gae_per_rel = cpu_per_rel / cell["t_ga_seconds"]
        u = sum(rho.get("unpriced_per_step", {}).values())
        needed = cell["fb_abscissae"] + RELATIONS_MARGIN
        readings = {}
        for w in BAND:
            c_rho = rho["expected_steps"] * (1 + w * u)
            readings[f"w={w}"] = {"rho_gae": c_rho,
                                  "per_relation_over_rho": gae_per_rel / c_rho,
                                  "projected_relation_phase_over_rho": needed * gae_per_rel / c_rho}
        if rho.get("measured_cpu_s_per_dlp"):
            readings["clock"] = {"rho_cpu_s": rho["measured_cpu_s_per_dlp"],
                                 "per_relation_over_rho": cpu_per_rel / rho["measured_cpu_s_per_dlp"],
                                 "projected_relation_phase_over_rho":
                                     needed * cpu_per_rel / rho["measured_cpu_s_per_dlp"]}
        ratios = [v["projected_relation_phase_over_rho"] for v in readings.values()]
        if "clock" not in readings or not rho.get("unpriced_emitted", True):
            verdict = "unresolved (rho reading incomplete; lower bound only)"
        elif all(x < 1 for x in ratios):
            verdict = "below matched rho at every reading"
        elif all(x > 1 for x in ratios):
            verdict = "above matched rho at every reading"
        else:
            verdict = "unresolved in band"
        out["cost_per_relation_cpu_s"] = cpu_per_rel
        out["cost_per_relation_gae"] = gae_per_rel
        out["rho_readings"] = readings
        out["rho_verdict"] = verdict
        out["distance_remaining_log2"] = {k: math.log2(v["projected_relation_phase_over_rho"])
                                          for k, v in readings.items()}
    return out


def pair_stats(a_rows: dict, b_rows: dict, cap: float, rng: random.Random) -> dict:
    keys = sorted(set(a_rows) & set(b_rows))
    d = []
    for k in keys:
        ra, rb = a_rows[k], b_rows[k]
        if "unavailable" in (ra["status"], rb["status"]):
            continue
        ca, cb = max(charged_cpu(ra, cap), 1e-6), max(charged_cpu(rb, cap), 1e-6)
        if ra["status"] == "censored" and rb["status"] == "censored":
            d.append(0.0)  # both at cap: no information, dropped by the test
            continue
        d.append(math.log2(ca) - math.log2(cb))
    if not d:
        return {"n": 0}
    p, n_eff = wilcoxon_exact(d)
    boots = []
    for _ in range(BOOTSTRAP):
        sample = [d[rng.randrange(len(d))] for _ in d]
        boots.append(median(sample))
    lo = quantile(boots, ALPHA / TESTS_IN_FAMILY / 2)
    hi = quantile(boots, 1 - ALPHA / TESTS_IN_FAMILY / 2)
    return {"n": len(d), "n_eff": n_eff, "median_log2_ratio": median(d),
            "ci_bonferroni": [lo, hi], "ci95": [quantile(boots, 0.025), quantile(boots, 0.975)],
            "p_two_sided": p, "significant": p < ALPHA / TESTS_IN_FAMILY,
            "resolvable": n_eff > 0 and 2 / 2 ** n_eff < ALPHA / TESTS_IN_FAMILY}


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("rows", type=Path)
    ap.add_argument("cells", type=Path)
    ap.add_argument("--protocol-sha256", required=True)
    ap.add_argument("--out", type=Path, required=True)
    ap.add_argument("--key", type=Path, help="unblinding key; only after phase 1 output exists")
    ap.add_argument("--aa", action="append", default=[],
                    help="arm-id pair 'a,b' that is an A/A control (phase 2 only)")
    args = ap.parse_args()

    rows = load_rows(args.rows)
    cells = json.loads(args.cells.read_text(encoding="utf-8"))
    args.out.mkdir(parents=True, exist_ok=True)
    phase1 = args.out / "summary.json"

    by_cell = defaultdict(lambda: defaultdict(list))
    for r in rows:
        by_cell[r["cell_id"]][r["arm"]].append(r)

    summary = {"protocol_sha256": args.protocol_sha256, "alpha": ALPHA,
               "tests_in_family": TESTS_IN_FAMILY, "bootstrap": BOOTSTRAP, "cells": {}}
    for cell_id in sorted(by_cell):
        cell = cells[cell_id]
        cap = float(cell["cap_cpu_seconds"])
        arms = by_cell[cell_id]
        entry = {"arms": {}, "pairs": {}}
        for arm in sorted(arms):
            entry["arms"][arm] = arm_cell_metrics(arms[arm], cap, cell)
        natural = {arm: {r["target_index"]: r for r in rs if r["population"] == "natural"}
                   for arm, rs in arms.items()}
        labels = sorted(a for a in natural if entry["arms"][a].get("availability") == "available")
        for i, a in enumerate(labels):
            for b in labels[i + 1:]:
                rng = seeded_rng(args.protocol_sha256, "bootstrap", cell_id, a, b)
                entry["pairs"][f"{a}|{b}"] = pair_stats(natural[a], natural[b], cap, rng)
        summary["cells"][cell_id] = entry

    if args.key is None:
        if phase1.exists():
            print(f"refusing to overwrite {phase1}", file=sys.stderr)
            return 2
        phase1.write_text(json.dumps(summary, indent=1) + "\n", encoding="utf-8")
        print(f"phase 1 (blind) written: {phase1}")
        return 0

    if not phase1.exists():
        print("phase 2 needs the blind phase-1 summary first", file=sys.stderr)
        return 2
    key = json.loads(args.key.read_text(encoding="utf-8"))
    unmask = {masked: arm for arm, masked in key["mapping"].items()}
    aa_pairs = {frozenset(p.split(",")) for p in args.aa}
    final = {"protocol_sha256": args.protocol_sha256, "cells": {}}
    for cell_id, entry in summary["cells"].items():
        arms = {unmask.get(k, k): v for k, v in entry["arms"].items()}
        pairs = {}
        for k, v in entry["pairs"].items():
            a, b = (unmask.get(x, x) for x in k.split("|"))
            pairs[(a, b)] = v
        floor = 0.0
        for (a, b), v in pairs.items():
            if frozenset((a, b)) in aa_pairs and v.get("n"):
                floor = max(floor, abs(v["ci95"][0]), abs(v["ci95"][1]))
        verdicts = {}
        for (a, b), v in pairs.items():
            if frozenset((a, b)) in aa_pairs:
                verdicts[f"{a}|{b}"] = {**v, "verdict": "A/A noise floor"}
                continue
            if not v.get("n"):
                verdict = "unresolved (no paired targets)"
            elif not v["resolvable"]:
                verdict = "unresolved (too few decided targets for the corrected threshold)"
            elif v["significant"] and abs(v["median_log2_ratio"]) > floor:
                verdict = f"{b if v['median_log2_ratio'] > 0 else a} uses less CPU per target"
            else:
                verdict = "within A/A noise or not significant"
            verdicts[f"{a}|{b}"] = {**v, "aa_floor_log2": floor, "verdict": verdict}
        final["cells"][cell_id] = {"arms": arms, "pairs": verdicts}
    out = args.out / "summary.unblinded.json"
    if out.exists():
        print(f"refusing to overwrite {out}", file=sys.stderr)
        return 2
    out.write_text(json.dumps(final, indent=1) + "\n", encoding="utf-8")
    with (args.out / "arms.tsv").open("x", encoding="utf-8") as f:
        f.write("cell\tarm\tavailability\tverified_relations\tcharged_cpu_s\trelations_per_cpu_s\t"
                "max_step_degree\tmax_matrix\tmax_peak_rss_bytes\trho_verdict\n")
        for cell_id, e in final["cells"].items():
            for arm, m in e["arms"].items():
                f.write("\t".join(str(x) for x in (cell_id, arm, m.get("availability"), m.get("verified_relations"),
                        m.get("charged_cpu_s"), m.get("relations_per_cpu_s"), m.get("max_step_degree"),
                        m.get("max_matrix"), m.get("max_peak_rss_bytes"), m.get("rho_verdict"))) + "\n")
    with (args.out / "pairs.tsv").open("x", encoding="utf-8") as f:
        f.write("cell\tpair\tn\tn_eff\tmedian_log2_ratio\tp\tverdict\n")
        for cell_id, e in final["cells"].items():
            for k, v in e["pairs"].items():
                f.write("\t".join(str(x) for x in (cell_id, k, v.get("n"), v.get("n_eff"),
                        v.get("median_log2_ratio"), v.get("p_two_sided"), v.get("verdict"))) + "\n")
    print(f"phase 2 written: {out}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
