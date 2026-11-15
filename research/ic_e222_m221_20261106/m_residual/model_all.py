#!/usr/bin/env python3
"""Exact pair-table model for every instance of the large Montgomery / Edwards sample (unvalidated, pre-gate).
Model code is IMPORTED from ../h1_shortfall/h1_shortfall.py (fb_off, fb_on, pair_table, mul, ...); this script
only loops over a new instance set and records, per instance, the exact quantities
   P_off = |T_off cap <G>| / n,   P_on = |T_on| / n,   pred = P_off / P_on
plus the split of the usable off-table points into diagonal (+-2F_i) and off-diagonal ones.
Usage: model_all.py INST_DIR[,INST_DIR...] OUT.json"""
import importlib.util, os, sys, json, glob
from multiprocessing import Pool
HERE = os.path.dirname(os.path.abspath(__file__))
_argv = sys.argv; sys.argv = [_argv[0]]            # the imported module reads sys.argv[1] at import time
spec = importlib.util.spec_from_file_location("h1s", os.path.join(HERE, "..", "h1_shortfall", "h1_shortfall.py"))
h1s = importlib.util.module_from_spec(spec); spec.loader.exec_module(h1s)
sys.argv = _argv
M = 32

def one(path):
    d = json.load(open(path))
    p = int(d["field"]["modulus"]); a = int(d["a"]); b = int(d["b"]); n = int(d["subgroup_order"]); h = int(d["cofactor"])
    name = d["name"]; fam = d["family"]; rung = int(name[1:name.index("-")])
    F = h1s.fb_off(a, b, p, 1, M)
    Fp, s_inf, s_dup, scanned = h1s.fb_on(a, b, p, h, 1, M)
    Toff = h1s.pair_table(F, a, p)
    Ton = h1s.pair_table(Fp, a, p)
    inG = {pt for pt in Toff if h1s.mul(n, pt, a, p) is None}
    diag_tot = sum(1 for k in Toff.values() if k[0] == k[2]); diag_in = sum(1 for pt in inG if Toff[pt][0] == Toff[pt][2])
    on_ok = all(h1s.mul(n, pt, a, p) is None for pt in Ton)
    return dict(name=name, family=fam, rung=rung, r_bits=d["r_bits"], p=p, n=n, h=h, fb_off=len(F), fb_on=len(Fp),
                on_skipped_inf=s_inf, on_skipped_dup=s_dup, table_off=len(Toff), table_on=len(Ton), off_in_G=len(inG),
                off_in_G_diag=diag_in, off_in_G_offdiag=len(inG) - diag_in, table_off_diag=diag_tot, table_off_offdiag=len(Toff) - diag_tot,
                on_all_in_G=on_ok, P_off=len(inG) / n, P_on=len(Ton) / n, pred=len(inG) / len(Ton))

if __name__ == "__main__":
    files = []
    for dd in sys.argv[1].split(","):
        files += [f for f in sorted(glob.glob(dd + "/*.json")) if not f.endswith("MANIFEST.json")]
    with Pool(4) as pool:
        res = pool.map(one, files, chunksize=8)
    json.dump(res, open(sys.argv[2], "w"), indent=0)
    print(len(res), "instances; on-table all in <G>:", all(r["on_all_in_G"] for r in res))
