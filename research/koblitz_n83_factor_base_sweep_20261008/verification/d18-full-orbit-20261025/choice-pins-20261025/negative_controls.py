"""Fail-closed receipt controls for audit.py; uses the same OBJECT and BINARY args."""

import copy
import json
import pathlib
import shutil
import sys
import tempfile

import audit


def run(object_path, binary_path):
    original_root = audit.ROOT
    results = []
    controls = [
        ("false_phase_status", "phase/result.json", ("status",), "PASS_NATURAL_RELATION"),
        ("altered_selector_row", "selector/result.json", ("relation", "row_decimal_blake3"), "0" * 64),
        ("false_host_isolation", "host.json", ("isolation",), "hard_cgroup"),
        ("changed_provider_checksum", "object-get-metadata.json", ("ChecksumSHA256",), "invalid"),
    ]
    with tempfile.TemporaryDirectory(prefix="n83-choice-pins-negative-") as temporary:
        root = pathlib.Path(temporary)
        for source in original_root.rglob("*"):
            if source.is_file():
                destination = root / source.relative_to(original_root)
                destination.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(source, destination)
        try:
            audit.ROOT = root
            for name, relative, keys, replacement in controls:
                path = root / relative
                original = json.loads(path.read_text())
                altered = copy.deepcopy(original)
                target = altered
                for key in keys[:-1]:
                    target = target[key]
                target[keys[-1]] = replacement
                path.write_text(json.dumps(altered))
                try:
                    audit.audit(object_path, binary_path)
                except AssertionError:
                    results.append({"control": name, "status": "REJECTED"})
                else:
                    raise RuntimeError(f"audit accepted {name}")
                path.write_text(json.dumps(original))
        finally:
            audit.ROOT = original_root
    return {
        "schema": "n83.full-d18-choice-pin-negative-controls/v1",
        "status": "PASS_FOUR_TAMPER_REJECTIONS",
        "controls": results,
    }


if __name__ == "__main__":
    if len(sys.argv) != 3:
        raise SystemExit("usage: negative_controls.py COMPRESSED_S3_OBJECT SOURCE_BINARY")
    print(json.dumps(run(pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])), indent=2, sort_keys=True))
