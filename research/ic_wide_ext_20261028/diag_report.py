"""Per-stage lane-round cost against lane count from a diag session (ECBENCH_LANE_DIAG=1).

Prints, per arm, the median over measured runs of: trials, relation-phase ns/trial, and the
four lane-stage ns/trial (prep = jump choice and request build; add = the batched additions
incl. the shared inversion; queue = result copy and queue fill; touch = the round's
guard/oracle touch loop), plus the remainder (everything after the round: guard insert,
oracle probe, row build)."""
import json, statistics, sys, pathlib

def med(xs): return statistics.median(xs)

def main(paths):
    recs = []
    for p in paths:
        recs += [json.loads(l) for l in open(pathlib.Path(p) / "records.jsonl") if l.strip()]
    recs = [r for r in recs if not r["warmup"] and r["method"]["id"] == "ic.pipeline"]
    arms = sorted({r["arm"] for r in recs}, key=lambda a: (a[0] != "w", len(a), a))
    print(f"{'arm':8s} {'trials':>8s} {'rel ns/trial':>13s} {'prep':>7s} {'add':>7s} {'queue':>7s} {'touch':>7s} {'rest':>7s}")
    for arm in arms:
        v = [r for r in recs if r["arm"] == arm]
        rows = []
        for r in v:
            rel = next(p for p in r["phases"] if p["name"] == "relations")
            c = {k: val for k, val in rel["counters"].items()} if "counters" in rel else {}
            t = c.get("trials") or r["counters"].get("targets_tried")
            rows.append((t, rel["wall_ns"] / t, [c.get(k, 0) / t for k in ("diag_prep_ns", "diag_add_ns", "diag_queue_ns", "diag_touch_ns")]))
        t = med([x[0] for x in rows]); tot = med([x[1] for x in rows])
        st = [med([x[2][i] for x in rows]) for i in range(4)]
        print(f"{arm:8s} {t:8.0f} {tot:13.1f} " + " ".join(f"{s:7.1f}" for s in st) + f" {tot - sum(st):7.1f}")

main(sys.argv[1:])
