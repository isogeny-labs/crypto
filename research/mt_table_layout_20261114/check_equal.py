#!/usr/bin/env python3
"""Correctness check of PROTOCOL.md before any timed run: for each (pb, T) given, run ic5 and every ic5b mode on
instance W{pb}-0 and require ok = targets and identical counted totals (I, M, S, H, L, N).
usage: check_equal.py BIN INSTANCES_DIR pb:T [pb:T ...]"""
import json, subprocess, sys
BIN, INST = sys.argv[1], sys.argv[2]
MODES = [("ic5", []), ("ic5b", ["std", "1"]), ("ic5b", ["std", "32"]), ("ic5b", ["flat", "1"]),
         ("ic5b", ["flat", "8"]), ("ic5b", ["flat", "32"])]
bad = 0
for cell in sys.argv[3:]:
    pb, T = cell.split(":")
    ref = None
    for arm, extra in MODES:
        cmd = [BIN, arm, f"{INST}/W{pb}-0.json", T, "43", "1.0", "4", "1.2"] + extra
        o = json.loads(subprocess.run(cmd, capture_output=True, text=True, check=True).stdout.strip().splitlines()[-1])
        tot = tuple(o[c] for c in "IMSHLN")
        ok = o["ok"] == o["targets"]
        if ref is None:
            ref = tot
        same = tot == ref
        bad += (not ok) + (not same)
        print(f"{pb} {T} {arm:5s} {' '.join(extra):8s} ok={o['ok']}/{o['targets']} counts_equal={same} "
              f"ms_arm={o['ms_arm']} ms_targets={o.get('ms_targets', '-')}")
print("CHECK", "PASS" if bad == 0 else f"FAIL ({bad})")
sys.exit(1 if bad else 0)
