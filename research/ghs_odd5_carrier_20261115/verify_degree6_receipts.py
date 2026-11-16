#!/usr/bin/env python3
"""Verify transport integrity and accounting of the frozen degree-six runs.

This checks producer receipts, not the independent mathematical replay.
"""
import hashlib
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
SOURCE_SHA = "ae65a83015e82b79fb3890677992ae55a61b2af442855fac3db896910ed55b4d"
GENERATED_SHA = "35232733acf17d7222b48aad33d1799bb16a07c3231cc53529f6a0a60643b74a"
MASKS = (3, 27, 30, 103, 139, 167, 182, 194, 230, 233)
ASSIGNMENTS = {
    "L": 243, "L3": 243, "LQi": 405, "Ldouble": 58806,
    "L5": 243, "L3Qi": 405, "L3double": 58806,
    "LQiQj": 270, "LQidouble": 98010, "LHsquare": 14288643,
}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def verify():
    assert sha((HERE / "degree6_screen.sage").read_bytes()) == SOURCE_SHA
    generated = HERE / "degree6_screen.sage.py"
    if generated.exists():
        assert sha(generated.read_bytes()) == GENERATED_SHA
    rows = []
    for mask in MASKS:
        output = (HERE / f"degree6_mask{mask}.stdout").read_bytes()
        error = (HERE / f"degree6_mask{mask}.stderr").read_bytes()
        obj = json.loads(output)
        assert error == b"", mask
        assert obj["schema"] == "ghs.odd5.degree6-screen/v1", mask
        assert obj["source_sha256"] == SOURCE_SHA, mask
        assert obj["a6_mask"] == mask, mask
        assert obj["field"] == "F3[a]/(a^5+2a+1)", mask
        assert obj["target_order"] == 218 and obj["trace"] == 26, mask
        assert obj["denominator_count"] == 60518, mask
        assert obj["branch_assignment_count"] == sum(ASSIGNMENTS.values()), mask
        assert obj["LQiQj_lambda_values_per_assignment"] == 242, mask
        assert set(obj["counts"]) == set(ASSIGNMENTS), mask
        for shape, expected in ASSIGNMENTS.items():
            counts = obj["counts"][shape]
            assert counts["branch_assignments"] == expected, (mask, shape)
            assert counts["numerator_degree_six"] >= counts["coprime"] >= counts["f_divides"] >= counts["full_equation_hits"], (mask, shape)
        assert sum(v["full_equation_hits"] for v in obj["counts"].values()) == len(obj["hits"]), mask
        rows.append({
            "mask": mask,
            "stdout_sha256": sha(output),
            "stderr_sha256": sha(error),
            "hits": len(obj["hits"]),
            "numerator_degree_six": sum(v["numerator_degree_six"] for v in obj["counts"].values()),
            "coprime": sum(v["coprime"] for v in obj["counts"].values()),
            "f_divides": sum(v["f_divides"] for v in obj["counts"].values()),
            "resultant_choice_count": obj["resultant_choice_count"],
            "fallback_choice_count": obj["fallback_choice_count"],
            "fallback_u_count": obj["fallback_u_count"],
        })
    return {
        "schema": "ghs.odd5.degree6-receipt-manifest/v1",
        "source_sha256": SOURCE_SHA,
        "generated_sha256": GENERATED_SHA,
        "denominators_per_target": 60518,
        "branch_assignments_per_target": sum(ASSIGNMENTS.values()),
        "mathematical_replay": "pending",
        "rows": rows,
    }


if __name__ == "__main__":
    print(json.dumps(verify(), indent=2, sort_keys=True))
