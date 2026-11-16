#!/usr/bin/env python3
"""Validate the frozen parity stdout and derive bounded storage diagnostics."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
STDOUT = HERE / "test-harness.stdout"
STDERR = HERE / "test-harness.stderr"
REPLAY_STDOUT = HERE / "replay.stdout"
REPLAY_STDERR = HERE / "replay.stderr"
REPLAY_META = HERE / "replay.meta"
TEST = ROOT / "tests/rootfind_paged_curve_parity.rs"
INDEX = ROOT / "research/rootfind_paged_pair_index_20261116/paged_index.rs"
BIN = ROOT / "target/release/deps/rootfind_paged_curve_parity-1b36afb9aa4f068d"

EXPECTED = {
    13: (17, 33, 272, 261, 11, 204, 290, 39_172),
    19: (70, 139, 4_830, 4_800, 30, 4_473, 4_901, 758_404),
    23: (138, 275, 18_906, 18_842, 64, 18_498, 19_045, 4_534_916),
}


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    assert sha256(TEST) == "5ee691d49e56c6ef2e1064bbfcd6e5091cfa82e6b986a80c1d79d754202d90d1"
    assert sha256(INDEX) == "5417bcd53b572c4060c5c7350900ef729a4fb9e6983ae83f980becddd98c5941"
    assert sha256(HERE / "harness/Cargo.toml") == "44d239499c31cfeae5744715cd80e54ec698d23a6ce0a158df80cd5deb6610b6"
    assert sha256(HERE / "harness/Cargo.lock") == "9d7f7a4dad73b32044e4913aa5fc047d975a03b50a483d7c98db2605507d0e4a"
    stdout = STDOUT.read_text()
    replay_stdout = REPLAY_STDOUT.read_text()
    assert "test result: ok. 5 passed; 0 failed" in stdout
    assert "test result: ok. 5 passed; 0 failed" in replay_stdout
    assert stdout.count("PARITY_JSON=") == 3
    assert replay_stdout.count("PARITY_JSON=") == 3
    first_records = sorted(line for line in stdout.splitlines() if line.startswith("PARITY_JSON="))
    replay_records = sorted(line for line in replay_stdout.splitlines() if line.startswith("PARITY_JSON="))
    assert first_records == replay_records
    replay_meta = dict(line.split("=", 1) for line in REPLAY_META.read_text().splitlines())
    assert replay_meta["exit_code"] == "0"
    assert "maximum resident set size" in REPLAY_STDERR.read_text()
    rows = []
    for line in stdout.splitlines():
        if not line.startswith("PARITY_JSON="):
            continue
        row = json.loads(line[len("PARITY_JSON="):])
        n = row["field_degree"]
        assert n in EXPECTED
        assert row["schema"] == "rootfind-paged-curve-parity/v1"
        assert row["status"] == "passed"
        actual = tuple(row[k] for k in (
            "base_abscissae", "base_points", "insert_attempts",
            "first_witness_keys", "production_collisions", "used_pages",
            "page_capacity", "requested_backing_bytes",
        ))
        assert actual == EXPECTED[n], (n, actual)
        assert row["full_group_checks"] == row["production_collisions"]
        directory_pages = ((1 << n) // 16) + 1
        directory_bytes = 4 * directory_pages
        assert row["requested_backing_bytes"] == directory_bytes + 128 * row["page_capacity"]
        row["directory_bytes"] = directory_bytes
        row["used_page_payload_bytes"] = 128 * row["used_pages"]
        row["logical_used_bytes"] = directory_bytes + row["used_page_payload_bytes"]
        row["page_utilization"] = round(row["used_pages"] / row["page_capacity"], 6)
        row["flat_requested_bytes"] = 8 * ((1 << n) + 1)
        rows.append(row)
    assert [r["field_degree"] for r in rows] == [13, 19, 23]
    assert "Finished `release` profile [optimized] target(s)" in STDERR.read_text()
    result = {
        "schema": "rootfind-paged-curve-parity-results/v1",
        "status": "passed",
        "scope": "actual Koblitz E0 torsion pair streams; correctness and storage only",
        "source_commit": "aa40df7ad378de5ab81fdd058dc076b204661432",
        "test_source_commit": "1103716b6d92c210b39ec426c6cb094b90f65b22",
        "toolchain": {"cargo": "1.99.1", "rustc": "1.99.1", "target": "aarch64-apple-darwin"},
        "command": "RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc CARGO_TARGET_DIR=/Volumes/SSD990-2/crypto/wt/codex-rootfind-k13-local-f456-screen-20261116/target rustup run 1.99.1 cargo test --manifest-path research/rootfind_paged_curve_parity_20261116/harness/Cargo.toml --release --locked --offline --test rootfind_paged_curve_parity -- --nocapture",
        "exit_code": 0,
        "tests_passed": 5,
        "hashes": {
            "test_source_sha256": sha256(TEST),
            "index_source_sha256": sha256(INDEX),
            "harness_manifest_sha256": sha256(HERE / "harness/Cargo.toml"),
            "harness_lock_sha256": sha256(HERE / "harness/Cargo.lock"),
            "stdout_sha256": sha256(STDOUT),
            "stderr_sha256": sha256(STDERR),
            "test_binary_sha256": sha256(BIN),
            "replay_stdout_sha256": sha256(REPLAY_STDOUT),
            "replay_stderr_sha256": sha256(REPLAY_STDERR),
            "replay_metadata_sha256": sha256(REPLAY_META),
        },
        "cells": rows,
        "same_host_replay": {
            "exit_code": 0,
            "start_utc": replay_meta["start_utc"],
            "end_utc": replay_meta["end_utc"],
            "parity_json_equal": True,
            "tests_passed": 5,
            "resource_log": "replay.stderr (/usr/bin/time -l; process-scope instrumentation, not an algorithm cost)",
            "command": "/usr/bin/time -l env RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc CARGO_TARGET_DIR=/Volumes/SSD990-2/crypto/wt/codex-rootfind-k13-local-f456-screen-20261116/target rustup run 1.99.1 cargo test --manifest-path research/rootfind_paged_curve_parity_20261116/harness/Cargo.toml --release --locked --offline --test rootfind_paged_curve_parity -- --nocapture",
        },
        "claim_boundary": "No collector integration, collector process-RSS measurement, complete-call timing, or F4/F5/F6 comparison in this run.",
    }
    (HERE / "RESULTS.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
