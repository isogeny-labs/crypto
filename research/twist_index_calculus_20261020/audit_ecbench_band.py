#!/usr/bin/env python3
"""Audit sealed ecbench statuses and the stated unpriced-operation band.

This reads immutable session records and audit receipts. It neither runs a
method nor measures runtime.
"""

import json
import math
from collections import Counter, defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
RATES = (0, 0.1, 1)


def records(b, pin):
    suffix = "_pin_reg" if pin else ""
    path = HERE / "sessions" / f"ecbench_b{b}{suffix}_l0" / "records.jsonl"
    return [json.loads(line) for line in path.read_text().splitlines()]


def audit_receipt(b, pin):
    suffix = "_pin_reg" if pin else ""
    receipt = json.loads((HERE / f"ecbench_b{b}{suffix}.audit.json").read_text())
    assert receipt["ok"] and not receipt["problems"]
    assert receipt["session_status"] == "complete"
    assert all(r["reproduced"] for r in receipt["replays"])
    return len(receipt["replays"])


def unpriced_count(row):
    """Require a count for every label declared unpriced by ecbench."""
    out = 0
    by_label = {}
    for label in row["cost"]["unpriced"]:
        key = label.removesuffix("_uncharged")
        values = [phase["native"][key] for phase in row["phases"]
                  if key in phase["native"]]
        if label in row["counters"]:
            values.append(row["counters"][label])
        elif key in row["counters"] and not values:
            values.append(row["counters"][key])
        assert values, (row["run_id"], label)
        by_label[label] = sum(values)
        out += by_label[label]
    return out, by_label


def main():
    output = []
    for b in (12, 24, 36):
        strict = records(b, False)
        pin = records(b, True)
        assert len(strict) == len(pin) == 324
        strict_status = Counter(r["outcome"]["status"] for r in strict)
        pin_status = Counter(r["outcome"]["status"] for r in pin)
        assert strict_status == (
            Counter({"verified": 306, "exhausted": 18}) if b == 36
            else Counter({"verified": 324})
        )
        assert pin_status == Counter({"verified": 324})
        assert audit_receipt(b, False) == (255 if b == 36 else 270)
        assert audit_receipt(b, True) == 270
        plan = json.loads((HERE / "sessions" /
                           f"ecbench_b{b}_pin_reg_l0" / "plan.json").read_text())
        assert len(plan["workloads"]) == 18
        assert all(w["curve"]["registered"] is True
                   for w in plan["workloads"])
        sums = defaultdict(lambda: [0.0, 0.0])
        labels = defaultdict(Counter)
        for row in pin:
            if row["warmup"] or row["arm"] not in ("rho-neg", "ic-native"):
                continue
            assert row["outcome"]["status"] == "verified"
            assert row["isolation"]["level"] == "L0"
            r = row["workload"]["curve"]["r"]
            arm = row["arm"]
            sums[arm][0] += row["cost"]["total_gae"] / math.sqrt(r)
            unpriced, by_label = unpriced_count(row)
            sums[arm][1] += unpriced / math.sqrt(r)
            labels[arm].update(by_label)
        comparison = json.loads((
            HERE / "sessions" / f"ecbench_b{b}_pin_reg_l0" / "comparisons"
            / "rho-neg__ic-native.json"
        ).read_text())
        a0, a1 = sums["rho-neg"]
        b0, b1 = sums["ic-native"]
        band = [(b0 + rate * b1) / (a0 + rate * a1) for rate in RATES]
        assert math.isclose(band[0], comparison["ops"]["ratio_b_over_a"],
                            rel_tol=1e-12)
        assert comparison["ops"]["status"] == "ok"
        assert comparison["ops"]["pairs"] == 90
        output.append({
            "B": b,
            "strict_status": dict(strict_status),
            "registered_target_pin_status": dict(pin_status),
            "strict_replays": 255 if b == 36 else 270,
            "registered_pin_replays": 270,
            "unpriced_primitive_counts": {k: dict(v) for k, v in labels.items()},
            "pricing_rates_gae": list(RATES),
            "ic_over_rho_s_band": [round(x, 6) for x in band],
        })
    print(json.dumps({"schema": "mixed-ecbench-band/v1", "rows": output},
                     indent=2))


if __name__ == "__main__":
    main()
