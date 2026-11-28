#!/usr/bin/env python3
"""Project the frozen ordered-pair CRT census onto fixed-generator workloads."""

import gzip
import hashlib
import json
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
PARENT = HERE.parent
SOURCE = PARENT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"
PAIRS = PARENT / "xedni_full_law_crt_20261120/pair-rows.jsonl.gz"
POSITIVE = PARENT / "xedni_full_law_crt_20261120/positive-rows.jsonl.gz"
SOURCE_SHA = "f8180c2d29368869f51528523e5f69da8552896826e476cdace47c2af23f017f"
PAIR_RAW_SHA = "9d269c3a789589684822edc561462b819ade32c27c13ff23b342490ca5de7954"
POSITIVE_RAW_SHA = "572a871db0b5f36c4e2c314435ae5ce6d9e4f738b88d0291657dc598f6b9090a"


def read_rows(path, expected_sha):
    raw = gzip.decompress(path.read_bytes())
    assert hashlib.sha256(raw).hexdigest() == expected_sha, path
    return [json.loads(line) for line in raw.splitlines()]


def main():
    source = read_rows(SOURCE, SOURCE_SHA)
    pairs = read_rows(PAIRS, PAIR_RAW_SHA)
    positive = read_rows(POSITIVE, POSITIVE_RAW_SHA)
    assert len(source) == len(pairs) == 1722
    assert all(row["pair_index"] == i for i, row in enumerate(pairs))
    assert len(positive) == 8

    generators = defaultdict(list)
    for i, (src, row) in enumerate(zip(source, pairs)):
        assert src["reduced_relation"] == row["reduced_relation"]
        target, generator = src["reduced_points"]
        assert target != generator
        generators[tuple(generator)].append((i, tuple(target), row))

    assert len(generators) == 42
    assert all(len(rows) == 41 for rows in generators.values())
    assert sum(row["reduced_relation"] for row in pairs) == 42
    assert all(sum(row["reduced_relation"] for _, _, row in rows) == 1
               for rows in generators.values())
    assert {x["pair_index"] for x in positive} == {425, 467}

    per_generator = []
    for generator, rows in sorted(generators.items()):
        event = [(i, target, row) for i, target, row in rows
                 if row["reduced_relation"]]
        assert len(event) == 1
        event_index, event_target, _ = event[0]
        hits = [(i, target, row) for i, target, row in rows
                if row["rational_relations"]]
        assert all(i == event_index for i, _, _ in hits)
        per_generator.append({
            "generator": list(generator),
            "eligible_targets": len(rows),
            "compatible_target": list(event_target),
            "compatible_pair_index": event_index,
            "rational_hit_target_count": len(hits),
            "rational_relation_records": sum(row["rational_relations"] for _, _, row in rows),
            "local_residues_7": sum(row["local_counts"].get("7", {}).get("residues", 0)
                                    for _, _, row in rows),
            "local_residues_11": sum(row["local_counts"].get("11", {}).get("residues", 0)
                                     for _, _, row in rows),
            "crt_candidates": sum(row["crt_candidates"] for _, _, row in rows),
        })

    hit_generators = [x for x in per_generator if x["rational_hit_target_count"]]
    assert len(hit_generators) == 2
    assert sum(x["rational_relation_records"] for x in hit_generators) == 8
    assert sum(x["crt_candidates"] for x in per_generator) == 32256
    assert sum(x["local_residues_7"] for x in per_generator) == 460992
    assert sum(x["local_residues_11"] for x in per_generator) == 1788864
    return {
        "schema": "xedni-fixed-target-preflight-v1",
        "source_raw_sha256": SOURCE_SHA,
        "pair_rows_raw_sha256": PAIR_RAW_SHA,
        "positive_rows_raw_sha256": POSITIVE_RAW_SHA,
        "status": "PASS",
        "generators": 42,
        "targets_per_generator": 41,
        "reduced_relation_compatible_targets_per_generator": 1,
        "generators_with_rational_hit": 2,
        "ordered_target_inputs_with_rational_hit": 2,
        "rational_relation_records": 8,
        "total_local_residues": 2249856,
        "total_crt_candidates": 32256,
        "hit_generators": hit_generators,
        "per_generator": per_generator,
    }


if __name__ == "__main__":
    print(json.dumps(main(), indent=2, sort_keys=True))
