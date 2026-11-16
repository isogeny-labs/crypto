#!/usr/bin/env python3
"""Join the frozen 1,260 targets and price complete cold one-target solves."""
import csv
import json
import math
import subprocess
import sys
from collections import defaultdict
from pathlib import Path

BITS = (16, 20, 24, 28, 32, 36, 40)
CONTROL_ARCHIVE = Path("research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst")
ARMS = ("external", "genus_projective", "rho_neg")


def read_rows(path):
    return [json.loads(line) for line in Path(path).read_text().splitlines()]


def read_frozen_rows(bits):
    member = f"sessions_projective_chart_v2/order-genus-{bits}/records.jsonl"
    archived = subprocess.run(
        ["tar", "--zstd", "-xOf", str(CONTROL_ARCHIVE), member],
        check=True,
        capture_output=True,
        text=True,
    ).stdout
    return [json.loads(line) for line in archived.splitlines()]


def logical(counters):
    return sum(value for key, value in counters.items() if key.endswith("_uncharged"))


def price(row, arm, square, inversion, native):
    if arm == "external":
        c = row["counts"]
        unit = row["source_add"]
        n = row["native"]
        logical_cost = logical(n) + c["field_adds"] + c["bit_ops"]
        arithmetic = c["m"] + square * c["s"] + inversion * c["i"]
        divisor = unit["m"] + square * unit["s"] + inversion * unit["i"]
    else:
        c = row["counters"]
        n = c
        logical_cost = logical(n)
        arithmetic = c["field_multiplications"] + square * c["field_squarings"] + inversion * c["field_inversions"]
        divisor = c["source_add_multiplications"] + square * c["source_add_squarings"] + inversion * c["source_add_inversions"]
    assert divisor > 0
    return arithmetic / divisor + native * logical_cost


def main():
    result_dir = Path(sys.argv[1])
    groups = {}
    for bits in BITS:
        fresh = read_rows(result_dir / f"order-genus-{bits}.jsonl")
        assert len(fresh) == 180
        frozen = read_frozen_rows(bits)
        by_arm = {arm: {r["workload"]["workload_id"]: r for r in frozen if r["arm"] == arm} for arm in ARMS[1:]}
        assert all(len(rows) == 180 for rows in by_arm.values())
        for row in fresh:
            wid = row["workload_id"]
            assert wid not in groups
            assert row["verified"] and not row["exhausted"]
            assert row["recovered"] == row["planted"]
            p = by_arm["genus_projective"][wid]
            rho = by_arm["rho_neg"][wid]
            assert p["workload"] == rho["workload"]
            assert p["workload"]["workload_sha256"] == row["workload_sha256"]
            assert p["algorithm_seed"] == rho["algorithm_seed"] == row["algorithm_seed"]
            assert int(p["method"]["params"]["trial_cap"]) == row["cap"]
            assert p["workload"]["planted"] == row["planted"]
            assert p["outcome"]["status"] == rho["outcome"]["status"] == "verified"
            for control in (p, rho):
                assert [row["source_add"][k] for k in ("m", "s", "i")] == [control["counters"][f"source_add_{k}"] for k in ("multiplications", "squarings", "inversions")]
            key = (bits, p["workload"]["curve"]["slug"], row["target_index"])
            groups[wid] = (key, {"external": row, "genus_projective": p, "rho_neg": rho})
    assert len(groups) == 1260
    curves = defaultdict(set)
    for (bits, slug, index), _ in groups.values():
        curves[bits, slug].add(index)
    assert len(curves) == 21
    assert all(indices == set(range(60)) for indices in curves.values())
    slices = [("all", "all", list(groups.values()))]
    slices += [("bits", str(bits), [g for g in groups.values() if g[0][0] == bits]) for bits in BITS]
    slices += [("curve", f"{bits}:{slug}", [g for g in groups.values() if g[0][:2] == (bits, slug)]) for bits, slug in sorted(curves)]
    control_report = json.loads(Path("research/five_leads_e2e_20261019/results/genus2-projective-summary.json").read_text())
    control_slices = {(row["kind"], row["label"]): row["representative"] for row in control_report["summary"]}
    grid = []
    summary = []
    for kind, label, selected in slices:
        ratios_rho, ratios_projective = [], []
        representative = None
        for square in (0, .5, 1):
            for inversion in (1, 10, 100, 1000):
                for native in (0, .1, 1):
                    totals = {arm: sum(price(arms[arm], arm, square, inversion, native) for _, arms in selected) for arm in ARMS}
                    row = {"kind": kind, "label": label, "targets": len(selected), "S_per_M": square, "I_per_M": inversion, "native_GAE_per_logical_operation": native, **{f"{arm}_GAE": totals[arm] for arm in ARMS}, "external_over_rho": totals["external"] / totals["rho_neg"], "external_over_projective": totals["external"] / totals["genus_projective"]}
                    grid.append(row)
                    ratios_rho.append(row["external_over_rho"])
                    ratios_projective.append(row["external_over_projective"])
                    if (square, inversion, native) == (.5, 10, .1):
                        representative = row
        ext = [arms["external"] for _, arms in selected]
        control = control_slices[kind, label]
        assert math.isclose(representative["rho_neg_GAE"], control["rho_neg_GAE"], rel_tol=1e-12)
        assert math.isclose(representative["genus_projective_GAE"], control["genus_projective_GAE"], rel_tol=1e-12)
        summary.append({"kind": kind, "label": label, "targets": len(selected), "all_verified": True, "failures": 0, "exhaustions": 0, "representative": representative, "external_over_rho_band": [min(ratios_rho), max(ratios_rho)], "external_over_projective_band": [min(ratios_projective), max(ratios_projective)], "external_win_cells_vs_rho": sum(x < 1 for x in ratios_rho), "external_win_cells_vs_projective": sum(x < 1 for x in ratios_projective), "processed_divisors": sum(r["native"]["processed_divisors"] for r in ext), "external_additions": sum(r["native"]["external_additions_uncharged"] for r in ext), "equality_dispatches": sum(r["native"]["external_equality_dispatches_uncharged"] for r in ext)})
    with (result_dir / "price-grid.csv").open("w", newline="") as fh:
        writer = csv.DictWriter(fh, fieldnames=grid[0].keys())
        writer.writeheader()
        writer.writerows(grid)
    (result_dir / "summary.json").write_text(json.dumps({"schema": "genus2-external-collector-analysis/v1", "records": len(groups), "summary": summary}, indent=2) + "\n")


if __name__ == "__main__":
    main()
