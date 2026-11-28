#!/usr/bin/env python3
"""Compare the validator's replay rows with the committed raw rows.

Adapted for research/quartic_diem_p0lines_20261128 from the parent validation's
compare_replay.py (research/quartic_diem_nofb_20261126/validation/): the per-run record
now carries this study's P0-line counters (p0_lines, p0_line_relations) and attempts
instead of the parent's fallback counters; files whose replay is absent are reported,
not assumed.

Usage:
  python3 -I compare_replay.py STUDY_DIR REPLAY_DIR OUT.json

STUDY_DIR is a `git archive` snapshot of the study at the validated commit.  Rows are
matched by (kind, family, arm, p, curve_seed, seed), separately per file.  Per row:
  bytes  : the committed JSON line and the replayed JSON line are byte-identical after
           masking only the wall-clock values ("wall_ns", "search_wall_s", "wall_s");
           the binary writes keys in a fixed (sorted) order, so this is a byte-level
           test of every counted field;
  fields : the parsed objects have the same key set and equal values on every key
           except the wall-clock keys.
The config rows (one per file) must be byte-identical; the done rows carry only
wall time.  Every replayed run row's recovered value is checked against t_secret.
"""
import json
import re
import sys

WALL = ("wall_ns", "search_wall_s", "wall_s")
MASK = re.compile(r'"(wall_ns|search_wall_s|wall_s)":[-+0-9.eE]+')
FILES = ("raw/run.jsonl", "raw/run131101.jsonl")


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
    report = {"schema": "validator-replay-compare/v1", "files": {}, "rows": [], "summary": {}}
    n = eq_b = eq_f = runs = runs_ok = 0
    covered = total_committed = dup = 0
    for f in FILES:
        committed = {}
        cfg_c = done_c = None
        for line, r in rows(f"{study}/{f}"):
            if r["kind"] == "config":
                cfg_c = line
                continue
            if r["kind"] == "done":
                done_c = r
                continue
            k = key(r)
            assert k not in committed, ("duplicate committed row", f, k)
            committed[k] = (line, r)
        seen = {}
        cfg_r = done_r = None
        if not __import__("os").path.exists(f"{replay}/{f}"):
            report["files"][f] = {"committed_rows": len(committed), "replayed_rows": 0, "covered": 0,
                                  "config_byte_identical": False, "replay_absent": True}
            total_committed += len(committed)
            continue
        for line, r in rows(f"{replay}/{f}"):
            if r["kind"] == "config":
                cfg_r = line
                continue
            if r["kind"] == "done":
                done_r = r
                continue
            k = key(r)
            seen[k] = seen.get(k, 0) + 1
            n += 1
            ent = {"file": f, "kind": r["kind"], "arm": r.get("arm"), "p": r.get("p"),
                   "curve_seed": r.get("curve_seed"), "seed": r.get("seed")}
            if k not in committed:
                ent.update(found=False, bytes_equal=False, fields_equal=False)
                report["rows"].append(ent)
                continue
            cline, c = committed[k]
            b = MASK.sub(r'"\1":0', cline) == MASK.sub(r'"\1":0', line)
            same_keys = set(c) == set(r)
            diff = sorted(x for x in set(c) | set(r)
                          if x not in WALL and c.get(x, object()) != r.get(x, object()))
            fe = same_keys and not diff
            ent.update(found=True, bytes_equal=b, same_keys=same_keys, differing_fields=diff,
                       fields_equal=fe, n_fields_compared=len(set(c) - set(WALL)))
            if r["kind"] == "run":
                runs += 1
                okr = (r["certified"] is True and r["ok"] is True and r["recovered"] == r["t_secret"])
                ent.update(recovered=r["recovered"], ok=okr, attempts=r["attempts"],
                           p0_lines=r["p0_lines"], p0_line_relations=r["p0_line_relations"],
                           wall_ns_replay_over_committed=r["wall_ns"] / c["wall_ns"])
                runs_ok += okr
            eq_b += b
            eq_f += fe
            report["rows"].append(ent)
        total_committed += len(committed)
        covered += sum(1 for k in committed if k in seen)
        dup += sum(1 for v in seen.values() if v > 1)
        report["files"][f] = {
            "committed_rows": len(committed), "replayed_rows": sum(seen.values()),
            "covered": sum(1 for k in committed if k in seen),
            "config_byte_identical": cfg_c is not None and cfg_c == cfg_r,
            "done_wall_s": {"committed": done_c["wall_s"] if done_c else None,
                            "replay": done_r["wall_s"] if done_r else None}}
    kinds = {}
    for e in report["rows"]:
        kinds.setdefault(e["kind"], [0, 0])
        kinds[e["kind"]][0] += 1
        kinds[e["kind"]][1] += bool(e.get("bytes_equal"))
    wr = [e["wall_ns_replay_over_committed"] for e in report["rows"] if "wall_ns_replay_over_committed" in e]
    report["summary"] = {"rows_replayed": n, "rows_bytes_equal": eq_b, "rows_fields_equal": eq_f,
                         "runs_replayed": runs, "runs_certified_and_correct": runs_ok,
                         "by_kind_replayed_equal": kinds,
                         "multi_attempt_runs_replayed": sum(1 for e in report["rows"]
                                                            if e["kind"] == "run" and e["attempts"] > 1),
                         "p0_runs_replayed": sum(1 for e in report["rows"]
                                                 if e["kind"] == "run" and e["p0_lines"] > 0),
                         "duplicate_replayed_keys": dup,
                         "committed_rows": total_committed, "committed_rows_covered": covered,
                         "configs_byte_identical": all(v["config_byte_identical"] for v in report["files"].values()),
                         "wall_ratio_range": [min(wr), max(wr)] if wr else None}
    with open(out_path, "w") as fh:
        json.dump(report, fh, indent=1)
        fh.write("\n")
    print(json.dumps(report["summary"]))


if __name__ == "__main__":
    main()
