#!/usr/bin/env python3
"""Solver-free audit of the retained pivot-renewal native receipts."""
import hashlib
import json
from pathlib import Path

W = Path("/home/ec2-user/pencil-native-solve4-pivot-swap-renewal-20261111")
O = Path(str(W) + ".outer")
SOURCE = Path("/home/ec2-user/crypto-pencil-evidence-recovery-20261105/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/solve4-pivot-swap-renewal-20261111")
BOUND_RELEASE = Path("/home/ec2-user/crypto-pencil-evidence-recovery-20261105/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/pivot-renewal-final-binding-20261111/release.json")
FREEZE = "dffa9f313ca8daf569d97f09a1103cf1c858b11e42a35d069a279146a8f3180b"
RELEASE = "b3acbb8650a1f370579e067fa89874d6348278a63517adb9cb1ee54aae8eecff"
OUTER_RELEASE = "336d1b10ee96615d5efd6ceb97a84296de9d488691eeec22325ca0a7587305e1"


def load(path: Path):
    return json.loads(path.read_text())


def sha(path: Path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


completion = load(O / "completion.json")
admission = load(O / "admission.json")
compiler = load(W / "compiler-receipt.json")
pre = load(W / "pre-receipt.json")
post = load(W / "post-receipt.json")
commands = load(W / "commands.json")

assert sha(O / "completion.json") == "05493f4fd38b2cc40696789db61bbf812c7ad9608021a3497a1da25be14e173f"
assert sha(SOURCE / "freeze.json") == FREEZE
assert sha(BOUND_RELEASE) == RELEASE
assert sha(O / "coordinator-release.json") == OUTER_RELEASE
assert load(O / "coordinator-release.json") == load(BOUND_RELEASE)
assert admission["freeze_sha256"] == FREEZE
assert admission["coordinator_release_sha256"] == RELEASE
assert admission["retries"] == 0 and admission["successor_attempt_limit"] == 1
assert completion["status"] == "failed" and completion["machine_review_complete"] is False
assert [row["phase"] for row in completion["phases"]] == ["pre", "compiler", "post"]
assert [row["passed"] for row in completion["phases"]] == [True, False, True]
assert all(row["terminal_owner_confirmed"] and not row["wait_timeout"] for row in completion["phases"])
assert [row["module"] for row in commands] == ["EliminationField", "PivotSwap"]
assert [row["exit"] for row in commands] == [0, 1]
assert all(row["reaped"] and row["remaining_group"] == [] for row in commands)
assert (W / "EliminationField.olean").is_file()
assert all(not (W / name).exists() for name in ["PivotSwap.olean", "Controls.olean", "Readback.olean", "ExpectedFailure.olean"])
assert pre["status"] == "passed" and post["status"] == "passed"
assert pre["checked_dependency_files"] == post["checked_dependency_files"] == 16_736
assert pre["checked_dependency_bytes"] == post["checked_dependency_bytes"] == 3_111_892_318
assert (W / "pre-hashes.json").read_bytes() == (W / "post-hashes.json").read_bytes()
assert compiler["status"] == "failed" and compiler["compiler_invocations"] == 2
assert compiler["checked_dependency_files"] == compiler["checked_dependency_bytes"] == 0
assert "/PivotSwap.lean:59:2: error: unsolved goals" in (W / "PivotSwap.stdout").read_text()
expected_sources = {
    "EliminationField.lean": "f3cbd5bf902740417c34ff20be9b89332103e510aaa8435d4453cc08b43134c4",
    "PivotSwap.lean": "9535830381d9f1a49e8769f9554d78a394415eb8fa658f35ca41edd89da91111",
    "Controls.lean": "f7e17f2ea9b1a331c805cb0faf2be610c4edc691e6d54e132814adaeb88bf9b9",
    "Readback.lean": "9580b51aa480241c05543c7b08ebe65b15ffa6f2a869669f43ffab04a34c8079",
    "ExpectedFailure.lean": "2be0ca38af19258af54466c6aa9b356f02071b7d2479395093885bab1dc08595",
}
assert {name: sha(W / name) for name in expected_sources} == expected_sources
assert {name.replace(".lean", ".lean.txt"): sha(SOURCE / name.replace(".lean", ".lean.txt")) for name in expected_sources} == {
    name.replace(".lean", ".lean.txt"): digest for name, digest in expected_sources.items()
}
print(json.dumps({
    "status": "pass_receipt_audit_failed_native_run",
    "completion_status": completion["status"],
    "elapsed_seconds": completion["elapsed_seconds"],
    "phase_passes": [row["passed"] for row in completion["phases"]],
    "module_exits": {row["module"]: row["exit"] for row in commands},
    "failure": "PivotSwap.lean:59:2 equal-row swap_involutive branch unsolved",
    "pre_post_hash_maps_equal": True,
    "source_hashes_match": True,
    "all_phase_owners_terminal": True,
    "bound_release_sha256": RELEASE,
    "outer_release_sha256": OUTER_RELEASE,
    "outer_release_semantically_equal": True,
    "compiled_outputs_present": ["EliminationField.olean"],
    "unattempted_modules": ["Controls", "Readback", "ExpectedFailure"],
}, sort_keys=True))

