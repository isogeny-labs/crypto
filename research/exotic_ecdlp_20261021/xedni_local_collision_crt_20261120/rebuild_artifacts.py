"""Reconstruct the frozen candidate-row gzip from Git-sized byte shards."""

import gzip
import hashlib
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
NAME = "candidate-rows.jsonl.gz"
CHUNK = 32768


def digest(data):
    return hashlib.sha256(data).hexdigest()


def main():
    result = json.loads((HERE / "result.json").read_text())
    parts = sorted(HERE.glob(NAME + ".part-*"))
    assert len(parts) == 17, len(parts)
    assert [p.name for p in parts] == [f"{NAME}.part-{i:04d}" for i in range(17)]
    pieces = [p.read_bytes() for p in parts]
    assert all(len(p) == CHUNK for p in pieces[:-1])
    assert 0 < len(pieces[-1]) <= CHUNK
    packed = b"".join(pieces)
    assert digest(packed) == result["candidate_rows_packed_sha256"]
    assert digest(gzip.decompress(packed)) == result["candidate_rows_raw_sha256"]
    (HERE / NAME).write_bytes(packed)
    print(f"reconstructed {NAME}: {len(packed)} bytes, sha256 {digest(packed)}")


if __name__ == "__main__":
    main()
