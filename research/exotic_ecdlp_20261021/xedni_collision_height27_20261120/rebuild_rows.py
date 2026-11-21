"""Rebuild and verify the exact frozen gzip row bytes from Git shards."""

import gzip
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    result = json.loads((HERE / "result.json").read_text())
    manifest = json.loads((HERE / "package-result.json").read_text())
    assert manifest["schema"] == "xedni-collision-height27-package-v1"
    for name, info in manifest["files"].items():
        parts = info["parts"]
        assert [p["name"] for p in parts] == [f"{name}.part-{i:04d}" for i in range(len(parts))]
        assert sorted(p.name for p in HERE.glob(name + ".part-*")) == [p["name"] for p in parts]
        chunks = []
        for part in parts:
            data = (HERE / part["name"]).read_bytes()
            assert len(data) == part["bytes"] <= manifest["chunk_bytes"]
            assert sha(data) == part["sha256"]
            chunks.append(data)
        packed = b"".join(chunks)
        key = name.split("-")[0] + "_rows"
        assert len(packed) == info["bytes"] and sha(packed) == info["sha256"]
        assert sha(packed) == result[f"{key}_packed_sha256"]
        assert sha(gzip.decompress(packed)) == result[f"{key}_raw_sha256"]
        (HERE / name).write_bytes(packed)
        print(f"rebuilt {name}: {len(packed)} bytes, sha256 {sha(packed)}")


if __name__ == "__main__":
    main()
