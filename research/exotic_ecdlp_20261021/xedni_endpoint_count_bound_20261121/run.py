"""Count fixed-curve word endpoints under the frozen uniform target law."""

import hashlib
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
SOURCE=HERE.parent/"xedni_fixed_rank1_height_20261121"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    p=json.loads((HERE/"protocol.json").read_text())
    assert sha(SOURCE/"rows.jsonl")==p["source_rows_sha256"]
    assert sha(SOURCE/"summary.json")==p["source_summary_sha256"]
    source=[json.loads(s) for s in (SOURCE/"rows.jsonl").read_text().splitlines()]
    assert [r["n"] for r in source]==list(range(1,22))
    records=[]
    for M in range(1,22):
        targets={c*n*sgn%43 for n in range(1,M+1) for c in (1,2,6) for sgn in (-1,1)}
        assert 0 not in targets
        count=len(targets)
        bound=min(42,6*M)
        assert count<=bound
        old=source[M-1]
        records.append({"prefix":M,"covered_targets":count,"target_count":42,
                        "uniform_hit_probability_exact":f"{count}/42",
                        "generic_count_bound":bound,"relation_rank":old["rank"],
                        "max_coordinate_bits":old["max_bits"]})
    print(json.dumps({"schema":"xedni-endpoint-count-result-v1","status":"PASS",
                      "first_full_coverage_prefix":next(r["prefix"] for r in records if r["covered_targets"]==42),
                      "first_rank20_prefix":next(r["prefix"] for r in records if r["relation_rank"]==20),
                      "prefixes":records},indent=2,sort_keys=True))


if __name__=="__main__":main()
