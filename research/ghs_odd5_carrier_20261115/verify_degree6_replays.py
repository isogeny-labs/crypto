#!/usr/bin/env python3
"""Compare all source-bound direct replays to producer receipts by shape."""
import hashlib
import json
from pathlib import Path

from verify_degree6_receipts import HERE, MASKS, SOURCE_SHA, verify as verify_producers

REPLAY_SHA = "e1fd209fe5c426f6b638f545d04bddbd3775a26b030e43b1f241642e8ab27569"
REPLAY_GENERATED_SHA = "b27c7350cf8f30e82e0f65bad3d078a98ebb23f3fe2577a5ba56dd34edfc02e0"
COUNTERS = (
    ("branch_assignments", "branch_assignments"),
    ("numerator_degree_six", "degree_six"),
    ("coprime", "coprime"),
    ("f_divides", "divisible"),
    ("full_equation_hits", "squares"),
)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def verify():
    producer_manifest = verify_producers()
    assert producer_manifest["source_sha256"] == SOURCE_SHA
    assert sha((HERE / "degree6_direct_replay.sage").read_bytes()) == REPLAY_SHA
    generated = HERE / "degree6_direct_replay.sage.py"
    if generated.exists():
        assert sha(generated.read_bytes()) == REPLAY_GENERATED_SHA
    rows = []
    for mask in MASKS:
        source = json.loads((HERE / f"degree6_mask{mask}.stdout").read_text())
        output = (HERE / f"degree6_replay_mask{mask}.stdout").read_bytes()
        error = (HERE / f"degree6_replay_mask{mask}.stderr").read_bytes()
        replay = json.loads(output)
        assert error == b"", mask
        assert replay["schema"] == "ghs.odd5.degree6-direct-replay/v1", mask
        assert replay["source_sha256"] == REPLAY_SHA, mask
        assert replay["mask"] == mask, mask
        assert set(source["counts"]) == set(replay["counts"]), mask
        for shape in source["counts"]:
            for left, right in COUNTERS:
                assert source["counts"][shape][left] == replay["counts"][shape][right], (mask, shape, left, right)
        assert len(source["hits"]) == len(replay["hits"]) == 0, mask
        rows.append({"mask": mask, "replay_stdout_sha256": sha(output),
                     "replay_stderr_sha256": sha(error), "shape_counters_agree": True,
                     "producer_hits": 0, "replay_hits": 0})
    return {"schema": "ghs.odd5.degree6-replay-manifest/v1",
            "producer_source_sha256": SOURCE_SHA,
            "replay_source_sha256": REPLAY_SHA,
            "replay_generated_sha256": REPLAY_GENERATED_SHA,
            "comparison": "all-ten-shapes-and-five-counters-agree-per-target",
            "rows": rows}


if __name__ == "__main__":
    print(json.dumps(verify(), indent=2, sort_keys=True))
