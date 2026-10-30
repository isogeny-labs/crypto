#!/usr/bin/env python3
"""Markdown tables for README.md, printed from results.json (no hand transcription)."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from analyse import root_call_gae, it_mults

rows = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "results.json")))
BASE = {41: "K12 norm16", 53: "K20 norm16", 61: "K18 norm32", 73: "K20 norm32", 83: "K600 (recorded, a=1)"}


def find(n, recipe):
    return next((r for r in rows if r["n"] == n and r["recipe"] == recipe), None)


def g(r, t):
    return r["bands"][t]["G"]


print("### Table 1: where d goes (per target, mean of the run's targets)\n")
print("| n | recipe | targets | root calls | probes (mean / median / max) | relation checks | PDP ms | relation-check ms | log combination ms | recovery check ms | PDP share |")
print("|--:|:--|--:|--:|:--|--:|--:|--:|--:|--:|--:|")
for r in rows:
    p = r["d_profile"]
    print(f"| {r['n']} | {r['recipe']} | {r['targets']} | {p['root_calls']:,.0f} | {p['probes']:,.0f} / {p['probes_median']:,.0f} / {p['probes_max']:,.0f} | {p['relation_checks']:.2f} | "
          f"{p['target_pdp_ms']:.3f} | {p['target_relation_check_ms']:.4f} | {p['target_descent_ms']:.5f} | {p['target_recovery_check_ms']:.4f} | {p['target_pdp_ms'] / p['target_ms']:.1%} |")

print("\n### Table 2: counted S0 and d, every run (GAE; root call priced in field ops; w = 0 unless named)\n")
print("| n | recipe | root call [GAE] | build root calls | rank root calls | S0 | d | S0*d/N' | sigma*delta | G, w=0 [95%] | G, B w=0.1 | G, B w=1 | G, A w=1 | G, root call = 1 GAE (IC-favourable) | root-call price for G=1 at A w=1 |")
print("|--:|:--|--:|--:|--:|--:|--:|--:|--:|:--|--:|--:|--:|--:|--:|")
for r in rows:
    b, p = r["bands"], r["parts"]
    print(f"| {r['n']} | {r['recipe']} | {r['root_call_gae']:.2f} | {p['build_root_calls']:.3g} | {p['rank_root_calls']:.3g} | {b['B0.0']['S0']:.3g} | {b['B0.0']['d']:.3g} | "
          f"{b['B0.0']['S0d_over_Nprime']:.2f} | {b['B0.0']['sigma_delta']:.2f} | {g(r, 'A0.0'):.2f} [{b['A0.0']['G_lo']:.2f}, {b['A0.0']['G_hi']:.2f}] | {g(r, 'B0.1'):.2f} | {g(r, 'B1.0'):.2f} | {g(r, 'A1.0'):.2f} | {r['unit_lb']['G']:.2f} | {r['root_call_gae_for_G1_at_A1']:.2f} |")

print("\n### Table 3: reductions of S0*d against the tuned KIC_NORM recipe (same producer, same unit, w = 0)\n")
print("| n | baseline (cold-tuned) S0*d/N' | retuned K, plain build | pair symmetry, same K | retuned K + pair symmetry | + batched root calls (count projection) | total reduction | G: baseline -> best measured -> projected |")
print("|--:|--:|:--|:--|:--|:--|--:|:--|")
for n in sorted({r["n"] for r in rows if r["n"] < 83}):
    base = find(n, BASE[n])
    if base is None:
        continue
    plain = [r for r in rows if r["n"] == n and "sym" not in r["recipe"] and "norm0" not in r["recipe"]]
    syms = [r for r in rows if r["n"] == n and "sym" in r["recipe"]]
    bs = base["bands"]["B0.0"]["S0d_over_Nprime"]
    bp = min(plain, key=lambda r: r["bands"]["B0.0"]["S0d_over_Nprime"])
    same = find(n, BASE[n] + " sym")
    if not syms:
        print(f"| {n} | {bs:.2f} | {bp['recipe']}: {bp['bands']['B0.0']['S0d_over_Nprime']:.2f} | n/a | n/a | n/a | n/a | {g(base, 'A0.0'):.2f} |")
        continue
    bb = min(syms, key=lambda r: r["bands"]["B0.0"]["S0d_over_Nprime"])
    v = bb["bands"]["B0.0"]["S0d_over_Nprime"]
    cr = root_call_gae(n)
    crb = (10.5 + it_mults(n) / (2 * n)) / 5.0
    proj = v * (crb / cr) ** 2
    gproj = g(bb, "A0.0") * (crb / cr)
    same_s = f"{same['bands']['B0.0']['S0d_over_Nprime']:.2f} ({bs / same['bands']['B0.0']['S0d_over_Nprime']:.2f}x)" if same else "n/a"
    print(f"| {n} | {bs:.2f} | {bp['recipe']}: {bp['bands']['B0.0']['S0d_over_Nprime']:.2f} ({bs / bp['bands']['B0.0']['S0d_over_Nprime']:.2f}x) | {same_s} | "
          f"{bb['recipe']}: {v:.2f} ({bs / v:.2f}x) | {proj:.2f} ({bs / proj:.2f}x, root call {cr:.2f} -> {crb:.2f} GAE) | {bs / proj:.1f}x | {g(base, 'A0.0'):.2f} -> {g(bb, 'A0.0'):.2f} -> {gproj:.2f} |")

print("\n### Table 4: single-target (k = 1) cold cost S = S0 + d, GAE, w = 0 and B w = 1\n")
print("| n | baseline recipe | baseline S | best measured recipe | best S | gain, w = 0 | gain, B w = 1 |")
print("|--:|:--|--:|:--|--:|--:|--:|")
for n in sorted({r["n"] for r in rows if r["n"] < 83}):
    base = find(n, BASE[n])
    if base is None:
        continue
    cand = [r for r in rows if r["n"] == n]
    best = min(cand, key=lambda r: r["bands"]["B0.0"]["cold_S"])
    b0, b1 = base["bands"]["B0.0"]["cold_S"], base["bands"]["B1.0"]["cold_S"]
    print(f"| {n} | {base['recipe']} | {b0:.3g} | {best['recipe']} | {best['bands']['B0.0']['cold_S']:.3g} | {b0 / best['bands']['B0.0']['cold_S']:.2f}x | {b1 / best['bands']['B1.0']['cold_S']:.2f}x |")
