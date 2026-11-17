#!/usr/bin/env python3
"""Frozen decision procedure of crypto-hashed-ecdl-counts-20261031 v2.

Written and frozen before any data (protocol v2, 2026-10-31). Standard
library only. Two subcommands:

  power  The minimum detectable ratio table of sample_size.power (no data).
  decide Run the frozen ecbench commands on the three sessions and apply the
         decision rule. Writes decision.json and the raw ecbench outputs.

    python3 decide.py power
    python3 decide.py decide --ecbench-c BIN_C --ecbench-b BIN_B \
        --c-small DIR --c16 DIR --b DIR --out DIR

Procedure (protocol decision_rule):

1. Per (comparison, curve) cell and per weight w in {0, 0.1, 1}:
   `ecbench compare --dir D --a A --b B --weight w --resamples 4000
   --seed 20261002 --json`. The statistic is the ratio of summed S over the
   8 paired targets, R_w = curves[].ratio_b_over_a, and the primary interval
   is compare's linearised_t (curves[].ci95, read on the log scale, df =
   K - 1 = 7). The cluster bootstrap (curves[].ci95_bootstrap) is reported
   beside it and decides nothing.
2. A cell is complete only when every measured row of both arms on that
   curve is `verified` (8 + 8 rows) and compare reports 8 pairs, 0 censored
   pairs, ci_method linearised_t, at every weight. Any other cell is
   incomplete_unknown: p = 1, never ranked, never used for P1/P4. A ratio
   with an exhausted pair is shown only as descriptive; a ratio with a
   timeout pair is not shown at all (compare would enter it at a wall-implied
   S), and no wall-implied S figure is written anywhere.
3. Per weight: cv_w = ln(hi_w / R_w) / t_{0.975, 7}, T_w = ln(R_w) / cv_w,
   p_w = 2 (1 - F_t(|T_w|; 7)). cv_w = 0 gives p_w = 0 when R_w != 1.
4. Band condition: ln R_w has the same sign at all three weights, and
   `ecbench band --weights 0,0.1,1 --ranks` gives the same order for that
   curve. Band ranks supply only this condition (they carry no interval).
   Failing it: p = 1, verdict no_ranking.
5. Native lower-bound arms (B-rho-source/selectors/width/partial): only
   R > 1 is rankable; R < 1 at every weight is no_ranking.
6. Cell p-value p = max_w p_w (intersection-union across the three weights).
7. Holm over the 32-test family at alpha = 0.05: sort p ascending (ties by
   comparison id, then log2 r); the k-th smallest is ranked iff p_(j) <=
   alpha / (32 - j + 1) for every j <= k. Equivalently its two-sided
   interval at level 1 - alpha/(32 - k + 1) excludes 1 on the same side at
   every weight.
8. Determinism (C-aa, B-aa-rho, B-aa-selectors): a track is invalid only
   when S or total_gae differs on a (workload, round) pair whose two rows are
   both verified or both exhausted. A status mismatch involving `timeout` is
   host censoring (the wall-clock timeout): the cell becomes incomplete; it
   is not a determinism failure and not a negative result.
"""
from __future__ import annotations

import argparse
import json
import math
import subprocess
import sys
from pathlib import Path

ALPHA = 0.05
WEIGHTS = (0.0, 0.1, 1.0)
RESAMPLES = 4000
SEED = 20261002
TARGETS = 8
DF = TARGETS - 1

