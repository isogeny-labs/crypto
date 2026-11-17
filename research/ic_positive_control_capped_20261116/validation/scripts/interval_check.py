#!/usr/bin/env python3
"""Claimed 95% intervals (transcribed from RESULTS.md) against the validator's own
two-stage bootstrap (4000 resamples, different RNG): point estimates must match
exactly; intervals are reported side by side (Monte Carlo noise expected).
Usage: interval_check.py OUTDIR"""
import json, os, sys
O = sys.argv[1]
L = json.load(open(os.path.join(O, "lucas_recompute.json")))
E = {e["p"]: e for e in L["per_p"]}
rows = []
def add(study, label, claimed, own):
    lo, hi = own[1], own[2]
    rows.append((study, label, claimed, (round(own[0], 3), round(lo, 3), round(hi, 3)),
                 "same side of 1" if ((claimed[1] > 1) == (lo > 1) and (claimed[2] < 1) == (hi < 1)) else "DIFFERENT side of 1",
                 round(max(abs(claimed[1] - lo) / max(lo, 1e-9), abs(claimed[2] - hi) / max(hi, 1e-9)), 3)))
for p, lc, mc in [(1031, (0.479, 0.466, 0.491), (0.223, 0.217, 0.229)), (4099, (0.467, 0.460, 0.473), (0.210, 0.207, 0.214)),
                  (16411, (0.462, 0.457, 0.468), (0.202, 0.200, 0.205)), (32771, (0.454, 0.450, 0.457), (0.196, 0.194, 0.197)),
                  (65537, (0.490, 0.487, 0.494), (0.490, 0.487, 0.494))]:
    add("lucas", f"L/C rel p={p}", lc, E[p]["L/C rel_mul (ratio of mean totals)"])
    add("lucas", f"M/C rel p={p}", mc, E[p]["M/C rel_mul (ratio of mean totals)"])
for p, c01, m01, m1 in [(4099, (10.43, 5.09, 30.84), (5.10, 2.48, 15.11), (26.82, 13.04, 79.90)),
                        (16411, (7.54, 4.41, 15.15), (3.69, 2.15, 7.41), (19.54, 11.41, 39.25)),
                        (32771, (7.44, 4.21, 14.33), (3.69, 2.08, 7.12), (19.57, 11.02, 37.84)),
                        (65537, (5.82, 3.84, 9.36), (3.97, 2.62, 6.38), (21.30, 14.06, 34.27))]:
    add("lucas", f"C/R w0.1 p={p}", c01, E[p]["C/R w=0.1"])
    add("lucas", f"M/R w0.1 p={p}", m01, E[p]["M/R w=0.1"])
    add("lucas", f"M/R w1 p={p}", m1, E[p]["M/R w=1.0"])
A = json.load(open(os.path.join(O, "la_recompute.json")))
E = {e["p"]: e for e in A["per_p"]}
for p, nm, sm, sa in [(4099, (0.669, 0.647, 0.694), (0.323, 0.307, 0.337), (0.405, 0.388, 0.423)),
                      (16411, (0.667, 0.661, 0.674), (0.326, 0.323, 0.329), (0.394, 0.390, 0.399)),
                      (32771, (0.667, 0.662, 0.673), (0.328, 0.325, 0.331), (0.388, 0.383, 0.392)),
                      (65537, (0.667, 0.664, 0.670), (0.329, 0.327, 0.330), (0.453, 0.451, 0.455))]:
    add("la", f"N/M LA p={p}", nm, E[p]["N/M la_mul"])
    add("la", f"S/M LA p={p}", sm, E[p]["S/M la_mul"])
    add("la", f"S/M native_all p={p}", sa, E[p]["S/M native_all"])
for p, s01, s1 in [(4099, (2.77, 1.35, 8.20), (14.0, 6.8, 41.6)), (16411, (1.97, 1.15, 3.96), (10.1, 5.9, 20.3)),
                   (32771, (1.92, 1.08, 3.71), (9.85, 5.55, 19.0)), (65537, (2.57, 1.70, 4.14), (13.6, 9.0, 21.9))]:
    add("la", f"S/R w0.1 p={p}", s01, E[p]["S/R w=0.1"])
    add("la", f"S/R w1 p={p}", s1, E[p]["S/R w=1.0"])
U = json.load(open(os.path.join(O, "unit_t_recompute.json")))
E = {e["p"]: e for e in U["per_p"]}
add("unit_t", "U/S LA p=8209", (0.778, 0.724, 0.936), E[8209]["U/S la_mul"])
add("unit_t", "U/S LA p=16411", (0.760, 0.750, 0.771), E[16411]["U/S la_mul"])
add("unit_t", "U/S LA p=32771", (0.772, 0.761, 0.782), E[32771]["U/S la_mul"])
add("unit_t", "U/S LA p=65537", (0.781, 0.774, 0.790), E[65537]["U/S la_mul"])
add("unit_t", "U/S LA p=131101", (0.791, 0.784, 0.797), E[131101]["U/S la_mul"])
add("unit_t", "V/S rel/trial p=8209", (0.770, 0.765, 0.776), E[8209]["V/S rel per trial"])
add("unit_t", "V/S rel/trial p=65537", (0.823, 0.822, 0.825), E[65537]["V/S rel per trial"])
add("unit_t", "V/R w0.1 p=8209", (4.81, 3.44, 6.80), E[8209]["V/R w=0.1"])
add("unit_t", "U/R w0.1 p=16411", (1.77, 1.03, 3.54), E[16411]["U/R w=0.1"])
add("unit_t", "U/R w0.1 p=32771", (1.72, 0.97, 3.32), E[32771]["U/R w=0.1"])
add("unit_t", "V/R w0.1 p=65537", (2.25, 1.49, 3.62), E[65537]["V/R w=0.1"])
add("unit_t", "U/R w0.1 p=131101", (1.78, 1.15, 4.52), E[131101]["U/R w=0.1"])
W = json.load(open(os.path.join(O, "capped_recompute.json")))
E = {e["p"]: e for e in W["per_p"]}
for p, t in [(4099, (0.898, 0.878, 0.917)), (8209, (0.786, 0.769, 0.819)), (16411, (0.898, 0.888, 0.907)),
             (32771, (0.895, 0.890, 0.901)), (65537, (0.821, 0.815, 0.827)), (131101, (0.902, 0.899, 0.906))]:
    add("capped", f"W/S w0.1 p={p}", t, E[p]["W/S w=0.1"])
for p, t in [(16411, (1.77, 1.03, 3.55)), (32771, (1.72, 0.95, 3.32)), (65537, (2.11, 1.39, 3.41)), (131101, (1.78, 1.15, 4.53))]:
    add("capped", f"W/R w0.1 p={p}", t, E[p]["W/R w=0.1"])
bad = [r for r in rows if r[4] != "same side of 1"]
json.dump(rows, open(os.path.join(O, "interval_check.json"), "w"), indent=1)
for r in rows:
    print(r)
print(len(rows), "intervals compared;", len(bad), "on a different side of 1;", "max relative endpoint gap", max(r[5] for r in rows))
