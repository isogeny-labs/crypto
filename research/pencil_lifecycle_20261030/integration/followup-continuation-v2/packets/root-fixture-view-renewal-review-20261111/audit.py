#!/usr/bin/env python3
"""Source-only review of the fixture-view operational renewal."""
import hashlib
import json
from pathlib import Path

ROOT = Path("/home/ec2-user/crypto-pencil-evidence-recovery-20261105")
PACKET = ROOT / "research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/root-fixture-view-renewal-20261111"
PREDECESSOR = ROOT / "research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/root-fixture-view-20261109"
PRIOR_REVIEW = ROOT / "research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/root-fixture-view-review-20261109/resolution-20261110.json"


def load(path):
    return json.loads(Path(path).read_text())


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


manifest = load(PACKET / "manifest.json")
assert sha(PACKET / "manifest.json") == "48e6e50c0b923e783876ee9db6e935f0e90e8375cb00c2228984f4d8278d791b"
for name, row in manifest["files"].items():
    path = PACKET / name
    assert path.is_file() and not path.is_symlink()
    assert sha(path) == row["sha256"] and path.stat().st_size == row["bytes"]

bindings = load(PACKET / "renewal-bindings.json")
assert sha(PREDECESSOR / "manifest.json") == bindings["predecessor_manifest_sha256"] == "2226e43890d3a8710f54c9f2fc1cf704236d01cb0845da601d5d6f7c58cb9377"
assert sha(PRIOR_REVIEW) == bindings["independent_source_review_sha256"] == "18b1ba02930b314bfc368f521ab03b5a0ae2fa75af4a25be50ef131071249bc4"
for name, digest in bindings["unchanged_source_bindings"].items():
    assert sha(PACKET / name) == digest

for phase in ("materialize", "suite"):
    for name in ("durable-launcher.py", "durable-worker.py"):
        assert (PACKET / phase / name).read_bytes() == (PREDECESSOR / phase / name).read_bytes()
    admission = load(PACKET / phase / "admission-draft.json")
    assert admission["admitted"] is False
    assert admission["phase"] == phase
    assert admission["total_seconds"] == (120 if phase == "materialize" else 1800)
    assert admission["campaign_ceiling_seconds"] == 129600
    assert admission["campaign_remaining_seconds"] is None
    assert admission["campaign_charged_seconds"] is None
    assert admission["absolute_deadline"] == "2026-11-11T12:00:00Z"
    assert admission["attempts"] == 1 and admission["automatic_retry"] is False
    assert admission["lease"] is None
    assert admission["serial_predecessor_terminal_sha256"] is None
    assert admission["independent_review_sha256"] is None
    assert admission["publication"] == {"verified_github_commit": None, "readback_sha256": None}
    assert "renewal-20261111" in admission["workspace"] and "renewal-20261111" in admission["unit"]

    old = (PREDECESSOR / phase / "durable_io.py").read_text()
    new = (PACKET / phase / "durable_io.py").read_text()
    assert new.replace(
        "assert type(a['campaign_charged_seconds']) is int and type(a['campaign_remaining_seconds']) is int\n"
        "    assert a['campaign_charged_seconds']>=0 and a['campaign_charged_seconds']+a['campaign_remaining_seconds']==129600\n"
        "    assert a['campaign_ceiling_seconds']==129600 and a['campaign_remaining_seconds']>=a['total_seconds']\n"
        "    assert a['absolute_deadline']=='2026-11-11T12:00:00Z'",
        "assert a['campaign_ceiling_seconds']==43200 and a['campaign_remaining_seconds']>=a['total_seconds']\n"
        "    assert a['absolute_deadline']=='2026-11-10T05:11:39Z'",
    ) == old

inventory = load(PACKET / "expected-test-inventory.json")
assert len(inventory) == len(set(inventory)) == 6179
additions = load(PACKET / "additive-inputs-proposal.json")
assert len(additions["files"]) == 135 and additions["total_bytes"] == 41117354
rows = load(PACKET / "rows.json")
assert rows["schema"] == "root-library-failure-source-triage/v1"
assert len(rows["rows"]) == 18

commands = load(PACKET / "commands.json")
assert commands["status"] == "not_released" and commands["no_automatic_continuation"] is True
assert set(commands["separate_attempts"]) == {"materialize", "suite"}
assert all("root-fixture-view-renewal-20261111" in argv[-1] for argv in commands["separate_attempts"].values())

print(json.dumps({
    "status": "pass_source_only_operational_delta",
    "producer_manifest_sha256": sha(PACKET / "manifest.json"),
    "manifest_entries_checked": len(manifest["files"]),
    "unchanged_launchers_and_workers": True,
    "inventory_count": len(inventory),
    "additive_inputs_count": len(additions["files"]),
    "historical_failure_rows": len(rows["rows"]),
    "materialize_seconds": 120,
    "suite_seconds": 1800,
    "campaign_seconds": 129600,
    "released": False,
}, sort_keys=True))
