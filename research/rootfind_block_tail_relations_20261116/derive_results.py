#!/usr/bin/env python3
"""Bind the fixed block-tail relation screen to its raw output and source."""

import hashlib
import json
import re
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SOURCE_COMMIT = "d9ba9ca5d6"
PROTOCOL_COMMIT = "fe79c218f3"
EXPECTED = {
    13: (2003, 2002, 33, 261, 11),
    19: (1024, 1024, 139, 4800, 30),
    23: (1024, 1024, 275, 18842, 64),
}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


stdout = (HERE / "run.stdout").read_text()
stderr = (HERE / "run.stderr").read_text()
meta = dict(line.split("=", 1) for line in (HERE / "run.meta").read_text().splitlines())
assert meta["exit_code"] == "0"
assert "test result: ok. 1 passed; 0 failed" in stdout
assert "Finished `release` profile" in stderr
assert git("merge-base", "--is-ancestor", SOURCE_COMMIT, "HEAD") == ""
for relative in [
    "tests/rootfind_block_tail_relations.rs",
    "research/rootfind_block_tail_relations_20261116/harness/Cargo.toml",
    "research/rootfind_block_tail_relations_20261116/harness/Cargo.lock",
    "research/rootfind_block_tail_relations_20261116/PROTOCOL.md",
]:
    assert not git("diff", SOURCE_COMMIT, "--", relative), f"source drift: {relative}"

lines = [line.removeprefix("BLOCK_TAIL_JSON=") for line in stdout.splitlines()
         if line.startswith("BLOCK_TAIL_JSON=")]
assert len(lines) == 3
cells = [json.loads(line) for line in lines]
assert [x["field_degree"] for x in cells] == [13, 19, 23]
derived = []
for cell in cells:
    n = cell["field_degree"]
    sample, queries, points, keys, collisions = EXPECTED[n]
    assert cell["schema"] == "rootfind-block-tail-screen/v1"
    assert cell["status"] == "passed"
    assert (cell["sample_targets"], cell["nonidentity_queries"],
            cell["base_points"], cell["pair_keys"], cell["collision_candidates"]) == (
                sample, queries, points, keys, collisions)
    assert cell["extra_hits"] == cell["total_hits"] - cell["hit_targets"]
    assert cell["tail_extra_hits"] <= cell["extra_hits"]
    assert cell["tail_extra_hit_targets"] <= cell["hit_targets"]
    assert cell["tail_extra_independent_rows"] <= cell["tail_extra_hits"]
    assert cell["all_extra_independent_rows"] <= cell["extra_hits"]
    assert cell["all_final_rank"] >= cell["tail_final_rank"] >= cell["first_final_rank"]
    assert cell["first_independent_rows"] <= cell["tail_independent_rows"] <= cell["all_independent_rows"]
    for arm in ("first", "tail", "all"):
        full = cell[f"{arm}_full_rank"]
        assert full is not None, f"rank censored: {arm} K{n}"
        assert cell[f"{arm}_final_rank"] == cell["columns_with_target"]
        assert 1 <= full["queries"] <= queries
        scan = full["scan"]
        assert scan["subtractions"] >= scan["lookups"] >= scan["pair_readds"]
        assert scan["modeled_group_adds"] == scan["subtractions"] + scan["pair_readds"]
        assert scan["row_attempts"] == scan["pair_readds"]
    first, tail, all_ = (cell[f"{arm}_full_rank"] for arm in ("first", "tail", "all"))
    derived.append({
        "field_degree": n,
        "tail_saved_queries": first["queries"] - tail["queries"],
        "tail_extra_subtractions": tail["scan"]["subtractions"] - first["scan"]["subtractions"],
        "tail_extra_lookups": tail["scan"]["lookups"] - first["scan"]["lookups"],
        "tail_extra_readds": tail["scan"]["pair_readds"] - first["scan"]["pair_readds"],
        "tail_extra_modeled_group_adds": tail["scan"]["modeled_group_adds"] - first["scan"]["modeled_group_adds"],
        "all_saved_queries": first["queries"] - all_["queries"],
        "all_extra_modeled_group_adds": all_["scan"]["modeled_group_adds"] - first["scan"]["modeled_group_adds"],
    })

match = re.search(r"^\s+Running .* \((/[^)]+rootfind_block_tail_relations-[0-9a-f]+)\)$",
                  stderr, re.MULTILINE)
assert match, "test binary path absent"
binary = Path(match.group(1))
assert binary.is_file()
result = {
    "schema": "rootfind-block-tail-result/v1",
    "status": "bounded_screen_passed",
    "source_commit": git("rev-parse", SOURCE_COMMIT),
    "protocol_commit": git("rev-parse", PROTOCOL_COMMIT),
    "execution": {
        "command": "cargo test --manifest-path research/rootfind_block_tail_relations_20261116/harness/Cargo.toml --release --locked --test rootfind_block_tail_relations -- --nocapture",
        "toolchain": "rustc 1.99.1 aarch64-apple-darwin",
        "start_utc": meta["start_utc"],
        "end_utc": meta["end_utc"],
        "exit_code": 0,
        "binary_path": str(binary),
        "binary_sha256": sha(binary),
        "stdout_sha256": sha(HERE / "run.stdout"),
        "stderr_sha256": sha(HERE / "run.stderr"),
        "meta_sha256": sha(HERE / "run.meta"),
    },
    "source_sha256": {
        "test": sha(ROOT / "tests/rootfind_block_tail_relations.rs"),
        "manifest": sha(HERE / "harness/Cargo.toml"),
        "lock": sha(HERE / "harness/Cargo.lock"),
        "protocol": sha(HERE / "PROTOCOL.md"),
    },
    "cells": cells,
    "derived": derived,
    "claim_boundary": "Deterministic holdout streams and a batch-block group-operation proxy only; native work and complete-call timing unknown.",
}
(HERE / "RESULTS.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
