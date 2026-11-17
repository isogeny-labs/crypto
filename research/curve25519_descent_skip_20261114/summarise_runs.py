#!/usr/bin/env python3
"""Tabulate runs/*.json (f4cost output). Run: python3 -I summarise_runs.py > runs_summary.txt"""
import json, os
here = os.path.join(os.path.dirname(os.path.abspath(__file__)), "runs")
rows = [("a", "square-bits@414271 t=5", ("A0", "A1", "A2")), ("b", "square-bits@984071 t=4", ("A0", "A1", "A2")),
        ("c", "square-bits@0 t=5", ("A0", "A1", "A2"))]
print("arm  lifts skip  targets  hits  F4_calls  F4_muladds   muladds/target  lifts_tried  skipped  legendre  setup_group_ops")
for tag, name, arms in rows:
    print(name)
    base = None
    for a in arms:
        d = json.load(open(os.path.join(here, f"{tag}_{a}.json"))); n = d.get("native", {})
        mt = d["muladds_per_target"]; base = base or mt
        print("%-4s %5d %4s %8d %5d %9d %12d %16.0f (x%.2f) %6s %8s %9s %10s" % (
            a, d["lifts_mode"], d["descent_skip"], d["targets_done"], d["hits"], d["f4_calls"], d["f4_muladds"], mt, mt / base,
            n.get("pkm_lift_targets_tried", "-"), n.get("pkm_descent_skipped", 0), n.get("pkm_descent_legendre", "-"), d.get("setup_group_ops")))
print("control (post hoc): bits lo=0 t=5 G=3")
base = None
for L in (0, 1):
    d = json.load(open(os.path.join(here, f"ctl_bits_L{L}.json"))); n = d.get("native", {})
    mt = d["muladds_per_target"]; base = base or mt
    print("L%d   %5d %4s %8d %5d %9d %12d %16.0f (x%.2f) %6s %8s %9s %10s" % (
        L, d["lifts_mode"], d["descent_skip"], d["targets_done"], d["hits"], d["f4_calls"], d["f4_muladds"], mt, mt / base,
        n.get("pkm_lift_targets_tried", "-"), 0, "-", d.get("setup_group_ops")))
