#!/usr/bin/env bash
# Synthetic metadata and invalid-input checks only: no scientific arithmetic.
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
bash -n "$here/check-prepared-inputs.sh"
python3 - "$here/check-prepared-inputs.sh" <<'PY'
import copy
import datetime
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

checks = []
with tempfile.TemporaryDirectory(prefix="prime31-metadata-only-") as directory:
    temp = Path(directory)
    repo = temp / "fixture"
    repo.mkdir()
    checker = repo / "check-prepared-inputs.sh"
    shutil.copyfile(sys.argv[1], checker)

    def put(name, data):
        path = repo / name
        path.write_text(json.dumps(data, sort_keys=True) + "\n")
        return path

    def digest(path):
        return hashlib.sha256(path.read_bytes()).hexdigest()

    # Intentionally incomplete scientific inputs. Only the metadata envelopes
    # are coherent; no public target, construction or IC parameters are present.
    common = {"cell_id": "synthetic-invalid-scientific-input", "stage": "screening",
              "execution_seconds": 120, "memory_mib": 4096}
    choice = put("choice.json", {**common, "schema": "prime31-isogeny-choice.cell/v3"})
    incumbent = put("incumbent.json", {**common, "schema": "prime31-complete-ic-incumbent.cell/v1"})
    unsupported = put("unsupported.json", {**common, "schema": "prime31-isogeny-choice.cell/v2"})
    malformed = repo / "malformed.json"
    malformed.write_text('{"schema":')
    duplicate = repo / "duplicate-key.json"
    duplicate.write_text('{"schema":"x","schema":"y"}')
    protocol = put("protocol.json", {"schema": "experiment.protocol/v1", "test_only": True})
    deadline = (datetime.datetime.now(datetime.timezone.utc) + datetime.timedelta(hours=1)).strftime("%Y-%m-%dT%H:%M:%SZ")

    def selection_for(cell):
        return {"schema": "prime31-isogeny-choice.selection/v1", "status": "admitted",
                "stage": "screening", "canonical_protocol_sha256": digest(protocol),
                "aggregate_lane_ceiling_usd": 5, "remaining_lane_budget_usd": 5,
                "retries": 0, "original_absolute_deadline_utc": deadline,
                "cells": [{"cell_id": common["cell_id"], "input_sha256": digest(cell)}]}

    selection = put("selection.json", selection_for(choice))
    incumbent_selection = put("incumbent-selection.json", selection_for(incumbent))
    unknown = selection_for(choice)
    unknown["remaining_lane_budget_usd"] = None
    unknown = put("unknown-balance.json", unknown)
    retries = selection_for(choice)
    retries["retries"] = 1
    retries = put("retry-selection.json", retries)
    duplicates = selection_for(choice)
    duplicates["cells"].append(copy.deepcopy(duplicates["cells"][0]))
    duplicates = put("duplicate-selection.json", duplicates)
    expired = selection_for(choice)
    expired["original_absolute_deadline_utc"] = "2000-01-01T00:00:00Z"
    expired = put("expired-selection.json", expired)

    def git(*args):
        subprocess.run(["git", "-C", str(repo), "-c", "core.hooksPath=/dev/null",
                        "-c", "commit.gpgsign=false", "-c", "user.name=Metadata fixture",
                        "-c", "user.email=metadata-fixture@invalid", *args],
                       check=True, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE)

    git("init", "-q")
    git("add", ".")
    git("commit", "-qm", "Synthetic metadata only")
    existing = temp / "existing-output"
    existing.mkdir()
    sentinel = existing / "sentinel"
    sentinel.write_text("existing receipts stay untouched\n")
    sentinel_hash = digest(sentinel)
    dangling = temp / "dangling-output"
    dangling.symlink_to(temp / "absent-target")
    proposed = temp / "never-created-output"
    environment = {k: v for k, v in os.environ.items()
                   if not k.startswith(("PRIME31_", "ISOLAB_"))}
    environment.update(PRIME31_SELECTION=str(selection), PRIME31_CANONICAL_PROTOCOL=str(protocol))

    def check(name, arguments, expected, selected=None, metadata_only=False):
        env = dict(environment)
        if selected is not None:
            env["PRIME31_SELECTION"] = str(selected)
        result = subprocess.run(["bash", str(checker), *map(str, arguments)],
                                env=env, capture_output=True, text=True, timeout=15)
        if result.returncode != expected:
            raise AssertionError(f"{name}: expected {expected}, got {result.returncode}: {result.stderr}")
        if metadata_only:
            report = json.loads(result.stdout)
            assert report["scope"] == "necessary_input_metadata_only"
            for field in ("launch_admitted", "experiment_executed", "output_created",
                          "arithmetic_input_validation_performed", "execution_receipt_validation_performed"):
                assert report[field] is False, (name, field)
        assert not proposed.exists()
        assert dangling.is_symlink() and not dangling.exists()
        assert digest(sentinel) == sentinel_hash
        checks.append({"name": name, "exit_code": result.returncode, "passed": True})

    check("usage_rejected", [], 64)
    check("existing_output_preserved", [choice, existing], 64)
    check("dangling_output_preserved", [choice, dangling], 64)
    check("old_schema_rejected", [unsupported, proposed], 65)
    check("malformed_json_rejected", [malformed, proposed], 65)
    check("duplicate_json_key_rejected", [duplicate, proposed], 65)
    check("unknown_balance_rejected", [choice, proposed], 65, unknown)
    check("retry_rejected", [choice, proposed], 65, retries)
    check("duplicate_selection_rejected", [choice, proposed], 65, duplicates)
    check("expired_original_deadline_rejected", [choice, proposed], 65, expired)
    check("choice_metadata_never_admits_incomplete_input", [choice, proposed], 0, metadata_only=True)
    check("incumbent_metadata_never_admits_incomplete_input", [incumbent, proposed], 0,
          incumbent_selection, metadata_only=True)

print(json.dumps({"scope": "synthetic_metadata_and_invalid_input_shell_checks",
                  "checks_passed": len(checks), "checks": checks,
                  "scientific_arithmetic_executed": False, "campaigns_executed": 0,
                  "launch_admitted": False, "output_creation_observed": False}, indent=2))
PY
