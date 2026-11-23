#!/usr/bin/env python3
"""Frozen summary of the two K1 rows this directory supports, for the IC leaderboard.

    python3 -I research/ml_cryptanalysis_pt_lgrow_k1_20261114/summarize.py          # write summary.json
    python3 -I research/ml_cryptanalysis_pt_lgrow_k1_20261114/summarize.py --check  # fail if stale

Reads only the copied ecbench session records in fb-pt-lgrow-k1/sessions/ and the
frozen producer output fb-pt-lgrow-k1/results.json.  Point estimates are
recomputed here from the records with the AGENTS.md section 5a band:

- rho arm: (total_gae + w * every *_uncharged counter the record lists as unpriced) / sqrt(r);
- IC arm: (sum of phase adds + doubles + w * native operations) / sqrt(r), native =
  solver ops + relation trials + relation lookups + factor-base AS solves +
  Frobenius maps + LA row ops + unpriced counters;
- bitop/v1: sum of work.phases[].bitops / sqrt(r) (calibration-free cross-check);

with sqrt(r) = cost.total_gae / cost.s of each record.  The gap to rho is
log2(sum S_arm / sum S_rho) over the 40 measured solves (8 targets x 5 rounds)
of one n.  95% intervals are copied from results.json (the frozen score.py's
target bootstrap, reproduced by the validator); they are not recomputed here.
"""
from __future__ import annotations

import collections
import json
import math
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
EXP = HERE / "fb-pt-lgrow-k1"
OUT = HERE / "summary.json"
W = {"0": 0.0, "0.1": 0.1, "1": 1.0}
ROWS = {23: ("ic-pt-l9", 9), 29: ("ic-pt-l10", 10)}
BEFORE = "ic-pt-l7"
RHO = "rho-strong"


def unpriced(r: dict) -> float:
    return sum(r["counters"].get(u, 0) for u in (r["cost"].get("unpriced") or []))


def native(r: dict) -> float:
    ph = {p["name"]: p for p in r["phases"]}

    def g(phase: str, key: str) -> float:
        return ((ph.get(phase) or {}).get("native") or {}).get(key, 0)
    return ((r.get("solver") or {}).get("ops", 0) + g("relations", "trials") + g("relations", "lookups")
            + g("factor_base", "as_solves") + g("factor_base", "frobenius_maps")
            + g("linear_algebra", "row_ops") + unpriced(r))


def sqrt_r(r: dict) -> float:
    return r["cost"]["total_gae"] / r["cost"]["s"]


def s_band(r: dict, w: float) -> float:
    if r["arm"].startswith("ic-"):
        return (sum(p["adds"] + p["doubles"] for p in r["phases"]) + w * native(r)) / sqrt_r(r)
    return (r["cost"]["total_gae"] + w * unpriced(r)) / sqrt_r(r)


def s_bitop(r: dict) -> float:
    return sum(int(p["bitops"]) for p in r["work"]["phases"]) / sqrt_r(r)


