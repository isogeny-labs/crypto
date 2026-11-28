#!/usr/bin/env python3
"""Independent pair-boundary and finite-field scalar replay audit."""

import hashlib
import json
import statistics
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0,str(HERE.parent / "xedni_coefficient_choice_20261120"))
from audit import add


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def replay(k):
    curve = [0,0,0,46,45]
    G = (43,19)
    P = None
    for _ in range(k):
        P = add(P,G,curve,47)
    return P


def main():
    protocol_raw = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_raw)
    result = json.loads((HERE / "result.json").read_text())
    assert sha(protocol_raw) == result["protocol_sha256"]
    raw = (HERE / "pair-rows.jsonl").read_bytes()
    assert sha(raw) == result["pair_rows_raw_sha256"]
    rows = [json.loads(line) for line in raw.splitlines()]
    assert len(rows) == 10 and result["paired_runs"] == 5
    previous = result["setup_stop_ns"]
    assert result["setup_ns"] == result["setup_stop_ns"]-result["setup_start_ns"] > 0
    seeds = set()
    for pair in range(1,6):
        two = [row for row in rows if row["pair"] == pair]
        assert len(two) == 2 and [r["position"] for r in two] == [1,2]
        assert [r["arm"] for r in two] == (["lift","rho"] if pair%2 else ["rho","lift"])
        for row in two:
            assert row["status"] == "verified"
            assert row["start_ns"] >= previous
            assert row["stop_ns"] > row["start_ns"]
            assert row["online_ns"] == row["stop_ns"]-row["start_ns"]
            previous = row["stop_ns"]
            assert replay(row["scalar"]) == tuple(protocol["target"])
            if row["arm"] == "lift":
                assert row["candidate_evaluations"] == 35
                assert row["collision_scalar"] == 9
            else:
                assert row["seed_mod_43"] == (17*pair+row["restarts"]+1)%43
                assert row["walk_updates"] >= 3*row["iterations"]
                seeds.add(row["seed_mod_43"])
    assert len(seeds) == 5
    assert statistics.median(r["online_ns"] for r in rows if r["arm"]=="lift") == result["lift_online_median_ns"]
    assert statistics.median(r["online_ns"] for r in rows if r["arm"]=="rho") == result["rho_online_median_ns"]
    initial = json.loads((HERE / "initial-result.json").read_text())
    initial_raw = (HERE / "initial-pair-rows.jsonl").read_bytes()
    assert sha(initial_raw) == initial["pair_rows_raw_sha256"]
    assert len(initial_raw.splitlines()) == 10
    return {"status":"PASS","same_public_point":protocol["target"],
            "verified_pairs":5,"distinct_rho_seeds":len(seeds),
            "replayed_scalars":len(rows),"pair_rows_raw_sha256":sha(raw),
            "initial_pair_rows_raw_sha256":sha(initial_raw)}


if __name__ == "__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
