"""Tests trigger.jq (PROTOCOL.md section 4, conditions 2 to 4) against (1) one rendering of each grading-v3 L2 blocker format string
of src/cryptanalysis/ecbench/record.rs at 3271a25d02, (2) the blockers the trigger must refuse, (3) the l2-gate.json files of the previous study's
six sessions (research/ic_dg_blocked_20261029 in the dg-blocked worktree), where only the 32-bit wall session is a noise case.
usage: python3 -I test_trigger.py TRIGGER_JQ SCRATCH_DIR [DGBLOCKED_RESEARCH_DIR]
Prints one line per case and the last line ALL_FIXTURES_AS_EXPECTED True or False."""
import json, os, subprocess, sys

trig, scratch = sys.argv[1:3]
real = sys.argv[3] if len(sys.argv) > 3 else None
L2 = [
    "L2: 3 ticks of foreign work on the run CPU during the run",
    "L2: 2 ticks of hypervisor steal on the run CPU",
    "L2: 5 busy ticks on the run CPU's idle siblings and the rest of the reservation",
    "L2: run-queue delay 412 µs is 0.640% of the solve",
    "L2: 3 involuntary context switches during a 12.345 ms solve (at most 1)",
    "L2: 3 involuntary context switches over the process (at most 2)",
    "L2: 20 interrupts on the run CPU during the solve (12 device, 3 IPI/other, 5 softirq; at most 10)",
    "L2: 3 foreign task slices on the run CPU during the solve (0 µs)",
]
L3 = ["L3: run CPU is in neither isolcpus nor nohz_full", "L3: governor is None, not performance"]
BAD = [
    "L2: solve too short to admit wall time: solve 523.1 µs is under the 1.0 ms minimum",
    "L2: 0.0301s of memory stall system-wide",
    "L2: no schedstat for the solve",
    "L2: no memory PSI",
    "L2: no /proc/stat ticks for the run CPU",
    "L2: preflight thresholds looser than the defaults: settle 1 s < 2 s",
    "L2: preflight not quiet: x",
    "L2: no preflight",
    "L2: solve too short to admit wall time: no solve wall time",
    "L1: something",
    "foreign",
    "L2: 3 foreign task slices on the run CPU during the solve (0 µs) trailing",
    "L2: 3 foreign task slices on the run CPU during the solve (0 μs)",
]


def run(doc):
    p = os.path.join(scratch, "gate-fixture.json")
    json.dump(doc, open(p, "w"), ensure_ascii=False)
    r = subprocess.run(["jq", "-e", "-f", trig, p], capture_output=True, text=True)
    return r.returncode, r.stdout.strip(), r.stderr.strip()


def doc(entries, inv=None):
    return {"required": "L2", "executions": 1344, "invalid_outcomes": [] if inv is None else inv,
            "below_l2": [{"seq": 1, "arm": "dgb", "isolation": {"level": lv, "blockers": b}} for lv, b in entries],
            "grades": [], "accepted": False}


ok = True
for s in L2:
    rc, o, e = run(doc([("L1", [s] + L3)]))
    print("positive rc=%d %s | %s" % (rc, o, s[:60]))
    ok &= rc == 0
rc, o, e = run(doc([("L1", L2[:3]), ("L1", L2[3:6] + L3), ("L1", L3)]))
print("positive multi-entry rc=%d %s" % (rc, o))
ok &= rc == 0
for s in BAD:
    rc, o, e = run(doc([("L1", [L2[0], s])]))
    print("negative rc=%d %s | %s" % (rc, o, s[:60]))
    ok &= rc == 1
for name, d in [
    ("level L2", doc([("L2", [L2[0]])])),
    ("level L0", doc([("L0", [L2[0]])])),
    ("empty below_l2", doc([])),
    ("invalid_outcomes non-empty", doc([("L1", [L2[0]])], inv=[{"seq": 2}])),
]:
    rc, o, e = run(d)
    print("negative rc=%d %s | %s" % (rc, o, name))
    ok &= rc == 1
d = doc([("L1", [L2[0]])])
del d["invalid_outcomes"]
rc, o, e = run(d)
print("negative rc=%d %s | invalid_outcomes key absent" % (rc, o))
ok &= rc == 1
d = doc([("L1", [L2[0]])])
d["below_l2"][0]["isolation"].pop("blockers")
rc, o, e = run(d)
print("negative rc=%d %s | blockers key absent" % (rc, o))
ok &= rc == 1
if real:
    want = {("24", "wall"): 1, ("28", "wall"): 1, ("32", "wall"): 0}
    for b in ("24", "28", "32"):
        p = os.path.join(real, "P%s/wall/eks-results/l2-gate.json" % b)
        if os.path.isfile(p):
            r = subprocess.run(["jq", "-e", "-f", trig, p], capture_output=True, text=True)
            exp = want[(b, "wall")]
            print("previous study P%s wall l2-gate.json: rc=%d (expected %d)" % (b, r.returncode, exp))
            ok &= r.returncode == exp
print("ALL_FIXTURES_AS_EXPECTED", ok)
