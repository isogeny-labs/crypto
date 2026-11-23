"""Audit the frozen rank panel, methods, statuses, and source identity."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "xedni_direct_word2_lifts_20261121" / "result.json"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    assert sha(SOURCE) == protocol["source_result_sha256"]
    source = {r["source_index"]: r for r in json.loads(SOURCE.read_text())["rows"]}
    rows = [json.loads(line) for line in (HERE / "cells.jsonl").read_text().splitlines()]
    assert [(r["source_index"], r["method"]) for r in rows] == [
        (i, m) for i in protocol["source_indices"] for m in protocol["methods_in_order"]]
    per_curve = []
    for i in protocol["source_indices"]:
        assert source[i]["status"] == "PASS"
        simon = next(r for r in rows if r["source_index"] == i and r["method"] == "simon_two_descent")
        pari = next(r for r in rows if r["source_index"] == i and r["method"] == "pari_proof_rank")
        assert simon["status"] == pari["status"] == "complete"
        assert simon["exit_code"] == pari["exit_code"] == 0
        assert simon["elapsed_ns"] > 0 and pari["elapsed_ns"] > 0
        assert simon["elapsed_ns"] < protocol["per_method_wall_cap_seconds"]*10**9
        assert pari["elapsed_ns"] < protocol["per_method_wall_cap_seconds"]*10**9
        bounds = simon["result"]
        exact = pari["result"]
        assert bounds["rank_lower"] == bounds["rank_upper"] == exact["rank"]
        assert exact["proof"] is True
        assert source[i]["Q"] == [int(v) for v in bounds["witnesses"][0]]
        per_curve.append({"source_index": i, "rank": exact["rank"],
                          "rank_premise_d_lt_2": exact["rank"] < 2,
                          "simon_bounds": [bounds["rank_lower"], bounds["rank_upper"]]})
    assert [(x["source_index"], x["rank"]) for x in per_curve] == [
        (2, 1), (85, 2), (201, 1), (353, 2), (511, 1)]
    print(json.dumps({"schema": "xedni-direct-bridge-ranks-audit-v1", "status": "PASS",
                      "source_sha256": sha(SOURCE), "cell_count": len(rows),
                      "cells_sha256": sha(HERE / "cells.jsonl"), "curves": per_curve},
                     indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
