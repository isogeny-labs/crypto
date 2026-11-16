#!/usr/bin/env python3
"""Read-only source-plan and archived-counter audit; launches no ecbench run."""

import argparse
import hashlib
import json
import pathlib
import subprocess
from collections import Counter

EXPECTED_BINARY = "7b062c58a5a1bfa0309d68732f57cf36f8d2394780bc3e111ee7b64b2e3092ed"
EXPECTED_RECORDS = "dc16c97ae21a250cc1d47237231d9642960c100dc3fdb68f4dd210af171bff44"
EXPECTED_ARMS = (
    "rho-strong", "rootfind-m3", "rootfind-m2", "batch-m3", "batch-m2", "batch-m3-aa"
)
SOLVER_ARMS = {"matrix-f4", "matrix-f5", "inherited-f4", "f6-ic"}


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def plan(binary, spec):
    command = [str(binary), "plan", "--spec", str(spec), "--doc"]
    result = subprocess.run(command, capture_output=True, text=True, check=True)
    return json.loads(result.stdout), result.stderr.strip()


def cells(document):
    names = [a["name"] for a in document["arms"]]
    out = {}
    for execution in document["executions"]:
        key = (names[execution["arm"]], execution["workload"], execution["round"], execution["warmup"])
        assert key not in out, f"duplicate execution cell {key}"
        out[key] = execution["algorithm_seed"]
    return out


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=pathlib.Path, required=True)
    parser.add_argument("--admission", type=pathlib.Path, required=True)
    parser.add_argument("--panel", type=pathlib.Path, required=True)
    parser.add_argument("--records", type=pathlib.Path, required=True)
    args = parser.parse_args()
    assert digest(args.binary) == EXPECTED_BINARY, "binary identity mismatch"
    assert digest(args.records) == EXPECTED_RECORDS, "archived object identity mismatch"
    admission, warning = plan(args.binary, args.admission)
    panel, panel_warning = plan(args.binary, args.panel)
    assert admission["workloads"] == panel["workloads"]
    assert len(admission["workloads"]) == 8
    assert tuple(a["name"] for a in admission["arms"]) == EXPECTED_ARMS
    panel_arms = {a["name"]: a for a in panel["arms"]}
    assert all(a == panel_arms[a["name"]] for a in admission["arms"])
    admission_cells, panel_cells = cells(admission), cells(panel)
    assert len(admission_cells) == 288 and len(panel_cells) == 560
    assert all(panel_cells.get(k) == v for k, v in admission_cells.items())
    assert Counter(k[2] for k in admission_cells) == {i: 48 for i in range(6)}
    assert Counter(k[2] for k in panel_cells) == {i: 80 for i in range(7)}

    records = [json.loads(line) for line in args.records.read_text().splitlines()]
    assert len(records) == 48
    counts = Counter(r["arm"] for r in records)
    assert counts == {name: 6 for name in counts} and len(counts) == 8
    for record in records:
        work = record["work"]
        assert work["unit"] == "bitop/v1"
        assert work["complete"] and not work["unpriced"] and not work["known_uncounted"]
        assert int(work["bitops"]) == sum(int(p["bitops"]) for p in work["phases"])
        if record["arm"] in SOLVER_ARMS:
            assert record["solver"]["extra"]["bitop_complete"] == 1
    print(json.dumps({
        "schema": "rootfind-k13-source-plan-audit/v1",
        "status": "pass_read_only_checks",
        "binary_sha256": digest(args.binary),
        "admission_spec_sha256": digest(args.admission),
        "panel_spec_sha256": digest(args.panel),
        "admission_spec_id": admission["spec_id"],
        "panel_spec_id": panel["spec_id"],
        "workloads_exact": len(admission["workloads"]),
        "arms_exact": list(EXPECTED_ARMS),
        "admission_executions": len(admission_cells),
        "panel_executions": len(panel_cells),
        "matching_execution_seeds": len(admission_cells),
        "admission_plan_warning": warning,
        "panel_plan_warning": panel_warning,
        "archived_records_sha256": digest(args.records),
        "archived_records": len(records),
        "archived_arm_counts": dict(sorted(counts.items())),
        "archived_bitop_v1_self_complete_rows": len(records),
        "archived_solver_flag_rows": sum(r["arm"] in SOLVER_ARMS for r in records),
        "instruction_audit_present": False,
        "scientific_trial_launched": False,
    }, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
