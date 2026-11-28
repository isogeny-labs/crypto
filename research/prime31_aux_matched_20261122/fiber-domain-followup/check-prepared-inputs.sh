#!/usr/bin/env bash
# Metadata-only checks for the prepared-oracle and complete-IC cell envelopes.
# Success never admits a launch. This script performs no arithmetic, checkpoint
# upload, native binary execution or cloud operation. See IMAGE_ROUTE.md.
set -euo pipefail
export LC_ALL=C GIT_OPTIONAL_LOCKS=0
if [[ $# != 2 ]]; then
  echo 'usage: bash check-prepared-inputs.sh ABSOLUTE_CELL.json ABSOLUTE_PROPOSED_NEW_OUTPUT_DIR' >&2
  exit 64
fi
if [[ "$1" != /* || ! -f "$1" || -L "$1" || "$2" != /* || -e "$2" || -L "$2" ]]; then
  echo 'check-prepared-inputs: input must be a regular absolute file; proposed output must be a new absolute path, never a symlink.' >&2
  exit 64
fi
for program in git python3; do
  command -v "$program" >/dev/null || { echo "check-prepared-inputs: missing $program" >&2; exit 69; }
done
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)

python3 - "$root" "$here/check-prepared-inputs.sh" "$1" "$2" <<'PY'
import datetime
import hashlib
import json
import math
import os
from pathlib import Path
import re
import subprocess
import sys


def refuse(kind, detail, code=65):
    print(f"check-prepared-inputs: {kind}: {detail}", file=sys.stderr)
    raise SystemExit(code)


def git(*args):
    result = subprocess.run(
        ["git", "-C", str(root), *args], capture_output=True, check=False
    )
    if result.returncode:
        refuse("source_identity", "required Git object or tracked path is unavailable")
    return result.stdout


def unique_object(items):
    value = {}
    for key, item in items:
        if key in value:
            raise ValueError("duplicate object key")
        value[key] = item
    return value


def reject_constant(_):
    raise ValueError("non-finite JSON number")


def read_object(path):
    try:
        raw = path.read_bytes()
        value = json.loads(raw, object_pairs_hook=unique_object, parse_constant=reject_constant)
        if not isinstance(value, dict):
            raise ValueError("object required")
        return value, raw
    except (OSError, ValueError, UnicodeError):
        refuse("invalid_metadata", "a required file is not unambiguous JSON object metadata")


def metadata_path(variable):
    value = os.environ.get(variable, "")
    candidate = Path(value)
    if not value or not candidate.is_absolute() or candidate.is_symlink():
        refuse("missing_metadata_path", f"{variable} must name an existing absolute regular file")
    try:
        resolved = candidate.resolve(strict=True)
        if not resolved.is_file() or resolved.stat().st_size == 0:
            raise ValueError("empty or nonregular file")
    except (OSError, ValueError):
        refuse("missing_metadata_path", f"{variable} must name an existing nonempty regular file")
    read_object(resolved)
    return resolved


def tracked_exact(path):
    try:
        relative = path.relative_to(root).as_posix()
    except ValueError:
        refuse("source_identity", "cell, selection, protocol and wrapper must be inside the checkout")
    git("ls-files", "--error-unmatch", "--", relative)
    if git("cat-file", "blob", f"HEAD:{relative}") != path.read_bytes():
        refuse("source_identity", "working bytes differ from the exact tracked source")


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def positive_finite_number(value):
    if type(value) is int:
        return value > 0
    return type(value) is float and math.isfinite(value) and value > 0


root = Path(sys.argv[1]).resolve(strict=True)
wrapper = Path(sys.argv[2]).resolve(strict=True)
cell_path = Path(sys.argv[3]).resolve(strict=True)
output = Path(sys.argv[4])
try:
    # Canonicalization catches paths such as new/../existing as well as symlinks.
    output = output.resolve(strict=False)
    if output.exists() or output.is_symlink() or not output.parent.is_dir():
        refuse("output_not_new", "output must have an existing parent and must not already exist", 64)
except (OSError, RuntimeError):
    refuse("output_not_new", "output path cannot be resolved safely", 64)

cell, raw_cell = read_object(cell_path)
binary_by_schema = {
    "prime31-isogeny-choice.cell/v3": "prime31-isogeny-choices",
    "prime31-complete-ic-incumbent.cell/v1": "prime31-ic-incumbent",
}
schema = cell.get("schema")
if not isinstance(schema, str) or schema not in binary_by_schema:
    refuse("unsupported_cell", "only ChoiceCell/v3 and complete-IC incumbent cell/v1 are supported")
stage = cell.get("stage")
cell_id = cell.get("cell_id")
if (stage not in ("screening", "complete_recovery")
        or not isinstance(cell_id, str) or not cell_id
        or type(cell.get("execution_seconds")) is not int or cell["execution_seconds"] != 120
        or type(cell.get("memory_mib")) is not int or cell["memory_mib"] != 4096):
    refuse("cell_envelope", "stage, identifier or inherited 120-second/4096-MiB limits do not match")

# These paths and the selection's `admitted` label are metadata, not authority.
# This checker does not authenticate any publication, budget, capacity, image,
# checkpoint or canonical-checker receipt, and does not report them as verified.
selection_path = metadata_path("PRIME31_SELECTION")
protocol_path = metadata_path("PRIME31_CANONICAL_PROTOCOL")

source_revision = git("rev-parse", "HEAD").decode("ascii").strip()
if not re.fullmatch(r"[0-9a-f]{40}", source_revision):
    refuse("source_identity", "full immutable source revision required")
for path in (wrapper, cell_path, selection_path, protocol_path):
    tracked_exact(path)
if git("status", "--porcelain", "--untracked-files=normal"):
    refuse("source_identity", "publish the exact inputs and use a clean checkout")

selection, _ = read_object(selection_path)
cell_hash = hashlib.sha256(raw_cell).hexdigest()
protocol, _ = read_object(protocol_path)
if protocol.get("schema") != "experiment.protocol/v1":
    refuse("canonical_protocol", "expected experiment.protocol/v1; this header check is not canonical validation")
cells = selection.get("cells")
if not isinstance(cells, list) or not all(isinstance(item, dict) for item in cells):
    refuse("selection_binding", "selection must contain an explicit cell list")
selected = [item for item in cells
            if item.get("cell_id") == cell_id and item.get("input_sha256") == cell_hash]
if (selection.get("schema") != "prime31-isogeny-choice.selection/v1"
        or selection.get("status") != "admitted" or selection.get("stage") != stage
        or type(selection.get("retries")) is not int or selection["retries"] != 0
        or len(selected) != 1
        or sum(item.get("cell_id") == cell_id for item in cells) != 1
        or selection.get("canonical_protocol_sha256") != sha256(protocol_path)):
    refuse("selection_binding", "stage, exact cell/protocol hashes, zero retries and unique selection are required")
ceiling = selection.get("aggregate_lane_ceiling_usd")
remaining = selection.get("remaining_lane_budget_usd")
if (not positive_finite_number(ceiling) or ceiling > 5
        or not positive_finite_number(remaining) or remaining > ceiling):
    refuse("budget_unknown", "a positive reconciled balance within the inherited aggregate USD5 ceiling is required")

deadline_text = selection.get("original_absolute_deadline_utc")
try:
    if not isinstance(deadline_text, str) or not re.fullmatch(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z", deadline_text):
        raise ValueError("UTC deadline required")
    deadline = datetime.datetime.strptime(deadline_text, "%Y-%m-%dT%H:%M:%SZ").replace(tzinfo=datetime.timezone.utc)
except ValueError:
    refuse("original_deadline", "an explicit original UTC deadline is required")
if datetime.datetime.now(datetime.timezone.utc) + datetime.timedelta(seconds=120) > deadline:
    refuse("original_deadline", "the original deadline cannot fit one full bounded cell")
if stage == "complete_recovery":
    pilot = metadata_path("PRIME31_PILOT_RECEIPT")
    if selection.get("pilot_receipt_sha256") != sha256(pilot):
        refuse("pilot_binding", "complete recovery requires the actual selected pilot receipt hash")

# A successful necessary metadata check is deliberately distinct from launch
# admission. The real receipts can be individually verified through legitimate
# existing interfaces; no new joint-verifier policy is imposed here.
print(json.dumps({
    "scope": "necessary_input_metadata_only",
    "supported_cell_envelope": True,
    "source_revision": source_revision,
    "cell_schema": schema,
    "cell_id": cell_id,
    "native_binary": binary_by_schema[schema],
    "input_sha256": cell_hash,
    "selection_sha256": sha256(selection_path),
    "protocol_sha256": sha256(protocol_path),
    "stage": stage,
    "original_absolute_deadline_utc": deadline_text,
    "declared_aggregate_lane_ceiling_usd": ceiling,
    "declared_remaining_lane_budget_usd": remaining,
    "limits": {"threads": 1, "affinity_cpus": 1, "memory_mib": 4096,
               "execution_seconds": 120, "term_seconds": 115, "kill_grace_seconds": 5,
               "retries": 0},
    "launch_admitted": False,
    "experiment_executed": False,
    "output_created": False,
    "arithmetic_input_validation_performed": False,
    "execution_receipt_validation_performed": False,
    "execution_prerequisites_not_checked": [
        "actual source/input publication readback",
        "current canonical protocol validation and freeze",
        "successful checkpoint gate and upload/readback",
        "reconciled live lane/account spend and attempt reservation",
        "live requested-resource capacity",
        "source/binary/OCI and actual execution identity",
        "pilot receipt validity beyond its selected hash when applicable",
    ],
}, sort_keys=True))
PY
