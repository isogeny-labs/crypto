#!/usr/bin/env python3
"""Bind an import receipt to the committed adapter source and local binary.

Requires blake3==1.0.8. Run before committing the final evidence so HEAD is
the importer implementation commit.
"""

from __future__ import annotations

import hashlib
import json
import subprocess
import sys
from pathlib import Path

import blake3


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        while block := stream.read(1 << 20):
            digest.update(block)
    return digest.hexdigest()


def main() -> None:
    if len(sys.argv) != 4:
        raise SystemExit("usage: attest.py BINARY IMPORT_JSON OUTPUT_JSON")
    binary, receipt, output = map(Path, sys.argv[1:])
    path = Path("research/koblitz_n83_factor_base_sweep_20261008/primary_adapter.rs")
    source = path.read_bytes()
    committed = subprocess.check_output(["git", "show", f"HEAD:{path}"])
    commit = subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip()
    imported = json.loads(receipt.read_text())
    source_b3 = blake3.blake3(source).hexdigest()
    valid = (
        source == committed
        and imported["status"] == "PASS"
        and source_b3 == imported["adapter_source_blake3"]
    )
    result = {
        "schema": "n83.d18-import-source-attestation/v1",
        "status": "PASS" if valid else "FAIL",
        "adapter_commit": commit,
        "adapter_source_path": str(path),
        "adapter_source_blake3": source_b3,
        "example_binary_sha256": sha256(binary),
        "import_receipt_sha256": sha256(receipt),
    }
    output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"status": result["status"], "adapter_commit": commit}))
    if not valid:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
