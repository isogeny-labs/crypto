#!/usr/bin/env python3
"""Derive fixed paged-index backing requests from pinned collision census JSON."""

import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CENSUS = ROOT / "research/rootfind_pair_collision_census_20261116/reports/run-20261116T0711Z"
OUTPUT = Path(__file__).resolve().parent / "storage.json"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def derive(n: int, path: Path) -> dict:
    census = json.loads(path.read_text())
    assert census["status"] == "complete", path
    a = census["base_abscissae"]
    attempts = census["insert_attempts"]
    entries = census["unique_keys"]
    assert isinstance(a, int) and a > 0
    assert isinstance(attempts, int) and isinstance(entries, int)
    assert 0 < entries <= attempts <= a * a + 1
    assert census["first_witness_count_parity"] is True

    max_key = 1 << n
    directory_words = (max_key >> 4) + 1
    page_capacity = min(directory_words, a * a + 1)
    assert entries <= page_capacity * 16
    directory_bytes = directory_words * 4
    page_bytes = page_capacity * 16 * 8
    paged_bytes = directory_bytes + page_bytes
    flat_words = max_key + 1
    flat_bytes = flat_words * 8
    assert paged_bytes < flat_bytes, path
    return {
        "cell": census["cell"],
        "curve_id": census["curve_id"],
        "base_sha256": census["base_sha256"],
        "census_path": str(path.relative_to(ROOT)),
        "census_sha256": sha256(path),
        "field_degree": n,
        "base_abscissae": a,
        "insert_attempts": attempts,
        "first_witness_keys": entries,
        "max_key": max_key,
        "directory_words": directory_words,
        "page_capacity": page_capacity,
        "page_size_words": 16,
        "directory_bytes": directory_bytes,
        "preallocated_page_bytes": page_bytes,
        "paged_requested_backing_bytes": paged_bytes,
        "flat_words_initialized": flat_words,
        "flat_requested_backing_bytes": flat_bytes,
        "flat_words_per_first_witness": {
            "numerator": flat_words,
            "denominator": entries,
        },
        "flat_to_paged_requested_backing": {
            "numerator": flat_bytes,
            "denominator": paged_bytes,
        },
    }


def main() -> None:
    rows = [derive(n, CENSUS / f"k{n}.json") for n in (13, 19, 23)]
    record = {
        "schema": "rootfind-paged-pair-storage/v1",
        "scope": "derived_requested_backing_only",
        "page_bits": 4,
        "page_words": 16,
        "directory_word_bytes": 4,
        "witness_word_bytes": 8,
        "actual_rss_or_wall": None,
        "rows": rows,
    }
    OUTPUT.write_text(json.dumps(record, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
