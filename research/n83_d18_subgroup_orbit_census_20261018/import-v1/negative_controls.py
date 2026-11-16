#!/usr/bin/env python3
"""Require the exact N83 d18 importer to reject altered bound inputs."""

from __future__ import annotations

import hashlib
import json
import shutil
import subprocess
import sys
from pathlib import Path


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        while block := stream.read(1 << 20):
            digest.update(block)
    return digest.hexdigest()


def main() -> None:
    if len(sys.argv) != 6:
        raise SystemExit("usage: negative_controls.py EXPORT EVIDENCE BINARY SCRATCH RECEIPT")
    source, evidence, binary, scratch, receipt = map(Path, sys.argv[1:])
    source, evidence, binary = source.resolve(), evidence.resolve(), binary.resolve()
    scratch.mkdir()
    manifest = json.loads((source / "manifest.json").read_text())
    cases = []

    changed_manifest = scratch / "changed-manifest"
    changed_manifest.mkdir()
    forged = dict(manifest)
    forged["columns"] = 16_626
    (changed_manifest / "manifest.json").write_text(json.dumps(forged) + "\n")
    shutil.copyfile(source / "replay.json", changed_manifest / "replay.json")
    (changed_manifest / "objects").symlink_to(source / "objects", target_is_directory=True)
    cases.append(("manifest", changed_manifest, evidence, changed_manifest / "manifest.json", "d18 export and committed evidence copies differ"))

    changed_evidence = scratch / "changed-custody"
    changed_evidence.mkdir()
    for name in ("manifest.json", "replay.json", "s3-custody.json"):
        shutil.copyfile(evidence / name, changed_evidence / name)
    custody = json.loads((changed_evidence / "s3-custody.json").read_text())
    custody["s3_uri"] += ".altered"
    (changed_evidence / "s3-custody.json").write_text(json.dumps(custody) + "\n")
    cases.append(("custody", source, changed_evidence, changed_evidence / "s3-custody.json", "d18 export, replay or S3 custody binding differs"))

    changed_archive = scratch / "changed-archive"
    changed_archive.mkdir()
    (changed_archive / "objects").mkdir()
    for name in ("manifest.json", "replay.json"):
        shutil.copyfile(source / name, changed_archive / name)
    forged_archive = changed_archive / manifest["object"]
    shutil.copyfile(source / manifest["object"], forged_archive)
    with forged_archive.open("r+b") as stream:
        stream.seek(-1, 2)
        value = stream.read(1)
        stream.seek(-1, 2)
        stream.write(bytes([value[0] ^ 1]))
    cases.append(("archive", changed_archive, evidence, forged_archive, "d18 archive length or SHA-256 differs"))

    results = []
    for name, export_root, evidence_root, changed, expected in cases:
        output = scratch / f"{name}-should-not-exist.json"
        completed = subprocess.run(
            [str(binary), "primary-d18-adapter-check", str(export_root), str(evidence_root), str(output)],
            capture_output=True,
            text=True,
            timeout=600,
        )
        observed = (completed.stdout + completed.stderr).strip()
        result = {
            "mode": name,
            "changed_sha256": sha256(changed),
            "exit_code": completed.returncode,
            "expected_rejection": expected,
            "observed": observed,
            "output_absent": not output.exists(),
            "pass": completed.returncode != 0 and expected in observed and not output.exists(),
        }
        results.append(result)
        receipt.write_text(json.dumps({"schema": "n83.d18-import-negative-controls/v1", "controls": results}, indent=2) + "\n")
        if not result["pass"]:
            raise SystemExit(f"{name} control failed: {observed}")
    print(json.dumps({"status": "PASS", "controls": len(results)}))


if __name__ == "__main__":
    main()
