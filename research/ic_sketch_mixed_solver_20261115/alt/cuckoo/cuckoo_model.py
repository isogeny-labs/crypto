#!/usr/bin/env python3
"""Cost-model translation for the cuckoo lane. Reuses (imports, does not edit) the xor-ribbon lane's
memory_model.py incumbent cost model: setup = F^2 GAE, trials = 0.68 F / h(F), c_t = 1.0479 GAE/trial.
Cuckoo parameters are MEASURED values from raw_output.txt (E1b/E3), all [pcc:unverified]:
  load 0.95 (200/200 builds), bits/key = (f+v)/0.95, neg lookup reads 2.0 buckets (priced w each),
  pos lookup 1.43 buckets, FPR = 8*0.95/(2^f-1) (measured 1.86e-3 at f=12; verified at 1 GAE per candidate)."""
import importlib.util, math, os
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("mm", os.path.join(here, "..", "xor-ribbon-build", "memory_model.py"))
mm = importlib.util.module_from_spec(spec); spec.loader.exec_module(mm)
old = mm.structs
LOAD = 0.95
def structs(F, p):
    d = old(F, p)
    v = mm.vbits(F)
    f = 12
    fpr = 8 * LOAD / (2**f - 1)
    d["cuckoo f=12 table (+value), load .95, 2 bucket-probes/neg lookup, verify 1 GAE/candidate"] = (
        (f + v) / LOAD, 2.0, lambda T, F, h: T * (h + fpr) * 1.0)
    d["cuckoo f=12 membership only, load .95, resolve by F adds/candidate"] = (
        f / LOAD, 2.0, lambda T, F, h: T * (h + fpr) * F)
    f = 16
    fpr = 8 * LOAD / (2**f - 1)
    d["cuckoo f=16 table (+value), load .95, verify 1 GAE/candidate"] = (
        (f + v) / LOAD, 2.0, lambda T, F, h: T * (h + fpr) * 1.0)
    return d
mm.structs = structs
if __name__ == "__main__":
    curves = [("p24", 16777199), ("p28", 268435399), ("p40", 2**40 - 87), ("p56", 2**56 - 5), ("p83", 2**83 - 1)]
    budgets = [("32KiB(L1)", 32 * 1024 * 8), ("1MiB(L2)", 2**20 * 8), ("32MiB(L3)", 32 * 2**20 * 8), ("16GiB", 16 * 2**30 * 8), ("1TiB", 2**40 * 8)]
    names = list(structs(100, 2**24).keys())
    ref = names[1]  # exact packed
    sel = [names[0], names[1], names[3]] + names[5:]
    for w in (0.0, 0.1, 1.0):
        print(f"\n##### w = {w} GAE per native probe #####")
        for cn, p in curves:
            unc = mm.solve(p, 1e30, ref, w)
            print(f"\n{cn}: unconstrained exact-packed optimum F*={unc[1]} total={mm.fmt(unc[0])} GAE")
            for bn, Mb in budgets:
                base = mm.solve(p, Mb, ref, w)
                row = []
                for nm in sel:
                    r = mm.solve(p, Mb, nm, w)
                    row.append("-" if r is None else f"F={r[1]} tot={mm.fmt(r[0])} ({base[0]/r[0]:.2f}x)")
                print(f"  {bn:10s}: " + " | ".join(f"[{i}] {x}" for i, x in enumerate(row)))
    print("\nstructures:"); [print(f"  [{i}] {names.index(n) and n or n}") for i, n in enumerate(sel)]
