#!/usr/bin/env python3
"""Replay the EKS benchmark's static admission predicates for probe v2.

This is a source/spec check, not the native launcher or an ecbench run.
"""

import hashlib
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
FULL = ROOT / "research/rootfind_f456_batch_combined_20261115/SPEC-K13.json"
EKS_SOURCE = ROOT / "taskq/deploy/eks-karpenter/mcp/src/main.rs"
PINS = {
    "PROTOCOL.md": "894d9258f0b06800f427a49c5c2284dfba8f43b23a5510d0f87bbb6e2aeaf512",
    "SPEC-K13-ADMISSION.json": "4eaae44bd7d3bd0c9f811b8cfe6ffba98bd560644b6e9d2415fbb8aeccb9cc16",
    "SPEC-K13-ADMISSION-v2.json": "e7a6e076806e0a78c0904927a025ee9f3fdac6f931adbac30bfe1560d3911cb7",
}


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load(path):
    return json.loads(path.read_text())


def named(spec, name):
    return next(arm for arm in spec["arms"] if arm["name"] == name)


def main():
    for file, expected in PINS.items():
        assert digest(HERE / file) == expected, file
    source_hash = digest(EKS_SOURCE)
    assert source_hash == "7353a57a151810173d36eaff6b8486a74d840c94dfe9954c543bad76972c8f4c"
    source = EKS_SOURCE.read_text()
    assert 'benchmark requires an L2 spec, >=5 rounds, >=1 warmup, reference and matching A/A control' in source
    full, v1, v2 = load(FULL), load(HERE / "SPEC-K13-ADMISSION.json"), load(HERE / "SPEC-K13-ADMISSION-v2.json")
    assert v2["schema"] == "ecbench.spec/v1"
    assert v2["workloads"] == full["workloads"]
    assert v2["measurement"]["isolation_required"] == "L2"
    assert v2["measurement"]["rounds"] >= 5
    assert v2["measurement"]["warmup"] >= 1
    assert v2["measurement"]["seed"] == full["measurement"]["seed"]
    assert v2["measurement"]["order"] == full["measurement"]["order"]
    assert named(v2, "f6-ic")["role"] == "reference"
    assert named(v2, "f6-ic")["method"] == named(full, "f6-ic")["method"]
    assert named(v2, "batch-m3") == named(full, "batch-m3")
    assert named(v2, "batch-m3-aa") == named(full, "batch-m3-aa")
    assert any(
        control["role"] == "control"
        and any(
            arm["role"] != "control"
            and arm["name"] != control["name"]
            and arm["method"] == control["method"]
            for arm in v2["arms"]
        )
        for control in v2["arms"]
    )
    assert v1["measurement"]["rounds"] == 2 and not any(a["role"] == "reference" for a in v1["arms"])
    total = len(v2["arms"]) * v2["workloads"]["targets_per_curve"] * (v2["measurement"]["rounds"] + v2["measurement"]["warmup"])
    measured = len(v2["arms"]) * v2["workloads"]["targets_per_curve"] * v2["measurement"]["rounds"]
    assert (total, measured) == (144, 120)
    print(json.dumps({
        "schema": "rootfind-k13-admission-static-validation/v1",
        "status": "pass_static_predicates_only",
        "source_sha256": source_hash,
        "pinned_file_sha256": PINS,
        "full_panel_spec_sha256": digest(FULL),
        "full_panel_workload_identity": "equal",
        "batch_and_f6_method_identity": "equal",
        "eks_static_admission_predicates": "pass",
        "planned_executions_derived": total,
        "measured_executions_derived": measured,
        "not_run": ["native EKS launcher", "source-bound ecbench plan", "image build", "experiment job"],
    }, indent=2))


if __name__ == "__main__":
    main()
