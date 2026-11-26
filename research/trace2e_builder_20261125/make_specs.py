#!/usr/bin/env python3
"""Specs for the native `binary-subspace select=trace2e` check (research/trace2e_builder_20261125).

Per curve (panel Koblitz; D frozen from workspace C1, docs/notes/binary-torsion-ic-20261125/ecbench/ in isogeny-labs/workspace):
  rho-frob, rho-neg                           references
  ic-sub{D_sub}                               binary-subspace baseline (C1's primary denominator)
  ic-t2e{D_cut}                               binary-subspace:dimension=D_cut,select=trace2e   (native, this PR)
  ic-cut{D_cut}                               explicit-x stand-in C1 measured (same abscissa set, built here)
  ic-sub{D_cut}                               matched-D baseline (when D_cut != D_sub)
Oracle mitm:m=2,negation_folded=1 for every IC arm. Fresh targets (seed 20261129 * curve id), 16 per curve, 2 rounds.
"""
import json, os
MOD = {17: 0x20009, 19: 0x80027, 23: 0x800021}
FROZEN = {"K1-17": (17, 1, 6, 6, 1701), "K0-19": (19, 0, 6, 7, 1900), "K1-19": (19, 1, 6, 7, 1901),
          "K0-23": (23, 0, 7, 8, 2300), "K1-23": (23, 1, 8, 8, 2301)}
def mul(a, b, n, m):
    r = 0
    while b:
        if b & 1: r ^= a
        b >>= 1; a <<= 1
        if a >> n & 1: a ^= m
    return r
def inv(a, n, m):
    r, e = 1, (1 << n) - 2
    while e:
        if e & 1: r = mul(r, a, n, m)
        a = mul(a, a, n, m); e >>= 1
    return r
def tr(a, n, m):
    t, y = 0, a
    for _ in range(n): t ^= y; y = mul(y, y, n, m)
    return t
def cut_list(n, a, d):
    m = MOD[n]
    on = lambda x: tr(x ^ a ^ inv(mul(x, x, n, m), n, m), n, m) == 0
    return [x for x in range(1, 1 << d) if on(x) and tr(x, n, m) == a]
def ic(fb):
    return {"id": "ic.pipeline", "params": {"factor_base": fb, "oracle": "mitm:m=2,negation_folded=1", "max_trials": "2000000000"}}
def spec(c):
    n, a, ds, dc, cid = FROZEN[c]; seed = int(f"20261129{cid}")
    arms = [{"name": "rho-frob", "role": "reference", "method": {"id": "rho.signed_frobenius"}},
            {"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}},
            {"name": f"ic-sub{ds}", "role": "baseline", "method": ic(f"binary-subspace:dimension={ds}")},
            {"name": f"ic-t2e{dc}", "role": "candidate", "method": ic(f"binary-subspace:dimension={dc},select=trace2e")},
            {"name": f"ic-cut{dc}", "role": "control", "method": ic("explicit-x:xs=" + ";".join(map(str, cut_list(n, a, dc))))}]
    if dc != ds:
        arms.append({"name": f"ic-sub{dc}", "role": "baseline", "method": ic(f"binary-subspace:dimension={dc}")})
    return {"schema": "ecbench.spec/v2",
            "curve_policy": {"mode": "diagnostic", "off_panel_reason": "panel Koblitz curves outside any phase lane: instrument check of the native binary-subspace select=trace2e factor base against its explicit-x stand-in and the C1 ratios (workspace PR #141); operation counts only"},
            "label": f"trace2e native builder check {c}",
            "description": "research/trace2e_builder_20261125/README.md: native select=trace2e vs the explicit-x stand-in (same abscissa set) and vs binary-subspace at C1's frozen D, fresh targets. Operation counts only.",
            "workloads": {"curves": [{"kind": "koblitz", "a": a, "n": n}], "targets_per_curve": 16,
                          "target_seed": seed, "target_kind": "public"},
            "arms": arms,
            "measurement": {"rounds": 2, "warmup": 1, "order": "alternate", "seed": seed + 1,
                            "isolation_required": "L0", "timeout_seconds": 1800}}
if __name__ == "__main__":
    here = os.path.dirname(os.path.abspath(__file__))
    for c in FROZEN:
        json.dump(spec(c), open(os.path.join(here, "specs", f"t2e-{c}.json"), "w"), indent=1)
        print(c, len(cut_list(*FROZEN[c][:2], FROZEN[c][3])), "cut abscissae")
