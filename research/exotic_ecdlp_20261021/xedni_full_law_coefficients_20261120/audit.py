"""Check full-law accounting and containment of the earlier 20-pair screen."""

import gzip
import hashlib
import json
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
PRIOR = HERE.parent / "xedni_coefficient_choice_20261120"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_bytes)
    result = json.loads((HERE / "result.json").read_text())
    source_bytes = (PRIOR / "source-rows-v2.jsonl.gz").read_bytes()
    pair_bytes = (HERE / "pair-rows.jsonl").read_bytes()
    assert protocol["source_sha256"] == sha(source_bytes) == result["source_sha256"]
    assert result["protocol_sha256"] == sha(protocol_bytes)
    assert result["script_sha256"] == sha((HERE / "run.py").read_bytes())
    assert result["arithmetic_sha256"] == sha((PRIOR / "audit.py").read_bytes())
    assert result["pair_rows_sha256"] == sha(pair_bytes)
    source = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    rows = [json.loads(x) for x in pair_bytes.splitlines()]
    assert len(rows) == len(source) == 1722
    totals = Counter()
    prior_result = json.loads((PRIOR / "result.json").read_text())
    selected_from_prior = {"strict": {}, "weak": {}}
    prior_index = 0
    for index, (row, original) in enumerate(zip(rows, source)):
        assert row["pair_index"] == index
        assert row["source_status"] == original["status"]
        assert row["reduced_relation"] == original["reduced_relation"]
        counts = row["counts"]
        if original["status"] == "same_x":
            assert counts == {"same_x": 1}
        else:
            assert counts["candidates"] == 27
            assert sum(counts.get("p7_"+x, 0) for x in ("nonintegral", "bad_reduction", "good")) == 27
            assert sum(counts.get("p11_"+x, 0) for x in ("nonintegral", "bad_reduction", "good")) == 27
        totals.update(counts)
        totals["source_"+original["status"]] += 1
        for arm, picked in row["selected"].items():
            if picked is not None:
                assert counts[arm+"_passing_choices"] > 0
                totals[arm+"_selected_pairs"] += 1
                totals[arm+"_selected_reduced"] += int(original["reduced_relation"])
                totals[arm+"_selected_rational"] += int(picked["rational_relation"])
        if original["accepted"]:
            for arm, picked in row["selected"].items():
                if picked is not None:
                    selected_from_prior[arm][str(prior_index)] = {
                        "k": picked["k"], "reduced_relation": original["reduced_relation"],
                        "rational_relation": picked["rational_relation"]}
            prior_index += 1
    assert prior_index == 20
    assert dict(totals) == result["counts"]
    assert all(selected_from_prior[arm] == prior_result["selected"][arm]["first_by_pair"]
               for arm in ("strict", "weak"))
    assert result["counts"]["candidates"] == 45360
    print(json.dumps({"status": "PASS", "source_pairs": len(source),
                      "candidate_choices": totals["candidates"],
                      "prior_panel_strict_pairs": len(selected_from_prior["strict"]),
                      "prior_panel_weak_pairs": len(selected_from_prior["weak"]),
                      "pair_rows_sha256": sha(pair_bytes)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
