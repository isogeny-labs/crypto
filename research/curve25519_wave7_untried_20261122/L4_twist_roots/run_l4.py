#!/usr/bin/env python3
"""L4 run script: exact count of F_p roots of S3(x1, x2, X) per pair class.

  python3 run_l4.py exhaustive --k 16 --out DIR [--rows N]   all unordered pairs x1 <= x2
  python3 run_l4.py sample     --k 20 --n 20000000 --out DIR  seeded stratified sample
  python3 run_l4.py sample     --k 16 --n 100000 --out DIR    (smoke)

Single core.  Memory: tables for p = 1048573 are about 40 MB; chunks of 2e6 pairs about 400 MB.
Output: DIR/<mode>_k<k>[_<variant>].json  with histograms, exception lists, curve facts.
"""
import argparse, json, os, sys, time
import numpy as np
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import l4core as L

CAP = 20000   # max records kept per exception list (counts are always exact)


class Acc:
    def __init__(self):
        self.h = np.zeros(L.KEYSIZE, dtype=np.int64)
        self.et_roots = []       # ET pairs (no Z) with >=1 F_p root
        self.ee_bad = []         # EE / TT pairs, no Z, x1 != x2, with nroots != 2
        self.counts = dict(et_roots=0, ee_bad=0, same_class_wrong_curve_root=0)
        self.wrong = []          # EE pair with a root on twist, TT pair with a root on E
        self.npairs = 0

    def add(self, x1, x2, r):
        self.npairs += len(x1)
        self.h += np.bincount(L.hist_key(r), minlength=L.KEYSIZE)
        pc, n = r["pc"], r["nroots"]
        m = (pc == 5) & (n > 0)
        if m.any():
            self.counts["et_roots"] += int(m.sum())
            for i in np.nonzero(m)[0]:
                if len(self.et_roots) < CAP:
                    self.et_roots.append([int(x1[i]), int(x2[i]), int(r["X1"][i]), int(r["X2"][i])])
        same = ((pc == 4) | (pc == 8)) & (~r["degen"]) & (n != 2)
        if same.any():
            self.counts["ee_bad"] += int(same.sum())
            for i in np.nonzero(same)[0]:
                if len(self.ee_bad) < CAP:
                    self.ee_bad.append([int(x1[i]), int(x2[i]), int(pc[i]), int(n[i])])
        wr = ((pc == 4) & (r["nT"] > 0)) | ((pc == 8) & (r["nE"] > 0))
        if wr.any():
            self.counts["same_class_wrong_curve_root"] += int(wr.sum())
            for i in np.nonzero(wr)[0]:
                if len(self.wrong) < CAP:
                    self.wrong.append([int(x1[i]), int(x2[i]), int(pc[i])])


def run_exhaustive(C, variant, rows):
    p = C.p
    acc = Acc()
    allx = np.arange(p, dtype=np.int64)
    last = p if rows is None else min(rows, p)
    t0 = time.time()
    for i in range(last):
        x2 = allx[i:]
        x1 = np.full(x2.shape, i, dtype=np.int64)
        r = L.roots(C, x1, x2, variant=variant, want_roots=True)
        acc.add(x1, x2, r)
        if i % 5000 == 0:
            print(f"  row {i}/{last} {time.time()-t0:.1f}s", file=sys.stderr, flush=True)
    return acc, last


def run_sample(C, variant, n, chunk=2_000_000, tag="main"):
    """n pairs per stratum; strata EE, ET, TT sampled uniformly (with replacement) from the class
    lists; EZ, TZ, ZZ strata are enumerated exhaustively (Z is at most 3 x-values)."""
    acc = Acc()
    lists = {"E": C.Ex, "T": C.Tx}
    plans = [("EE", "E", "E"), ("ET", "E", "T"), ("TT", "T", "T")]
    for name, ca, cb in plans:
        done, ci = 0, 0
        while done < n:
            m = min(chunk, n - done)
            rng = np.random.default_rng(L.derived_seed("L4", C.k, tag, name, ci))
            x1 = lists[ca][rng.integers(0, len(lists[ca]), m)].astype(np.int64)
            x2 = lists[cb][rng.integers(0, len(lists[cb]), m)].astype(np.int64)
            r = L.roots(C, x1, x2, variant=variant, want_roots=True)
            acc.add(x1, x2, r)
            done += m
            ci += 1
    # Z strata exhaustively: every z in Z against every x in F_p
    allx = np.arange(C.p, dtype=np.int64)
    for z in C.Zx:
        x1 = np.full(allx.shape, int(z), dtype=np.int64)
        r = L.roots(C, x1, allx, variant=variant, want_roots=True)
        acc.add(x1, allx, r)
    return acc


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("mode", choices=["exhaustive", "sample"])
    ap.add_argument("--k", type=int, required=True)
    ap.add_argument("--variant", default="std", choices=["std", "given"])
    ap.add_argument("--n", type=int, default=20_000_000, help="pairs per stratum (sample)")
    ap.add_argument("--rows", type=int, default=None, help="limit rows (smoke only)")
    ap.add_argument("--tag", default="main")
    ap.add_argument("--out", required=True)
    a = ap.parse_args()
    os.makedirs(a.out, exist_ok=True)
    t0 = time.time()
    C = L.Curve(a.k)
    facts = C.facts()
    if a.mode == "exhaustive":
        acc, rows = run_exhaustive(C, a.variant, a.rows)
        extra = dict(rows_done=rows, complete=(rows == C.p))
    else:
        acc = run_sample(C, a.variant, a.n, tag=a.tag)
        extra = dict(n_per_stratum=a.n, tag=a.tag)
    res = dict(mode=a.mode, k=a.k, variant=a.variant, curve_facts=facts, npairs=acc.npairs,
               cpu_seconds=time.time() - t0, hist=acc.h.tolist(), hist_rows=L.decode_hist(acc.h),
               exception_counts=acc.counts, et_with_root=acc.et_roots, same_class_bad=acc.ee_bad,
               same_class_wrong_curve=acc.wrong, **extra)
    fn = os.path.join(a.out, f"{a.mode}_k{a.k}_{a.variant}_{a.tag}.json")
    with open(fn, "w") as f:
        json.dump(res, f)
    print(fn, f"{res['cpu_seconds']:.1f}s", acc.counts)


if __name__ == "__main__":
    main()
