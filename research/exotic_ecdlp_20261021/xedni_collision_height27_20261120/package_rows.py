"""Keep frozen gzip row bytes as small, ordered Git shards."""

import gzip
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
CHUNK = 32768
FILES = ("candidate-rows.jsonl.gz", "positive-rows.jsonl.gz")


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    result = json.loads((HERE / "result.json").read_text())
    manifest = {"schema": "xedni-collision-height27-package-v1", "chunk_bytes": CHUNK, "files": {}}
    for name in FILES:
        data = (HERE / name).read_bytes()
        key = name.split("-")[0] + "_rows"
        assert sha(data) == result[f"{key}_packed_sha256"]
        assert sha(gzip.decompress(data)) == result[f"{key}_raw_sha256"]
        parts = []
        for index, start in enumerate(range(0, len(data), CHUNK)):
            part_name = f"{name}.part-{index:04d}"
            chunk = data[start:start+CHUNK]
            (HERE / part_name).write_bytes(chunk)
            parts.append({"name": part_name, "bytes": len(chunk), "sha256": sha(chunk)})
        manifest["files"][name] = {"bytes": len(data), "sha256": sha(data), "parts": parts}
    (HERE / "package-result.json").write_text(json.dumps(manifest, sort_keys=True, indent=2) + "\n")
    print(json.dumps({name: len(info["parts"]) for name, info in manifest["files"].items()}, sort_keys=True))


if __name__ == "__main__":
    main()