def build() -> dict:
    recs: dict = collections.defaultdict(list)
    for f in sorted(EXP.glob("sessions/*/session/records.jsonl")):
        for line in f.read_text().splitlines():
            r = json.loads(line)
            if not r["warmup"]:
                recs[(r["workload"]["curve_spec"]["n"], r["arm"])].append(r)
    results = json.loads((EXP / "results.json").read_text())
    rows = []
    for n, (arm, dim) in ROWS.items():
        cell = {a: recs[(n, a)] for a in (RHO, BEFORE, arm)}
        for a, rs in cell.items():
            bad = [r["seq"] for r in rs if not (r["outcome"]["status"] == "verified"
                                               and r["outcome"]["matches_planted"] is True
                                               and r["outcome"]["matches_target"] is True)]
            if len(rs) != 40 or bad:
                raise SystemExit(f"n={n} {a}: {len(rs)} measured records, unverified seqs {bad}")
        curves = {json.dumps(r["workload"]["curve"], sort_keys=True) for rs in cell.values() for r in rs}
        targets = sorted({r["workload"]["workload_id"] for rs in cell.values() for r in rs})
        if len(curves) != 1 or len(targets) != 8:
            raise SystemExit(f"n={n}: {len(curves)} curves, {len(targets)} targets")
        curve = json.loads(curves.pop())
        methods = {a: rs[0]["method"] for a, rs in cell.items()}
        if any(r["method"] != methods[a] for a, rs in cell.items() for r in rs):
            raise SystemExit(f"n={n}: method differs within an arm")
        binaries = {r["binary_sha256"] for rs in cell.values() for r in rs}

        def mean(a: str, f) -> float:
            return sum(f(r) for r in cell[a]) / len(cell[a])

        def gap(a: str, f) -> float:
            return math.log2(sum(f(r) for r in cell[a]) / sum(f(r) for r in cell[RHO]))
        band = {}
        for k, w in W.items():
            f = lambda r, w=w: s_band(r, w)  # noqa: E731
            band[k] = {"s_after": mean(arm, f), "s_before": mean(BEFORE, f), "s_rho": mean(RHO, f),
                       "gap_after_bits": gap(arm, f), "gap_before_bits": gap(BEFORE, f)}
            band[k]["delta_bits"] = band[k]["gap_before_bits"] - band[k]["gap_after_bits"]
            u = results["units"][f"band@{k}"]
            band[k]["gap_after_ci95"] = u[f"gap_lstar_n{n}_ci"]
            band[k]["gap_before_ci95"] = u[f"gap_l7_n{n}_ci"]
            band[k]["delta_ci95"] = u[f"R_n{n}_ci"]
        bitop = {"gap_after_bits": gap(arm, s_bitop), "gap_before_bits": gap(BEFORE, s_bitop)}
        rows.append({
            "slug": curve["slug"], "n": n, "r": curve["r"], "log2_r": math.log2(curve["r"]),
            "cofactor": curve["cofactor"], "group_order": curve["group_order"], "icv1": curve["icv1"],
            "targets": len(targets), "rounds": 5, "solves_per_arm": 40,
            "arm": arm, "before_arm": BEFORE, "reference_arm": RHO,
            "recipe": {"method": methods[arm]["id"], **{k: methods[arm]["params"][k]
                       for k in ("factor_base", "oracle", "linalg", "targets")}},
            "before_recipe": {k: methods[BEFORE]["params"][k] for k in ("factor_base", "oracle")},
            "reference": methods[RHO]["id"], "pair_table_dimension": dim,
            "binary_sha256": sorted(binaries),
            "band": band, "bitop": bitop,
        })
    return {
        "schema": "crypto.ic_k1_pt_lgrow_summary/v1",
        "source": {"repository": "github.com/aburan28/ml-cryptanalysis", "branch": "main",
                   "commit": "7d4e8e0fccb43148c80fd2004821fd7e48f9df76",
                   "path": "experiments/fb-pt-lgrow-k1", "validation_id": "VAL-20261114-d9190c",
                   "validator_files_redacted_at": "b504b611 (ml-cryptanalysis PR #314: model id redacted, sha256 re-pinned)"},
        "unit": "ecbench counted GAE per sqrt(r); IC native operations and rho *_uncharged counters at w GAE each "
                "(AGENTS.md section 5a); bitop/v1 as a calibration-free cross-check",
        "isolation": "L0 (counted cost only; wall time not certified)",
        "rows": rows,
        "not_a_row": "D (slope difference over n = 17..29) equals (R_n23 + 7 R_n29) / 84 by construction: an offset, "
                     "not a bent slope",
    }


def main() -> int:
    text = json.dumps(build(), indent=1, sort_keys=False) + "\n"
    if "--check" in sys.argv:
        if not OUT.is_file() or OUT.read_text() != text:
            print(f"{OUT.relative_to(HERE.parent.parent)} is stale; rerun summarize.py", file=sys.stderr)
            return 1
        return 0
    OUT.write_text(text)
    return 0


if __name__ == "__main__":
    sys.exit(main())