# (comparison id, track, A arm (denominator), B arm (numerator), lower_bound_only)
FAMILY = [
    ("C-rho", "C", "rho-neg", "tensor-m2", False),
    ("C-ic", "C", "ic-mitm", "tensor-m2", False),
    ("B-rho-source", "B", "rho-neg", "native-source", True),
    ("B-rho-selectors", "B", "rho-neg", "native-selectors", True),
    ("B-rho-width", "B", "rho-neg", "native-width", True),
    ("B-rho-partial", "B", "rho-neg", "native-partial", True),
    ("B-rho-ic", "B", "rho-neg", "ic-mitm", False),
    ("B-source-selectors", "B", "native-source", "native-selectors", False),
]
DESCRIPTIVE = [
    ("C-rho-ic", "C", "rho-neg", "ic-mitm"),
    ("B-pos", "B", "rho-neg", "rho-neg-v2"),
]
AA = [("C-aa", "C", "rho-neg", "rho-neg-aa"), ("B-aa-rho", "B", "rho-neg", "rho-neg-aa"),
      ("B-aa-selectors", "B", "native-selectors", "native-selectors-aa")]
CURVES_PER_COMPARISON = 4
M = len(FAMILY) * CURVES_PER_COMPARISON  # 32


# ---- Student t (regularized incomplete beta, Lentz continued fraction) ----

def _betacf(a: float, b: float, x: float) -> float:
    tiny, eps = 1e-300, 1e-15
    qab, qap, qam = a + b, a + 1.0, a - 1.0
    c, d = 1.0, 1.0 - qab * x / qap
    d = 1.0 / (d if abs(d) > tiny else tiny)
    h = d
    for m in range(1, 400):
        m2 = 2 * m
        aa = m * (b - m) * x / ((qam + m2) * (a + m2))
        d = 1.0 + aa * d
        d = 1.0 / (d if abs(d) > tiny else tiny)
        c = 1.0 + aa / c
        c = c if abs(c) > tiny else tiny
        h *= d * c
        aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2))
        d = 1.0 + aa * d
        d = 1.0 / (d if abs(d) > tiny else tiny)
        c = 1.0 + aa / c
        c = c if abs(c) > tiny else tiny
        delta = d * c
        h *= delta
        if abs(delta - 1.0) < eps:
            break
    return h


def betai(a: float, b: float, x: float) -> float:
    if x <= 0.0:
        return 0.0
    if x >= 1.0:
        return 1.0
    lbt = math.lgamma(a + b) - math.lgamma(a) - math.lgamma(b) + a * math.log(x) + b * math.log1p(-x)
    if x < (a + 1.0) / (a + b + 2.0):
        return math.exp(lbt) * _betacf(a, b, x) / a
    return 1.0 - math.exp(lbt) * _betacf(b, a, 1.0 - x) / b


def t_cdf(t: float, df: float) -> float:
    if t == 0.0:
        return 0.5
    tail = 0.5 * betai(df / 2.0, 0.5, df / (df + t * t))
    return 1.0 - tail if t > 0 else tail


def t_quantile(p: float, df: float) -> float:
    lo, hi = 0.0, 1.0
    while t_cdf(hi, df) < p:
        lo, hi = hi, hi * 2.0
    for _ in range(200):
        mid = 0.5 * (lo + hi)
        if t_cdf(mid, df) < p:
            lo = mid
        else:
            hi = mid
    return 0.5 * (lo + hi)


def two_sided_p(t: float, df: float) -> float:
    if math.isinf(t):
        return 0.0
    return max(0.0, min(1.0, 2.0 * (1.0 - t_cdf(abs(t), df))))


# ---- power (no data) ----

