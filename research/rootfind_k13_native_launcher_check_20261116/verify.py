#!/usr/bin/env python3
"""Verify the source-bound local launcher test output without submitting a Job."""

import gzip
import hashlib
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
STUDY = Path(__file__).resolve().parent


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    source = ROOT / "taskq/deploy/eks-karpenter/mcp/src/main.rs"
    lock = ROOT / "taskq/deploy/eks-karpenter/mcp/Cargo.lock"
    spec = ROOT / "research/rootfind_k13_l2_admission_probe_20261116/SPEC-K13-ADMISSION-v3.json"
    stdout = STUDY / "test.stdout.gz"
    stderr = STUDY / "test.stderr"
    budget_daily = STUDY / "budget-daily.stderr"
    budget_monthly = STUDY / "budget-monthly.stderr"
    raw_stdout = gzip.decompress(stdout.read_bytes())
    out = raw_stdout.decode()
    err = stderr.read_text()
    assert "test tests::frozen_k13_admission_spec_passes_argument_gate_without_submission ... ok" in out
    assert "test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out" in out
    assert "fn frozen_k13_admission_spec_passes_argument_gate_without_submission()" in source.read_text()
    assert sha(spec) == "ef890a6193c6b7c45d09775a27dede3c3eb16d7ab2ffaac8b359d18e976ef308"
    binary_line = re.search(r"Running unittests src/main.rs \(([^)]+)\)", err)
    assert binary_line, "test binary path missing from cargo stderr"
    binary = Path(binary_line.group(1))
    assert "UnrecognizedClientException" in budget_daily.read_text()
    assert "UnrecognizedClientException" in budget_monthly.read_text()
    print(json.dumps({
        "schema": "rootfind-k13-native-launcher-check/v1",
        "status": "pass_local_argument_gate",
        "source_sha256": sha(source),
        "cargo_lock_sha256": sha(lock),
        "spec_sha256": sha(spec),
        "stdout_sha256": hashlib.sha256(raw_stdout).hexdigest(),
        "stdout_gzip_sha256": sha(stdout),
        "stderr_sha256": sha(stderr),
        "test_binary_sha256": sha(binary),
        "rustc_version": "rustc 1.99.1 (58cf8f9c3 2026-10-13)",
        "command": "CARGO_TARGET_DIR=/Volumes/SSD990-2/crypto/target/isolab-mcp RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc rustup run 1.99.1 cargo test --manifest-path taskq/deploy/eks-karpenter/mcp/Cargo.toml --locked --release",
        "tests_passed": 8,
        "tests_failed": 0,
        "spec_embedded_exactly": True,
        "cluster_request_sent": False,
        "budget_readback": "unavailable_invalid_aws_token",
        "budget_daily_stderr_sha256": sha(budget_daily),
        "budget_monthly_stderr_sha256": sha(budget_monthly),
    }, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
