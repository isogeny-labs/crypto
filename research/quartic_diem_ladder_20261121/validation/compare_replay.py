#!/usr/bin/env python3
"""Validator: compare replayed rows (validation/replay.sh) with the committed rows.

Usage: python3 -I compare_replay.py REPO SHA OUT.json REPLAY.jsonl [REPLAY.jsonl ...]

Committed rows are read with `git show SHA:<path>`.  Every replayed run/skip row
must match exactly one committed row with the same (kind, arm, p, curve_seed,
seed); the two rows must have the same key set, and every value must be equal
except wall-clock fields (wall_ns).  Replayed instance rows must equal the
committed instance rows of the same (p, curve_seed).
"""
import json
import subprocess
import sys

STUDY = "research/quartic_diem_ladder_20261121"
FILES = ["raw/run.jsonl", "raw/run131101_odd.jsonl", "raw/run131101_even.jsonl"]
WALL = {"wall_ns", "search_wall_s", "wall_s"}


def git_rows(repo, sha, path):
    out = subprocess.run(["git", "-C", repo, "show", f"{sha}:{STUDY}/{path}"],
                         check=True, capture_output=True).stdout
    return [json.loads(l) for l in out.decode().splitlines() if l.strip()]


def main():
    repo, sha, outp = sys.argv[1:4]
    committed = {}
    cinst = {}
    for fn in FILES:
        for r in git_rows(repo, sha, fn):
            if r.get("kind") in ("run", "skip") and r.get("family") == "quartic":
                key = (r["kind"], r["arm"], r["p"], r["curve_seed"], r["seed"])
                assert key not in committed, key
                committed[key] = (fn, r)
            elif r.get("kind") == "instance" and r.get("family") == "quartic":
                cinst[(r["p"], r["curve_seed"])] = r
    res = {"sha": sha, "runs": [], "instances": []}
    for path in sys.argv[4:]:
        for line in open(path):
            r = json.loads(line)
            if r.get("kind") == "instance" and r.get("family") == "quartic":
                c = cinst.get((r["p"], r["curve_seed"]))
                res["instances"].append({"p": r["p"], "curve_seed": r["curve_seed"],
                                         "equal": c == r})
                continue
            if r.get("kind") not in ("run", "skip") or r.get("family") != "quartic":
                continue
            key = (r["kind"], r["arm"], r["p"], r["curve_seed"], r["seed"])
            fn, c = committed.get(key, (None, None))
            entry = {"kind": r["kind"], "arm": r["arm"], "p": r["p"], "curve_seed": r["curve_seed"],
                     "seed": r["seed"], "committed_file": fn, "found": c is not None}
            if c is not None:
                keys_r, keys_c = set(r) - WALL, set(c) - WALL
                entry["same_keys"] = keys_r == keys_c
                diffs = sorted(k for k in keys_r & keys_c if r[k] != c[k])
                entry["differing_fields"] = diffs
                entry["n_fields_compared"] = len(keys_r & keys_c)
                entry["equal"] = entry["same_keys"] and not diffs
                if r["kind"] == "run":
                    entry["recovered"] = r.get("recovered")
                    entry["certified"] = r.get("certified")
                    entry["ok"] = r.get("ok")
                    entry["wall_ns_replay_over_committed"] = r["wall_ns"] / c["wall_ns"]
            else:
                entry["equal"] = False
            res["runs"].append(entry)
    res["summary"] = {
        "rows_replayed": len(res["runs"]),
        "rows_equal": sum(e["equal"] for e in res["runs"]),
        "ic_runs_replayed": sum(1 for e in res["runs"] if e["kind"] == "run" and e["arm"] != "Q_rho_neg"),
        "rho_runs_replayed": sum(1 for e in res["runs"] if e["kind"] == "run" and e["arm"] == "Q_rho_neg"),
        "instances_replayed": len(res["instances"]),
        "instances_equal": sum(e["equal"] for e in res["instances"]),
    }
    json.dump(res, open(outp, "w"), indent=1)
    print(json.dumps(res["summary"]))
    for e in res["runs"]:
        if not e["equal"]:
            print("MISMATCH", json.dumps(e))


if __name__ == "__main__":
    main()
