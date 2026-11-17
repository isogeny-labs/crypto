#!/usr/bin/env python3
"""Audit pinned pair-stream parity and derive requested backing counts."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
CURVE_BINARY = Path(
    "/Volumes/SSD990-2/crypto/wt/codex-rootfind-k13-local-f456-screen-20261116/"
    "target/release/deps/rootfind_inline_page_parity-e3406e48d6aafb91"
)

EXPECTED = {
    13: (17, 33, 272, 261, 11, 204, 161, 43, 145, 11_332, 39_172),
    19: (70, 139, 4_830, 4_800, 30, 4_473, 4_159, 314, 512, 163_844, 758_404),
    23: (138, 275, 18_906, 18_842, 64, 18_498, 18_158, 340, 512, 2_129_924, 4_534_916),
}
SHA = {
    "index_source": "5166fff336e1b8d6e09cfee3901b9df00879571da8045b1dfd52673f2e9c166a",
    "test_source": "87f8fd96db87671a964a121f38d052ac020ce161f732eaa0017970b3482367a7",
    "harness_manifest": "66b7ec4934a534ebed5dafeeb1ea57be449eadc2d7cf33eb3f184bb7f889ec6a",
    "harness_lock": "33907f3e9e26bbe3a0039d95941581e15c83147bd190a0f3ea05408d90c49e21",
    "unit_binary": "918fa932af98e35ff3866d51849ea249ddc37161537d923902657cf1be08976a",
    "curve_binary": "0a4023c8b6b7bf32b1a71a7284c1f0672c7f9f884cfdbb7423c904e80706812a",
}


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def parse_meta(name):
    return dict(line.split("=", 1) for line in (HERE / name).read_text().splitlines())


def parity_rows(name):
    stdout = (HERE / name).read_text()
    rows = [
        json.loads(line.removeprefix("INLINE_PARITY_JSON="))
        for line in stdout.splitlines()
        if line.startswith("INLINE_PARITY_JSON=")
    ]
    assert len(rows) == 3
    assert [row["field_degree"] for row in rows] == [13, 19, 23]
    return rows


def main():
    paths = {
        "index_source": HERE / "inline_page_index.rs",
        "test_source": ROOT / "tests/rootfind_inline_page_parity.rs",
        "harness_manifest": HERE / "harness/Cargo.toml",
        "harness_lock": HERE / "harness/Cargo.lock",
        "unit_binary": HERE / "inline_index_tests_final",
        "curve_binary": CURVE_BINARY,
    }
    for name, path in paths.items():
        assert sha256(path) == SHA[name], (name, path)

    assert parse_meta("curve.meta")["exit_code"] == "101"
    assert "cannot update the lock file" in (HERE / "curve.stderr").read_text()
    assert parse_meta("unit.meta")["test_exit_code"] == "0"
    assert parse_meta("unit-final.meta")["test_exit_code"] == "0"
    assert parse_meta("curve-locked.meta")["exit_code"] == "0"
    assert parse_meta("curve-final.meta")["exit_code"] == "0"
    assert "test result: ok. 3 passed; 0 failed" in (HERE / "unit-test.stdout").read_text()
    assert "test result: ok. 4 passed; 0 failed" in (HERE / "unit-final-test.stdout").read_text()
    assert "test result: ok. 8 passed; 0 failed" in (HERE / "curve-locked.stdout").read_text()
    assert "test result: ok. 9 passed; 0 failed" in (HERE / "curve-final.stdout").read_text()
    first_rows = parity_rows("curve-locked.stdout")
    rows = parity_rows("curve-final.stdout")
    assert rows == first_rows
    previous = json.loads(
        (ROOT / "research/rootfind_paged_curve_parity_20261116/RESULTS.json").read_text()
    )
    assert previous["status"] == "passed"
    assert len(previous["cells"]) == 3

    for row, prior in zip(rows, previous["cells"]):
        n = row["field_degree"]
        assert row["status"] == "passed"
        assert row["schema"] == "rootfind-inline-page-parity/v1"
        actual = tuple(row[k] for k in (
            "base_abscissae", "base_points", "insert_attempts",
            "first_witness_keys", "production_collisions", "used_pages",
            "inline_singleton_pages", "inline_spill_pages", "inline_spill_capacity",
            "inline_requested_backing_bytes", "paged_requested_backing_bytes",
        ))
        assert actual == EXPECTED[n], (n, actual)
        for k in ("curve_id", "base_sha256", "base_points", "base_abscissae", "insert_attempts",
                  "first_witness_keys", "production_collisions", "used_pages"):
            assert row[k] == prior[k], (n, k)
        assert row["paged_requested_backing_bytes"] == prior["requested_backing_bytes"]
        assert row["full_group_checks"] == row["production_collisions"]
        assert row["inline_singleton_pages"] + row["inline_spill_pages"] == row["used_pages"]
        assert row["inline_spill_pages"] <= row["first_witness_keys"] - row["used_pages"]
        assert row["inline_spill_pages"] <= row["inline_spill_capacity"]
        assert row["inline_directory_words"] == ((1 << n) // 16) + 1
        assert row["inline_requested_backing_bytes"] == (
            4 * row["inline_directory_words"] + 64 * row["inline_spill_capacity"]
        )
        row["inline_directory_bytes"] = 4 * row["inline_directory_words"]
        row["inline_spill_reserved_bytes"] = 64 * row["inline_spill_capacity"]
        row["inline_spill_used_bytes"] = 64 * row["inline_spill_pages"]

    result = {
        "schema": "rootfind-inline-page-results/v1",
        "status": "passed",
        "source_commit": "c3ad96f4315ed4452ab9e3fbc4862392eb1eef62",
        "test_source_commit": "9f5e09d0ea81d3c37ae0f061daff0a01171ba924",
        "lock_correction_commit": "4589d65ea605d1e6aa43c88c6c92b379a818c08f",
        "host": "macOS Darwin 27.2.0 arm64; shared L0 host",
        "toolchain": "rustc/cargo 1.99.1 aarch64-apple-darwin",
        "scope": "standalone representation correctness and requested backing on actual torsion pair streams",
        "unit_command": "rustup run 1.99.1 rustc --edition=2021 -O --test research/rootfind_inline_page_index_20261116/inline_page_index.rs -o research/rootfind_inline_page_index_20261116/inline_index_tests_final; inline_index_tests_final --nocapture",
        "curve_command": "RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc CARGO_TARGET_DIR=/Volumes/SSD990-2/crypto/wt/codex-rootfind-k13-local-f456-screen-20261116/target rustup run 1.99.1 cargo test --manifest-path research/rootfind_inline_page_index_20261116/harness/Cargo.toml --release --locked --offline --test rootfind_inline_page_parity -- --nocapture",
        "final_unit_meta": parse_meta("unit-final.meta"),
        "final_curve_meta": parse_meta("curve-final.meta"),
        "final_tests_passed": {"unit": 4, "curve_harness": 9},
        "previous_paged_result_sha256": sha256(ROOT / "research/rootfind_paged_curve_parity_20261116/RESULTS.json"),
        "source_hashes": SHA,
        "raw_hashes": {
            name: sha256(HERE / name)
            for name in (
                "unit.meta", "unit-build.stdout", "unit-build.stderr", "unit-test.stdout",
                "unit-test.stderr", "curve.meta", "curve.stdout", "curve.stderr",
                "curve-locked.meta", "curve-locked.stdout", "curve-locked.stderr",
                "unit-final.meta", "unit-final-build.stdout", "unit-final-build.stderr",
                "unit-final-test.stdout", "unit-final-test.stderr",
                "curve-final.meta", "curve-final.stdout", "curve-final.stderr",
            )
        },
        "cells": rows,
        "claim_boundary": "No production integration, collector RSS, complete-call timing, or F4/F5/F6 result in this screen.",
    }
    (HERE / "RESULTS.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
