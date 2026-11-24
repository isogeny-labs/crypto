#!/usr/bin/env python3
"""Compare the validator's replay rows with the committed raw rows.

Usage:
  python3 -I compare_replay.py STUDY_DIR REPLAY_DIR OUT.json

STUDY_DIR is a `git archive` snapshot of the study at the validated commit.
Rows are matched by (kind, family, arm, p, curve_seed, seed).  Two checks per row:
  bytes  : the committed JSON line and the replayed JSON line are byte-identical
           after masking only the wall-clock values ("wall_ns", "search_wall_s",
           "wall_s") -- the binary writes keys in a fixed (sorted) order, so this
           is a byte-level test of every counted field;
  fields : the parsed objects have the same key set and equal values on every key
           except the wall-clock keys (listed per row).
Every replayed run row's recovered value is also checked against t_secret.
"""
import json
import re
import sys

WALL = ("wall_ns", "search_wall_s", "wall_s")
MASK = re.compile(r'"(wall_ns|search_wall_s|wall_s)":[-+0-9.eE]+')


def rows(path):
    out = []
    with open(path) as fh:
        for line in fh:
            line = line.rstrip("\n")
            if line.strip():
                out.append((line, json.loads(line)))
    return out


def key(r):
    return (r["kind"], r.get("family"), r.get("arm"), r.get("p"), r.get("curve_seed"), r.get("seed"))


def main():
    study, replay, out_path = sys.argv[1], sys.argv[2], sys.argv[3]
    committed = {}
    for f in ("raw/run.jsonl", "raw/run131101.jsonl"):
        for line, r in rows(f"{study}/{f}"):
            if r["kind"] in ("config", "done"):
                continue
            k = key(r)
            assert k not in committed, ("duplicate committed row", k)
            committed[k] = (f, line, r)
    report = {"schema": "validator-replay-compare/v1", "rows": [], "summary": {}}
    n = eq_b = eq_f = runs = runs_ok = 0
    cfg = []
    for f in ("replay_main.jsonl", "replay_131101.jsonl"):
        try:
            rr = rows(f"{replay}/{f}")
        except FileNotFoundError:
            continue
        for line, r in rr:
            if r["kind"] == "config":
                cfg.append(r)
                continue
            if r["kind"] == "done":
                continue
            k = key(r)
            n += 1
            ent = {"replay_file": f, "kind": r["kind"], "arm": r.get("arm"), "p": r.get("p"),
                   "curve_seed": r.get("curve_seed"), "seed": r.get("seed")}
            if k not in committed:
                ent.update(found=False, bytes_equal=False, fields_equal=False)
                report["rows"].append(ent)
                continue
            cf, cline, c = committed[k]
            b = MASK.sub(r'"\1":0', cline) == MASK.sub(r'"\1":0', line)
            same_keys = set(c) == set(r)
            diff = sorted(x for x in set(c) | set(r) if x not in WALL and c.get(x, object()) != r.get(x, object()))
            fe = same_keys and not diff
            ent.update(found=True, committed_file=cf, bytes_equal=b, same_keys=same_keys,
                       differing_fields=diff, fields_equal=fe,
                       n_fields_compared=len(set(c) - set(WALL)))
            if r["kind"] == "run":
                runs += 1
                okr = (r["certified"] is True and r["ok"] is True and r["recovered"] == r["t_secret"])
                ent.update(recovered=r["recovered"], certified=r["certified"], ok=okr,
                           la_fallbacks=r["la_fallbacks"], attempts=r["attempts"],
                           wall_ns_replay_over_committed=r["wall_ns"] / c["wall_ns"])
                runs_ok += okr
            eq_b += b
            eq_f += fe
            report["rows"].append(ent)
    kinds = {}
    for e in report["rows"]:
        kinds.setdefault(e["kind"], [0, 0])
        kinds[e["kind"]][0] += 1
        kinds[e["kind"]][1] += bool(e.get("bytes_equal"))
    report["summary"] = {"rows_replayed": n, "rows_bytes_equal": eq_b, "rows_fields_equal": eq_f,
                         "runs_replayed": runs, "runs_certified_and_correct": runs_ok,
                         "by_kind_replayed_equal": kinds,
                         "fallback_runs_replayed": sum(1 for e in report["rows"]
                                                       if e["kind"] == "run" and e.get("la_fallbacks", 0) > 0)}
    report["replay_configs"] = cfg
    json.dump(report, open(out_path, "w"), indent=1)
    print(json.dumps(report["summary"]))


if __name__ == "__main__":
    main()
