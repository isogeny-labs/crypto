"""Fail-closed audit of the source-bound B677 split-2+2 admission arms."""

import hashlib
import json
from pathlib import Path

from verify_mitm_graph import A, B, G, N, P, Q, add

ROOT = Path(__file__).parent
EVIDENCE = ROOT / "evidence/rr5-b677-prerequisite-01"
INPUT = EVIDENCE / "structured-positive.stdout"
OUT = EVIDENCE / "split22-admission-summary.json"
ARMS = [
    "split22-final-positive-m2m-p1-s8k",
    "split22-final-positive-m2m-p4-s8k",
    "split22-final-public-m2m-p1-s8k",
    "split22-positive-m2m-p1-s16k",
    "split22-positive-m2m-p1-s32k",
    "split22-positive-m2m-p1-s64k",
    "split22-positive-m8m-p1-s64k",
]


def main():
    frozen = json.loads(next(line.split(" ", 1)[1] for line in INPUT.read_text().splitlines()
                             if line.startswith("B677_INPUT ")))
    curve = {"p": P, "a": A, "b": B, "n": N, "g": list(G)}
    assert frozen["curve"] == curve
    positive = tuple(frozen["public_target"])
    total = None
    for index, sign in frozen["control_expected_row"]["terms"]:
        x, y = frozen["public_base"][index]
        total = add(total, (x, y if sign == 1 else (-y) % P))
    assert total == positive
    audited = []
    for name in ARMS:
        path = EVIDENCE / f"{name}.json"
        raw = path.read_bytes()
        arm = json.loads(raw)
        result = arm["result"]
        trace = result["traversal"]
        assert arm["curve"] == curve
        assert arm["fixture_certificate"] == frozen["fixture_certificate"]
        assert tuple(arm["public_query"]) == (positive if arm["case"] == "positive" else Q)
        assert result["status"] == "ResourceLimit"
        assert result["relations"] == []
        assert result["diagnostics"]["input_variables"] == 61
        assert result["diagnostics"]["input_polynomials"] == 62
        assert result["diagnostics"]["forbidden_branch_attempts"] == 0
        assert result["diagnostics"]["forbidden_domain_evaluations"] == 0
        assert trace["solutions_visited"] == 0 and not trace["complete"]
        assert trace["resource_limit"] is not None
        audited.append({
            "file": path.name,
            "sha256": hashlib.sha256(raw).hexdigest(),
            "case": arm["case"],
            "matrix_cap": arm["matrix_cap"],
            "pair_batch": arm["pair_batch"],
            "monomial_cap": arm["monomial_cap"],
            "cold_wall_ms": arm["cold_wall_ms"],
            "resource_limit": trace["resource_limit"],
            "solutions_visited": 0,
            "accepted_relations": 0,
        })
    output = {
        "curve": curve,
        "positive_query": positive,
        "positive_witness_point_replay": True,
        "source_closure": "split22-sources.sha256",
        "audited_arms": audited,
        "claim_boundary": "all arms incomplete; no unknown-query extraction or target solve",
    }
    OUT.write_text(json.dumps(output, indent=2) + "\n")
    print(json.dumps(output, indent=2))


if __name__ == "__main__":
    main()
