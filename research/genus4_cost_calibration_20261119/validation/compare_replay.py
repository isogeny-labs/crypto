#!/usr/bin/env python3
"""Compare the validator's count replays (validation/replay/replay_p{P}_cc{CC}.jsonl, made by replay.sh with the
frozen binary pinned to core 3) against the committed pass-1 and pass-2 rows, field by field.
Every field except wall-clock fields (wall_ns, search_wall_s, wall_s) must be equal; config and instance rows must
be equal in every field. Writes validation/replay.json (which embeds the replayed rows)."""
import gzip, hashlib, json, os, sys

V = os.path.dirname(os.path.abspath(__file__))
S = os.path.dirname(V)
WALL = {"wall_ns", "search_wall_s", "wall_s"}
B = "/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/genus_ladder.po.validator_copy"


def rows_gz(path):
    return [json.loads(l) for l in gzip.open(path, "rt") if l.strip()]


def rows(path):
    return [json.loads(l) for l in open(path) if l.strip()]


def k(r):
    return (r["p"], r["curve_seed"], r["target_seed"], r["seed"], r["w_max"], r["fill_max"])


out = {"binary_sha256": hashlib.sha256(open(B, "rb").read()).hexdigest(),
       "binary_expected": open(os.path.join(S, "binary.sha256")).read().split()[0],
       "pinning": "taskset -c 3 (protocol used -c 2); cores 0-3 shared with a concurrent curve search, so wall times are not compared",
       "argv_template": "genus_ladder.po run --primes P --genera 4 --max-h 1 --curves 2 --curve-seeds P=C1/C2 --targets 1 --seeds 2 "
                        "--arms x --cc CC --sge 4,1.5 --sge 8,2 --threads 1 --out validation/replay/replay_pP_ccCC.jsonl",
       "configs": [], "all_equal": True}
for fn in sorted(os.listdir(os.path.join(V, "replay"))):
    if not (fn.startswith("replay_p") and fn.endswith(".jsonl")):
        continue
    P, CC = fn[len("replay_p"):-len(".jsonl")].split("_cc")
    rep = rows(os.path.join(V, "replay", fn))
    if not any(r["kind"] == "done" for r in rep):
        out["configs"].append({"file": fn, "status": "incomplete (no done row)"})
        out["all_equal"] = False
        continue
    entry = {"file": fn, "p": int(P), "cc": CC, "rows": [], "config_equal": {}, "instances_equal": {}}
    for ps in ("pass1", "pass2"):
        orig = rows_gz(os.path.join(S, f"ic_{ps}_p{P}_cc{CC}.jsonl.gz"))
        oc = [r for r in orig if r["kind"] == "config"][0]
        rc = [r for r in rep if r["kind"] == "config"][0]
        entry["config_equal"][ps] = oc == rc
        oi = sorted((r for r in orig if r["kind"] == "instance"), key=lambda r: r["curve_seed"])
        ri = sorted((r for r in rep if r["kind"] == "instance"), key=lambda r: r["curve_seed"])
        entry["instances_equal"][ps] = oi == ri
        oruns = {k(r): r for r in orig if r["kind"] == "run"}
        rruns = {k(r): r for r in rep if r["kind"] == "run"}
        assert set(oruns) == set(rruns), (fn, ps)
        for key in sorted(rruns):
            a, b = oruns[key], rruns[key]
            fields = sorted((set(a) | set(b)) - WALL)
            diff = [f for f in fields if a.get(f, "<missing>") != b.get(f, "<missing>")]
            entry["rows"].append({"vs": ps, "key": list(key), "count_fields_compared": len(fields), "differing_fields": diff,
                                  "wall_ns_orig": a["wall_ns"], "wall_ns_replay": b["wall_ns"]})
            if diff:
                out["all_equal"] = False
        if not (entry["config_equal"][ps] and entry["instances_equal"][ps]):
            out["all_equal"] = False
    entry["replayed_rows"] = rep
    entry["n_run_rows"] = sum(r["kind"] == "run" for r in rep)
    out["configs"].append(entry)
out["run_rows_replayed"] = sum(c.get("n_run_rows", 0) for c in out["configs"])
out["comparisons"] = sum(len(c.get("rows", [])) for c in out["configs"])
out["comparisons_with_differences"] = sum(1 for c in out["configs"] for r in c.get("rows", []) if r["differing_fields"])
json.dump(out, open(os.path.join(V, "replay.json"), "w"), indent=1)
print({k: out[k] for k in ("binary_sha256", "all_equal", "run_rows_replayed", "comparisons", "comparisons_with_differences")})
for c in out["configs"]:
    print(c["file"], c.get("status", ""), c.get("config_equal"), c.get("instances_equal"),
          [(r["vs"], r["key"][1], r["key"][3], r["key"][4], r["count_fields_compared"], r["differing_fields"]) for r in c.get("rows", [])][:4])
