#!/usr/bin/env python3
"""Prospective file-custody refresh; never recompute benchmark measurements."""
from decimal import Decimal
import argparse
import copy
import hashlib
import json
from pathlib import Path
import subprocess

HERE = Path(__file__).resolve().parent
BASE = HERE.parent
REPO = BASE.parent.parent
REGISTRY = REPO / "docs/curves/registry.json"
REGISTRY_PATH = "../../docs/curves/registry.json"
ORIGINAL_COMMIT = "ddaba2819f4ee96139c4145fbed905db1c1db46e"
ORIGINAL_REGISTRY_SHA = "53fe929b9b971fa2c74ee29fcd5cea7fca969d411ae28e8cbf413c9550f3032e"
ORIGINAL_REPORT_SHA = "505327cf69bf6b6554244ee1214c71c87ee1331ba73dc74e1c36ebaa6388ee57"
TARGET = "icv1-f2m53-tm56619371-dac20a85"


def digest(data):
    return hashlib.sha256(data).hexdigest()


def exact(data):
    return json.loads(data, parse_int=Decimal, parse_float=Decimal)


def equal(a, b):
    if type(a) is not type(b):
        return False
    if isinstance(a, dict):
        return a.keys() == b.keys() and all(equal(a[k], b[k]) for k in a)
    if isinstance(a, list):
        return len(a) == len(b) and all(equal(x, y) for x, y in zip(a, b))
    return a == b


def target_comparison(old, current):
    """Permit only the two documented catalog source-list changes."""
    matches = [[r for r in x["curves"] if r["slug"] == TARGET] for x in (old, current)]
    if any(len(m) != 1 for m in matches):
        raise ValueError("target is not unique")
    before, after = (copy.deepcopy(m[0]) for m in matches)
    complete_equal = equal(before, after)
    provenance = []
    for row in (before, after):
        source_lists = {"sources": row.pop("sources")}
        if len(row["representations"]) != 1:
            raise ValueError("unexpected representation count")
        source_lists["representations[0].sources"] = row["representations"][0].pop("sources")
        if any(not isinstance(v, list) or any(type(s) is not str for s in v)
               for v in source_lists.values()):
            raise ValueError("source provenance must be string lists")
        provenance.append(source_lists)
    if not equal(before, after):
        raise ValueError("target changed beyond allowed source provenance")
    return complete_equal, provenance


def stable_write(path, data):
    if path.exists() and path.read_bytes() != data:
        raise ValueError(f"refusing to replace existing evidence: {path}")
    path.write_bytes(data)


def pretty(obj):
    return (json.dumps(obj, indent=2, sort_keys=True) + "\n").encode()


def refresh(publish_pointer=False):
    old_registry = subprocess.check_output(
        ["git", "show", f"{ORIGINAL_COMMIT}:docs/curves/registry.json"], cwd=REPO)
    original = (BASE / "validator-report.json").read_bytes()
    if digest(old_registry) != ORIGINAL_REGISTRY_SHA or digest(original) != ORIGINAL_REPORT_SHA:
        raise ValueError("historical input pin differs")
    current = REGISTRY.read_bytes()
    old_json, current_json = exact(old_registry), exact(current)
    complete_equal, provenance = target_comparison(old_json, current_json)
    parsed = exact(original)
    registry_bindings = [r for r in parsed["raw_data"] if r["path"] == REGISTRY_PATH]
    if len(registry_bindings) != 1 or registry_bindings[0]["sha256"] != ORIGINAL_REGISTRY_SHA:
        raise ValueError("original registry binding differs")
    inventory = []
    for raw in parsed["raw_data"]:
        if raw["path"] == REGISTRY_PATH:
            continue
        path = (BASE / raw["path"]).resolve()
        path.relative_to(REPO)
        data = path.read_bytes()
        if digest(data) != raw["sha256"]:
            raise ValueError(f"raw input pin differs: {raw['path']}")
        inventory.append({"path": raw["path"], "sha256": digest(data), "bytes": len(data)})
    if len(inventory) != 211:
        raise ValueError("unexpected non-registry raw count")
    receipt = {
        "schema": "crypto.aimed-gather.registry-custody/v2",
        "checked_on": "2026-11-26",
        "scope": "Metadata-only custody refresh. No benchmark or scalar replay executed.",
        "predecessor": "registry-binding-20261123",
        "target_slug": TARGET,
        "original_registry_commit": ORIGINAL_COMMIT,
        "original_registry_sha256": ORIGINAL_REGISTRY_SHA,
        "original_report_sha256": ORIGINAL_REPORT_SHA,
        "current_registry_sha256": digest(current),
        "original_registry_rows": len(old_json["curves"]),
        "current_registry_rows": len(current_json["curves"]),
        "complete_target_record_equal": complete_equal,
        "target_equal_except_two_source_lists": True,
        "source_lists_before": provenance[0],
        "source_lists_after": provenance[1],
        "inherited_scientific_fields_equal": True,
        "other_raw_input_hashes_verified": len(inventory),
        "other_raw_input_hashes_verified_via_store_manifest": 0,
        "changed_raw_bindings": 1,
    }
    # Preserve every inherited numeric lexeme and boolean byte-for-byte.
    text = original.decode()
    if text.count(ORIGINAL_REGISTRY_SHA) != 1 or not text.rstrip().endswith("}"):
        raise ValueError("unexpected original report layout")
    text = text.replace(ORIGINAL_REGISTRY_SHA, digest(current))
    candidate = (text.rstrip()[:-1] + ',\n "current_registry_source_equivalence": ' +
                 json.dumps(receipt, indent=1, sort_keys=True) + "\n}\n").encode()
    from check_numeric_fidelity import check
    stable_write(HERE / "validator-report.json", candidate)
    check(BASE / "validator-report.json", HERE / "validator-report.json")
    stable_write(HERE / "source-equivalence.json", pretty(receipt))
    stable_write(HERE / "raw-pin-inventory.json", pretty(inventory))
    manifest = BASE / "VALIDATION.json"
    previous = HERE / "previous-manifest.json"
    if not previous.exists():
        stable_write(previous, manifest.read_bytes())
    old_manifest = json.loads(previous.read_bytes())
    updated = copy.deepcopy(old_manifest)
    updated["validator_report"] = {"path": "registry-binding-20261126/validator-report.json",
                                   "sha256": digest(candidate)}
    if publish_pointer:
        if json.loads(manifest.read_bytes()) not in (old_manifest, updated):
            raise ValueError("live manifest changed unexpectedly")
        pointer_text = previous.read_text()
        for key in ("path", "sha256"):
            old_value = old_manifest["validator_report"][key]
            if pointer_text.count(old_value) != 1:
                raise ValueError("ambiguous live pointer replacement")
            pointer_text = pointer_text.replace(old_value, updated["validator_report"][key])
        if json.loads(pointer_text) != updated:
            raise ValueError("live pointer changed beyond report reference")
        manifest.write_text(pointer_text)
    print(json.dumps({"status": "pass", "complete_target_record_equal": complete_equal,
                      "raw_pins_verified": len(inventory), "pointer_updated": publish_pointer}))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--publish-pointer", action="store_true")
    refresh(parser.parse_args().publish_pointer)
