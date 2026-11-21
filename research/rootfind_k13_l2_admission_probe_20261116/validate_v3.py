#!/usr/bin/env python3
"""Check v3 non-Groebner coverage and static EKS admission against pinned source."""

import hashlib
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
FULL = ROOT / "research/rootfind_f456_batch_combined_20261115/SPEC-K13.json"
EKS = ROOT / "taskq/deploy/eks-karpenter/mcp/src/main.rs"
EXPECTED = (
    "rho-strong", "rootfind-m3", "rootfind-m2",
    "batch-m3", "batch-m2", "batch-m3-aa",
)
V3_SHA = "ef890a6193c6b7c45d09775a27dede3c3eb16d7ab2ffaac8b359d18e976ef308"
EKS_SHA = "7353a57a151810173d36eaff6b8486a74d840c94dfe9954c543bad76972c8f4c"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read(path):
    return json.loads(path.read_text())


def main():
    v2_path = HERE / "SPEC-K13-ADMISSION-v2.json"
    v3_path = HERE / "SPEC-K13-ADMISSION-v3.json"
    assert sha(EKS) == EKS_SHA
    assert sha(v3_path) == V3_SHA
    assert 'rejects ANY record below L2' in EKS.read_text()
    full, v2, v3 = read(FULL), read(v2_path), read(v3_path)
    assert v3["schema"] == "ecbench.spec/v1"
    assert v3["workloads"] == full["workloads"]
    assert tuple(a["name"] for a in v3["arms"]) == EXPECTED
    assert v3["arms"] == [a for a in full["arms"] if a["name"] in EXPECTED]
    assert v3["measurement"] == dict(full["measurement"], rounds=5, timeout_seconds=30)
    assert v3["measurement"]["isolation_required"] == "L2"
    assert v3["measurement"]["warmup"] >= 1
    assert len([a for a in v3["arms"] if a["role"] == "reference"]) == 1
    assert next(a for a in v3["arms"] if a["role"] == "reference")["name"] == "rho-strong"
    assert any(
        c["role"] == "control"
        and any(a["role"] != "control" and a["name"] != c["name"] and a["method"] == c["method"] for a in v3["arms"])
        for c in v3["arms"]
    )
    v2_names = {a["name"] for a in v2["arms"]}
    omitted_v2 = sorted(set(EXPECTED) - v2_names)
    assert omitted_v2 == ["batch-m2", "rho-strong", "rootfind-m2", "rootfind-m3"]
    target_count = v3["workloads"]["targets_per_curve"]
    total = len(v3["arms"]) * target_count * (v3["measurement"]["warmup"] + v3["measurement"]["rounds"])
    measured = len(v3["arms"]) * target_count * v3["measurement"]["rounds"]
    assert (total, measured) == (288, 240)
    print(json.dumps({
        "schema": "rootfind-k13-admission-static-validation/v3",
        "status": "pass_static_predicates_and_coverage_only",
        "v3_sha256": V3_SHA,
        "eks_source_sha256": EKS_SHA,
        "full_panel_spec_sha256": sha(FULL),
        "exact_full_panel_workload": True,
        "exact_full_panel_arm_objects": list(EXPECTED),
        "v2_omitted_non_groebner_arms": omitted_v2,
        "native_eks_static_predicates": "pass",
        "planned_executions_derived": total,
        "measured_executions_derived": measured,
        "not_run": ["native benchmark validator", "source-bound ecbench plan", "image build", "experiment job"],
    }, indent=2))


if __name__ == "__main__":
    main()