def power_table() -> dict:
    """Minimum detectable ratio (MDR) of one cell at the strictest and the
    loosest Holm step, as a function of the per-target coefficient of
    variation of the ratio-of-totals influence values.

    Model (assumption, not data): with 8 independent targets, se(ln R) is
    about cv_pair / sqrt(8), where cv_pair^2 = cv_A^2 + cv_B^2 for independent
    per-target costs. A rho walk's collision time has a Rayleigh-like law,
    cv about sqrt(4/pi - 1) = 0.523. A cell is detectable (at power about
    0.5) when |ln R| >= t_crit * se; power 0.8 needs about t_crit + 0.9 se
    more (normal approximation of the noncentral t, df 7)."""
    rows = []
    rayleigh = math.sqrt(4.0 / math.pi - 1.0)
    t_first = t_quantile(1.0 - ALPHA / M / 2.0, DF)  # alpha/32, two-sided
    t_last = t_quantile(1.0 - ALPHA / 2.0, DF)       # alpha/1
    for label, cv_a, cv_b in [
        ("rho vs deterministic-cost arm (cv_B = 0)", rayleigh, 0.0),
        ("rho vs arm with cv_B = 0.25", rayleigh, 0.25),
        ("rho vs arm with rho-like spread (cv_B = 0.523)", rayleigh, rayleigh),
        ("IC vs IC, both cv 0.25 (C-ic, assumed)", 0.25, 0.25),
        ("IC vs IC, both cv 0.5 (C-ic, assumed)", 0.5, 0.5),
    ]:
        se = math.sqrt(cv_a ** 2 + cv_b ** 2) / math.sqrt(TARGETS)
        row = {"case": label, "cv_A": round(cv_a, 3), "cv_B": round(cv_b, 3), "se_lnR": round(se, 4)}
        for tag, t in (("holm_first_alpha_over_32", t_first), ("holm_last_alpha", t_last)):
            row[tag] = {"t_crit": round(t, 4),
                        "mdr_power50": round(math.exp(t * se), 3),
                        "mdr_power80_approx": round(math.exp((t + 0.9) * se), 3)}
        rows.append(row)
    return {"alpha": ALPHA, "tests_in_family": M, "targets_per_cell": TARGETS, "df": DF,
            "rho_cv_model": round(rayleigh, 4), "rows": rows,
            "note": "ratios are symmetric on the log scale: MDR x means R >= x or R <= 1/x"}


# ---- data ----

def read_records(session: Path) -> list[dict]:
    out = []
    with (session / "records.jsonl").open() as fh:
        for line in fh:
            line = line.strip()
            if line:
                out.append(json.loads(line))
    return [r for r in out if not r.get("warmup")]


def run_json(argv: list[str]) -> tuple[dict | None, str | None]:
    proc = subprocess.run(argv, capture_output=True, text=True)
    if proc.returncode != 0:
        return None, (proc.stderr or proc.stdout).strip()[-2000:]
    try:
        return json.loads(proc.stdout), None
    except json.JSONDecodeError as e:
        return None, f"non-JSON output: {e}"


def compare(binary: str, session: Path, a: str, b: str, w: float, raw: Path) -> tuple[dict | None, str | None]:
    argv = [binary, "compare", "--dir", str(session), "--a", a, "--b", b, "--weight", repr(w),
            "--resamples", str(RESAMPLES), "--seed", str(SEED), "--json"]
    out, err = run_json(argv)
    raw.write_text(json.dumps({"argv": argv[1:], "result": out, "error": err}, indent=1) + "\n")
    return out, err


def arm_rows(records: list[dict], arm: str, slug: str) -> list[dict]:
    return [r for r in records if r["arm"] == arm and r["workload"]["curve"]["slug"] == slug]


