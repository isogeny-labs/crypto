#!/usr/bin/env python3
"""Hash the immutable raw outputs of the source-bound lift screen."""

import hashlib
import json
import re
from pathlib import Path


HERE = Path(__file__).resolve().parent


def file_record(name: str) -> dict:
    path = HERE / name
    data = path.read_bytes()
    return {
        "path": name,
        "bytes": len(data),
        "sha256": hashlib.sha256(data).hexdigest(),
    }


def run(prefix: str, command: str, interpretation: str) -> dict:
    exit_text = (HERE / f"{prefix}.exit").read_text().strip()
    return {
        "id": prefix,
        "command": command,
        "exit_code": int(exit_text),
        "interpretation": interpretation,
        "stdout": file_record(f"{prefix}.stdout"),
        "stderr": file_record(f"{prefix}.stderr"),
        "exit": file_record(f"{prefix}.exit"),
    }


def main() -> None:
    rustc = "/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc"
    cargo = "/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/cargo"
    source = "src/cryptanalysis/koblitz_index_calculus.rs"
    baseline_binary = Path(
        "/Volumes/SSD990-2/crypto/wt/codex-rootfind-k13-admission-20261115/"
        "target/release/deps/crypto_lib-7afba7a5a589a2c4"
    )
    baseline_source = Path(
        "/Volumes/SSD990-2/crypto/wt/codex-rootfind-k13-admission-20261115/"
        "src/cryptanalysis/koblitz_index_calculus.rs"
    )
    suite = (HERE / "full2.stdout").read_text()
    module_results = re.findall(
        r"^test cryptanalysis::koblitz_index_calculus::tests::[^\n]+$",
        suite,
        re.MULTILINE,
    )
    assert len(module_results) == 185
    assert sum(line.endswith(" ... FAILED") for line in module_results) == 1
    assert "batched_packed_lifts_match_individual_lifts ... ok" in suite
    assert "batched_standard_subspace_base_matches_legacy_order_and_columns ... ok" in suite
    assert (HERE / "full.exit").read_text().strip() == "124"
    assert (HERE / "full2.exit").read_text().strip() == "124"
    assert (HERE / "isolate.exit").read_text().strip() == "101"
    assert (HERE / "baseline.exit").read_text().strip() == "101"
    doc = {
        "schema": "rootfind-batch-lift-source-screen/v1",
        "protocol_commit": "3febee7878",
        "amended_source_commit": "0c23166d62",
        "source": {
            "path": source,
            "sha256": hashlib.sha256((HERE.parents[1] / source).read_bytes()).hexdigest(),
        },
        "host_arch": "aarch64-apple-darwin",
        "host_os": "macOS 27.2 (26B123)",
        "rustc_version": "rustc 1.99.1 (58cf8f9c3 2026-10-13)",
        "cargo_version": "cargo 1.99.1 (5f94df478 2026-08-27)",
        "runs": [
            run(
                "test",
                f"RUSTC={rustc} {cargo} test --release --locked -j 2 --lib batched_ -- --nocapture",
                "28 passed, 0 failed; local producer correctness screen",
            ),
            run(
                "full",
                f"RUSTC={rustc} /opt/homebrew/bin/gtimeout 600 {cargo} test --release --locked -j 2 --lib -- --test-threads=4",
                "watchdog-censored at 600 seconds; chained S3 test failed before censor",
            ),
            run(
                "full2",
                f"RUSTC={rustc} /opt/homebrew/bin/gtimeout 1800 {cargo} test --release --locked -j 2 --lib -- --test-threads=4",
                "watchdog-censored at 1800 seconds; chained S3 test failed before censor",
            ),
            run(
                "isolate",
                "/opt/homebrew/bin/gtimeout 300 target/release/deps/crypto_lib-7afba7a5a589a2c4 cryptanalysis::koblitz_index_calculus::tests::chained_s3_strategy_feeds_verified_relation_rank_and_target_solve --exact --nocapture",
                "same 16-versus-0 assertion, candidate binary, isolated",
            ),
            run(
                "baseline",
                f"/opt/homebrew/bin/gtimeout 300 {baseline_binary} cryptanalysis::koblitz_index_calculus::tests::chained_s3_strategy_feeds_verified_relation_rank_and_target_solve --exact --nocapture",
                "same 16-versus-0 assertion in a pre-candidate executable; its build commit is not independently pinned",
            ),
            run(
                "fmt",
                "rustfmt --edition 2021 --check src/cryptanalysis/koblitz_index_calculus.rs",
                "targeted formatting check",
            ),
        ],
        "koblitz_module_screen": {
            "full2_tests_reported": len(module_results),
            "full2_failed": 1,
            "new_batch_tests_passed": 2,
            "pre_candidate_baseline_binary": {
                "path": str(baseline_binary),
                "sha256": hashlib.sha256(baseline_binary.read_bytes()).hexdigest(),
                "worktree_head_at_replay": "557f28efa880a07589dd806d42693697c0993411",
                "source_sha256": hashlib.sha256(baseline_source.read_bytes()).hexdigest(),
                "binary_build_commit_provenance": "not established",
            },
        },
        "prior_attempts": [
            {
                "id": "test-first",
                "source_commit": "5b74bae8ff",
                "interpretation": "K17 constructor fixture failure, amended before retest",
                "exit_code": int((HERE / "test-first.exit").read_text().strip()),
                "stdout": file_record("test-first.stdout"),
                "stderr": file_record("test-first.stderr"),
                "exit": file_record("test-first.exit"),
            },
            {
                "id": "test-interrupted",
                "source_commit": "5b74bae8ff",
                "interpretation": "external interruption before exit receipt; not a test result",
                "exit_code": None,
                "stdout": file_record("test-interrupted.stdout"),
                "stderr": file_record("test-interrupted.stderr"),
            },
        ],
        "claim": "PCC-20261116-7748c7",
        "claim_standing": "unverified",
        "timing_or_speed_claim": False,
        "independent_replay": False,
    }
    (HERE / "CHECKS.json").write_text(json.dumps(doc, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
