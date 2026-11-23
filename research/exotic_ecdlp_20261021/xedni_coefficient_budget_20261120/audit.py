"""Audit nested-budget accounting and exact K=1 replay."""

import gzip
import hashlib
import json
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
PRIOR = HERE.parent / "xedni_coefficient_choice_20261120"
BASELINE = HERE.parent / "xedni_full_law_coefficients_20261120"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    result = json.loads((HERE / "result.json").read_text())
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    source_bytes = (PRIOR / "source-rows-v2.jsonl.gz").read_bytes()
    packed = (HERE / "pair-rows.jsonl.gz").read_bytes()
    raw = gzip.decompress(packed)
    assert result["protocol_sha256"] == sha(protocol_bytes)
    assert result["source_sha256"] == sha(source_bytes)
    assert result["script_sha256"] == sha((HERE / "run.py").read_bytes())
    assert result["arithmetic_sha256"] == sha((PRIOR / "audit.py").read_bytes())
    assert result["packed_rows_sha256"] == sha(packed)
    assert result["raw_rows_sha256"] == sha(raw)
    source = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    rows = [json.loads(x) for x in raw.splitlines()]
    prior_rows = [json.loads(x) for x in gzip.decompress((BASELINE / "pair-rows.jsonl.gz").read_bytes()).splitlines()]
    assert len(rows) == len(source) == len(prior_rows) == 1722
    totals = {"1": Counter(), "2": Counter()}
    new_strict = new_weak = 0
    for index, (row, input_row, old) in enumerate(zip(rows, source, prior_rows)):
        assert row["pair_index"] == old["pair_index"] == index
        assert row["source_status"] == old["source_status"] == input_row["status"]
        assert row["reduced_relation"] == old["reduced_relation"] == input_row["reduced_relation"]
        assert row["counts"]["1"] == old["counts"]
        assert row["selected"]["1"] == old["selected"]
        for K, limit in (("1", 27), ("2", 125)):
            counts = row["counts"][K]
            if input_row["status"] == "same_x":
                assert counts == {"same_x": 1}
            else:
                assert counts["candidates"] == limit
                for prime in ("p7", "p11"):
                    assert sum(counts.get(prime+"_"+s, 0) for s in ("good", "nonintegral", "bad_reduction")) == limit
            totals[K].update(counts)
            for arm, pick in row["selected"][K].items():
                if pick is not None:
                    assert counts[arm+"_passing_choices"] > 0
                    totals[K][arm+"_selected_pairs"] += 1
                    totals[K][arm+"_selected_reduced"] += int(input_row["reduced_relation"])
                    totals[K][arm+"_selected_rational"] += int(pick["rational_relation"])
        if row["selected"]["2"]["strict"] and not row["selected"]["1"]["strict"]:
            new_strict += 1
        if row["selected"]["2"]["weak"] and not row["selected"]["1"]["weak"]:
            new_weak += 1
        for arm in ("strict", "weak"):
            assert row["selected"]["1"][arm] is None or row["selected"]["2"][arm] is not None
    assert {k: dict(v) for k, v in totals.items()} == result["counts"]
    assert new_strict == 8 and new_weak == 8
    print(json.dumps({"status": "PASS", "pairs_replayed": len(rows),
                      "k1_exact_baseline": True, "new_strict_pairs_at_k2": new_strict,
                      "new_weak_pairs_at_k2": new_weak,
                      "raw_rows_sha256": sha(raw)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
