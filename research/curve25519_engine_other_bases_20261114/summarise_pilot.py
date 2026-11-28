#!/usr/bin/env python3
"""Calibration pilot (100 targets, seed 2026111404; sizing only). Run: python3 -I summarise_pilot.py > pilot_summary.txt"""
import glob, json, os
here = os.path.join(os.path.dirname(os.path.abspath(__file__)), "pilot")
for f in sorted(glob.glob(os.path.join(here, "*_L?.json"))):
    d = json.load(open(f)); n = d.get("native", {}); mt = d["muladds_per_target"]; tag = os.path.basename(f)[:-5]
    tried = n.get("pkm_lift_targets_tried")
    print("%-8s done %4d hits %d calls/target %5.1f muladds/target %.3e lifts_tried/target %s setup_ops %s -> N=%d" % (
        tag, d["targets_done"], d["hits"], d["f4_calls"] / d["targets_done"], mt, "-" if tried is None else "%.2f" % (tried / d["targets_done"]),
        d.get("setup_group_ops"), min(20000, int(4e11 / mt))))
