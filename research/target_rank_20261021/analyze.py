#!/usr/bin/env python3
"""Join the rank arm to frozen matched controls and price every cold record."""

import json
from collections import Counter, defaultdict
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OLD = ROOT / "five_leads_e2e_20261019"
NEW = Path(__file__).resolve().parent
BITS = (16, 20, 24, 28, 32, 36, 40)
ARMS = ("genus_target_rank", "genus_projective", "genus_choice2", "genus_ic", "rho_neg")


def read_records(path):
    return [json.loads(line) for line in path.read_text().splitlines()]


def price(row, s, i, native):
    c = row["counters"]
    unit = c["source_add_multiplications"] + s * c["source_add_squarings"] + i * c["source_add_inversions"]
    arithmetic = c["field_multiplications"] + s * c["field_squarings"] + i * c["field_inversions"]
    logical = sum(v for k, v in c.items() if k.endswith("_uncharged"))
    assert unit > 0
    return arithmetic / unit + native * logical


def main():
    groups = defaultdict(list)
    totals = {arm: Counter() for arm in ARMS}
    sessions = []
    for bits in BITS:
        fresh_dir = NEW / "sessions" / f"order-genus-{bits}"
        session = json.loads((fresh_dir / "session.json").read_text())
        replay = json.loads((NEW / "replay" / f"order-genus-{bits}.json").read_text())
        assert session["status"] == "complete" and session["records_written"] == 180
        assert replay["ok"] is True and replay["verified_records"] == 180
        assert replay["session_id"] == session["session_id"]
        sessions.append({"bits": bits, "session_id": session["session_id"], "records_sha256": session["records_sha256"]})
        newer = read_records(fresh_dir / "records.jsonl")
        prior_projective = read_records(OLD / "sessions_projective_chart_v2" / f"order-genus-{bits}" / "records.jsonl")
        prior_original = read_records(OLD / "sessions" / f"order-genus-{bits}" / "records.jsonl")
        assert len(newer) == 180 and len(prior_projective) == 540 and len(prior_original) == 900
        prior = {(r["workload"]["workload_id"], r["arm"]): r for r in prior_projective + prior_original}
        for rank in newer:
            assert rank["arm"] == "genus_target_rank"
            assert rank["outcome"]["status"] == "verified"
            assert rank["outcome"]["matches_target"] and rank["outcome"]["matches_planted"]
            wid = rank["workload"]["workload_id"]
            controls = {arm: prior[(wid, arm)] for arm in ARMS[1:]}
            for arm, row in controls.items():
                assert row["outcome"]["status"] == "verified"
                assert row["outcome"]["matches_target"] and row["outcome"]["matches_planted"]
                assert row["workload"] == rank["workload"]
                assert row["algorithm_seed"] == rank["algorithm_seed"]
                assert row["outcome"]["recovered"] == rank["outcome"]["recovered"]
                totals[arm].update(row["counters"])
            totals["genus_target_rank"].update(rank["counters"])
            arms = {"genus_target_rank": rank, **controls}
            slug = rank["workload"]["curve"]["slug"]
            for key in (("all", "all"), ("bits", str(bits)), ("curve", slug)):
                groups[key].append(arms)
    assert len(groups[("all", "all")]) == 1260
    cells = []
    for (kind, label), samples in sorted(groups.items()):
        for s in (0.0, 0.5, 1.0):
            for i in (1.0, 10.0, 100.0, 1000.0):
                for native in (0.0, 0.1, 1.0):
                    costs = {arm: sum(price(sample[arm], s, i, native) for sample in samples) for arm in ARMS}
                    cells.append({"kind": kind, "label": label, "targets": len(samples), "S_per_M": s, "I_per_M": i, "native_per_logical": native, "GAE": costs, "ratios": {"rank_over_projective": costs["genus_target_rank"] / costs["genus_projective"], "rank_over_choice2": costs["genus_target_rank"] / costs["genus_choice2"], "rank_over_incumbent": costs["genus_target_rank"] / costs["genus_ic"], "rank_over_rho": costs["genus_target_rank"] / costs["rho_neg"]}})
    representative = next(c for c in cells if c["kind"] == "all" and c["S_per_M"] == 0.5 and c["I_per_M"] == 10.0 and c["native_per_logical"] == 0.1)
    output = {"schema": "target-rank-analysis/v1", "targets": 1260, "sessions": sessions, "representative": representative, "all_cells": [c for c in cells if c["kind"] == "all"], "per_size_representative": [c for c in cells if c["kind"] == "bits" and c["S_per_M"] == 0.5 and c["I_per_M"] == 10.0 and c["native_per_logical"] == 0.1], "per_curve_representative": [c for c in cells if c["kind"] == "curve" and c["S_per_M"] == 0.5 and c["I_per_M"] == 10.0 and c["native_per_logical"] == 0.1], "pooled_wins": {name: sum(c["ratios"][name] < 1 for c in cells if c["kind"] == "all") for name in ("rank_over_projective", "rank_over_choice2", "rank_over_incumbent", "rank_over_rho")}, "counters": {arm: dict(c) for arm, c in totals.items()}}
    print(json.dumps(output, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