def cell_from_compares(by_w: dict, a_rows: list[dict], b_rows: list[dict], slug: str) -> dict:
    statuses = sorted({r["outcome"]["status"] for r in a_rows + b_rows})
    all_verified = (len(a_rows) == TARGETS and len(b_rows) == TARGETS
                    and all(r["outcome"]["status"] == "verified" for r in a_rows + b_rows))
    has_timeout = any(r["outcome"]["status"] == "timeout" for r in a_rows + b_rows)
    has_exhausted = any(r["outcome"]["status"] == "exhausted" for r in a_rows + b_rows)
    per_w, problems = [], []
    for w in WEIGHTS:
        comp, err = by_w[w]
        if comp is None:
            problems.append(f"compare at w={w} failed: {err}")
            per_w.append(None)
            continue
        cr = next((c for c in comp.get("curves", []) if c["slug"] == slug), None)
        if cr is None:
            problems.append(f"compare at w={w} has no curve {slug}")
            per_w.append(None)
            continue
        per_w.append(cr)
        if cr["pairs"] != TARGETS or cr.get("censored_pairs", 0) != 0 or cr["ci_method"] != "linearised_t":
            problems.append(f"w={w}: pairs {cr['pairs']}, censored {cr.get('censored_pairs', 0)}, ci {cr['ci_method']}")
    complete = all_verified and not problems
    cell = {"slug": slug, "statuses": statuses, "complete": complete, "problems": problems}
    if complete:
        cell["log2_r"] = per_w[0]["log2_r"]
        cell["ratio"] = [c["ratio_b_over_a"] for c in per_w]
        cell["log2_ratio"] = [math.log2(c["ratio_b_over_a"]) if c["ratio_b_over_a"] and c["ratio_b_over_a"] > 0 else None
                              for c in per_w]
        cell["ci95_linearised_t"] = [c["ci95"] for c in per_w]
        cell["ci95_cluster_bootstrap_descriptive"] = [c.get("ci95_bootstrap") for c in per_w]
    else:
        cell["verdict"] = "incomplete_unknown"
        # Descriptive only. A timeout pair would enter compare's ratio at a
        # wall-implied S, so no compare ratio is shown when one is present.
        if per_w and per_w[0] is not None and not has_timeout:
            cell["descriptive_ratio_incomplete_unknown"] = [c["ratio_b_over_a"] if c else None for c in per_w]
            cell["descriptive_note"] = ("includes exhausted pairs at the S they spent" if has_exhausted
                                        else "completed pairs only")
        # Counted-only ratio over pairs where both rows verified (recorded S).
        sa = {(r["workload"]["workload_id"], r["round"]): r for r in a_rows if r["outcome"]["status"] == "verified"}
        sb = {(r["workload"]["workload_id"], r["round"]): r for r in b_rows if r["outcome"]["status"] == "verified"}
        keys = sorted(set(sa) & set(sb))
        ta = sum(sa[k]["cost"]["s"] for k in keys if sa[k]["cost"].get("s") is not None)
        tb = sum(sb[k]["cost"]["s"] for k in keys if sb[k]["cost"].get("s") is not None)
        cell["verified_pairs"] = len(keys)
        cell["descriptive_verified_pairs_ratio_recorded_s"] = (tb / ta) if keys and ta > 0 else None
    return cell


def p_value(cell: dict, lower_bound_only: bool, band_order: str | None, a: str, b: str) -> tuple[float, str]:
    if not cell["complete"]:
        return 1.0, "incomplete_unknown"
    t975 = t_quantile(0.975, DF)
    ps, signs = [], []
    for r, ci in zip(cell["ratio"], cell["ci95_linearised_t"]):
        if r is None or r <= 0 or ci is None:
            return 1.0, "no_ranking"
        lnr = math.log(r)
        cv = math.log(ci[1] / r) / t975
        signs.append(0 if lnr == 0 else (1 if lnr > 0 else -1))
        if cv <= 0:
            ps.append(0.0 if lnr != 0 else 1.0)
        else:
            ps.append(two_sided_p(lnr / cv, DF))
    if len(set(signs)) != 1 or signs[0] == 0:
        return 1.0, "no_ranking"
    expected = f"{a} < {b}" if signs[0] > 0 else f"{b} < {a}"
    if band_order != expected:
        return 1.0, "no_ranking"
    if lower_bound_only and signs[0] < 0:
        return 1.0, "no_ranking"
    cell["p_per_weight"] = ps
    return max(ps), "candidate"


