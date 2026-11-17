"""Descriptive integer sums of the wall session's recorded times, per arm and per arm and curve (PROTOCOL.md section 7, model risk: the online windows are reported descriptively and decide nothing).
Over measured (not warm-up), verified records only. Sums of integers; no ratio, no mean.
usage: phase_sums.py EKS_DIR ANALYSIS_DIR
Writes ANALYSIS_DIR/phase_sums.json and prints it compactly. Exits 1 with a line beginning STOP: only when records.jsonl is absent or unreadable."""
import collections, json, os, sys

eks, an = sys.argv[1:3]
try:
    recs = [json.loads(l) for l in open(os.path.join(eks, "session", "records.jsonl")) if l.strip()]
except Exception as e:
    print("STOP: cannot read records.jsonl: " + type(e).__name__)
    sys.exit(1)

def add(acc, key, v):
    if isinstance(v, int) and not isinstance(v, bool):
        acc[key] += v

arm_sums = collections.defaultdict(collections.Counter)
arm_curve_sums = collections.defaultdict(collections.Counter)
for r in recs:
    if r.get("warmup") or (r.get("outcome") or {}).get("status") != "verified":
        continue
    arm = r.get("arm")
    slug = ((r.get("workload") or {}).get("curve") or {}).get("slug")
    for acc in (arm_sums[arm], arm_curve_sums[arm + " | " + str(slug)]):
        acc["n_records"] += 1
        add(acc, "time.solve_wall_ns", (r.get("time") or {}).get("solve_wall_ns"))
        add(acc, "online.wall_ns", (r.get("online") or {}).get("wall_ns"))
        for k, v in sorted(((r.get("online") or {}).get("phases_ns") or {}).items()):
            add(acc, "online.phases_ns." + k, v)
        for p in r.get("phases") or []:
            add(acc, "phases." + str(p.get("name")) + ".wall_ns", p.get("wall_ns"))
        add(acc, "cost.total_gae_as_int", int((r.get("cost") or {}).get("total_gae") or 0))
out = {"by_arm": {a: dict(sorted(c.items())) for a, c in sorted(arm_sums.items())},
       "by_arm_and_curve": {a: dict(sorted(c.items())) for a, c in sorted(arm_curve_sums.items())}}
json.dump(out, open(os.path.join(an, "phase_sums.json"), "w"), indent=1, sort_keys=True)
for a, c in out["by_arm"].items():
    print("PHASE_SUMS_ARM " + a + " " + json.dumps(c, sort_keys=True))
for a, c in out["by_arm_and_curve"].items():
    print("PHASE_SUMS_ARM_CURVE " + a + " " + json.dumps(c, sort_keys=True))
print("PHASE_SUMS_OK")
