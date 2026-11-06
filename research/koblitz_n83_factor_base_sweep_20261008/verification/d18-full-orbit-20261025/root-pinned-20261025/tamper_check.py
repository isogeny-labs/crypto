"""Check that the independent verifier rejects altered retained receipts."""

import json
import pathlib
import shutil
import sys
import tempfile

import verify


def check(object_path, binary_path):
    assert verify.replay(object_path, binary_path)["status"] == "PASS_INDEPENDENT_GROUP_AND_ROW_REPLAY"
    with tempfile.TemporaryDirectory(prefix="n83-root-pin-tamper-") as temporary:
        parent = pathlib.Path(temporary)
        (parent / "root-pinned-20261025").mkdir()
        (parent / "shared-root-20261025").mkdir()
        shutil.copy2(verify.PARENT / "manifest.json", parent / "manifest.json")
        for name in ("s3-manifest.json", "object-get-metadata.json"):
            shutil.copy2(verify.PARENT / "shared-root-20261025" / name, parent / "shared-root-20261025" / name)
        for name in ("result.json", "result.json.phases.jsonl", "binary-sha256.txt", "exit-code.txt", "stderr-time.txt", "host.json"):
            shutil.copy2(verify.ROOT / name, parent / "root-pinned-20261025" / name)
        original_root, original_parent = verify.ROOT, verify.PARENT
        verify.ROOT, verify.PARENT = parent / "root-pinned-20261025", parent
        result_path = verify.ROOT / "result.json"
        original = json.loads(result_path.read_text())
        checks = []
        try:
            for name in ("target", "row_digest", "source_commit", "get_length"):
                result = json.loads(json.dumps(original))
                metadata_path = parent / "shared-root-20261025/object-get-metadata.json"
                metadata = json.loads(metadata_path.read_text())
                if name == "target":
                    result["target"][1] = str(int(result["target"][1]) ^ int(result["target"][0]))
                elif name == "row_digest":
                    result["relation"]["row_decimal_blake3"] = "0" * 64
                elif name == "source_commit":
                    result["source_commit"] = "0" * 40
                else:
                    metadata["ContentLength"] += 1
                result_path.write_text(json.dumps(result))
                metadata_path.write_text(json.dumps(metadata))
                rejected = False
                try:
                    verify.replay(object_path, binary_path)
                except (AssertionError, ValueError):
                    rejected = True
                assert rejected, name
                checks.append({"tamper": name, "rejected": True})
                metadata_path.write_text(json.dumps(json.loads((original_parent / "shared-root-20261025/object-get-metadata.json").read_text())))
        finally:
            verify.ROOT, verify.PARENT = original_root, original_parent
    return {"schema":"n83.full-d18-root-pinned-tamper/v1", "status":"PASS_TAMPER_REJECTION", "checks":checks}


if __name__ == "__main__":
    if len(sys.argv) != 3:
        raise SystemExit("usage: tamper_check.py COMPRESSED_S3_OBJECT SOURCE_BINARY")
    print(json.dumps(check(pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])), indent=2, sort_keys=True))
