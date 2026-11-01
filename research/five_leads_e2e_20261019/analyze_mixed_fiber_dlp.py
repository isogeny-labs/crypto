#!/usr/bin/env python3
"""Audit retained native records and render their complete-cost price grid."""
import csv
import json
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = HERE / "correctness/mixed-fiber-dlp-v2.jsonl"
ROWS = [json.loads(line) for line in SOURCE.read_text().splitlines()]
ARMS = ("homogeneous", "mixed", "random")


def native_candidate(row):
    c = row["candidate"]
    return (
        row["selection_draws_uncharged"]
        + c["rng_draws_uncharged"]
        + c["table_lookups_uncharged"]
        + c["row_operations_uncharged"]
        + c["scalar_adds_uncharged"]
        + c["scalar_muls_uncharged"]
        + c["scalar_inversions_uncharged"]
        + c["pair_insertions"]
    )


def native_rho(row):
    return sum(value for key, value in row["rho"]["native"].items() if key.endswith("_uncharged"))


def cost(counts, s, i, native_operations, native_weight):
    return counts["m"] + s * counts["s"] + i * counts["i"] + native_weight * native_operations


def main():
    assert len(ROWS) == 540
    assert all(row["schema"] == "mixed-fiber-dlp/v2" for row in ROWS)
    by_target = defaultdict(list)
    for row in ROWS:
        target_key = (row["p"], row["a"], row["b"], row["target_index"])
        by_target[target_key].append(row)
        assert row["candidate"]["verified"] and not row["candidate"]["exhausted"]
        assert row["rho"]["verified"]
        assert row["candidate"]["recovered"] == row["planted_scalar_check_only"]
        assert row["rho"]["recovered"] == row["planted_scalar_check_only"]
        assert row["candidate"]["trials"] <= row["candidate"]["cap"]
        for symbol in ("m", "s", "i"):
            assert row["candidate"]["total_counts"][symbol] >= row["selection_counts"][symbol]
    assert len(by_target) == 180
    by_curve = defaultdict(set)
    for key, rows in by_target.items():
        assert sorted(row["arm"] for row in rows) == sorted(ARMS)
        assert len({tuple(row["target"]) for row in rows}) == 1
        assert len({json.dumps(row["rho"], sort_keys=True) for row in rows}) == 1
        by_curve[key[:3]].add(key[3])
    assert len(by_curve) == 9
    assert all(indices == set(range(20)) for indices in by_curve.values())
    assert {p for p, _, _ in by_curve} == {97, 193, 257}
    assert all(sum(key[0] == p for key in by_curve) == 3 for p in (97, 193, 257))

    groups = [("all", "all", ROWS)]
    groups += [("prime", str(p), [row for row in ROWS if row["p"] == p]) for p in (97, 193, 257)]
    groups += [("curve", f"{p}:{a}:{b}", [row for row in ROWS if (row["p"], row["a"], row["b"]) == (p, a, b)]) for p, a, b in sorted(by_curve)]
    grid = []
    summary = []
    for kind, label, records in groups:
        for arm in ARMS:
            subset = [row for row in records if row["arm"] == arm]
            price_rows = []
            for s in (0, 0.5, 1):
                for i in (1, 10, 100, 1000):
                    for weight in (0, 0.1, 1):
                        cand = sum(cost(row["candidate"]["total_counts"], s, i, native_candidate(row), weight) for row in subset)
                        rho = sum(cost(row["rho"]["counts"], s, i, native_rho(row), weight) for row in subset)
                        price = {"kind":kind,"label":label,"arm":arm,"targets":len(subset),
                                 "S_per_M":s,"I_per_M":i,"native_M_per_logical_op":weight,
                                 "candidate_M":cand,"rho_M":rho,"candidate_over_rho":cand / rho}
                        grid.append(price); price_rows.append(price)
            representative = next(row for row in price_rows if (row["S_per_M"],row["I_per_M"],row["native_M_per_logical_op"]) == (0.5,10,0.1))
            summary.append({"kind":kind,"label":label,"arm":arm,"targets":len(subset),
                            "candidate_verified":len(subset),"rho_verified":len(subset),
                            "failures":0,"exhaustions":0,"representative":representative,
                            "ratio_band":[min(row["candidate_over_rho"] for row in price_rows),max(row["candidate_over_rho"] for row in price_rows)],
                            "candidate_trials":sum(row["candidate"]["trials"] for row in subset),
                            "relations":sum(row["candidate"]["relations"] for row in subset),
                            "selection_M":sum(row["selection_counts"]["m"] for row in subset),
                            "pair_insertions":sum(row["candidate"]["pair_insertions"] for row in subset)})
    output = HERE / "results"
    output.mkdir(exist_ok=True)
    (output / "mixed-fiber-dlp-v2-summary.json").write_text(json.dumps({"schema":"mixed-fiber-dlp-analysis/v2","rows":len(ROWS),"targets":len(by_target),"curves":len(by_curve),"summary":summary}, indent=2) + "\n")
    with (output / "mixed-fiber-dlp-v2-price-grid.csv").open("w", newline="") as file:
        writer = csv.DictWriter(file, fieldnames=grid[0].keys(), lineterminator="\n")
        writer.writeheader(); writer.writerows(grid)
    print(json.dumps([row for row in summary if row["kind"] == "all"], indent=2))


if __name__ == "__main__":
    main()