def holm(cells: list[dict]) -> None:
    order = sorted(range(len(cells)), key=lambda i: (cells[i]["p"], cells[i]["comparison"], cells[i].get("log2_r", 99)))
    alive = True
    for k, i in enumerate(order, start=1):
        c = cells[i]
        c["holm_step"] = k
        c["holm_level"] = ALPHA / (M - k + 1)
        if alive and c["pre"] == "candidate" and c["p"] <= c["holm_level"]:
            c["verdict"] = "supported_within_scope"
            c["direction"] = "B > A" if c["log2_ratio"][0] > 0 else "B < A"
        else:
            alive = False
            c.setdefault("verdict", "no_ranking" if c["pre"] != "incomplete_unknown" else "incomplete_unknown")


def determinism(records: list[dict], a: str, b: str) -> dict:
    ra = {(r["workload"]["workload_id"], r["round"]): r for r in records if r["arm"] == a}
    rb = {(r["workload"]["workload_id"], r["round"]): r for r in records if r["arm"] == b}
    out = {"pairs": 0, "s_mismatch": [], "timeout_censoring": [], "other_status_mismatch": []}
    for k in sorted(set(ra) & set(rb)):
        x, y = ra[k], rb[k]
        out["pairs"] += 1
        sx, sy = x["outcome"]["status"], y["outcome"]["status"]
        if sx == sy and sx in ("verified", "exhausted"):
            if (x["cost"].get("s"), x["cost"].get("total_gae")) != (y["cost"].get("s"), y["cost"].get("total_gae")):
                out["s_mismatch"].append(list(k))
        elif sx != sy and "timeout" in (sx, sy):
            out["timeout_censoring"].append(list(k) + [sx, sy])
        elif sx != sy:
            out["other_status_mismatch"].append(list(k) + [sx, sy])
    out["invalidates_track"] = bool(out["s_mismatch"])
    return out


