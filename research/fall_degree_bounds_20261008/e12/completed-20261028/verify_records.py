#!/usr/bin/env python3
"""Read-only verification of retained E12b records; never invokes a solver.

Usage: python3 verify_records.py /path/to/output.tar.gz [--repo /path/to/crypto]
The optional repository checks frozen Git blobs as well as archive contents.
"""
import argparse
import hashlib
import io
import json
import math
from pathlib import Path
import subprocess
import tarfile


def require(condition, message):
    if not condition:
        raise ValueError(message)


def digest(data):
    return hashlib.sha256(data).hexdigest()


def without_seconds(value):
    if isinstance(value, dict):
        return {k: without_seconds(v) for k, v in value.items() if k != "secs"}
    if isinstance(value, list):
        return [without_seconds(v) for v in value]
    return value


def verify(archive, repo=None):
    here = Path(__file__).resolve().parent
    provenance = json.loads((here / "provenance.json").read_text())
    raw = Path(archive).read_bytes()
    require(len(raw) == provenance["archive_bytes"], "archive size mismatch")
    require(digest(raw) == provenance["archive_sha256"], "archive hash mismatch")
    with tarfile.open(fileobj=io.BytesIO(raw), mode="r:gz") as bundle:
        files = {}
        for member in bundle.getmembers():
            if member.isfile():
                require(member.name not in files, "duplicate archive member")
                files[member.name] = bundle.extractfile(member).read()
    dataset = files[provenance["raw_dataset_member"]]
    require(digest(dataset) == provenance["raw_dataset_sha256"], "dataset hash mismatch")
    require(dataset == (here / "E_3-24-7.jsonl").read_bytes(), "Git dataset differs")
    rows = [json.loads(line) for line in dataset.splitlines()]
    require([r["draw"] for r in rows] == [0, 1, 2, 3], "expected four ordered draws")
    expected = dict(schema="hky_par_newcell/v1", m=3, n=24, ell=7,
                    seed=20261012, solutions=0, threads=16, n_vars=45, n_eqs=48)
    for row in rows:
        require(all(row[k] == v for k, v in expected.items()), "cell identity differs")
        require(set(row["per_degree"]) == {"2", "3", "4"}, "degree coverage differs")
        for degree, entry in row["per_degree"].items():
            columns = sum(math.comb(45, d) for d in range(int(degree) + 1))
            require(entry["of"] == columns, "Boolean column count differs")
            require(0 <= entry["dim"] <= columns, "dimension out of range")
        require((row["d_f"], row["d_f_at_least"]) in [(4, None), (None, 5)],
                "unexpected exact/censored result")
    reference = files["out/gate4_reference.jsonl"]
    require(digest(reference) == provenance["frozen_draw0_sha256"], "reference hash differs")
    require(without_seconds(rows[0]) == without_seconds(json.loads(reference)),
            "draw 0 non-timing identity differs")
    for key in ["source_commit", "protocol_commit", "protocol_sha256"]:
        require(files["out/" + key].decode().strip() == provenance[key], key + " differs")
    require(files["out/sha256_hky_par"].decode().strip() == provenance["recorded_binary_sha256"],
            "recorded binary identity differs")
    require(set(files["out/status"].decode().splitlines()) ==
            {"gate_passed", "done", "completion_checks_passed"}, "status incomplete")
    require(files["out/cells_status"].decode().strip() == "3:24:7 exit=0", "cell failed")
    require(b"test result: ok. 3 passed; 0 failed" in files["out/gate0.log"], "gate 0 missing")
    require(files["out/gate1.log"].decode().splitlines()[-1] == "AGREE 24 DISAGREE 0",
            "gate 1 missing")
    for cell in ["3-18-6", "4-8-2", "5-10-2", "5-8-2"]:
        require(files[f"out/gate_{cell}.diff"] == b"", "control mismatch: " + cell)
    require(files["out/gate4.diff"] == b"", "gate 4 difference retained")
    completion = json.loads(files["out/completion_check.json"])
    require(completion == dict(schema="e12b_completion/v1", passed=True, draws=4,
                              gate4="equal_non_timing_fields", scientific_reading=None),
            "producer completion check differs")
    if repo:
        def blob(commit, path):
            return subprocess.check_output(["git", "-C", str(repo), "show", commit + ":" + path])
        prefix = "research/fall_degree_bounds_20261008/"
        protocol = provenance["protocol_commit"]
        require(digest(blob(protocol, prefix + "PREREGISTRATION.md")) == provenance["protocol_sha256"],
                "frozen protocol blob differs")
        require(blob(protocol, prefix + "e12/run-b2/out/E_3-24-7.jsonl") == reference,
                "frozen draw 0 blob differs")
        for cell in provenance["carried_forward"]:
            original = blob(protocol, cell["path"])
            require(digest(original) == cell["sha256"], "carried-forward blob differs")
            require([json.loads(line)["d_f"] for line in original.splitlines()] == [4] * 4,
                    "carried-forward result differs")
    # Amendment 12/15: substitute these four rows; do not pool old draw 0.
    scores = [r["d_f"] if r["d_f"] is not None else 5 for r in rows]
    lower_median = sorted(scores)[1]
    return dict(schema="e12b_record_verification/v1", passed=True,
                archive_sha256=digest(raw), raw_dataset_sha256=digest(dataset),
                ordered_scores=scores, sorted_scores=sorted(scores), lower_median=lower_median,
                count_at_least_5=sum(s >= 5 for s in scores),
                registered_reading="consistent with H_ell" if lower_median >= 5 else "mixed",
                frozen_git_blobs_checked=repo is not None,
                independent_closure_replay=False, independently_verified_unsatisfiability=False,
                new_lean_verification=False)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("archive", type=Path)
    parser.add_argument("--repo", type=Path)
    args = parser.parse_args()
    print(json.dumps(verify(args.archive, args.repo), indent=2))
