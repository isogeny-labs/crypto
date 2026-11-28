#!/usr/bin/env python3
"""Bind the fixed multihit screen to raw output and exact source bytes."""

import hashlib
import json
import re
import subprocess
from pathlib import Path


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SOURCE_COMMIT = "aa129e0ab0"
CELLS = {
    13: (2003, 33, 261, 11, 2002),
    19: (1024, 139, 4800, 30, 1024),
    23: (1024, 275, 18842, 64, 1024),
}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


stdout = (HERE / "run-locked.stdout").read_text()
stderr = (HERE / "run-locked.stderr").read_text()
meta = dict(
    line.split("=", 1)
    for line in (HERE / "run-locked.meta").read_text().splitlines()
)
assert meta["exit_code"] == "0"
assert "test result: ok. 1 passed; 0 failed" in stdout
assert "Finished `release` profile" in stderr
assert git("merge-base", "--is-ancestor", SOURCE_COMMIT, "HEAD") == ""
for path in [
    "tests/rootfind_multihit_relations.rs",
    "research/rootfind_multihit_relations_20261116/harness/Cargo.toml",
    "research/rootfind_multihit_relations_20261116/harness/Cargo.lock",
]:
    assert not git("diff", SOURCE_COMMIT, "--", path), f"source drift: {path}"

lines = [line.removeprefix("MULTIHIT_JSON=") for line in stdout.splitlines()
         if line.startswith("MULTIHIT_JSON=")]
assert len(lines) == 3
cells = [json.loads(line) for line in lines]
assert [cell["field_degree"] for cell in cells] == [13, 19, 23]
derived = []
for cell in cells:
    n = cell["field_degree"]
    sample, points, keys, collisions, queries = CELLS[n]
    assert cell["schema"] == "rootfind-multihit-screen/v1"
    assert cell["status"] == "passed"
    assert (cell["sample_targets"], cell["base_points"], cell["pair_keys"],
            cell["collision_candidates"], cell["nonidentity_queries"]) == (
                sample, points, keys, collisions, queries)
    assert cell["extra_hits"] == cell["total_hits"] - cell["hit_targets"]
    assert cell["hit_targets"] >= cell["multihit_targets"]
    assert cell["extra_independent_rows"] <= cell["extra_hits"]
    assert cell["all_final_rank"] >= cell["first_final_rank"]
    for arm in ("first", "all"):
        full = cell[f"{arm}_full_rank"]
        assert full is not None, f"full rank censored: n={n}, arm={arm}"
        assert cell[f"{arm}_final_rank"] == cell["columns_with_target"]
        assert 1 <= full["queries"] <= queries
        scan = full["scan"]
        assert scan["subtractions"] >= scan["lookups"] >= scan["pair_readds"]
    first = cell["first_full_rank"]
    all_ = cell["all_full_rank"]
    derived.append({
        "field_degree": n,
        "saved_target_queries": first["queries"] - all_["queries"],
        "extra_pair_lookups": all_["scan"]["lookups"] - first["scan"]["lookups"],
        "extra_scan_subtractions": all_["scan"]["subtractions"] - first["scan"]["subtractions"],
        "extra_pair_readds": all_["scan"]["pair_readds"] - first["scan"]["pair_readds"],
    })

match = re.search(r"^\s+Running .* \((/[^)]+rootfind_multihit_relations-[0-9a-f]+)\)$",
                  stderr, re.MULTILINE)
assert match, "Cargo test binary path absent"
binary = Path(match.group(1))
assert binary.is_file()
frozen_protocol = subprocess.check_output([
    "git", "show", "15a7782ef3:research/rootfind_multihit_relations_20261116/PROTOCOL.md"
], cwd=ROOT)
frozen_protocol_sha = hashlib.sha256(frozen_protocol).hexdigest()
assert frozen_protocol_sha == "ac04d731f2f5af3beb93fbf7afa38659bdb3cd027da212e9bd6cc4a6a01bc18c"

result = {
    "schema": "rootfind-multihit-result/v1",
    "status": "bounded_screen_passed",
    "source_commit": git("rev-parse", SOURCE_COMMIT),
    "protocol_commit": git("rev-parse", "15a7782ef3"),
    "amendment_commit": git("rev-parse", "ccb82776e7"),
    "execution": {
        "command": "cargo test --manifest-path research/rootfind_multihit_relations_20261116/harness/Cargo.toml --release --locked --test rootfind_multihit_relations -- --nocapture",
        "toolchain": "rustc 1.99.1 aarch64-apple-darwin",
        "start_utc": meta["start_utc"],
        "end_utc": meta["end_utc"],
        "exit_code": 0,
        "binary_path": str(binary),
        "binary_sha256": sha(binary),
        "stdout_sha256": sha(HERE / "run-locked.stdout"),
        "stderr_sha256": sha(HERE / "run-locked.stderr"),
        "meta_sha256": sha(HERE / "run-locked.meta"),
    },
    "source_sha256": {
        "test": sha(ROOT / "tests/rootfind_multihit_relations.rs"),
        "manifest": sha(HERE / "harness/Cargo.toml"),
        "lock": sha(HERE / "harness/Cargo.lock"),
        "protocol": sha(HERE / "PROTOCOL.md"),
        "frozen_protocol": frozen_protocol_sha,
        "amendment": sha(HERE / "AMENDMENT-1.md"),
        "post_run_annotation": sha(HERE / "AMENDMENT-2.md"),
    },
    "cells": cells,
    "derived": derived,
    "claim_boundary": "Deterministic target stream and scan-operation counts only; no complete-call timing or attack-cost ordering.",
}
(HERE / "RESULTS.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