def cmd_decide(args) -> int:
    out = Path(args.out)
    (out / "raw").mkdir(parents=True, exist_ok=True)
    tracks = {"C": {"bin": args.ecbench_c, "dirs": [Path(args.c_small), Path(args.c16)]},
              "B": {"bin": args.ecbench_b, "dirs": [Path(args.b)]}}
    result = {"schema": "hashed-ecdl-counts.decision/v2", "alpha": ALPHA, "tests_in_family": M,
              "weights": list(WEIGHTS), "resamples": RESAMPLES, "seed": SEED, "tracks": {}, "cells": []}
    band_orders: dict[tuple[str, str, str], str] = {}
    recs: dict[str, list[dict]] = {}
    for t, cfg in tracks.items():
        recs[t] = [r for d in cfg["dirs"] for r in read_records(d)]
        bpath = out / "raw" / f"band-{t}.json"
        argv = [cfg["bin"], "band", "--dir", *map(str, cfg["dirs"]), "--weights", "0,0.1,1", "--ranks",
                "--json", str(bpath)]
        proc = subprocess.run(argv, capture_output=True, text=True)
        (out / "raw" / f"band-{t}.txt").write_text(proc.stdout + proc.stderr)
        if proc.returncode == 0 and bpath.is_file():
            for rk in json.loads(bpath.read_text())["ranks"]:
                band_orders[(rk["curve"], rk["a"], rk["b"])] = rk["verdict"]
                band_orders[(rk["curve"], rk["b"], rk["a"])] = rk["verdict"]
        verify = []
        for d in cfg["dirs"]:
            p = subprocess.run([cfg["bin"], "verify", "--dir", str(d), "--replay-all"], capture_output=True, text=True)
            verify.append({"dir": str(d), "exit": p.returncode})
            (out / "raw" / f"verify-{t}-{d.name}.txt").write_text(p.stdout + p.stderr)
        det = {cid: determinism(recs[t], a, b) for cid, tt, a, b in AA if tt == t}
        result["tracks"][t] = {"verify_replay_all": verify, "determinism": det,
                               "valid": all(v["exit"] == 0 for v in verify)
                               and not any(x["invalidates_track"] for x in det.values())}
    if not result["tracks"]["B"]["valid"] or any(
            r["outcome"]["status"] != "verified" for r in recs["B"] if r["arm"] == "rho-neg-v2"):
        result["tracks"]["B"]["valid"] = False
    cells = []
    for cid, t, a, b, lb in FAMILY:
        cfg = tracks[t]
        for d in cfg["dirs"]:
            by_w = {w: compare(cfg["bin"], d, a, b, w, out / "raw" / f"compare-{cid}-{d.name}-w{w}.json") for w in WEIGHTS}
            slugs = sorted({r["workload"]["curve"]["slug"] for r in read_records(d)})
            for slug in slugs:
                cell = cell_from_compares(by_w, arm_rows(recs[t], a, slug), arm_rows(recs[t], b, slug), slug)
                cell.update({"comparison": cid, "track": t, "a": a, "b": b, "lower_bound_only": lb})
                if not result["tracks"][t]["valid"]:
                    cell["p"], cell["pre"], cell["verdict"] = 1.0, "invalid_measurement", "invalid_measurement"
                else:
                    raw = band_orders.get((slug, a, b))
                    cell["band_order"] = raw
                    cell["p"], cell["pre"] = p_value(cell, lb, raw, a, b)
                cells.append(cell)
    if len(cells) != M:
        print(f"expected {M} family cells, got {len(cells)}", file=sys.stderr)
        result["family_size_error"] = len(cells)
    holm(cells)
    result["cells"] = cells
    desc = []
    for cid, t, a, b in DESCRIPTIVE:
        for d in tracks[t]["dirs"]:
            by_w = {w: compare(tracks[t]["bin"], d, a, b, w, out / "raw" / f"compare-{cid}-{d.name}-w{w}.json") for w in WEIGHTS}
            for slug in sorted({r["workload"]["curve"]["slug"] for r in read_records(d)}):
                c = cell_from_compares(by_w, arm_rows(recs[t], a, slug), arm_rows(recs[t], b, slug), slug)
                c.update({"comparison": cid, "verdict": c.get("verdict", "descriptive")})
                desc.append(c)
    result["descriptive"] = desc
    c_rho = sorted((c for c in cells if c["comparison"] == "C-rho"), key=lambda c: c.get("log2_r", 99))
    p1_ok = len(c_rho) == 4 and all(c["verdict"] == "supported_within_scope" and c["direction"] == "B > A" for c in c_rho)
    if p1_ok:
        p1_ok = all(c_rho[i]["ratio"][j] < c_rho[i + 1]["ratio"][j] for i in range(3) for j in range(3))
    result["P1"] = ("supported_within_scope" if p1_ok else
                    "incomplete_unknown" if any(c["verdict"] == "incomplete_unknown" for c in c_rho) else "not_supported")
    native = [c for c in cells if c["lower_bound_only"]]
    complete = [c for c in native if c["complete"]]
    result["P4"] = {"complete_cells_host_dependent": [[c["comparison"], c["slug"]] for c in complete],
                    "verdict": ("not_attempted" if not complete else
                                "supported_within_scope" if all(c["verdict"] == "supported_within_scope" for c in complete)
                                else "not_supported_on_some_complete_cell")}
    (out / "decision.json").write_text(json.dumps(result, indent=1) + "\n")
    print(json.dumps({"P1": result["P1"], "P4": result["P4"]["verdict"],
                      "ranked": sum(c["verdict"] == "supported_within_scope" for c in cells)}))
    return 0


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)
    sub.add_parser("power")
    d = sub.add_parser("decide")
    for k in ("--ecbench-c", "--ecbench-b", "--c-small", "--c16", "--b", "--out"):
        d.add_argument(k, required=True)
    a = ap.parse_args(argv)
    if a.cmd == "power":
        print(json.dumps(power_table(), indent=1))
        return 0
    return cmd_decide(a)


if __name__ == "__main__":
    raise SystemExit(main())
