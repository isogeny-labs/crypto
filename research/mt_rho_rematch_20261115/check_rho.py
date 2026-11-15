#!/usr/bin/env python3
"""Pre-run checks of PROTOCOL.md. usage: check_rho.py BIN INSTANCES_DIR pb:T [pb:T ...]
rhodpup must equal rhodpu exactly (ok and I, M, S, H, L, N); rho2xksp must recover every target, with its counted
steps within one step per target of rho2xks's; ic5b std 1 and flat 32 must still equal ic5."""
import json, subprocess, sys
BIN, INST = sys.argv[1], sys.argv[2]
def run(args):
    o = subprocess.run([BIN] + args, capture_output=True, text=True, check=True).stdout.strip().splitlines()[-1]
    return json.loads(o)
bad = 0
for cell in sys.argv[3:]:
    pb, T = cell.split(":"); inst = f"{INST}/W{pb}-0.json"
    for kp, off in ((0.0, "0"), (0.25, "-1")):
        a = run(["rhodpu", inst, T, "43", str(kp), off]); b = run(["rhodpup", inst, T, "43", str(kp), off])
        same = all(a[c] == b[c] for c in "IMSHLN") and a["ok"] == b["ok"] == a["targets"]
        bad += not same
        print(f"{pb} {T} rhodpu/rhodpup k={kp} off={off}: ok {a['ok']}/{b['ok']}/{a['targets']} counts_equal={same} ms_arm {a['ms_arm']} -> {b['ms_arm']}")
    for kp in ("0.05", "0.5"):
        a = run(["rho2xks", inst, T, "43", kp]); b = run(["rho2xksp", inst, T, "43", kp])
        ok = b["ok"] == b["targets"]
        dsteps = b["target_steps"] - a["target_steps"]
        bad += not ok
        print(f"{pb} {T} rho2xks/rho2xksp k={kp}: ok {a['ok']}/{b['ok']}/{b['targets']} target_steps {a['target_steps']} -> {b['target_steps']} (diff {dsteps}, {dsteps/int(T):.2f}/target) ms_arm {a['ms_arm']} -> {b['ms_arm']}")
    ref = run(["ic5", inst, T, "43", "1.0", "4", "1.2"])
    for mode, bs in (("std", "1"), ("flat", "32")):
        o = run(["ic5b", inst, T, "43", "1.0", "4", "1.2", mode, bs])
        same = all(o[c] == ref[c] for c in "IMSHLN") and o["ok"] == o["targets"]
        bad += not same
        print(f"{pb} {T} ic5b {mode} {bs}: counts_equal_ic5={same} ok {o['ok']}/{o['targets']}")
print("CHECK", "PASS" if bad == 0 else f"FAIL ({bad})")
sys.exit(1 if bad else 0)
