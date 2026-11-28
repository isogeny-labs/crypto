"""Old binary against new binary on the same spec: paired relation-phase wall time and counted identity.

usage: ab_compare.py <dir-old-1> [<dir-old-2> ...] -- <dir-new-1> [<dir-new-2> ...]

Sessions are paired by position (old-i with new-i); within a pair, records are matched on
(arm, target_index, round).  Reports, per arm, the paired geometric mean of new/old for the
relations phase and for the solve, and whether every record is identical outside timing
fields (the counted-output identity the round-2 and round-3 studies used).
Standard library only."""
import json
import math
import pathlib
import statistics
import sys

VOLATILE = {"time", "isolation", "env_class_id", "binary_sha256", "record_id", "session_id", "run_id"}


def load(d):
    return [json.loads(l) for l in (pathlib.Path(d) / "records.jsonl").read_text().splitlines() if l.strip()]


def counted(r):
    r = {k: v for k, v in r.items() if k not in VOLATILE}
    r["phases"] = [{k: v for k, v in p.items() if k != "wall_ns"} for p in r["phases"]]
    if isinstance(r.get("online"), dict):
        o = dict(r["online"])
        o.pop("wall_ns", None)
        o["phases_ns"] = sorted(o.get("phases_ns") or {})
        r["online"] = o
    return json.dumps(r, sort_keys=True)


def rel(r):
    return next(p["wall_ns"] for p in r["phases"] if p["name"] == "relations")


def main(argv):
    cut = argv.index("--")
    old, new = argv[:cut], argv[cut + 1:]
    assert len(old) == len(new)
    same = total = 0
    rel_logs, solve_logs = {}, {}
    for a, b in zip(old, new):
        ra = {(r["arm"], r["workload"]["target_index"], r["round"], r["warmup"]): r for r in load(a)}
        rb = {(r["arm"], r["workload"]["target_index"], r["round"], r["warmup"]): r for r in load(b)}
        for k, x in ra.items():
            y = rb[k]
            total += 1
            same += counted(x) == counted(y)
            if not k[3]:
                rel_logs.setdefault(k[0], []).append(math.log(rel(y) / rel(x))) if x["method"]["id"] == "ic.pipeline" else None
                solve_logs.setdefault(k[0], []).append(math.log(y["time"]["solve_wall_ns"] / x["time"]["solve_wall_ns"]))
    print(f"records identical outside timing: {same}/{total}")
    print(f"{'arm':8s} {'n':>4s} {'relations new/old':>18s} {'95% interval':>18s} {'solve new/old':>14s}")
    for arm in sorted(solve_logs, key=lambda a: (a[0] != "w", a == "fast", len(a), a)):
        s = math.exp(statistics.mean(solve_logs[arm]))
        if arm in rel_logs:
            l = rel_logs[arm]
            m = statistics.mean(l)
            h = 1.96 * statistics.stdev(l) / math.sqrt(len(l))
            print(f"{arm:8s} {len(l):4d} {math.exp(m):18.3f} [{math.exp(m-h):.3f},{math.exp(m+h):.3f}] {s:14.3f}")
        else:
            print(f"{arm:8s} {len(solve_logs[arm]):4d} {'':>18s} {'':>18s} {s:14.3f}")


if __name__ == "__main__":
    main(sys.argv[1:])
