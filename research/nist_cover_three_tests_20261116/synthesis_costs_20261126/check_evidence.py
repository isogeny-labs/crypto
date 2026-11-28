#!/usr/bin/env python3
"""Check byte/JSON custody and stage costs; performs no curve arithmetic."""
import argparse
from decimal import Decimal
import hashlib
import json
from pathlib import Path
import re

HERE = Path(__file__).resolve().parent
ORIGINAL = HERE.parent
BASELINE = ORIGINAL / "local_collection_20261124/evidence/attempt-v1/pipeline/baseline-e2e/relations/input"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def exact(path):
    return json.loads(path.read_bytes(), parse_int=Decimal, parse_float=Decimal)


def equal(a, b):
    if type(a) is not type(b):
        return False
    if isinstance(a, dict):
        return a.keys() == b.keys() and all(equal(a[k], b[k]) for k in a)
    if isinstance(a, list):
        return len(a) == len(b) and all(equal(x, y) for x, y in zip(a, b))
    return a == b


def check(attempt, require_complete):
    attempt = attempt.resolve()
    exit_path = attempt / "exit-code.txt"
    if not exit_path.is_file():
        raise ValueError("attempt still running or has no terminal receipt")
    code = int(exit_path.read_text())
    input_count = 0
    for line in (attempt / "pre-call-manifest.sha256").read_text().splitlines():
        expected_hash, recorded = line.split("  ", 1)
        for directory in ("source", "instrumentation"):
            marker = f"/{directory}/"
            if marker in recorded:
                local = attempt / directory / recorded.split(marker, 1)[1]
                break
        else:
            raise ValueError("unexpected pre-call subject path")
        if sha(local) != expected_hash:
            raise ValueError("pre-call source/input bytes changed")
        input_count += 1
    pairs = [
        ("quartic", "construction/replay/nist_cover_three_tests_20261116/evidence/p192-d7-kernel3-plane-quartic.json", BASELINE / "evidence/p192-d7-kernel3-plane-quartic.json"),
        ("destination_pool", "construction/replay/nist_cover_three_tests_20261116/evidence/destination-relations-v2/result.json", BASELINE / "evidence/destination-relations-v2/result.json"),
        ("elliptic_backmap", "backmap/result.json", BASELINE / "backmap.json"),
        ("formal_series", "formal/result.json", ORIGINAL / "evidence/formal-series-v1/result.json"),
        ("global_correspondence", "global/result.json", ORIGINAL / "evidence/global-correspondence-v2/result.json"),
    ]
    comparisons = []
    for label, rel, retained in pairs:
        regenerated = attempt / rel
        row = {"artifact": label, "regenerated_path": str(regenerated.relative_to(HERE)),
               "retained_path": str(retained.relative_to(ORIGINAL)), "present": regenerated.exists()}
        if regenerated.exists():
            row.update(regenerated_sha256=sha(regenerated), retained_sha256=sha(retained),
                       bytes_equal=regenerated.read_bytes() == retained.read_bytes())
            try:
                row["complete_json_equal"] = equal(exact(regenerated), exact(retained))
            except json.JSONDecodeError:
                row.update(complete_json_equal=False, incomplete_json=True)
            if code == 0 and not row["complete_json_equal"]:
                raise ValueError(f"regenerated {label} differs from the retained artifact")
        comparisons.append(row)
    calls = []
    for call in sorted((attempt / "calls").iterdir()):
        argv = [x.decode() for x in (call / "argv.nul").read_bytes().split(b"\0")[:-1]]
        call_code = int((call / "exit-code.txt").read_text())
        role = call.name.split("-", 1)[1]
        limit = int((call / "timeout-seconds.txt").read_text())
        if limit != {"build": 300, "native": 600}[role]:
            raise ValueError("invocation cap changed")
        executable_hash = (call / "executable.sha256").read_text().split()[0]
        if sha(Path(argv[0])) != executable_hash:
            raise ValueError("executed binary bytes changed")
        cost = json.loads((call / "cost.json").read_bytes())
        expected = 1 if role == "native" and "--canary" in argv else 0
        calls.append({"call": call.name, "role": role, "argv": argv, "exit_code": call_code,
                      "expected_exit_code": expected, "exit_matches_expected": call_code == expected,
                      "executable_sha256": executable_hash, "timeout_seconds": limit, **cost})
    match = re.search(r'(\{"wall_seconds":[^\n]+\})\s*$',
                      (attempt / "wrapper-stderr-and-cost.txt").read_text())
    if not match:
        raise ValueError("outer wrapper cost missing")
    wrapper = json.loads(match[1])
    complete = (code == 0 and all(x.get("complete_json_equal") for x in comparisons)
                and all(x["exit_matches_expected"] for x in calls))
    if require_complete and not complete:
        raise ValueError("complete chain not established")
    return {"scope": "fixed-kernel-construction-from-given-inputs", "wrapper_exit_code": code,
            "pre_call_subjects_rehashed": input_count,
            "resource_envelope": (attempt / "resource-envelope.txt").read_text(),
            "complete_chain": complete,
            "artifact_comparisons": comparisons, "calls": calls, "wrapper_cost": wrapper,
            "nested_call_totals_not_additive_to_wrapper": {
                role: {key: round(sum(x[key] for x in calls if x["role"] == role), 6)
                       for key in ("wall_seconds", "user_seconds", "system_seconds")}
                for role in ("build", "native")},
            "historical_given_input_generation_cost": "unknown; excluded from replay scope",
            "standing": "producer-unverified; JSON custody checker is not an arithmetic reimplementation"}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("attempt", type=Path)
    parser.add_argument("--require-complete", action="store_true")
    args = parser.parse_args()
    print(json.dumps(check(args.attempt, args.require_complete), indent=2, sort_keys=True))
