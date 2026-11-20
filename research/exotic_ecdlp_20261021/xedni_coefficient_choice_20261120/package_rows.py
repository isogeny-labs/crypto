"""Pack or restore the two exact xedni row streams with SHA-256 checks."""

import gzip
import hashlib
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
TARGETS = (
    (HERE, "rows.jsonl", "rows_sha256"),
    (HERE.parent / "xedni_full_law_coefficients_20261120", "pair-rows.jsonl", "pair_rows_sha256"),
)


def main(mode):
    assert mode in ("pack", "restore", "verify")
    for folder, name, key in TARGETS:
        raw = folder / name
        packed = folder / (name + ".gz")
        expected = json.loads((folder / "result.json").read_text())[key]
        if mode == "pack":
            data = raw.read_bytes()
            assert hashlib.sha256(data).hexdigest() == expected
            packed.write_bytes(gzip.compress(data, mtime=0))
            assert gzip.decompress(packed.read_bytes()) == data
            raw.unlink()
        elif mode == "restore":
            data = gzip.decompress(packed.read_bytes())
            assert hashlib.sha256(data).hexdigest() == expected
            raw.write_bytes(data)
        else:
            data = gzip.decompress(packed.read_bytes())
            assert hashlib.sha256(data).hexdigest() == expected
            if raw.exists():
                assert raw.read_bytes() == data
        print(f"{name}: {mode} PASS {expected}")


if __name__ == "__main__":
    main(sys.argv[1])
